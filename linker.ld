ENTRY(_start)

SECTIONS
{
    /* virt RAM starts at 0x40000000. Leave 512 KiB of headroom so we
       don't collide with the device tree blob QEMU places near the
       start of RAM. */
    . = 0x40080000;

    .text : {
        KEEP(*(.text._start))
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .bss (NOLOAD) : ALIGN(8) {
        __bss_start = .;
        *(.bss .bss.*)
        . = ALIGN(8);
        __bss_end = .;
    }

    . = ALIGN(16);
    . += 0x80000; /* 512 KiB boot stack */
    _stack_top = .;

    /DISCARD/ : { *(.comment .gnu.*) }
}
