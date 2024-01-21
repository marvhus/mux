#!/usr/bin/env bash

echo "====
Debugging. Start GDB on target/target/debug/myos
and run \"target remote localhost:1234\".
You can now debug it while it is running.
(you may want to use rust-gdb)
===="

set -xe

qemu-system-i386 -s -S -cdrom myos.iso
