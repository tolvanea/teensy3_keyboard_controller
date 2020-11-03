# Teensy with Rust

This is an example project that uses bindings from [teensy3-rs](https://github.com/tolvanea/teensy3-rs) crate. This aims to be dead simple template to get teensy project running. It is intended to be forked or copied into a new project.

## About in this fork
This fork is based on James Munns' [teensy3-rs-demo](https://github.com/jamesmunns/teensy3-rs-demo). Main modifications are:
* Getting it to work on my machine
* Change Cargo to [Cross](https://github.com/rust-embedded/cross), and so compilation is done in Docker container
* Support for teensy 3.0-3.6
* Reduce need for device specific configuration: Teensy model is only needed to be specified once in Makefile.
* Place teensy3-rs crate in same directory as a sub module, so that library build process is easy to tweak
* For other modification notes see [teensy3-rs](https://github.com/tolvanea/teensy3-rs)

## About this project
This project uses [Cross](https://github.com/rust-embedded/cross) instead of Cargo, and compilation is done in Docker container. Docker container is useful in this situation, because binding generation and cross compilation is very sensitive to installed dependencies and libraries. Different version numbering of gcc may cause failing compilation, which docker solves. 

However, optionally, Cargo can be also used, if dependencies found in `Dockerfile` are installed on system by using system's package manager. Makefile can be configured to use Cargo instead of Cross. 

## Installations
Install [teensy-loader-cli](https://www.pjrc.com/teensy/loader_cli.html), which will be used to flash hex-files on teensy. Also install objcopy tool, which transforms compiled binary to hex-file. Installation on ubuntu is:
```
sudo apt-get install teensy-loader-cli binutils-arm-none-eabi
```
Install [Cross](https://github.com/rust-embedded/cross):
```
cargo install cross
```

Install Docker. On ubuntu it can be done by following "Step 1" in [this link](https://www.digitalocean.com/community/tutorials/how-to-install-and-use-docker-on-ubuntu-20-04).

By default docker usage requires super user rights. Allow current user to use docker, so Cross and Docker need not to be called with `sudo` every time:
```
sudo usermod -aG docker ${USER}
```
However, the above command takes effect only after logging out and in. To avoid relogging, it can be temporarily fixed with 
```
su - $USER                                           
```
which fixes need to use `sudo` only for that one terminal window. This command resets current working directory, so there is need to navigate back to the teensy3-rs-demo directory.

## Setting up
Clone this repository with command
```
git clone --recurse-submodules https://github.com/tolvanea/teensy3-rs-demo
```
Flag `--recurse-submodules` is needed, because all git submodules need to be downloaded too.

Then build docker image described in `Dockerfile` with
```
docker build -t teensy3/cross:tag .
``` 
This docker image will be used by cross. All available docker images on system can be listed with
```
docker images
```

Then, teensy model should be specified. Uncomment the line corresponding your teensy model in `Makefile`. For example:
```
#MODEL=TEENSY32
#MODEL=TEENSY35
MODEL=TEENSY36
```
No other device specific configuration is needed. Everything else is automated.

## Compiling, flashing and running
Build project with Makefile
```
make
```
Plug teensy in with usb cable and flash it
```
make flash
```
You may need to press button on board to finish the flash.
And that's it! You should see quickly blinking led going on and of.
Linux users can read output from teensy by running following shell script:
```
./read_output_from_usb
``` 


## Other makefile usage
Debug build with:
```
make debug
```
Generate documentation for current project and bindings with:
```
make doc
```
Documentantation can be then opened in browser by navigating to file
```target/thumbv7em-none-eabi/doc/teensy3_sys/index.html```



## Safe Components

Items used from the `teensy3` crate directly can be used as safe rust code. In this function, notice how there is no `unsafe` marker:

```rust
extern crate teensy3;
use teensy3::serial::Serial;

// ...

/// Send a message over the USB Serial port
pub fn hello(ser: &Serial) -> Result<(),()> {
    let msg = "Hello Teensy Rusty World!\n\r";
    ser.write_bytes(msg.as_bytes())
}
```

Items used from the `teensy3::bindings` module are NOT marked as safe (because they are direct C++ code mappings). These require an `unsafe` mark at either the function or block level:

```rust
extern crate teensy3;
use teensy3::bindings;

// ...

/// Blink the light twice to know we're alive
pub unsafe fn alive() {
    for _ in 0..2 {
        bindings::digitalWrite(13, bindings::LOW as u8);
        bindings::delay(200);
        bindings::digitalWrite(13, bindings::HIGH as u8);
        bindings::delay(200);
        bindings::digitalWrite(13, bindings::LOW as u8);
        bindings::delay(200);
    }
}
```

# License

Rust contributions are licensed under the MIT License.

**Please Note:** ASM, C, C++, and Linker Components of the `teensy3-sys` crate (a dependency of the `teensy3` crate) contain components licensed under the MIT License, PJRC's modified MIT License, and the LGPL v2.1. Please refer to individual components for more details.
