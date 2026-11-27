/* Predator: compare hardware multiplication with an alternative low-word bit decomposition for small magnitudes; alternate path deliberately targets multiplication mutations.
 * Win64 i64 development ABI. Independently authored proposal precursor. */
.text
.globl p015_seed
p015_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L15_add
    cmp r8d, 1
    je .L15_sub
    cmp r8d, 2
    je .L15_mul
    cmp r8d, 3
    je .L15_div
    cmp r8d, 4
    je .L15_neg
    cmp r8d, 5
    je .L15_div
    cmp r8d, 6
    je .L15_cmp
    jmp .L15_invalid
.L15_add:
    add rax, rdx
    jo .L15_over
    jmp .L15_store
.L15_sub:
    sub rax, rdx
    jo .L15_over
    jmp .L15_store
.L15_mul:
    imul rax, rdx
    jo .L15_over
    cmp rdx, 255
    ja .L15_store
    xor r10d, r10d
    mov r11, rcx
.L15_hunt:
    test rdx, rdx
    jz .L15_hunt_done
    test dl, 1
    jz .L15_hunt_shift
    add r10, r11
.L15_hunt_shift:
    shl r11, 1
    shr rdx, 1
    jmp .L15_hunt
.L15_hunt_done:
    cmp r10, rax
    jne .L15_invalid
    jmp .L15_store
.L15_neg:
    neg rax
    jo .L15_over
    jmp .L15_store
.L15_div:
    test rdx, rdx
    jz .L15_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L15_idiv
    cmp r10, -1
    je .L15_over
.L15_idiv:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L15_store
.L15_cmp:
    xor eax, eax
    xor r10d, r10d
    cmp rcx, rdx
    setg al
    setl r10b
    sub rax, r10
    jmp .L15_store
.L15_less:
    mov rax, -1
    jmp .L15_store
.L15_more:
    mov eax, 1
    jmp .L15_store

.L15_store:
    mov [r9], rax
    xor eax, eax
    ret
.L15_over:
    mov eax, 1
    ret
.L15_zero:
    mov eax, 2
    ret
.L15_invalid:
    mov eax, 3
    ret
