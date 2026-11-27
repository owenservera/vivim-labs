/* Unsigned-limb witness: use restoring division rather than DIV/IDIV.
 * Other operations check signed ALU overflow; division uses explicit sign
 * reconstruction and a 64-step quotient/remainder state machine.
 */
.text
.globl p005_seed
p005_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L05_add
    cmp r8d, 1
    je .L05_sub
    cmp r8d, 2
    je .L05_mul
    cmp r8d, 3
    je .L05_div
    cmp r8d, 4
    je .L05_neg
    cmp r8d, 5
    je .L05_div
    cmp r8d, 6
    je .L05_cmp
    mov eax, 3
    ret
.L05_add:
    add rax, rdx
    jo .L05_over
    jmp .L05_store
.L05_sub:
    sub rax, rdx
    jo .L05_over
    jmp .L05_store
.L05_mul:
    imul rax, rdx
    jo .L05_over
    jmp .L05_store
.L05_neg:
    neg rax
    jo .L05_over
    jmp .L05_store
.L05_div:
    test rdx, rdx
    jz .L05_zero
    mov r11, rax
    sar r11, 63
    mov [rsp+8], r11
    xor rax, r11
    sub rax, r11
    mov r10, rdx
    sar r10, 63
    xor r11, r10
    mov [rsp+16], r11
    xor rdx, r10
    sub rdx, r10
    mov r10, rdx
    xor edx, edx
    mov ecx, 64
.L05_bit:
    shl rax, 1
    rcl rdx, 1
    cmp rdx, r10
    jb .L05_next
    sub rdx, r10
    or rax, 1
.L05_next:
    dec ecx
    jnz .L05_bit
    mov r11, [rsp+16]
    test r11, r11
    js .L05_q_ok
    test rax, rax
    js .L05_over
.L05_q_ok:
    cmp r8d, 5
    jne .L05_sign
    mov rax, rdx
    mov r11, [rsp+8]
.L05_sign:
    xor rax, r11
    sub rax, r11
    jmp .L05_store
.L05_cmp:
    xor eax, eax
    xor r10d, r10d
    cmp rcx, rdx
    setg al
    setl r10b
    sub rax, r10
.L05_store:
    mov [r9], rax
    xor eax, eax
    ret
.L05_over:
    mov eax, 1
    ret
.L05_zero:
    mov eax, 2
    ret
