/* Dependency DAG: addition/subtraction/negation are explicit carry
 * propagation graphs with independent signed-overflow relations.
 */
.text
.globl p007_seed
p007_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L07_add
    cmp r8d, 1
    je .L07_sub
    cmp r8d, 2
    je .L07_mul
    cmp r8d, 3
    je .L07_div
    cmp r8d, 4
    je .L07_neg
    cmp r8d, 5
    je .L07_div
    cmp r8d, 6
    je .L07_cmp
    mov eax, 3
    ret
.L07_add:
    mov [rsp+8], rcx
    mov [rsp+16], rdx
    jmp .L07_carry
.L07_sub:
    mov [rsp+8], rcx
    mov [rsp+16], rdx
    /* a-b = a + (~b) + 1 modulo 2^64. */
    not rdx
    inc rdx
    jmp .L07_carry
.L07_neg:
    mov r10, 0x8000000000000000
    cmp rax, r10
    je .L07_over
    not rax
    mov edx, 1
    jmp .L07_neg_carry
.L07_carry:
    test rdx, rdx
    jz .L07_sign
    mov r10, rax
    and r10, rdx
    xor rax, rdx
    shl r10, 1
    mov rdx, r10
    jmp .L07_carry
.L07_neg_carry:
    test rdx, rdx
    jz .L07_store
    mov r10, rax
    and r10, rdx
    xor rax, rdx
    shl r10, 1
    mov rdx, r10
    jmp .L07_neg_carry
.L07_sign:
    mov r10, [rsp+8]
    mov r11, [rsp+16]
    xor r11, r10
    xor r10, rax
    cmp r8d, 1
    je .L07_sub_sign
    not r11
.L07_sub_sign:
    test r10, r11
    js .L07_over
    jmp .L07_store
.L07_mul:
    imul rdx
    mov r10, rax
    sar r10, 63
    cmp r10, rdx
    jne .L07_over
    jmp .L07_store
.L07_div:
    test rdx, rdx
    jz .L07_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L07_idiv
    cmp r10, -1
    je .L07_over
.L07_idiv:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L07_store
.L07_cmp:
    /* Signed order from sign difference, otherwise unsigned comparison. */
    mov r10, rcx
    xor r10, rdx
    js .L07_sign_cmp
    cmp rcx, rdx
    jb .L07_less
    ja .L07_more
    xor eax, eax
    jmp .L07_store
.L07_sign_cmp:
    test rcx, rcx
    js .L07_less
.L07_more:
    mov eax, 1
    jmp .L07_store
.L07_less:
    mov rax, -1
.L07_store:
    mov [r9], rax
    xor eax, eax
    ret
.L07_over:
    mov eax, 1
    ret
.L07_zero:
    mov eax, 2
    ret
