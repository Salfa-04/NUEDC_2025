#include <stdint.h>

uint8_t c_buffer[16] = {0};

int32_t c_add(int32_t a, int32_t b) { return a + b; }
int32_t c_sub(int32_t a, int32_t b) { return a - b; }
int32_t c_mul(int32_t a, int32_t b) { return a * b; }
int32_t c_div(int32_t a, int32_t b) { return a / b; }
int32_t c_mod(int32_t a, int32_t b) { return a % b; }
