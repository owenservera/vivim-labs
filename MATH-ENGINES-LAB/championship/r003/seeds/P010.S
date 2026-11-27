/* Hybrid: signed checked hardware result and independent unsigned carry/borrow replay for add/sub; compare both before commit.
 * Win64 i64 development ABI. Independently authored proposal precursor. */
.text
.globl p010_seed
p010_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L10_add
    cmp r8d, 1
    je .L10_sub
    cmp r8d, 2
    je .L10_mul
    cmp r8d, 3
    je .L10_div
    cmp r8d, 4
    je .L10_neg
    cmp r8d, 5
    je .L10_div
    cmp r8d, 6
    je .L10_cmp
    jmp .L10_invalid
.L10_add:
    add rax, rdx
    jo .L10_over
    mov r10, rcx
    lea r10, [r10+rdx]
    cmp r10, rax
    jne .L10_invalid
    jmp .L10_store
.L10_sub:
    sub rax, rdx
    jo .L10_over
    mov r10, rdx
    not r10
    lea r10, [rcx+r10+1]
    cmp r10, rax
    jne .L10_invalid
    jmp .L10_store
.L10_mul:
    imul rax, rdx
    jo .L10_over
    mov r10, rdx
    imul r10, rcx
    cmp r10, rax
    jne .L10_invalid
    jmp .L10_store
.L10_neg:
    neg rax
    jo .L10_over
    not rcx
    inc rcx
    cmp rax, rcx
    jne .L10_invalid
    jmp .L10_store
.L10_div:
    test rdx, rdx
    jz .L10_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L10_idiv
    cmp r10, -1
    je .L10_over
.L10_idiv:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L10_store
.L10_cmp:
    xor eax, eax
    xor r10d, r10d
    cmp rcx, rdx
    setg al
    setl r10b
    sub rax, r10
    jmp .L10_store
.L10_less:
    mov rax, -1
    jmp .L10_store
.L10_more:
    mov eax, 1
    jmp .L10_store

.L10_store:
    mov [r9], rax
    xor eax, eax
    ret
.L10_over:
    mov eax, 1
    ret
.L10_zero:
    mov eax, 2
    ret
.L10_invalid:
    mov eax, 3
    ret
