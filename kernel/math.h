#if !defined (KERNEL_MATH_H)
#define KERNEL_MATH_H

int ipow(int base, int exp) {
    int result = 1;
    for (;;) {
        if (exp & 1) result *= base;
        exp >>= 1;
        if (!exp) break;
        base *= base;
    }

    return result;
}

#endif
