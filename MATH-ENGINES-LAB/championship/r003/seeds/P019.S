/* Counterfactual: materialize hypothetical wrapping and checked outcomes; compare alternatives before commit. No actual ledger fork claimed.
 * Win64 i64 development ABI. Independently authored proposal precursor. */
.text
.globl p019_seed
p019_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L19_add
    cmp r8d, 1
    je .L19_sub
    cmp r8d, 2
    je .L19_mul
    cmp r8d, 3
    je .L19_div
    cmp r8d, 4
    je .L19_neg
    cmp r8d, 5
    je .L19_div
    cmp r8d, 6
    je .L19_cmp
    jmp .L19_invalid
.L19_add:
    lea r10, [rcx+rdx]
    add rax, rdx
    jo .L19_over
    cmp r10, rax
    jne .L19_invalid
    jmp .L19_store
.L19_sub:
    mov r10, rdx
    neg r10
    lea r10, [rcx+r10]
    sub rax, rdx
    jo .L19_over
    cmp r10, rax
    jne .L19_invalid
    jmp .L19_store
.L19_mul:
    imul rax, rdx
    jo .L19_over
    mov r10, rcx
    imul r10, rdx
    cmp r10, rax
    jne .L19_invalid
    jmp .L19_store
.L19_neg:
    mov r10, rcx
    not r10
    inc r10
    neg rax
    jo .L19_over
    cmp rax, r10
    jne .L19_invalid
    jmp .L19_store
.L19_div:
    test rdx, rdx
    jz .L19_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L19_idiv
    cmp r10, -1
    je .L19_over
.L19_idiv:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L19_store
.L19_cmp:
    cmp rcx, rdx
    jl .L19_less
    jg .L19_more
    xor eax, eax
    jmp .L19_store
.L19_less:
    mov rax, -1
    jmp .L19_store
.L19_more:
    mov eax, 1
    jmp .L19_store

.L19_store:
    mov [r9], rax
    xor eax, eax
    ret
.L19_over:
    mov eax, 1
    ret
.L19_zero:
    mov eax, 2
    ret
.L19_invalid:
    mov eax, 3
    ret
