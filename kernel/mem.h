#include <stddef.h>
#if !defined (KERNEL_MEM_H)
#define KERNEL_MEM_H

#include <stdint.h>

void memmove(void* dest, const void* src, size_t count) {
    for (size_t i = 0; i < count; ++i) {
        ((uint8_t*) dest)[i] = ((uint8_t*) src)[i];
    }
}

// @TODO: figure out a solution for situations where the values are larger than a byte.
void memset(void* dest, uint8_t val, size_t count) {
    for (size_t i = 0; i < count; ++i) {
        ((uint8_t*) dest)[i] = val;
    }
}

size_t strlen(const char* str) {
    size_t len = 0;
    while (str[len]) {
        len++;
    }
    return len;
}

#endif
