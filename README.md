# Teensy3 keyboard controller with Rust

This project is based on template [teensy3-rs-demo](https://github.com/tolvanea/teensy3-rs-demo).

The main aim here is to create usb keyboard by using teensy3 and laptop keyboard spare part. Teensy is model 3.6, and laptop keyboard is from Lenovo ThinkPad T480. Keyboard is connected to teensy with flat cable. When keys are pressed, they connect different lanes on flat cable, and teensy uses that information to send corresponding keys over USB port. 

This project would not been possible without following useful information sources:
* https://www.instructables.com/How-to-Make-a-USB-Laptop-Keyboard-Controller/
* https://github.com/thedalles77/USB_Laptop_Keyboard_Controller  
* https://github.com/jamesmunns/teensy3-rs-demo
* https://branan.github.io/teensy/2017/01/12/bootup.html


# License
Rust contributions are licensed under the MIT License.

**Please Note:** ASM, C, C++, and Linker Components of the `teensy3-sys` crate (a dependency of the `teensy3` crate) contain components licensed under the MIT License, PJRC's modified MIT License, and the LGPL v2.1. Please refer to individual components for more details.
