#!/usr/bin/env sh
aarch64-none-elf-objcopy -O binary "$1" "$1.bin"
exec qemu-system-aarch64 -M virt,gic-version=2 -cpu cortex-a72 -m 512M \
    -nographic -kernel "$1.bin"
