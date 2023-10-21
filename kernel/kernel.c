#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "../include/defs.h"
#include "../include/types.h"
#include "./vga_text.h"

#if defined (__linux__)
#error "You are not using a cross-compiler, you will most certainly run into trouble"
#endif

#if !defined (__i386__)
#error "This needs to be compiled with a ix86-elf compiler"
#endif

struct MultibootInfo {
    u32 flags;
    u32 mem_lower;
    u32 mem_upper;
    // Other stuff
};

void kernel_main(u32 multiboot_magic, const struct MultibootInfo* info) {
    terminal_initialize();

    terminal_put_hex((u8*) &multiboot_magic, sizeof(u32), true);
    terminal_putchar(' ');
    terminal_writestring(
        (multiboot_magic == 0x2BADB002) 
        ? "VALID"
        : "INVALID"
    );
    terminal_writestring(" multiboot magic\n");

    terminal_put_u32(info->flags);
    terminal_putchar('\n');

    terminal_put_u32(info->mem_lower);
    terminal_putchar('\n');

    terminal_put_u32(info->mem_upper);
    terminal_putchar('\n');

//    terminal_writestring("Hello, Kernel!\n");
//    terminal_writestring("the quick brown fox jumped over the lazy dog\n");
//    terminal_writestring("THE QUICK BROWN FOX JUMPED OVER THE LAZY DOG\n");
}
