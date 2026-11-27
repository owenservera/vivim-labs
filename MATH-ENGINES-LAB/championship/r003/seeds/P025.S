/* Metabolic seed: fast integer division for powers of two, including
 * negative operands with truncation correction. Otherwise use IDIV.
 * The candidate does not claim a performance win without measurement. */
.text
.globl p025_seed
p025_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L25_add
    cmp r8d, 1
    je .L25_sub
    cmp r8d, 2
    je .L25_mul
    cmp r8d, 3
    je .L25_div
    cmp r8d, 4
    je .L25_neg
    cmp r8d, 5
    je .L25_div
    cmp r8d, 6
    je .L25_cmp
    mov eax, 3
    ret
.L25_add:
    add rax, rdx
    jo .L25_over
    jmp .L25_store
.L25_sub:
    sub rax, rdx
    jo .L25_over
    jmp .L25_store
.L25_mul:
    imul rax, rdx
    jo .L25_over
    jmp .L25_store
.L25_neg:
    neg rax
    jo .L25_over
    jmp .L25_store
.L25_div:
    test rdx, rdx
    jz .L25_zero
    mov r10, 0x8000000000000000
    cmp rax, r10
    jne .L25_classify
    cmp rdx, -1
    je .L25_over
.L25_classify:
    mov r10, rdx
    sar r10, 63
    mov [rsp+8], r10
    xor rdx, r10
    sub rdx, r10
    lea r11, [rdx-1]
    test rdx, r11
    jnz .L25_general
    /* Power-of-two unsigned divisor magnitude, use unsigned shift of |a|. */
    mov r10, rax
    sar r10, 63
    mov [rsp+16], r10
    xor rax, r10
    sub rax, r10
    mov r10, rax
    and r10, r11
    mov [rsp+24], r10
    bsf rcx, rdx
    shr rax, cl
    mov r11, [rsp+16]
    cmp r8d, 5
    je .L25_remainder
    xor r11, [rsp+8]
    jmp .L25_restore_sign
.L25_remainder:
    mov rax, [rsp+24]
.L25_restore_sign:
    xor rax, r11
    sub rax, r11
    jmp .L25_store
.L25_general:
    mov r10, [rsp+8]
    xor rdx, r10
    sub rdx, r10
    mov r10, rdx
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L25_store
.L25_cmp:
    xor eax, eax
    xor r10d, r10d
    cmp rcx, rdx
    setg al
    setl r10b
    sub rax, r10
.L25_store:
    mov [r9], rax
    xor eax, eax
    ret
.L25_over:
    mov eax, 1
    ret
.L25_zero:
    mov eax, 2
    ret
