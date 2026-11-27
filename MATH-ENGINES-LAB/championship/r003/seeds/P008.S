/* Checkpoint: write staged output to caller shadow space, re-read and compare a canonical complement checkpoint before committing. Preview only.
 * Win64 i64 development ABI. Independently authored proposal precursor. */
.text
.globl p008_seed
p008_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L08_add
    cmp r8d, 1
    je .L08_sub
    cmp r8d, 2
    je .L08_mul
    cmp r8d, 3
    je .L08_div
    cmp r8d, 4
    je .L08_neg
    cmp r8d, 5
    je .L08_div
    cmp r8d, 6
    je .L08_cmp
    jmp .L08_invalid
.L08_add:
    add rax, rdx
    jo .L08_over
    jmp .L08_store
.L08_sub:
    sub rax, rdx
    jo .L08_over
    jmp .L08_store
.L08_mul:
    imul rax, rdx
    jo .L08_over
    jmp .L08_store
.L08_neg:
    neg rax
    jo .L08_over
    jmp .L08_store
.L08_div:
    test rdx, rdx
    jz .L08_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L08_idiv
    cmp r10, -1
    je .L08_over
.L08_idiv:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L08_store
.L08_cmp:
    cmp rcx, rdx
    jl .L08_less
    jg .L08_more
    xor eax, eax
    jmp .L08_store
.L08_less:
    mov rax, -1
    jmp .L08_store
.L08_more:
    mov eax, 1
    jmp .L08_store

.L08_store:
    mov [rsp+8], rax
    not rax
    mov [rsp+16], rax
    mov r10, [rsp+8]
    not rax
    cmp r10, rax
    jne .L08_invalid
    mov r11, [rsp+16]
    not r11
    cmp r10, r11
    jne .L08_invalid
    mov [r9], r10
    xor eax, eax
    ret
.L08_over:
    mov eax, 1
    ret
.L08_zero:
    mov eax, 2
    ret
.L08_invalid:
    mov eax, 3
    ret
