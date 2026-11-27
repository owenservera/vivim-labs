/* Smoke comparator seed: compact opcode jump tree, overflow captured by a conditional move of the status. This is a candidate, not sufficient smoke-only qualification.
 * Win64 i64 development ABI. Independently authored proposal precursor. */
.text
.globl p002_seed
p002_seed:
    mov rax, rcx
    mov r10d, r8d
    test r10d,r10d
    jz .L02_add
    dec r10d
    jz .L02_sub
    dec r10d
    jz .L02_mul
    dec r10d
    jz .L02_div
    dec r10d
    jz .L02_neg
    dec r10d
    jz .L02_div
    dec r10d
    jz .L02_cmp
    jmp .L02_invalid
.L02_add:
    add rax, rdx
    seto r10b
    jmp .L02_capture
    jmp .L02_store
.L02_sub:
    sub rax, rdx
    seto r10b
    jmp .L02_capture
    jmp .L02_store
.L02_mul:
    imul rax, rdx
    seto r10b
    jmp .L02_capture
    jmp .L02_store
.L02_neg:
    neg rax
    seto r10b
    jmp .L02_capture
    jmp .L02_store
.L02_div:
    test rdx, rdx
    jz .L02_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L02_idiv
    cmp r10, -1
    je .L02_over
.L02_idiv:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L02_store
.L02_cmp:
    xor eax, eax
    cmp rcx, rdx
    setl al
    neg rax
    mov r10d, 1
    cmp rcx, rdx
    cmovg rax, r10
    jmp .L02_store
.L02_less:
    mov rax, -1
    jmp .L02_store
.L02_more:
    mov eax, 1
    jmp .L02_store
.L02_capture:
    movzx r10d, r10b
    test r10d, r10d
    jnz .L02_over
    jmp .L02_store
.L02_store:
    mov [r9], rax
    xor eax, eax
    ret
.L02_over:
    mov eax, 1
    ret
.L02_zero:
    mov eax, 2
    ret
.L02_invalid:
    mov eax, 3
    ret
