#!/usr/bin/env bash

mkdir -p isodir/boot/grub

set -xe

cp boot/myos.bin isodir/boot/myos.bin
cp grub.cfg isodir/boot/grub/grub.cfg
grub-mkrescue -o boot/myos.iso isodir
