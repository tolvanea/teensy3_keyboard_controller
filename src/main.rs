#![no_std]
#![no_main]

#[macro_use]
extern crate teensy3;

use teensy3::serial::Serial;
use teensy3::util::{delay};
use teensy3::pins::{PinMode, Pin, PinRowSingleton};
use teensy3::bindings;

fn setup() -> PinRowSingleton {
    // It's unsafe because caller verifies that it's called only once
    unsafe{PinRowSingleton::new_once()}
}


#[no_mangle]
pub extern fn main() {
    let mut pinrow = setup();
    let mut led = pinrow.get_led();
    led.digital_write(false); // Set led off
    let ser = Serial{};
    let mut i = 0;

    // Blink Loop
    loop {
        // Send a message over the USB Serial port
        // Print with println! wrapper macro, which just uses serial write on background
        println!("Hello! Count: {}", i);
        // Print over usb manually
        ser.write_bytes("Hello !\n".as_bytes()).unwrap_or_else(|_| {println!("Fail!"); ()});
        i += 1;
        // Show we are alive by blinking
        alive(&mut led);

        // Keep 2 second pause in blinking the led, also don't spam the console
        delay(2000);
    }
}

/// Blink the light twice to know we're alive
pub fn alive(led: &mut Pin) {
    // Blink led with custom wrapper
    for i in 0..10 {
        led.digital_write(i%2 == 0);
        delay(50);
    }
    // Blink led with raw bindings
    // (Safe wrapper is more recommended because it keep book what pins are in what state)
    for _ in 0..5 {
        unsafe{bindings::digitalWrite(13, bindings::HIGH as u8)};
        unsafe{bindings::delay(50)};
        unsafe{bindings::digitalWrite(13, bindings::LOW as u8)};
        unsafe{bindings::delay(50)};
    }
}


