/* Kernel's enterypoint function */
ENTRY(boot)

/*
 * Sections:
 *   .text   : Code section
 *   .rodata : Read-only constants
 *   .data   : Data to read/write
 *   .bss    : Data to read/write (Zero-initialized variables)
 */

SECTIONS {
    . = 0x80200000; /* base address */

    .text :{
        KEEP(*(.text.boot)); /* Start with .text.boot section */
        *(.text .text.*);
    }

    /*
     * *(.rodata .rodata.*); means that all .rodata and .rodata.* sections in all files (*)
     * should be allocated at this section
     */
    .rodata : ALIGN(4) {
        *(.rodata .rodata.*);
    }

    .data : ALIGN(4) {
        *(.data .data.*);
    }

    .bss : ALIGN(4) {
        __bss = .;
        *(.bss .bss.* .sbss .sbss.*);
        __bss_end = .;
    }

    /* Put stack at the end of bss section */
    . = ALIGN(4);
    . += 128 * 1024; /* 128KB */
    __stack_top = .;

    /* Allocate 64MB for heap memory */
    . = ALIGN(4096);
    __free_ram = .;
    . += 64 * 1024 * 1024; /* 64MB */
    __free_ram_end = .;
}
