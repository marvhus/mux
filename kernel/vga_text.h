#if !defined (KERNEL_VGA_TEXT_H)
#define KERNEL_VGA_TEXT_H

#include "../include/types.h"
#include "../include/defs.h"
#include "./mem.h"
#include "./math.h"

// VGA Text Mode Stuff

// Hardware text mode color constants.
enum vga_color {
    VGA_COLOR_BLACK         =  0,
    VGA_COLOR_BLUE          =  1,
    VGA_COLOR_GREEN         =  2,
    VGA_COLOR_CYAN          =  3,
    VGA_COLOR_RED           =  4,
    VGA_COLOR_MAGENTA       =  5,
    VGA_COLOR_BROWN         =  6,
    VGA_COLOR_LIGHT_GREY    =  7,
    VGA_COLOR_DARK_GREY     =  8,
    VGA_COLOR_LIGHT_BLUE    =  9,
    VGA_COLOR_LIGHT_GREEN   = 10,
    VGA_COLOR_LIGHT_CYAN    = 11,
    VGA_COLOR_LIGHT_RED     = 12,
    VGA_COLOR_LIGHT_MAGENTA = 13,
    VGA_COLOR_LIGHT_BROWN   = 14,
    VGA_COLOR_WHITE         = 15,
};

internal inline
u8 vga_entry_color(enum vga_color fg, enum vga_color bg) {
    return fg | bg << 4;
}

internal inline
u16 vga_entry(u8 uc, u8 color) {
    return (u16) uc | (u16) color << 8;
}

global_variable const size_t VGA_WIDTH  = 80;
global_variable const size_t VGA_HEIGHT = 25;

// Terminal Stuff

size terminal_row;
size terminal_col;
u8   terminal_color;
u16* terminal_buffer;

internal inline
size terminal_coord_to_index(size x, size y) {
    return y * VGA_WIDTH + x;
}

internal inline
void terminal_newline() {
    terminal_col = 0;
    if (++terminal_row == VGA_HEIGHT) {
        // Scroll
        terminal_row = VGA_HEIGHT - 1;
        memmove(
            terminal_buffer,
            terminal_buffer + terminal_coord_to_index(0, 1),
            VGA_WIDTH * (VGA_HEIGHT - 1) * sizeof(u16)
        );
        u16* offset_buffer = terminal_buffer + terminal_coord_to_index(0, VGA_HEIGHT - 1);
        for (size i = 0; i < VGA_WIDTH; ++i) {
            offset_buffer[i] = vga_entry(' ', terminal_color);
        }
    }
}

internal inline
void terminal_newcol() {
    if (++terminal_col == VGA_WIDTH) {
        terminal_newline();
    }
}

void terminal_initialize(void) {
    terminal_row    = 0;
    terminal_col    = 0;
    terminal_color  = vga_entry_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    terminal_buffer = (u16*) 0xB8000;
    for (size y = 0; y < VGA_HEIGHT; ++y) {
        for (size x = 0; x < VGA_WIDTH; ++x) {
            const size index = terminal_coord_to_index(x, y);
            terminal_buffer[index] = vga_entry(' ', terminal_color);
        }
    }
}

// @Note: is this redundant?
void terminal_setcolor(u8 color) {
    terminal_color = color;
}

void terminal_putentryat(u8 c, u8 color, size x, size y) {
    const size index = terminal_coord_to_index(x, y);
    terminal_buffer[index] = vga_entry(c, color);
}

void terminal_putchar(u8 c) {
    switch (c) {
        case '\n': {
            terminal_newline();
        } break;
        default: {
            terminal_putentryat(c, terminal_color, terminal_col, terminal_row);
            terminal_newcol();
        } break;
    }
}

void terminal_write(const u8* data, size count) {
    for (size i = 0; i < count; ++i) {
        terminal_putchar(data[i]);
    }
}

internal inline
u8 int_to_hex_nible(u8 val) {
    val &= 0x0F;
    if (val < 10) return val + '0';
    return (val - 10) + 'A';
}

void terminal_put_hex(const u8* bytes, size count, bool prefix) {
    if (prefix) {
        terminal_putchar('0');
        terminal_putchar('x');
    }
    for (size i = count; i > 0; --i) {
        u8 byte = bytes[i-1];
        terminal_putchar(int_to_hex_nible(byte >> 4));
        terminal_putchar(int_to_hex_nible(byte));
    }
}

void terminal_put_u32(u32 val) {
    int num_digits = 1;
    while (true) {
        if (val / ipow(10, num_digits) == 0) break;
        num_digits++;
    }

    for (int digit = num_digits; digit > 0; --digit) {
        u8 c = (val / ipow(10, digit - 1)) % 10;
        terminal_putchar(c + '0');
    }
}

void terminal_put_s32(s32 val) {
    if (val < 0) {
        terminal_putchar('-');
        val = -val;
    }
    terminal_put_u32((u32) val);
}

// @Note has to be null terminated
void terminal_writestring(const u8* data) {
    terminal_write(data, strlen(data));
}

#endif
