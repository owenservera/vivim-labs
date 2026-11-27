/* Parallel Worlds: for add/sub/neg, translate two's-complement to
 * biased signed-order representation, operate on unsigned offsets,
 * and translate back. This is only an integer representation experiment;
 * decimal/rational world and independent translation proofs remain open. */
.text
.globl p016_seed
p016_seed:
    mov rax, rcx
    mov r10, 0x8000000000000000
    cmp r8d, 0
    je .L16_add
    cmp r8d, 1
    je .L16_sub
    cmp r8d, 2
    je .L16_mul
    cmp r8d, 3
    je .L16_div
    cmp r8d, 4
    je .L16_neg
    cmp r8d, 5
    je .L16_div
    cmp r8d, 6
    je .L16_cmp
    mov eax, 3
    ret
.L16_add:
    xor rax, r10
    test rdx, rdx
    js .L16_add_negative
    add rax, rdx
    jc .L16_over
    jmp .L16_unbias
.L16_add_negative:
    neg rdx
    sub rax, rdx
    jc .L16_over
    jmp .L16_unbias
.L16_sub:
    xor rax, r10
    test rdx, rdx
    js .L16_sub_negative
    sub rax, rdx
    jc .L16_over
    jmp .L16_unbias
.L16_sub_negative:
    neg rdx
    add rax, rdx
    jc .L16_over
.L16_unbias:
    xor rax, r10
    jmp .L16_store
.L16_neg:
    xor rax, r10
    test rax, rax
    jz .L16_over
    neg rax
    jmp .L16_unbias
.L16_mul:
    imul rdx
    mov r11, rax
    sar r11, 63
    cmp r11, rdx
    jne .L16_over
    jmp .L16_store
.L16_div:
    test rdx, rdx
    jz .L16_zero
    cmp rax, r10
    jne .L16_do
    cmp rdx, -1
    je .L16_over
.L16_do:
    mov r11, rdx
    cqo
    idiv r11
    cmp r8d, 5
    cmove rax, rdx
    jmp .L16_store
.L16_cmp:
    xor rcx, r10
    xor rdx, r10
    xor eax, eax
    xor r11d, r11d
    cmp rcx, rdx
    seta al
    setb r11b
    sub rax, r11
.L16_store:
    mov [r9], rax
    xor eax, eax
    ret
.L16_over:
    mov eax, 1
    ret
.L16_zero:
    mov eax, 2
    ret
