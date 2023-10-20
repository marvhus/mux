#!/usr/bin/env bash

set -xe

qemu-system-i386 -kernel boot/myos.bin
