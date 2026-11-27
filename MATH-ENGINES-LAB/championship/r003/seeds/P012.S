/* Immune: explicit special-case antibodies for 0, +/-1, MIN.
 * General operations still run checked native ALU; no silent fallback.
 */
.text
.globl p012_seed
p012_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L12_add
    cmp r8d, 1
    je .L12_sub
    cmp r8d, 2
    je .L12_mul
    cmp r8d, 3
    je .L12_div
    cmp r8d, 4
    je .L12_neg
    cmp r8d, 5
    je .L12_div
    cmp r8d, 6
    je .L12_cmp
    mov eax, 3
    ret
.L12_add:
    test rdx, rdx
    jz .L12_store
    test rcx, rcx
    jnz .L12_add_general
    mov rax, rdx
    jmp .L12_store
.L12_add_general:
    add rax, rdx
    jo .L12_over
    jmp .L12_store
.L12_sub:
    cmp rcx, rdx
    je .L12_result_zero
    test rdx, rdx
    jz .L12_store
    sub rax, rdx
    jo .L12_over
    jmp .L12_store
.L12_mul:
    test rcx, rcx
    jz .L12_result_zero
    test rdx, rdx
    jz .L12_result_zero
    cmp rdx, 1
    je .L12_store
    cmp rdx, -1
    je .L12_neg
    imul rax, rdx
    jo .L12_over
    jmp .L12_store
.L12_neg:
    neg rax
    jo .L12_over
    jmp .L12_store
.L12_div:
    test rdx, rdx
    jz .L12_zero
    cmp rdx, 1
    je .L12_unit
    cmp rdx, -1
    je .L12_negative_unit
    test rax, rax
    jz .L12_result_zero
    mov r10, rdx
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L12_store
.L12_negative_unit:
    neg rax
    jo .L12_over
.L12_unit:
    cmp r8d, 5
    je .L12_result_zero
    jmp .L12_store
.L12_cmp:
    cmp rcx, rdx
    jl .L12_less
    jg .L12_more
.L12_result_zero:
    xor eax, eax
    jmp .L12_store
.L12_less:
    mov rax, -1
    jmp .L12_store
.L12_more:
    mov eax, 1
.L12_store:
    mov [r9], rax
    xor eax, eax
    ret
.L12_over:
    mov eax, 1
    ret
.L12_zero:
    mov eax, 2
    ret
