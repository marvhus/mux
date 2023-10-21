#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "../include/defs.h"
#include "../include/types.h"
#include "./vga_text.h"
#include "./multiboot.h"

#if defined (__linux__)
#error "You are not using a cross-compiler, you will most certainly run into trouble"
#endif

#if !defined (__i386__)
#error "This needs to be compiled with a ix86-elf compiler"
#endif


internal
void check_multiboot_magic(u32 multiboot_magic) {
    terminal_put_hex((u8*) &multiboot_magic, sizeof(u32), true);
    terminal_putchar(' ');
    terminal_writestring(
        (multiboot_magic == 0x2BADB002) 
        ? (u8*) "VALID"
        : (u8*) "INVALID"
    );
    terminal_writestring((u8*) " multiboot magic\n");
}

internal
void check_boot_loader_name(const struct Multiboot_Info* info) {
    if (info->flags & MULTIBOOT_INFO_Boot_Loader_Name) {
        terminal_writestring((u8*) "Has Name: '");
        terminal_writestring((u8*) info->boot_loader_name);
        terminal_writestring((u8*) "'\n");
    } else {
        terminal_writestring((u8*) "Doesn't Have Name\n");
    }
}

void kernel_main(u32 multiboot_magic, const struct Multiboot_Info* info) {
    terminal_initialize();
    check_multiboot_magic(multiboot_magic);
    check_boot_loader_name(info);
}
