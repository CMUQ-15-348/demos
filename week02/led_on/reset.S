.syntax unified
.cpu cortex-m0plus
.thumb_func
.global main

.section .reset, "ax"

// Load the three values from the table into r1-r3
adr r4, data_cpy_table
ldmia r4!, {r1-r3}
// Call our data copy routine to copy the .data from flash to RAM
bl data_cpy

// I want my reset code to just call main after copying .data
b main

// Code for copying data
data_cpy_loop:
    ldm r1!, {r0}
    stm r2!, {r0}
data_cpy:
    cmp r2, r3
    blo data_cpy_loop
    bx lr

data_cpy_table:
.word __rodata_end__
.word __data_start__
.word __data_end__
.word 0 // null terminator
