#include <stddef.h>
#if !defined (KERNEL_MEM_H)
#define KERNEL_MEM_H

#include "../include/types.h"

void memmove(void* dest, const void* src, size count) {
    for (size i = 0; i < count; ++i) {
        ((u8*) dest)[i] = ((u8*) src)[i];
    }
}

// @TODO: figure out a solution for situations where the values are larger than a byte.
void memset(void* dest, u8 val, size count) {
    for (size i = 0; i < count; ++i) {
        ((u8*) dest)[i] = val;
    }
}

size strlen(const u8* str) {
    size len = 0;
    while (str[len]) {
        len++;
    }
    return len;
}

#endif
