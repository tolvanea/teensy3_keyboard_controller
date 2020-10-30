# Teensy with Rust

This is an example project that uses bindings from [teensy3-rs](https://github.com/tolvanea/teensy3-rs) crate. This aims to be dead simple template to get teensy project running. It is intended to be forked, or copied into a new project.

## Modification in this fork
* Get it working on my machine
* Support for teensy 3.0-3.6
* Change Cargo to Cross, and do compilation in isolated Docker container
* Reduce need for configuration: The only needed thing is to set teensy model in makefile
* For other notes see [teensy3-rs](https://github.com/tolvanea/teensy3-rs)

## About this project
This project uses [Cross](https://github.com/rust-embedded/cross) instead of cargo, which runs compilation in Docker container. Docker container is useful, because cross compilation is sensitive to installed dependencies.

## Installations
Install [teensy-loader-cli](https://www.pjrc.com/teensy/loader_cli.html), which will be used to flash compiled binary on teensy. Installation on ubuntu is:
```
sudo apt-get install teensy-loader-cli
```
Install [Cross](https://github.com/rust-embedded/cross)
```
cargo install cross
```

Install Docker. On ubuntu it can be done with:
```
sudo apt-get install docker
```
You can check that docker is running
```
sudo systemctl status docker
```
Allow current user to use docker, so there is no need to write `sudo` with every docker command
```
sudo usermod -aG docker ${USER}
```
However, above command takes effect after loggin out and in. To avoid relogging, it can be temporarily fixed with 
```
su - $USER                                           
```
which works only in that terminal it is called.

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

Then, teensy model should be specified. Uncomment the line corresponding your teensy model in `Makefile`.
```
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
And that's it! You should see fastly blinking led going on and of.
Linux users can run following script to read output from teensy
```
./read_output_from_usb
``` 


## Other makefile usage
Debug build with
```
make debug
```
Generate documentation for current project and bindings with
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
