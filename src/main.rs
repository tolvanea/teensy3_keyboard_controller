#![no_std]
#![no_main]

#![deny(unused_must_use)]
#![allow(clippy::needless_return)]

#[macro_use]
extern crate teensy3;

mod process_keys;
mod custom_key_codes;
mod record_keyboard_matrix;


use heapless::{Vec, ArrayLength};  // fixed capacity `std::Vec`

use teensy3::util::{delay};
use teensy3::pins::{Pin, PinRow};
use teensy3::bindings as b;

use process_keys::{KeyCode, MatrixCap};

/// Initialise vector filled with some value
fn full_vec<T, U>(value: T, len: usize) -> Vec<T,U>
where T: Clone, U: ArrayLength<T>
{
    let mut a = Vec::<T, U>::new();
    a.resize(len, value).unwrap();
    return a;
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

fn extract_key_type(key_code: u32) -> Key {
    // Few examples from core/teensy3/keylayouts.h:
    // KEY_A: u32            =    4 | 0xF000;
    // MODIFIERKEY_CTRL: u32 = 0x01 | 0xE000;
    let bytes = key_code.to_le_bytes();
    const FN_MASK: u8 = custom_key_codes::MODIFIERKEY_FN.to_le_bytes()[1];
    match bytes[1] {
        0xF0 => Key::Normal(bytes[0]),
        0xE0 => Key::Modifier(u16::from_le_bytes([bytes[0], bytes[1]])),
        0xE2 => panic!("System keys not supported"),
        0xE4 => panic!("Media keys not supported"),
        FN_MASK => Key::Fn,
        _ => panic!("Dafuq is that key?"),
    }
}

#[no_mangle]
pub extern fn main() {
    let mut pinrow = setup();
    let mut led = pinrow.get_led();
    for _ in 0..2 {
        alive(&mut led);
    }
    println!("Hellouu!");
    //let mut mat = custom_key_codes::ask_key_codes_and_print_them(&mut pinrow);
    let mut mat = custom_key_codes::get_stored_key_codes(&mut pinrow);

    // Key presses from previous cycle
    let mut key_states: [Option<u8>; 6] = [None; 6];
    let mut modifiers_pressed_old: u16 = 0;
    let mut fn_pressed_old: bool = false;



    use b::usb_keyboard_class as KB;
    let mut keyboard = unsafe{b::Keyboard};
    for _ in 0..10000 {
        delay(20);
        let mut keys_pressed: Vec<KeyCode<u8>, MatrixCap> = Vec::new();
        let mut modifiers_pressed: u16 = 0;
        let mut fn_pressed: bool = false;
        // if something is pressed
        if let Some(v) = mat.scan_key_press() {
            for state in v.into_iter() {
                match state {
                    KeyCode::Certain(code) => {
                        // Some key is pressed without ambiguities
                        match extract_key_type(code) {
                            Key::Normal(c) => {
                                keys_pressed.push(KeyCode::Certain(c)).unwrap_or(());
                            },
                            Key::Modifier(c) => {
                                modifiers_pressed |= c;
                            },
                            Key::Fn => {
                                fn_pressed = true;
                            }
                        }
                    },
                    KeyCode::Uncertain(code) => {
                        // Not sure whether or not key is really pressed. If that key was
                        // previously pressed, keep pressing, otherwise do not register.
                        match extract_key_type(code) {
                            Key::Normal(c) => {
                                keys_pressed.push(KeyCode::Uncertain(c)).unwrap_or(());
                            },
                            Key::Modifier(c) => {
                                // If modifier key was pressed on previous round
                                if modifiers_pressed_old == (modifiers_pressed_old | c) {
                                    modifiers_pressed |= c;
                                }
                            },
                            Key::Fn => {
                                if fn_pressed_old {
                                    fn_pressed = true;
                                }
                            },
                        }
                    },
                };

                //println!("Uh oh! Multible keys pressed! Nothing is registered.");
            }
        }

        modifiers_pressed_old = modifiers_pressed;
        fn_pressed_old = fn_pressed;

        fn send_modifier_keys(keyboard: &mut KB, modifiers_pressed: u16) {
            unsafe{ keyboard.set_modifier(modifiers_pressed); }
        }
        send_modifier_keys(&mut keyboard, modifiers_pressed);

        // // Send Fn and media keys
        // fn send_media_keys(keyboard: &mut KB, keys_pressed: &Vec<u8, MatrixCap>, fn_pressed: bool) {
        //     //keyboard.set_modifier(modifiers_pressed);
        // }
        // send_media_keys(&mut keyboard, &keys_pressed, fn_pressed);

        fn send_normal_keys(
            keyboard: &mut KB,
            key_slots: &mut [Option<u8>; 6],
            keys_pressed: &Vec<KeyCode<u8>, MatrixCap>)
        {
            // Remove released keys, i.e. keys that are in `key_slots` but not in `keys_pressed`.
            // If key press is uncertain, keep it in slots.
            key_slots.iter_mut()
                .filter(|s| s.filter(|s| !keys_pressed.iter().any(|k| k.to_code() == *s)).is_some())
                .for_each(|s| *s = None);
            // Add those keys of `keys_pressed` to `key_slots` that are not already there
            // Also, if key press is uncertain, do not add.
            for k in keys_pressed.iter().filter_map(|x| x.to_option()) {
                // Skip keys that are already in `key_slots`
                if key_slots.iter().any(|s| *s == Some(k)) {
                    continue
                }
                // add them to first free `None` spot
                for slot in key_slots.iter_mut() {
                    if slot.is_none() {
                        *slot = Some(k);
                        break;
                    }
                }
            }
            unsafe {
                keyboard.set_key1(key_slots[0].unwrap_or(0));
                keyboard.set_key2(key_slots[1].unwrap_or(0));
                keyboard.set_key3(key_slots[2].unwrap_or(0));
                keyboard.set_key4(key_slots[3].unwrap_or(0));
                keyboard.set_key5(key_slots[4].unwrap_or(0));
                keyboard.set_key6(key_slots[5].unwrap_or(0));
            }
        }
        send_normal_keys(&mut keyboard, &mut key_states, &keys_pressed);

        println!("{:?}, {:?}", key_states, keys_pressed);

        unsafe {
            keyboard.send_now();
        }
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


