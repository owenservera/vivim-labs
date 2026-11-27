/* Sign-magnitude witness seed. Addition/subtraction use unsigned magnitudes,
 * effective sign and explicit signed range reconstruction; multiplication
 * uses the full unsigned product. Quotient/remainder use unsigned DIV.
 */
.text
.globl p004_seed
p004_seed:
    mov rax, rcx
    mov r10, rdx
    cmp r8d, 6
    je .L04_cmp
    cmp r8d, 4
    je .L04_neg
    cmp r8d, 5
    ja .L04_invalid
    mov r11, rax
    sar r11, 63
    mov [rsp+8], r11
    xor rax, r11
    sub rax, r11
    mov r11, r10
    sar r11, 63
    mov [rsp+16], r11
    xor r10, r11
    sub r10, r11
    cmp r8d, 2
    je .L04_mul
    cmp r8d, 3
    jae .L04_div
    cmp r8d, 1
    jne .L04_sum
    not qword ptr [rsp+16]
.L04_sum:
    mov r11, [rsp+8]
    cmp r11, [rsp+16]
    jne .L04_difference
    add rax, r10
    jc .L04_over
    jmp .L04_signed
.L04_difference:
    cmp rax, r10
    jae .L04_keep
    xchg rax, r10
    mov r11, [rsp+16]
.L04_keep:
    sub rax, r10
    jmp .L04_signed
.L04_mul:
    mul r10
    test rdx, rdx
    jnz .L04_over
    mov r11, [rsp+8]
    xor r11, [rsp+16]
    jmp .L04_signed
.L04_div:
    test r10, r10
    jz .L04_zero
    xor edx, edx
    div r10
    mov r11, [rsp+8]
    xor r11, [rsp+16]
    /* MIN/-1 fails for both quotient and remainder by contract. */
    test r11, r11
    js .L04_div_valid
    test rax, rax
    js .L04_over
.L04_div_valid:
    cmp r8d, 5
    jne .L04_signed
    mov rax, rdx
    mov r11, [rsp+8]
.L04_signed:
    test r11, r11
    js .L04_negative
    test rax, rax
    js .L04_over
    jmp .L04_store
.L04_negative:
    mov r10, 0x8000000000000000
    cmp rax, r10
    ja .L04_over
    neg rax
    jmp .L04_store
.L04_neg:
    mov r10, 0x8000000000000000
    cmp rax, r10
    je .L04_over
    not rax
    inc rax
    jmp .L04_store
.L04_cmp:
    cmp rcx, rdx
    jl .L04_less
    jg .L04_more
    xor eax, eax
    jmp .L04_store
.L04_less:
    mov rax, -1
    jmp .L04_store
.L04_more:
    mov eax, 1
.L04_store:
    mov [r9], rax
    xor eax, eax
    ret
.L04_over:
    mov eax, 1
    ret
.L04_zero:
    mov eax, 2
    ret
.L04_invalid:
    mov eax, 3
    ret
