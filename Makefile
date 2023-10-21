TARGET := i686-elf
CC := $(TARGET)-gcc
AS := $(TARGET)-as

KERNEL_SOURCE := $(shell find kernel/ -type f -name '*.c')
BOOT_SOURCE := $(shell find kernel/ -type f -name '*.s')

all: boot/myos.bin

# Compile asm part of boot
bin/boot.o: $(BOOT_SOURCE) ;
	mkdir -p bin
	$(AS) $(BOOT_SOURCE) -o bin/boot.o

# Compile c part of kernel
bin/kernel.o: $(KERNEL_SOURCE) ;
	mkdir -p bin
	$(CC) -c $(KERNEL_SOURCE) -o bin/kernel.o -std=gnu99 -ffreestanding -O2 -Wall -Wextra

# Create kernel binary
boot/myos.bin: bin/kernel.o bin/boot.o ;
	mkdir -p boot
	$(CC) -T kernel/linker.ld -o boot/myos.bin -ffreestanding -O2 -nostdlib bin/kernel.o bin/boot.o -lgcc


# Check if compiler kernel is multiboot
check: boot/myos.bin ;
	(grub-file --is-x86-multiboot boot/myos.bin && echo "SUCESS: multiboot confifmed") || echo "FAILED: the file is not multiboot"


# Create an iso file
boot/myos.iso: boot/myos.bin ;
	mkdir -p isodir/boot/grub
	cp boot/myos.bin isodir/boot/myos.bin
	cp grub.cfg isodir/boot/grub/grub.cfg
	grub-mkrescue -o boot/myos.iso isodir

iso: boot/myos.iso


# Run qemu using kernel binary
run: boot/myos.bin ;
	qemu-system-i386 -kernel boot/myos.bin


# Run qemu using iso file
run-iso: boot/myos.iso ;
	qemu-system-i386 -cdrom boot/myos.iso


# Clean stuff up
clean:
	rm -rf bin
	rm -rf boot
	rm -rf isodir

