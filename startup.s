.section .text
.global _start

_start:
    // Load stack pointer
    ldr r0, _stack_start
    move sp, r0

    b1 main

infinite_loop:
    wfi
    b infinite_loop