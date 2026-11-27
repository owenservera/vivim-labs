/* Transplant: distinct per-operation leaf organs with a stable dispatcher.
 * Each organ accepts the same ABI, enabling independently tested binding.
 * This version is statically linked; no live transplant claimed.
 */
.text
.globl p029_seed
p029_seed:
    cmp r8d, 0
    je p029_add
    cmp r8d, 1
    je p029_sub
    cmp r8d, 2
    je p029_mul
    cmp r8d, 3
    je p029_divrem
    cmp r8d, 4
    je p029_neg
    cmp r8d, 5
    je p029_divrem
    cmp r8d, 6
    je p029_cmp
    mov eax, 3
    ret
.globl p029_add
p029_add:
    mov rax, rcx
    add rax, rdx
    jo .L29_over
    jmp .L29_commit
.globl p029_sub
p029_sub:
    mov rax, rcx
    sub rax, rdx
    jo .L29_over
    jmp .L29_commit
.globl p029_mul
p029_mul:
    mov rax, rcx
    imul rax, rdx
    jo .L29_over
    jmp .L29_commit
.globl p029_neg
p029_neg:
    mov rax, rcx
    neg rax
    jo .L29_over
    jmp .L29_commit
.globl p029_divrem
p029_divrem:
    test rdx, rdx
    jz .L29_zero
    mov r10, rdx
    mov rax, rcx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L29_idiv
    cmp r10, -1
    je .L29_over
.L29_idiv:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L29_commit
.globl p029_cmp
p029_cmp:
    xor eax, eax
    xor r10d, r10d
    cmp rcx, rdx
    setg al
    setl r10b
    sub rax, r10
.L29_commit:
    mov [r9], rax
    xor eax, eax
    ret
.L29_over:
    mov eax, 1
    ret
.L29_zero:
    mov eax, 2
    ret
