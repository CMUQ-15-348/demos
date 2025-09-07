.syntax unified
.cpu cortex-m0plus

.thumb_func
.global main

.equ    PADS_BANK0_BASE,0x4001c000
.equ    IO_BANK0_BASE,  0x40014000
.equ    SIO_BASE,       0xd0000000
.equ    RESETS_BASE,    0x4000c000
.equ    RESETS_BASE_CLR,0x4000f000

.equ    PADS_BANK0_GPIO25,  0x68 @ Offset for GPIO 25 pad control register
.equ    GPIO_25_CTRL,   0x0CC    @ Offset for GPIO 25 control register


main:
    bl unreset
    movs r0, #25
    bl pinout

    // Turn LED on
    ldr r0, =SIO_BASE
    str r1, [r0, #0x014]         @ Set output high

    loop:
        str r1, [r0, #0x018]
        ldr r2, =0xfffff
    delay1:
        subs r2, r2, #1
        bne delay1

        str r1, [r0, #0x014]
        ldr r2, =0xfffff
    delay2:
        subs r2, r2, #1
        bne delay2

        b loop

unreset:
    ////////////////////
    // Pull IO_BANK0 and PADS_BANK0 out of reset
    ldr r0, =RESETS_BASE_CLR	// atomic register for clearing reset controller (0x4000c000+0x3000) 
    // IO_BANK0
    movs r1, #1
    lsls r1, r1, #5     // load a 1 into bit 5 (IO_BANK0)
    str r1, [r0, #0] 	// store the bitmask into the atomic register to clear register
    // PADS_BANK0
    movs r1, #1
    lsls r1, r1, #8     // load a 1 into bit 8 (PADS_BANK0)
    str r1, [r0, #0] 	// store the bitmask into the atomic register to clear register
    bx lr

pinout:
    ////////////////////
    // Initialize GPIO 25 for output
    ldr r3, =PADS_BANK0_BASE     @ Load PADS_BANK0 base address
    movs r1, #0                   @ Clear all bits
    // Calculate pad offset from r0
    adds r2, r0, #1
    lsls r2, r2, #2
    str r1, [r3, r2]          @ Configure pad (GPIO 25)

    ldr r3, =IO_BANK0_BASE       @ Load IO_BANK0 base address
    lsls r2, r0, #3
    adds r2, r2, #4
    movs r1, #5                    @ Function 5 (SIO)
    str r1, [r3, r2]   @ Set GPIO 25 function

    ldr r3, =SIO_BASE            @ Load SIO base address
    movs r1, #1                    @ Bit mask for GPIO 25
    lsls r1, r0                  @ Shift to bit 25
    str r1, [r3, #0x024]         @ Set GPIO 25 as output
    bx lr