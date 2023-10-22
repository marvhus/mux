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
    printf(
        (multiboot_magic == 0x2BADB002) 
        ? "VALID multiboot magic\n"
        : "INVALID multiboot magic\n"
    );
}

internal
void check_boot_loader_name(const struct Multiboot_Info* info) {
    if (info->flags & MULTIBOOT_INFO_Boot_Loader_Name) {
        const char* boot_loader_name = (const char*) info->boot_loader_name;
        printf("Has name: '%s'\n", boot_loader_name);
    } else {
        printf("Doesn't have name\n");
    }
}

void kernel_main(u32 multiboot_magic, const struct Multiboot_Info* info) {
    terminal_initialize();
    check_multiboot_magic(multiboot_magic);
    check_boot_loader_name(info);

    for (u32 i = 0; i < info->mmap_length; i += sizeof(struct Multiboot_Memory_Map)) {
        struct Multiboot_Memory_Map* mmap = (struct Multiboot_Memory_Map*) (info->mmap_addr + i);
        printf("entry: | Start Addr: %x6 | Length: %x6 \n"
               "       | Size:       %x6 | Type:   %du3\n",
            mmap->addr, mmap->len, mmap->size, mmap->type
        );
    }
}
