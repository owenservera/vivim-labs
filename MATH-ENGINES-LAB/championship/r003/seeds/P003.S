/* Full replay: wide signed add/sub/neg, explicit high-limb consistency.
 * Compute exact signed 128-bit intermediate from 64-bit operands and
 * reject if the high limb is not the sign extension of the low limb. */
.text
.globl p003_seed
p003_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L03_wide
    cmp r8d, 1
    je .L03_wide
    cmp r8d, 2
    je .L03_mul
    cmp r8d, 3
    je .L03_div
    cmp r8d, 4
    je .L03_neg
    cmp r8d, 5
    je .L03_div
    cmp r8d, 6
    je .L03_cmp
    mov eax, 3
    ret
.L03_wide:
    mov r10, rcx
    sar r10, 63
    mov r11, rdx
    sar r11, 63
    cmp r8d, 1
    je .L03_sub
    add rax, rdx
    adc r10, r11
    jmp .L03_narrow
.L03_sub:
    sub rax, rdx
    sbb r10, r11
    jmp .L03_narrow
.L03_neg:
    mov r11, rcx
    sar r11, 63
    xor eax, eax
    xor r10d, r10d
    sub rax, rcx
    sbb r10, r11
    jmp .L03_narrow
.L03_mul:
    imul rdx
    mov r10, rdx
.L03_narrow:
    mov r11, rax
    sar r11, 63
    cmp r10, r11
    jne .L03_over
    jmp .L03_store
.L03_div:
    test rdx, rdx
    jz .L03_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L03_idiv
    cmp r10, -1
    je .L03_over
.L03_idiv:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L03_store
.L03_cmp:
    cmp rcx, rdx
    jl .L03_less
    jg .L03_more
    xor eax, eax
    jmp .L03_store
.L03_less:
    mov rax, -1
    jmp .L03_store
.L03_more:
    mov eax, 1
.L03_store:
    mov [r9], rax
    xor eax, eax
    ret
.L03_over:
    mov eax, 1
    ret
.L03_zero:
    mov eax, 2
    ret
