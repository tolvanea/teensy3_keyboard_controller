FROM rustembedded/cross:thumbv7em-none-eabi-0.2.1
# Above image is based on ubuntu 16.04

RUN apt-get update
# Some of the following dependencies may be not needed, but I'm too lazy to test
RUN apt-get install --assume-yes --no-install-recommends \
# Following two dependencies are already installed in rustembedded image,
# but they are written here again for clarity
    gcc-arm-none-eabi \
    libnewlib-arm-none-eabi \
    libnewlib-dev \
    libstdc++-arm-none-eabi-newlib \
    clang \
    libclang-8-dev \
    gcc-multilib

# For floating point support there is also following template:
# FROM rustembedded/cross:thumbv7em-none-eabihf-0.2.1
# but I think it has exactly the same dependencies as this one
