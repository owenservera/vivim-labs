/* Inverse: division qualification by quotient/remainder reconstruction,
 * remainder magnitude and sign, and overflow-safe quotient identity.
 * Addition and subtraction check the inverse relation; multiplication
 * returns checked hardware product. Volatile registers and shadow only. */
.text
.globl p014_seed
p014_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L14_add
    cmp r8d, 1
    je .L14_sub
    cmp r8d, 2
    je .L14_mul
    cmp r8d, 3
    je .L14_div
    cmp r8d, 4
    je .L14_neg
    cmp r8d, 5
    je .L14_div
    cmp r8d, 6
    je .L14_cmp
    jmp .L14_invalid
.L14_add:
    add rax, rdx
    jo .L14_over
    mov r10, rax
    sub r10, rcx
    cmp r10, rdx
    jne .L14_invalid
    jmp .L14_store
.L14_sub:
    sub rax, rdx
    jo .L14_over
    mov r10, rcx
    sub r10, rax
    cmp r10, rdx
    jne .L14_invalid
    jmp .L14_store
.L14_mul:
    imul rax, rdx
    jo .L14_over
    jmp .L14_store
.L14_neg:
    neg rax
    jo .L14_over
    jmp .L14_store
.L14_div:
    test rdx, rdx
    jz .L14_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L14_execute
    cmp r10, -1
    je .L14_over
.L14_execute:
    cqo
    idiv r10
    mov r11, rax
    imul r11, r10
    add r11, rdx
    cmp r11, rcx
    jne .L14_invalid
    test rdx, rdx
    jz .L14_select
    mov r11, rdx
    xor r11, rcx
    js .L14_invalid
    mov r11, rdx
    test r11, r11
    jns .L14_rem_abs
    neg r11
.L14_rem_abs:
    test r10, r10
    jns .L14_div_abs
    neg r10
.L14_div_abs:
    cmp r11, r10
    jae .L14_invalid
.L14_select:
    cmp r8d, 5
    cmove rax, rdx
    jmp .L14_store
.L14_cmp:
    xor eax, eax
    xor r10d, r10d
    cmp rcx, rdx
    setg al
    setl r10b
    sub rax, r10
.L14_store:
    mov [r9], rax
    xor eax, eax
    ret
.L14_over:
    mov eax, 1
    ret
.L14_zero:
    mov eax, 2
    ret
.L14_invalid:
    mov eax, 3
    ret
