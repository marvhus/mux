#!/usr/bin/env bash

set -xe

cargo build

rm -rf isodir
mkdir -p isodir/boot/grub

cp target/target/debug/myos isodir/boot/myos.bin
cp grub.cfg isodir/boot/grub/grub.cfg

grub-mkrescue -o myos.iso isodir
