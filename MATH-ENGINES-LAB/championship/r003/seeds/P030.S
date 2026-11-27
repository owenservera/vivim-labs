/* Reconstruction: candidate output converted to two 32-bit pieces, persisted in shadow space and reconstructed/compared before ABI commit. Artifact restoration architecture still open.
 * Win64 i64 development ABI. Independently authored proposal precursor. */
.text
.globl p030_seed
p030_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L30_add
    cmp r8d, 1
    je .L30_sub
    cmp r8d, 2
    je .L30_mul
    cmp r8d, 3
    je .L30_div
    cmp r8d, 4
    je .L30_neg
    cmp r8d, 5
    je .L30_div
    cmp r8d, 6
    je .L30_cmp
    jmp .L30_invalid
.L30_add:
    add rax, rdx
    jo .L30_over
    jmp .L30_store
.L30_sub:
    sub rax, rdx
    jo .L30_over
    jmp .L30_store
.L30_mul:
    imul rax, rdx
    jo .L30_over
    jmp .L30_store
.L30_neg:
    neg rax
    jo .L30_over
    jmp .L30_store
.L30_div:
    test rdx, rdx
    jz .L30_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L30_idiv
    cmp r10, -1
    je .L30_over
.L30_idiv:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L30_store
.L30_cmp:
    cmp rcx, rdx
    jl .L30_less
    jg .L30_more
    xor eax, eax
    jmp .L30_store
.L30_less:
    mov rax, -1
    jmp .L30_store
.L30_more:
    mov eax, 1
    jmp .L30_store

.L30_store:
    mov [rsp+8], eax
    mov r10, rax
    shr r10, 32
    mov [rsp+16], r10d
    mov r11d, [rsp+16]
    shl r11, 32
    mov r10d, [rsp+8]
    or r11, r10
    cmp r11, rax
    jne .L30_invalid
    mov [r9], r11
    xor eax, eax
    ret
.L30_over:
    mov eax, 1
    ret
.L30_zero:
    mov eax, 2
    ret
.L30_invalid:
    mov eax, 3
    ret
