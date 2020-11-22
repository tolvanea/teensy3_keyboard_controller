#![no_std]
#![no_main]

#[macro_use]
extern crate teensy3;

mod record_keyboard_matrix;
mod process_keys;


use heapless::{Vec, ArrayLength};  // fixed capacity `std::Vec`

use teensy3::util::{delay};
use teensy3::pins::{Pin, PinRow};
use teensy3::bindings as b;

use record_keyboard_matrix::figure_out_key_matrix;
use process_keys::{KeyMatrix, ScanRowState};

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

#[no_mangle]
pub extern fn main() {
    let mut pinrow = setup();
    let mut led = pinrow.get_led();
    for _ in 0..2 {
        alive(&mut led);
    }
    println!("Hellouu!");

    let key_codes: &[&[u32]] = &[
        &[b::KEY_ENTER, b::KEY_SPACE],
        &[b::KEY_Q, b::KEY_W, b::KEY_R],
        &[b::KEY_A, b::KEY_S, b::KEY_D],
    ];

    let key_names: &[&[&str]] = &[
        &["b::KEY_ENTER", "b::KEY_SPACE"],
        &["b::KEY_Q", "b::KEY_W", "b::KEY_R"],
        &["b::KEY_A", "b::KEY_S", "b::KEY_D"],
    ];

    let mut keymat: KeyMatrix = figure_out_key_matrix(
        &mut pinrow, key_codes, key_names
    );
    // let matrix: &[&[usize]] = [
    //     [       0,        0,        0, b::KEY_ENTER,        0,            0, ],
    //     [       0,        0,        0,            0,        0, b::KEY_SPACE, ],
    //     [       0,        0,        0,            0, b::KEY_Q,            0, ],
    //     [       0,        0,        0,            0, b::KEY_W,            0, ],
    //     [       0,        0,        0,            0, b::KEY_R,            0, ],
    //     [b::KEY_A, b::KEY_S, b::KEY_D,            0,        0,            0, ],
    // ];


    let mut keyboard = unsafe{b::Keyboard};
    for _ in 0..10000 {
        delay(30);
        let v = match keymat.scan_key_press() {
            Some(v) => v,
            None => {continue;} // Nothing is pressed
        };
        for state in v.into_iter() {
            match state {
                ScanRowState::NotPressed => continue,
                ScanRowState::Pressed(c) => {
                    if let Some(code) = c {
                        unsafe { keyboard.press(code as u16); }
                        delay(30);
                        unsafe { keyboard.release(code as u16); }
                    } else {
                        println!("Warning! Unknown key in matrix.");
                    }
                },
                ScanRowState::TooManyKeysPressed => {
                    println!("Uh oh! Multible keys pressed! Nothing is registered.");
                },
            }
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


