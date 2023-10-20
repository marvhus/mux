#!/usr/bin/env bash

source ./source.sh # Add cross compiler to path, and target env variable
mkdir -p bin
mkdir -p boot

set -xe

$TARGET-as kernel/boot.s -o bin/boot.o

$TARGET-gcc -c kernel/kernel.c -o bin/kernel.o -std=gnu99 -ffreestanding -O2 -Wall -Wextra

$TARGET-gcc -T kernel/linker.ld -o boot/myos.bin -ffreestanding -O2 -nostdlib bin/boot.o bin/kernel.o -lgcc

{ set +x; } 2>/dev/null

if grub-file --is-x86-multiboot boot/myos.bin; then
    echo "SUCCESS: multiboot confirmed"
else
    echo "ERROR: the file is not multiboot"
    exit 1
fi

set -x

rm bin/boot.o
rm bin/kernel.o
