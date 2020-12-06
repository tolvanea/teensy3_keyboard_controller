# Keyboard controller with Teensy 3 and Rust
Make DIY usb-keyboard from raw keyboard pad that is extracted from laptop.

## Introduction
### What?
This project implements a keyboard controller with teensy 3 microcontroller. More precisely, this project turns raw keyboard pad into typical usb-keyboard. This keyboard pad may be obtained, for example, by extracting it from an old laptop, or buying it as a spare part. 

This repository contains code for keyboard controller, but this README also documents example of one hardware implementation.

### Why?
I like my laptop's keyboard so much that I want to use it on my home pc also. Thanks to this project, I can have external keyboard that has the exact same layout as my laptop. It is pleasure to have unified _typing feel_ both on travel and at home.

### How?
**Hardware required:**
* Teensy 3 microcontroller
    * I had Teensy 3.6.
* Raw keyboard pad 
    * I had spare part for Lenovo Thinkpad T480.
* Way to connect the keyboard into microcontroller's pins.
    * In my case that is flat cable connector PCB, which can be found from ebay.
 
**Software used:**
* Keyboard controller (content of this repository). The keyboard controller measures pin connections, translates them into key presses, and sends them over USB. This project implements it in Rust. 
* [Teensy C/C++ libraries](https://github.com/PaulStoffregen/cores). These libraries read microcontroller's pins, and they tell host pc that this is an USB keyboard.
* Bindings between Rust code and C/C++ libraries. The library [teensy3-rs](https://github.com/tolvanea/teensy3-rs) was made on the way for this purpose.

## Hardware
Hardware aspects was not really main point of this project. I'm a programmer, and the only thing that matters is that keyboard works reliably and that the code is pretty. The appearance of physical keyboard nor soldering beauty did not play any role. Clear? Now that questionable hardware aesthetics are sorted out of the way, here's finished product:

![My final product](documentation/finished_product.jpg)

It is build on top of plastic chopping board that was previously used in kitchen. Underneath the board lies Teensy microcontroller. I soldered flat cable adapter to its GPIO pins. Here's image of my solderings:

![Ugly!](documentation/not_like_this.jpg)

The copper wires are unprotected, because they were the only type of wire I had in hands at the build moment. I later submerged the whole thing in hot glue to prevent short circuits, which also made it more rigid and durable. The flat cable connector connects into keyboard. The flat cable in keyboard looks something like this:

![Flat cable](documentation/flat_cable.jpg)

[Credits of above image goes to ebay seller _20come12_.] 
The flat cable has about 30 lanes, which are directly connected into digital pins of microcontroller. When some key is pressed on a keyboard, then two of these lanes will be electrically connected. The microcontroller scans these lanes 100 times in a second, finds out what lanes are connected, and then translates them into corresponding key presses.

## Software
There are suprisingly many small things to consider when translating pin connections into key presses. A naive key detection would is quite straight forward to implement, but this keyboard controller goes further, and features a complete implementation with all the fine details sorted out. Most of these fine details are rare corner cases, and they will almost never play a real role in practice. For example, if three or more non-modifier keys are simultaneously pressed, then some non-pressed keys may be registered. Anyways, I'm happy to announce that this keyboard controller detects all the key presses it possibly can, and it never reports any invalid presses. As far as I know, it could not handle the job better, though I'm not counting potential bugs.

Below is listed few features of this project, and they are compared to one of the most well known DIY keyboard [controller template](https://github.com/thedalles77/USB_Laptop_Keyboard_Controller). Apples and oranges are compared here: This project is considerably more complex than the template, and also has three times more the code lines (~900 vs ~300). 

**Defining features of this project:**
* Very easy key configuration: Just press each key once through and the correct key-to-pin configuration is figured out for you. This configuration, (a.k.a. "key matrix",) is printed out, and it can be directly copy-pasted to source code. This feature makes the software generic for any keyboard possible. (The only "hard coding" it requires is copy-pasting automatically generated keyboard matrix to source code.) Comparing to the above mentioned [controller template](https://github.com/thedalles77/USB_Laptop_Keyboard_Controller), it does not have any key matrix generation feature, which is why each different keyboard model has its own custom source code fork.
* Quick responsiveness: Keys are sent over usb only when they have changed state. This greatly reduces lag by not flooding USB with unnecessary packets. Again, this a contrasting feature to the [controller template](https://github.com/thedalles77/USB_Laptop_Keyboard_Controller)
* As mentioned previously, this software goes in lengths to handle simultaneous key presses correctly. As comparison, the [controller template](https://github.com/thedalles77/USB_Laptop_Keyboard_Controller) may register non-existent ghost presses if many (non-modifier) keys are presses simultaneously. This may be expected from the simplicity of the template. However, noteworthy is that **this keyboard controller projects is even slightly more nuanced than the original keyboard controller made by Lenovo itself**. For example, Lenovo's keyboard controller built in my laptop can not register key presses like _F_ + _5_ + _F9_, but this keyboard controller can. If I was to guess why that is, Lenovo probably uses the exact same keyboard controller software for both keyboards with and without numpad. If there is no numpad, then there is also less possible pin connections, which makes some ambiguous combinations uniquely defined. (Even though no one would ever benefit from being able to use such combination, why leave capabilities on a table in first place?)
* It's worth mentioning that Fn and media keys are supported as is also in [controller template](https://github.com/thedalles77/USB_Laptop_Keyboard_Controller) also. This feature is explicitly mentioned because Fn key is not really a standard key, and requires some extra configuration. Oh, and by the way, automatic key matrix generation does not cover media keys, so they need to be hard coded by hand. One needs to only state that, for example, F2 key corresponds to volume decrease.

**Known downsides / missing features**
* Detection of complex key combinations requires some processing power. On teensy 3.6 it takes up to 100 microseconds, which is negligible within 10 millisecond refresh rate. However, if microcontroller would have only 1/100th of the perfomance, then this may arise a considerable problem. 
    * There is probably no way around this performance requirement if correct behaviour is aimed for. 
* No key backlight.
    * This would be fairly easy to implement, but it is not (yet) made.
* If the program happens to crash, the microcontroller does not restart itself, so it requires replugging the USB.
    * Automatic restarting should be fairly doable, but that becomes greater concern if the controller manages to crash itself.
* The Thinkpad keyboard has integrated mouse buttons and trackpoint, but they are disabled.
    * They would require another flat cable adapter, and _a lot of_ extra coding. No support planned for them.



## External Links
This project would not been possible without the following useful information sources:
* https://www.instructables.com/How-to-Make-a-USB-Laptop-Keyboard-Controller/
* https://github.com/thedalles77/USB_Laptop_Keyboard_Controller  
* https://github.com/jamesmunns/teensy3-rs-demo
* https://branan.github.io/teensy/2017/01/12/bootup.html


## License
Rust contributions are licensed under the MIT License.

**Please Note:** ASM, C, C++, and Linker Components of the `teensy3-sys` crate (a dependency of the `teensy3` crate) contain components licensed under the MIT License, PJRC's modified MIT License, and the LGPL v2.1. Please refer to individual components for more details.
