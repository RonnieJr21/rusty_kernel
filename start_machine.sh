#!/bin/bash

cargo bootimage

qemu-system-x86_64 -drive format=raw,file=target/x86_64-rusty_kernel/debug/bootimage-rusty_kernel.bin