#![no_std]
#![no_main]

#![deny(unused_must_use)]
#![allow(clippy::needless_return)]

#[macro_use]
extern crate teensy3;

mod process_keys;
mod custom_key_codes;
mod record_keyboard_matrix;
pub use typenum::{U24 as MatrixCap};  // Maximum capacities

use heapless::{Vec, ArrayLength};  // fixed capacity `std::Vec`

use teensy3::util::{delay, MillisTimer};
use teensy3::pins::{Pin, PinRow};
use teensy3::bindings as b;
use b::usb_keyboard_class as KBoard;

use process_keys::{KeyCode, ExtraKeyInfo};
use core::convert::TryInto;

type ShortVec<T> = Vec<T, MatrixCap>;

/// Shorthand function to initialise new vector filled with some value
fn full_vec<T, U>(value: T, len: usize) -> Vec<T,U>
where T: Clone, U: ArrayLength<T>
{
    let mut a = Vec::<T, U>::new();
    a.resize(len, value).unwrap();
    return a;
}

/// Shorthand trait so that it is easy to check if iterator contains some value
/// # Examples
/// ```
/// assert!([1,2,3].iter().contains(&3));
/// assert!(![1,2,3].iter().contains(&4));
/// ```
pub trait Contains<T: PartialEq>: Iterator<Item=T> {
    fn contains(self, val: T) -> bool;
}
impl<I: Iterator<Item=T>, T: PartialEq> Contains<T> for I {
    fn contains(mut self, val: T) -> bool{
        self.any(|x| x == val)
    }
}

fn setup() -> PinRow {
    // It's unsafe because caller verifies that it's called only once
    unsafe{ PinRow::new_once()}
}

enum Key {
    Normal(u8),
    Modifier(u16),
    Fn,
}

fn extract_key_type(key_code: u32, info: &ExtraKeyInfo) -> Key {
    // Few examples from core/teensy3/keylayouts.h:
    // KEY_A: u32            =    4 | 0xF000;
    // MODIFIERKEY_CTRL: u32 = 0x01 | 0xE000;
    let bytes = key_code.to_le_bytes();
    let fn_mask: u8 = info.fn_key.to_le_bytes()[1];
    match bytes[1] {
        m if m == info.regular_key_mask => Key::Normal(bytes[0]),
        m if m == info.modifier_key_mask => Key::Modifier(key_code as u16),
        0xE2 => panic!("System keys not supported here."),
        0xE4 => panic!("Media keys not supported here."),
        m if m==fn_mask => Key::Fn,
        _ => panic!("Dafuq is that key?"),
    }
}

/// Categorize key presses to regular keys, modifier keys and Fn key. Also crop out those keys
/// that are unsure and has not been pressed on last time.
fn categorize_key_presses(
    scanned_keys: Option<ShortVec<KeyCode<u32>>>,
    key_slots: &[Option<u8>; 6],
    modifiers_pressed_old: u16,
    fn_pressed_old: bool,
    info: &ExtraKeyInfo,
) -> (ShortVec<KeyCode<u8>>, ShortVec<KeyCode<u16>>, bool) {
    let mut regular_keys: ShortVec<KeyCode<u8>> = Vec::new();
    let mut modifier_keys: ShortVec<KeyCode<u16>> = Vec::new();
    let mut fn_key: bool = false;
    let scanned_keys = match scanned_keys {
        Some(v) => v,
        None => return (regular_keys, modifier_keys, fn_key),
    };
    // Now something is pressed
    for state in scanned_keys.into_iter() {
        match state {
            KeyCode::Certain(code) => {
                // Some key is pressed without ambiguities
                match extract_key_type(code, info) {
                    Key::Normal(c) => {
                        regular_keys.push(KeyCode::Certain(c)).unwrap_or(());
                    },
                    Key::Modifier(c) => {
                        modifier_keys.push(KeyCode::Certain(c)).unwrap_or(())
                    },
                    Key::Fn => {
                        fn_key = true;
                    }
                }
            },
            KeyCode::Uncertain(code) => {
                // Now can not be sure whether or not key is really pressed. If key was
                // previously pressed, it is kept pressing, otherwise it is not registered.
                match extract_key_type(code, info) {
                    Key::Normal(c) => {
                        // Add only if key was pressed on previous round
                        if key_slots.iter().any(|s| s.filter(|s| *s == c).is_some()) {
                            regular_keys.push(KeyCode::Uncertain(c)).unwrap_or(());
                        }
                    },
                    Key::Modifier(c) => {
                        // Add only if modifier key was pressed on previous round
                        if modifiers_pressed_old == (modifiers_pressed_old | c) {
                            modifier_keys.push(KeyCode::Uncertain(c)).unwrap_or(())
                        }
                    },
                    Key::Fn => {
                        if fn_pressed_old {
                            fn_key = true;
                        }
                    },
                }
            },
        };
    }
    return (regular_keys, modifier_keys, fn_key)
}

