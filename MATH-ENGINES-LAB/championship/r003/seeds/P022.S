/* Reversible Chronicle: independent forward/inverse checked arithmetic.
 * Win64 ABI: rcx=a, rdx=b, r8d=opcode, r9=out; return eax status.
 * 0 add,1 sub,2 mul,3 div,4 neg,5 rem,6 cmp. No output on failure.
 */
.text
.globl p022_seed
p022_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L22_add
    cmp r8d, 1
    je .L22_sub
    cmp r8d, 2
    je .L22_mul
    cmp r8d, 3
    je .L22_div
    cmp r8d, 4
    je .L22_neg
    cmp r8d, 5
    je .L22_div
    cmp r8d, 6
    je .L22_cmp
    mov eax, 3
    ret
.L22_add:
    add rax, rdx
    jo .L22_over
    mov r10, rax
    sub r10, rdx
    cmp r10, rcx
    jne .L22_invalid
    jmp .L22_store
.L22_sub:
    sub rax, rdx
    jo .L22_over
    mov r10, rax
    add r10, rdx
    cmp r10, rcx
    jne .L22_invalid
    jmp .L22_store
.L22_mul:
    imul rax, rdx
    jo .L22_over
    jmp .L22_store
.L22_neg:
    neg rax
    jo .L22_over
    mov r10, rax
    neg r10
    cmp r10, rcx
    jne .L22_invalid
    jmp .L22_store
.L22_div:
    test rdx, rdx
    jz .L22_zero
    mov r10, 0x8000000000000000
    cmp rax, r10
    jne .L22_do_div
    cmp rdx, -1
    je .L22_over
.L22_do_div:
    mov r10, rdx
    cqo
    idiv r10
    mov r11, rax
    imul r11, r10
    add r11, rdx
    cmp r11, rcx
    jne .L22_invalid
    cmp r8d, 5
    jne .L22_store
    mov rax, rdx
    jmp .L22_store
.L22_cmp:
    cmp rcx, rdx
    jl .L22_less
    jg .L22_more
    xor eax, eax
    jmp .L22_store
.L22_less:
    mov rax, -1
    jmp .L22_store
.L22_more:
    mov eax, 1
.L22_store:
    mov [r9], rax
    xor eax, eax
    ret
.L22_over:
    mov eax, 1
    ret
.L22_zero:
    mov eax, 2
    ret
.L22_invalid:
    mov eax, 3
    ret
