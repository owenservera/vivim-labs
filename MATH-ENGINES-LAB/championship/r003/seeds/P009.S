/* Certificate: derive a local carry/sign relation independently from the ALU overflow flag; disagreement is invalid. Local checks are not a universal certificate.
 * Win64 i64 development ABI. Independently authored proposal precursor. */
.text
.globl p009_seed
p009_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L09_add
    cmp r8d, 1
    je .L09_sub
    cmp r8d, 2
    je .L09_mul
    cmp r8d, 3
    je .L09_div
    cmp r8d, 4
    je .L09_neg
    cmp r8d, 5
    je .L09_div
    cmp r8d, 6
    je .L09_cmp
    jmp .L09_invalid
.L09_add:
    mov r10, rcx
    xor r10, rdx
    not r10
    add rax, rdx
    seto r11b
    xor rcx, rax
    test rcx, r10
    sets r10b
    cmp r10b, r11b
    jne .L09_invalid
    test r10b, r10b
    jnz .L09_over
    jmp .L09_store
.L09_sub:
    mov r10, rcx
    xor r10, rdx
    sub rax, rdx
    seto r11b
    xor rcx, rax
    test rcx, r10
    sets r10b
    cmp r10b, r11b
    jne .L09_invalid
    test r10b, r10b
    jnz .L09_over
    jmp .L09_store
.L09_mul:
    imul rdx
    seto r10b
    mov r11, rax
    sar r11, 63
    cmp r11, rdx
    setne r11b
    cmp r10b, r11b
    jne .L09_invalid
    test r10b, r10b
    jnz .L09_over
    jmp .L09_store
.L09_neg:
    neg rax
    jo .L09_over
    jmp .L09_store
.L09_div:
    test rdx, rdx
    jz .L09_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L09_idiv
    cmp r10, -1
    je .L09_over
.L09_idiv:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L09_store
.L09_cmp:
    xor eax, eax
    xor r10d, r10d
    cmp rcx, rdx
    setg al
    setl r10b
    sub rax, r10
    jmp .L09_store
.L09_less:
    mov rax, -1
    jmp .L09_store
.L09_more:
    mov eax, 1
    jmp .L09_store

.L09_store:
    mov [r9], rax
    xor eax, eax
    ret
.L09_over:
    mov eax, 1
    ret
.L09_zero:
    mov eax, 2
    ret
.L09_invalid:
    mov eax, 3
    ret
