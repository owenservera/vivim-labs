/* Self-Verifier: compute independent Boolean sign/overflow rule for negation and sign relations for compare; checked status for other instructions. Checker evolution still open.
 * Win64 i64 development ABI. Independently authored proposal precursor. */
.text
.globl p021_seed
p021_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L21_add
    cmp r8d, 1
    je .L21_sub
    cmp r8d, 2
    je .L21_mul
    cmp r8d, 3
    je .L21_div
    cmp r8d, 4
    je .L21_neg
    cmp r8d, 5
    je .L21_div
    cmp r8d, 6
    je .L21_cmp
    jmp .L21_invalid
.L21_add:
    add rax, rdx
    jo .L21_over
    jmp .L21_store
.L21_sub:
    sub rax, rdx
    jo .L21_over
    jmp .L21_store
.L21_mul:
    imul rax, rdx
    jo .L21_over
    jmp .L21_store
.L21_neg:
    mov r10, 0x8000000000000000
    cmp rcx, r10
    sete r11b
    neg rax
    seto r10b
    cmp r10b, r11b
    jne .L21_invalid
    test r11b, r11b
    jnz .L21_over
    jmp .L21_store
.L21_div:
    test rdx, rdx
    jz .L21_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L21_idiv
    cmp r10, -1
    je .L21_over
.L21_idiv:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L21_store
.L21_cmp:
    mov r10, rcx
    xor r10, rdx
    js .L21_cmp_diff
    cmp rcx, rdx
    jb .L21_less
    ja .L21_more
    xor eax, eax
    jmp .L21_store
.L21_cmp_diff:
    test rcx, rcx
    js .L21_less
    jmp .L21_more
    jmp .L21_store
.L21_less:
    mov rax, -1
    jmp .L21_store
.L21_more:
    mov eax, 1
    jmp .L21_store

.L21_store:
    mov [r9], rax
    xor eax, eax
    ret
.L21_over:
    mov eax, 1
    ret
.L21_zero:
    mov eax, 2
    ret
.L21_invalid:
    mov eax, 3
    ret
