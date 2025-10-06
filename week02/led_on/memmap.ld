MEMORY
{
    FLASH(rx) : ORIGIN = 0x10000000, LENGTH = 2048k
    RAM(rwx) : ORIGIN =  0x20000000, LENGTH = 256k
    SCRATCH_X(rwx) : ORIGIN = 0x20040000, LENGTH = 4k
    SCRATCH_Y(rwx) : ORIGIN = 0x20041000, LENGTH = 4k
}

SECTIONS
{
    .flash_begin : {
        __flash_binary_start = .;
    } > FLASH

    /* The stage 2 bootloader.  This sets up the flash chip a bit and then jumps
     * to an entry in the vector table.
     * This is at 0x10000000 (the start of the flash memory)
     */
    .boot2 : {
        __boot2_start__ = .;
        KEEP (*(.boot2))
        __boot2_end__ = .;
    } > FLASH

    ASSERT(__boot2_end__ - __boot2_start__ == 256,
        "ERROR: Pico second stage bootloader must be 256 bytes in size")

    /* After the second stage bootloader, we place the vector table.
     * This contains an initial stack pointer, the initial code entry point,
     * and the interrupt table.
     * The second stage bootloader loads the stack pointer out of the first
     * entry in the vector table and then jumps to the location specified in
     * the second entry of the vector table.
     * This is at 0x10000100 (boot2_start + 256 bytes)
     */
     .vectors : {
        __vectors_start__ = .;
        KEEP (*(.vectors))
        __vectors_end__ = .;
     } > FLASH

    ASSERT(__vectors_end__ - __vectors_start__ == 192,
        "ERROR: Vector table must be 192 bytes in size")

    .reset : {
        /* Bring in the reset code and force the location of it to be 256 bytes
         * after the vector table, location 0x10000200.
         * There are 64 bytes of "dead space" after the
         * the vector table, but before the code.  For now, we don't use it,
         * but we could put a silly interrupt handler there if we wanted.
         */
        . = __vectors_start__ + 0x100;
        KEEP (*(.reset))
    } > FLASH

    /* The rest of the code. It gets linked in from main.o because main.o has a 
     * section named .text and so gets pulled in here.
     */
    .text : {        
        KEEP (*(.text))
    } > FLASH	

    /* Any read-only (constant) variables */
    .rodata : {
        . = ALIGN(4);
        *(.rodata*)
        . = ALIGN(4);
        *(SORT_BY_ALIGNMENT(SORT_BY_NAME(.flashdata*)))
        . = ALIGN(4);
        __rodata_end__ = .;	
    } > FLASH

    /* Mutable variables that have initial values.
    * They start in flash (the initial values) but are copied
    * to RAM so that they can be modified at runtime.
    */
    .data : {
        __data_start__ = .;	
        *(.data*)
        . = ALIGN(4);
        __data_end__ = .;
    } > RAM AT> FLASH

    /* Mutable variables that don't have initial values.
     * They are allocated in RAM. On most systems, the entire BSS is
     * initialized to 0s, but you shouldn't rely on that.
     */	
    .bss  : {
        . = ALIGN(4);
        __bss_start__ = .;
        *(SORT_BY_ALIGNMENT(SORT_BY_NAME(.bss*)))
        *(COMMON)
        . = ALIGN(4);
        __bss_end__ = .;
    } > RAM

    /* Space for the heap, if used.
     * Right now we aren't using this for anything.
     */
    .heap (COPY):
    {
        __end__ = .;
        end = __end__;
        *(.heap*)
        __HeapLimit = .;
    } > RAM
}
