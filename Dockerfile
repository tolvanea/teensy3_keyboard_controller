FROM rustembedded/cross:thumbv7em-none-eabi-0.2.1

# For floating point support there is also following template, but I think it's the same
#FROM rustembedded/cross:thumbv7em-none-eabihf-0.2.1

RUN apt-get update
# Some of these may be not needed, but I'm not going to test everything one by one
RUN apt-get install --assume-yes --no-install-recommends \
# Reinstall?
    gcc-arm-none-eabi \
# Reinstall?
    libnewlib-arm-none-eabi \
    libnewlib-dev \
    libstdc++-arm-none-eabi-newlib \
    clang \
    libclang-8-dev \
    gcc-multilib
#    binutils-arm-none-eabi \
#    libusb-dev \
