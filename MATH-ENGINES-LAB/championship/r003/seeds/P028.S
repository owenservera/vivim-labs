/* Negative-Space Atlas: sign-partitioned add/sub bounds are checked
 * BEFORE execution, independently of the ALU overflow flag.
 */
.text
.globl p028_seed
p028_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L28_add
    cmp r8d, 1
    je .L28_sub
    cmp r8d, 2
    je .L28_mul
    cmp r8d, 3
    je .L28_div
    cmp r8d, 4
    je .L28_neg
    cmp r8d, 5
    je .L28_div
    cmp r8d, 6
    je .L28_cmp
    mov eax, 3
    ret
.L28_add:
    test rdx, rdx
    js .L28_add_neg
    mov r10, 0x7fffffffffffffff
    sub r10, rdx
    cmp rcx, r10
    jg .L28_over
    jmp .L28_add_execute
.L28_add_neg:
    mov r10, 0x8000000000000000
    sub r10, rdx
    cmp rcx, r10
    jl .L28_over
.L28_add_execute:
    add rax, rdx
    jmp .L28_store
.L28_sub:
    test rdx, rdx
    js .L28_sub_neg
    mov r10, 0x8000000000000000
    add r10, rdx
    cmp rcx, r10
    jl .L28_over
    jmp .L28_sub_execute
.L28_sub_neg:
    mov r10, 0x7fffffffffffffff
    add r10, rdx
    cmp rcx, r10
    jg .L28_over
.L28_sub_execute:
    sub rax, rdx
    jmp .L28_store
.L28_mul:
    imul rdx
    mov r10, rax
    sar r10, 63
    cmp rdx, r10
    jne .L28_over
    jmp .L28_store
.L28_neg:
    mov r10, 0x8000000000000000
    cmp rax, r10
    je .L28_over
    neg rax
    jmp .L28_store
.L28_div:
    test rdx, rdx
    jz .L28_zero
    mov r10, rdx
    cmp r10, -1
    jne .L28_safe
    mov r11, 0x8000000000000000
    cmp rax, r11
    je .L28_over
.L28_safe:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L28_store
.L28_cmp:
    cmp rcx, rdx
    jl .L28_less
    jg .L28_more
    xor eax, eax
    jmp .L28_store
.L28_less:
    mov rax, -1
    jmp .L28_store
.L28_more:
    mov eax, 1
.L28_store:
    mov [r9], rax
    xor eax, eax
    ret
.L28_over:
    mov eax, 1
    ret
.L28_zero:
    mov eax, 2
    ret
