#![no_std]
#![no_main]

#[macro_use]
extern crate teensy3;

use teensy3::bindings;
use teensy3::serial::Serial;
use teensy3::util::{pin_mode, digital_write, PinMode};

#[no_mangle]
pub unsafe extern fn main() {
    // Blink Loop
    pin_mode(13, PinMode::Output); // Set led pin to be output
    digital_write(13, false); // Set led off
    let ser = Serial{};
    let mut i = 0;

    loop {

        // Send a message over the USB Serial port
        // Print with println! wrapper macro, which just uses serial write on background
        println!("Hello! Count: {}", i);
        // Print over usb manually
        ser.write_bytes("Hello !\n".as_bytes()).unwrap_or_else(|_| {println!("Fail!"); ()});
        i += 1;
        // Show we are alive by blinking
        alive();

        // Keep 2 second pause in blinking the led, also don't spam the console
        bindings::delay(2000);
    }
}

/// Blink the light twice to know we're alive
pub unsafe fn alive() {
    // Blink led with custom wrapper
    for _ in 0..5 {
        digital_write(13, true);
        bindings::delay(50);
        digital_write(13, false);
        bindings::delay(50);
    }
    // Blink led with raw bindings
    for _ in 0..5 {
        bindings::digitalWrite(13, bindings::HIGH as u8);
        bindings::delay(50);
        bindings::digitalWrite(13, bindings::LOW as u8);
        bindings::delay(50);
    }
}


