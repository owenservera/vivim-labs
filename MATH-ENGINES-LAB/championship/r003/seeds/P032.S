/* Instruction Witness: explicit flag capture and no writes until status checked.
 * ABI uses only volatile registers. Arithmetic flags are captured immediately.
 */
.text
.globl p032_seed
p032_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L32_add
    cmp r8d, 1
    je .L32_sub
    cmp r8d, 2
    je .L32_mul
    cmp r8d, 3
    je .L32_div
    cmp r8d, 4
    je .L32_neg
    cmp r8d, 5
    je .L32_div
    cmp r8d, 6
    je .L32_cmp
    mov eax, 3
    ret
.L32_add:
    add rax, rdx
    jmp .L32_capture
.L32_sub:
    sub rax, rdx
    jmp .L32_capture
.L32_mul:
    imul rax, rdx
    jmp .L32_capture
.L32_neg:
    neg rax
.L32_capture:
    seto r10b
    test r10b, r10b
    jnz .L32_over
    jmp .L32_store
.L32_div:
    test rdx, rdx
    jz .L32_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L32_do_div
    cmp r10, -1
    je .L32_over
.L32_do_div:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L32_store
.L32_cmp:
    xor r10d, r10d
    xor r11d, r11d
    cmp rcx, rdx
    setg r10b
    setl r11b
    mov rax, r10
    sub rax, r11
.L32_store:
    mov [r9], rax
    xor eax, eax
    ret
.L32_over:
    mov eax, 1
    ret
.L32_zero:
    mov eax, 2
    ret
