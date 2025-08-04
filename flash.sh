#! /bin/bash

cargo b --release

cd target/thumbv7em-none-eabihf/release

arm-none-eabi-objcopy -O binary Blinky kernel.img

st-flash write kernel.img 0x08000000

cd ../../..
