/* Theorem Factory: construct subtraction as addition of negation with
 * a special boundary theorem for MIN; multiply by 2 as checked doubling;
 * derive division-by-2 from signed shift plus truncation correction.
 * This is a hand-authored initial law-derived seed, not an inferred theorem. */
.text
.globl p020_seed
p020_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L20_add
    cmp r8d, 1
    je .L20_sub
    cmp r8d, 2
    je .L20_mul
    cmp r8d, 3
    je .L20_div
    cmp r8d, 4
    je .L20_neg
    cmp r8d, 5
    je .L20_div
    cmp r8d, 6
    je .L20_cmp
    mov eax, 3
    ret
.L20_sub:
    mov r10, 0x8000000000000000
    cmp rdx, r10
    je .L20_min_sub
    neg rdx
.L20_add:
    add rax, rdx
    jo .L20_over
    jmp .L20_store
.L20_min_sub:
    test rax, rax
    jns .L20_over
    xor rax, r10
    jmp .L20_store
.L20_mul:
    cmp rdx, 2
    jne .L20_general_mul
    add rax, rax
    jo .L20_over
    jmp .L20_store
.L20_general_mul:
    imul rax, rdx
    jo .L20_over
    jmp .L20_store
.L20_neg:
    xor eax, eax
    sub rax, rcx
    jo .L20_over
    jmp .L20_store
.L20_div:
    test rdx, rdx
    jz .L20_zero
    cmp rdx, 2
    je .L20_half
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L20_idiv
    cmp r10, -1
    je .L20_over
.L20_idiv:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L20_store
.L20_half:
    mov r10, rax
    shr r10, 63
    add rax, r10
    sar rax, 1
    cmp r8d, 5
    jne .L20_store
    lea r10, [rax+rax]
    mov rax, rcx
    sub rax, r10
    jmp .L20_store
.L20_cmp:
    xor eax, eax
    xor r10d, r10d
    cmp rcx, rdx
    setg al
    setl r10b
    sub rax, r10
.L20_store:
    mov [r9], rax
    xor eax, eax
    ret
.L20_over:
    mov eax, 1
    ret
.L20_zero:
    mov eax, 2
    ret
