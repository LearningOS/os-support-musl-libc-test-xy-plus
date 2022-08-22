    .section .text.entry
    .globl _start
_start:
    la tp, user_stack_top
    call umain

    .section .bss.stack
    .globl user_stack
user_stack:
    .space 4096
    .globl user_stack_top
user_stack_top:
