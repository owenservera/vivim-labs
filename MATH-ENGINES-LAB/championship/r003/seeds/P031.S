/* Constitutional Succession: predecessor and candidate subtraction forms agree on valid inputs, independently compare overflow status using signed operand relation. Governance migration is not implemented by this seed.
 * Win64 i64 development ABI. Independently authored proposal precursor. */
.text
.globl p031_seed
p031_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L31_add
    cmp r8d, 1
    je .L31_sub
    cmp r8d, 2
    je .L31_mul
    cmp r8d, 3
    je .L31_div
    cmp r8d, 4
    je .L31_neg
    cmp r8d, 5
    je .L31_div
    cmp r8d, 6
    je .L31_cmp
    jmp .L31_invalid
.L31_add:
    add rax, rdx
    jo .L31_over
    jmp .L31_store
.L31_sub:
    mov r10, rcx
    xor r10, rdx
    sub rax, rdx
    seto r11b
    xor rcx, rax
    test r10, rcx
    sets r10b
    cmp r10b, r11b
    jne .L31_invalid
    test r11b, r11b
    jnz .L31_over
    mov r10, rdx
    not r10
    lea r10, [rcx+r10+1]
/* rcx is sign-relation scratch, restore original from result + rhs */
    mov r10, rax
    add r10, rdx
    mov r11, rdx
    not r11
    lea r10, [r10+r11+1]
    cmp r10, rax
    jne .L31_invalid
    jmp .L31_store
.L31_mul:
    imul rax, rdx
    jo .L31_over
    jmp .L31_store
.L31_neg:
    neg rax
    jo .L31_over
    jmp .L31_store
.L31_div:
    test rdx, rdx
    jz .L31_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L31_idiv
    cmp r10, -1
    je .L31_over
.L31_idiv:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L31_store
.L31_cmp:
    xor eax, eax
    xor r10d, r10d
    cmp rcx, rdx
    setg al
    setl r10b
    sub rax, r10
    jmp .L31_store
.L31_less:
    mov rax, -1
    jmp .L31_store
.L31_more:
    mov eax, 1
    jmp .L31_store

.L31_store:
    mov [r9], rax
    xor eax, eax
    ret
.L31_over:
    mov eax, 1
    ret
.L31_zero:
    mov eax, 2
    ret
.L31_invalid:
    mov eax, 3
    ret
