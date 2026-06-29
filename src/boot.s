.section ".text._start"

.global _start
_start:
        // save x0 (it holds pointer to FDT)
        mov x19, x0
        // read system register
        mrs x1, mpidr_el1
        and x1, x1, #0xff
        cbz x1, .L_continue // compare and goto if 0 (core 0)
.L_park:
        wfe
        b .L_park

.L_continue:
        // set up stack
        // _stack_top comes from linker script
        adrp x1, _stack_top // this loads top 32 bits
        add x1, x1, :lo12:_stack_top // this loads low 12 bit
        mov sp, x1


        // set up bss (zero initialized section)
        adrp x1, __bss_start
        add x1, x1, :lo12:__bss_start
        adrp x2, __bss_end
        add x2, x2, :lo12:__bss_end
.L_bss_zero:
        cmp x1, x2
        b.ge .L_bss_done
        str xzr, [x1], #8 // store 0, x1 += 8
        b .L_bss_zero
.L_bss_done:
        mov x0, x19 // this will be the first argument
        bl kernel_main        
