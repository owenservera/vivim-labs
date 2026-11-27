/* Dream: accumulator-style seed; multiplication by small nonnegative factors uses sequential checked accumulator updates, otherwise hardware IMUL.
 * Win64 i64 development ABI. Independently authored proposal precursor. */
.text
.globl p013_seed
p013_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L13_add
    cmp r8d, 1
    je .L13_sub
    cmp r8d, 2
    je .L13_mul
    cmp r8d, 3
    je .L13_div
    cmp r8d, 4
    je .L13_neg
    cmp r8d, 5
    je .L13_div
    cmp r8d, 6
    je .L13_cmp
    jmp .L13_invalid
.L13_add:
    add rax, rdx
    jo .L13_over
    jmp .L13_store
.L13_sub:
    sub rax, rdx
    jo .L13_over
    jmp .L13_store
.L13_mul:
    cmp rdx, 8
    ja .L13_mul_general
    xor eax, eax
    mov r10, rdx
.L13_acc:
    test r10, r10
    jz .L13_store
    add rax, rcx
    jo .L13_over
    dec r10
    jmp .L13_acc
.L13_mul_general:
    imul rax, rdx
    jo .L13_over
    jmp .L13_store
.L13_neg:
    xor eax, eax
    sub rax, rcx
    jo .L13_over
    jmp .L13_store
.L13_div:
    test rdx, rdx
    jz .L13_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L13_idiv
    cmp r10, -1
    je .L13_over
.L13_idiv:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L13_store
.L13_cmp:
    cmp rcx, rdx
    jl .L13_less
    jg .L13_more
    xor eax, eax
    jmp .L13_store
.L13_less:
    mov rax, -1
    jmp .L13_store
.L13_more:
    mov eax, 1
    jmp .L13_store

.L13_store:
    mov [r9], rax
    xor eax, eax
    ret
.L13_over:
    mov eax, 1
    ret
.L13_zero:
    mov eax, 2
    ret
.L13_invalid:
    mov eax, 3
    ret
