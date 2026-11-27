/* Compression: share the flag/status/commit suffix across checked ALU
 * operations; use compact fall-through dispatch and branchless comparison.
 * Compactness is a hypothesis, not a measured speed claim. */
.text
.globl p018_seed
p018_seed:
    mov rax, rcx
    cmp r8d, 0
    jne .L18_not_add
    add rax, rdx
    jmp .L18_checked
.L18_not_add:
    cmp r8d, 1
    jne .L18_not_sub
    sub rax, rdx
    jmp .L18_checked
.L18_not_sub:
    cmp r8d, 2
    jne .L18_not_mul
    imul rax, rdx
    jmp .L18_checked
.L18_not_mul:
    cmp r8d, 4
    jne .L18_other
    neg rax
.L18_checked:
    jo .L18_over
    jmp .L18_store
.L18_other:
    cmp r8d, 6
    je .L18_cmp
    cmp r8d, 3
    je .L18_div
    cmp r8d, 5
    je .L18_div
    mov eax, 3
    ret
.L18_div:
    test rdx, rdx
    jz .L18_zero
    mov r10, rdx
    cmp r10, -1
    je .L18_unit
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L18_store
.L18_unit:
    neg rax
    jo .L18_over
    xor edx, edx
    cmp r8d, 5
    cmove rax, rdx
    jmp .L18_store
.L18_cmp:
    xor eax, eax
    cmp rcx, rdx
    setg al
    setl r10b
    movzx r10d, r10b
    sub rax, r10
.L18_store:
    mov [r9], rax
    xor eax, eax
    ret
.L18_over:
    mov eax, 1
    ret
.L18_zero:
    mov eax, 2
    ret