/// Write pressed keys to 6 slots that are send over usb.
/// Performance info: about 3 microseconds (negligible)
fn update_slots(
    key_slots_prev: &[Option<u8>; 6],
    regular_keys: &ShortVec<KeyCode<u8>>
) -> [Option<u8>; 6] {
    let mut key_slots_new = *key_slots_prev;  // Copy
    // Remove released keys, i.e. keys that are in `key_slots` but not in `keys_pressed`.
    // If key press is uncertain, keep it in slots.
    key_slots_new.iter_mut()
        .filter(|s| s.filter(|s| !regular_keys.iter().any(|k| k.into_inner() == *s)).is_some())
        .for_each(|s| *s = None);
    // Add those keys of `keys_pressed` to `key_slots` that are not already there
    // Also, if key press is uncertain, do not add.
    for k in regular_keys.iter().filter_map(|x| x.into_option()) {
        // Skip keys that are already in `key_slots`
        if key_slots_new.iter().contains(&Some(k)) {
            continue
        }
        // add them to first free `None` spot
        for slot in key_slots_new.iter_mut() {
            if slot.is_none() {
                *slot = Some(k);
                break;
            }
        }
    }
    return key_slots_new;
}

fn send_modifier_keys(keyboard: &mut KBoard, modifier_slots: u16) {
    unsafe{ keyboard.set_modifier(modifier_slots); }
}
fn send_regular_keys(keyboard: &mut KBoard, key_slots: &[Option<u8>; 6]) {
    unsafe {
        keyboard.set_key1(key_slots[0].unwrap_or(0));
        keyboard.set_key2(key_slots[1].unwrap_or(0));
        keyboard.set_key3(key_slots[2].unwrap_or(0));
        keyboard.set_key4(key_slots[3].unwrap_or(0));
        keyboard.set_key5(key_slots[4].unwrap_or(0));
        keyboard.set_key6(key_slots[5].unwrap_or(0));
    }
}
/// Send Fn and media keys (volmue up and down)
/// Media keys does not have u8 key codes, so their presses must be emulated on higher level
fn send_media_keys(
    keyboard: &mut KBoard,
    key_slots: &[Option<u8>; 6],
    key_slots_prev: &[Option<u8>; 6],
    info: &ExtraKeyInfo
) {
    //keyboard.set_modifier(modifiers_pressed);
    let keys = key_slots.iter().filter_map(|k| *k);
    let keys_old = key_slots_prev.iter().filter_map(|k| *k);
    for k in keys.clone() {
        // If this key was not pressed on last time, prepare to press down corresponding media key
        if !keys_old.clone().contains(k) {
            for &(regular_key, media_key) in info.media_key_bindings.iter() {
                if regular_key == ((k as u32) | 0xF000) {
                    unsafe{ keyboard.press(media_key.try_into().unwrap()); }
                }
            }
        }
    }
    for k_old in keys_old {
        // If this key has disappeared from current list, prepare to release corresponding media key
        if !keys.clone().contains(k_old) {
            for &(regular_key, media_key) in info.media_key_bindings.iter() {
                if regular_key == ((k_old as u32) | 0xF000) {
                    unsafe{ keyboard.release(media_key.try_into().unwrap()); }
                }
            }
        }
    }
}

/// Pause so that keys are sent synchronously every `rescan_interval` milliseconds
fn wait(rescan_interval: u32, prev_loop: &mut MillisTimer) {
    let elapsed = prev_loop.elapsed();
    let sleep_time = if rescan_interval > elapsed {
        rescan_interval - elapsed
    } else {
        0
    };
    delay(sleep_time);
    *prev_loop = MillisTimer::new();
}

#[no_mangle]
pub extern fn main() {
    let mut pinrow = setup();
    let mut led = pinrow.get_led();
    for _ in 0..2 {
        alive(&mut led);
    }
    println!("Hellouu!");
    // loop {
    //     let (i,j) = record_keyboard_matrix::wait_for_key(&mut pinrow);
    //     println!("Recorded: {} {}", i, j);
    // }

    //let mut mat = custom_key_codes::ask_key_codes_and_print_them(&mut pinrow);
    let mut mat = custom_key_codes::get_stored_key_codes(&mut pinrow);

    // Key presses from previous cycle
    let mut key_slots_prev: [Option<u8>; 6] = [None; 6];
    let mut modifier_slots_prev: u16 = 0;
    let mut fn_key_prev: bool = false;

    // Note that due to GPIO pin settlement (sleep 1ms) best scan rate is about 10ms.
    let rescan_interval = 20;  // milliseconds
    let mut prev_loop = MillisTimer::new();

    let mut keyboard = unsafe{b::Keyboard};
    for _ in 0..10000 {
        wait(rescan_interval, &mut prev_loop);

        let scan = mat.scan_key_press();
        let (regular_keys, modifier_keys, fn_key) = categorize_key_presses(
            scan,
            &key_slots_prev,
            modifier_slots_prev,
            fn_key_prev,
            &mat.info
        );

        let key_slots = update_slots(&key_slots_prev, &regular_keys);
        let modifier_slots = modifier_keys.iter().fold(0, |acc, k| k.into_inner() | acc);

        if !fn_key {
            send_regular_keys(&mut keyboard, &key_slots);
        } else {
            send_media_keys(&mut keyboard, &key_slots, &key_slots_prev, &mat.info);
        }
        send_modifier_keys(&mut keyboard, modifier_slots);

        unsafe {
            keyboard.send_now();
        }

        key_slots_prev = key_slots;
        modifier_slots_prev = modifier_slots;
        fn_key_prev = fn_key;
    }


    led.digital_write(false); // Set led off

    // Blink Loop
    for i in 0.. {
        println!("{}", i);
        // Show we are alive by blinking
        alive(&mut led);
        // Keep 2 second pause in blinking the led, also don't spam the console
        delay(1000);
    }
}

/// Blink the light twice to know we're alive
pub fn alive(led: &mut Pin) {
    // Blink led with custom wrapper
    for i in 0..6 {
        led.digital_write(i%2 == 0);
        delay(50);
    }
    delay(200)
}


