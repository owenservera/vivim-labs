/* Archaeology: translate input limbs through an explicitly versioned byte-swapped shadow representation and recover them before native computation. Representation migration precursor.
 * Win64 i64 development ABI. Independently authored proposal precursor. */
.text
.globl p017_seed
p017_seed:
    mov r10, rcx
    mov r11, rdx
    bswap r10
    bswap r11
    mov [rsp+8], r10
    mov [rsp+16], r11
    mov rcx, [rsp+8]
    mov rdx, [rsp+16]
    bswap rcx
    bswap rdx
    mov rax, rcx
    cmp r8d, 0
    je .L17_add
    cmp r8d, 1
    je .L17_sub
    cmp r8d, 2
    je .L17_mul
    cmp r8d, 3
    je .L17_div
    cmp r8d, 4
    je .L17_neg
    cmp r8d, 5
    je .L17_div
    cmp r8d, 6
    je .L17_cmp
    jmp .L17_invalid
.L17_add:
    add rax, rdx
    jo .L17_over
    jmp .L17_store
.L17_sub:
    sub rax, rdx
    jo .L17_over
    jmp .L17_store
.L17_mul:
    imul rax, rdx
    jo .L17_over
    jmp .L17_store
.L17_neg:
    neg rax
    jo .L17_over
    jmp .L17_store
.L17_div:
    test rdx, rdx
    jz .L17_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L17_idiv
    cmp r10, -1
    je .L17_over
.L17_idiv:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L17_store
.L17_cmp:
    cmp rcx, rdx
    jl .L17_less
    jg .L17_more
    xor eax, eax
    jmp .L17_store
.L17_less:
    mov rax, -1
    jmp .L17_store
.L17_more:
    mov eax, 1
    jmp .L17_store

.L17_store:
    mov [r9], rax
    xor eax, eax
    ret
.L17_over:
    mov eax, 1
    ret
.L17_zero:
    mov eax, 2
    ret
.L17_invalid:
    mov eax, 3
    ret
