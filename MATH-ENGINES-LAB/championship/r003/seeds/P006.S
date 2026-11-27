/* Optimistic: speculate arithmetic result, capture failure status,
 * commit exactly once only if status is zero. Signed division trap cases
 * are screened before executing IDIV. */
.text
.globl p006_seed
p006_seed:
    mov rax, rcx
    xor r11d, r11d
    cmp r8d, 0
    je .L06_add
    cmp r8d, 1
    je .L06_sub
    cmp r8d, 2
    je .L06_mul
    cmp r8d, 3
    je .L06_div
    cmp r8d, 4
    je .L06_neg
    cmp r8d, 5
    je .L06_div
    cmp r8d, 6
    je .L06_cmp
    mov r11d, 3
    jmp .L06_decide
.L06_add:
    add rax, rdx
    seto r11b
    jmp .L06_decide
.L06_sub:
    sub rax, rdx
    seto r11b
    jmp .L06_decide
.L06_mul:
    imul rax, rdx
    seto r11b
    jmp .L06_decide
.L06_neg:
    neg rax
    seto r11b
    jmp .L06_decide
.L06_div:
    test rdx, rdx
    jnz .L06_check
    mov r11d, 2
    jmp .L06_decide
.L06_check:
    mov r10, 0x8000000000000000
    cmp rax, r10
    jne .L06_idiv
    cmp rdx, -1
    jne .L06_idiv
    mov r11d, 1
    jmp .L06_decide
.L06_idiv:
    mov r10, rdx
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L06_decide
.L06_cmp:
    xor eax, eax
    xor r10d, r10d
    cmp rcx, rdx
    setg al
    setl r10b
    sub rax, r10
.L06_decide:
    test r11d, r11d
    jnz .L06_return
    mov [r9], rax
.L06_return:
    mov eax, r11d
    ret
