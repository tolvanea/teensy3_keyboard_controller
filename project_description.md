# Keyboard controller project description
This file lists project criteria for course COMP.530-01 Bare Metal Rust. 

1. **The general idea of the application.**
    * Make usb keyboard from laptop's keyboard part.
    * This project requires:
        * Microcontroller model that haves USB HID libraries
        * Keyboard spare part from laptop
        * Flat cable connector board, which connects to GPIO pins to keyboard

2. **How many primary participants is reasonable for the project? Can be a span, the final number of participants does not need to be an exact match, and a single consultant does not count towards the number of participants.**
    * From one to three participants, depending what devices people use. The amount of work varies a lot depending on how good library support does microcontroller have. 
        * Big bulk of work is just getting libraries to work with rust. So if all members have different board, then there can be more participants. But if board happens to have fairly ok library support, then this can be one person project.
        * Also, different keyboards have different connections, so each person has to figure out how pins in flat cable correspond to keys in keyboard.
        * Keyboard firmware can be shared between participants.
    * USB library considerations
        * Using existing rust usb-libraries is easy. However, currently only two microcontrollers have this support. (One of them is STM32 bluepill.)
        * Generating bindings to c/c++ is harder, but should be valid option for any microcontroller. (e.g. for Teensy and Longan nano)
        * Modifying existing rust usb-libraries to add device support is probably outside the scope of this course. (BTW, Longan nano is very similar to STM32 bluepill)
    
3. **Tentative list of features per credit (1 cr = 27 hours / person), and a tentative span of study credits per participant per list of features (1-3 cr per student). The point is to give an idea of how many study credits a person can earn by participating in this project. This requirement is set by university policy.**
    * USB-libraries, choose one:
        * 0cr: Using existing rust library
        * 1cr: Generating bindings to existing c/c++ library. It would be good to publish this.
        * 3-5cr: Modifying existing rust library to add compatibility for own device. It is mandatory to publish it if this is achieved.
    * 0cr: connecting keyboard to microcontroller
    * 1-2cr: Writing keyboard firmware
      
4. **A brief assessment of project difficulty.**
    * Depends on library support, could be easy, could be hard. I expect it to be from easy to medium:

5. **Availability and location of related physical devices**
    * Microcontroller: buy online
    * Keyboard: Disassemble from laptop or buy online
    * Flat cable connector: buy online
        * This is hardest to obtain, as it need to be bought from china.
        * For example: https://www.ebay.com/itm/1pcs-0-5mm-1-0mm-to-DIP-2-54-FPC-FFC-Flat-Flexible-Cable-Adapter-Board-Connector/264384466999?hash=item3d8e8ad037:g:zCsAAOSwspddGxpt
        * Some development boards may have already installed flat cable connector, for example: https://wiki.seeedstudio.com/SeeedStudio-GD32-RISC-V-Dev-Board/
