#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "../include/defs.h"
#include "./mem.h"

#if defined (__linux__)
#error "You are not using a cross-compiler, you will most certainly run into trouble"
#endif

#if !defined (__i386__)
#error "This needs to be compiled with a ix86-elf compiler"
#endif

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
uint8_t vga_entry_color(enum vga_color fg, enum vga_color bg) {
    return fg | bg << 4;
}

internal inline
uint16_t vga_entry(unsigned char uc, uint8_t color) {
    return (uint16_t) uc | (uint16_t) color << 8;
}

global_variable const size_t VGA_WIDTH  = 80;
global_variable const size_t VGA_HEIGHT = 25;

size_t    terminal_row;
size_t    terminal_col;
uint8_t   terminal_color;
uint16_t* terminal_buffer;

internal inline
size_t terminal_coord_to_index(size_t x, size_t y) {
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
            VGA_WIDTH * (VGA_HEIGHT - 1) * sizeof(uint16_t)
        );
        memset(
            terminal_buffer + terminal_coord_to_index(0, VGA_HEIGHT - 1),
            0,
            VGA_WIDTH * sizeof(uint16_t)
        );
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
    terminal_buffer = (uint16_t*) 0xB8000;
    for (size_t y = 0; y < VGA_HEIGHT; ++y) {
        for (size_t x = 0; x < VGA_WIDTH; ++x) {
            const size_t index = terminal_coord_to_index(x, y);
            terminal_buffer[index] = vga_entry(' ', terminal_color);
        }
    }
}

// @Note: is this redundant?
void terminal_setcolor(uint8_t color) {
    terminal_color = color;
}

void terminal_putentryat(char c, uint8_t color, size_t x, size_t y) {
    const size_t index = terminal_coord_to_index(x, y);
    terminal_buffer[index] = vga_entry(c, color);
}

void terminal_putchar(char c) {
    terminal_putentryat(c, terminal_color, terminal_col, terminal_row);
    terminal_newcol();
}

void terminal_write(const char* data, size_t size) {
    for (size_t i = 0; i < size; ++i) {
        char c = data[i];
        switch (c) {
            case '\n': {
                terminal_newline();
            } break;
            default: {
                terminal_putchar(data[i]);
            } break;
        }
    }
}

// @Note has to be null terminated
void terminal_writestring(const char* data) {
    terminal_write(data, strlen(data));
}

void kernel_main(void) {
    terminal_initialize();

    terminal_writestring("Hello, Kernel World!  1 \n");
    terminal_writestring("Hello, Kernel World!  2 \n");
    terminal_writestring("Hello, Kernel World!  3 \n");
    terminal_writestring("Hello, Kernel World!  4 \n");
    terminal_writestring("Hello, Kernel World!  5 \n");
    terminal_writestring("Hello, Kernel World!  6 \n");
    terminal_writestring("Hello, Kernel World!  7 \n");
    terminal_writestring("Hello, Kernel World!  8 \n");
    terminal_writestring("Hello, Kernel World!  9 \n");
    terminal_writestring("Hello, Kernel World! 10 \n");
    terminal_writestring("Hello, Kernel World! 11 \n");
    terminal_writestring("Hello, Kernel World! 12 \n");
    terminal_writestring("Hello, Kernel World! 13 \n");
    terminal_writestring("Hello, Kernel World! 14 \n");
    terminal_writestring("Hello, Kernel World! 15 \n");
    terminal_writestring("Hello, Kernel World! 16 \n");
    terminal_writestring("Hello, Kernel World! 17 \n");
    terminal_writestring("Hello, Kernel World! 18 \n");
    terminal_writestring("Hello, Kernel World! 19 \n");
    terminal_writestring("Hello, Kernel World! 20 \n");
    terminal_writestring("Hello, Kernel World! 21 \n");
    terminal_writestring("Hello, Kernel World! 22 \n");
    terminal_writestring("Hello, Kernel World! 23 \n");
    terminal_writestring("Hello, Kernel World! 24 \n");
    terminal_writestring("Hello, Kernel World! 25 \n");
    terminal_writestring("Hello, Kernel World! 26 \n");
    terminal_writestring("Hello, Kernel World! 27 \n");
    terminal_writestring("Hello, Kernel World! 28 \n");
    terminal_writestring("Hello, Kernel World! 29 \n");
    terminal_writestring("Hello, Kernel World! 30 \n");
}
