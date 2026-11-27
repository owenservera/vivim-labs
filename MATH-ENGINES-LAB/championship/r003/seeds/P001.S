/* Hash-only comparator seed: direct flag-checked ALU, single return site. No self-certification or promotion claim.
 * Win64 i64 development ABI. Independently authored proposal precursor. */
.text
.globl p001_seed
p001_seed:
    mov rax, rcx
    cmp r8d, 3
    ja .L01_upper
    cmp r8d, 1
    ja .L01_middle
    je .L01_sub
    jmp .L01_add
.L01_middle:
    cmp r8d, 2
    je .L01_mul
    jmp .L01_div
.L01_upper:
    cmp r8d, 5
    ja .L01_last
    je .L01_div
    jmp .L01_neg
.L01_last:
    cmp r8d, 6
    je .L01_cmp
    jmp .L01_invalid
.L01_add:
    add rax, rdx
    jo .L01_over
    jmp .L01_store
.L01_sub:
    sub rax, rdx
    jo .L01_over
    jmp .L01_store
.L01_mul:
    imul rax, rdx
    jo .L01_over
    jmp .L01_store
.L01_neg:
    neg rax
    jo .L01_over
    jmp .L01_store
.L01_div:
    test rdx, rdx
    jz .L01_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L01_idiv
    cmp r10, -1
    je .L01_over
.L01_idiv:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L01_store
.L01_cmp:
    cmp rcx, rdx
    mov rax, -1
    jl .L01_store
    mov eax, 1
    jg .L01_store
    xor eax, eax
    jmp .L01_store
.L01_less:
    mov rax, -1
    jmp .L01_store
.L01_more:
    mov eax, 1
    jmp .L01_store

.L01_store:
    mov [r9], rax
    xor eax, eax
    ret
.L01_over:
    mov eax, 1
    ret
.L01_zero:
    mov eax, 2
    ret
.L01_invalid:
    mov eax, 3
    ret
