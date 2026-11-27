/* Shadow Twin precursor: checked ALU result challenged by a second
 * bounded carry-propagation computation for add/sub/neg and a shift-add
 * multiplication computation. Division independently checks reconstruction.
 * Both paths share a process and ABI; not yet independently evolved twins.
 */
.text
.globl p027_seed
p027_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L27_add
    cmp r8d, 1
    je .L27_sub
    cmp r8d, 2
    je .L27_mul
    cmp r8d, 3
    je .L27_div
    cmp r8d, 4
    je .L27_neg
    cmp r8d, 5
    je .L27_div
    cmp r8d, 6
    je .L27_cmp
    jmp .L27_invalid
.L27_add:
    add rax, rdx
    jo .L27_over
    jmp .L27_carry
.L27_sub:
    sub rax, rdx
    jo .L27_over
    neg rdx
    jmp .L27_carry
.L27_neg:
    neg rax
    jo .L27_over
    not rcx
    mov edx, 1
.L27_carry:
    test rdx, rdx
    jz .L27_compare
    mov r10, rcx
    and r10, rdx
    xor rcx, rdx
    shl r10, 1
    mov rdx, r10
    jmp .L27_carry
.L27_mul:
    imul rax, rdx
    jo .L27_over
    xor r10d, r10d
.L27_shiftadd:
    test rdx, rdx
    jz .L27_mul_compare
    test rdx, 1
    jz .L27_shift
    add r10, rcx
.L27_shift:
    shl rcx, 1
    shr rdx, 1
    jmp .L27_shiftadd
.L27_mul_compare:
    mov rcx, r10
.L27_compare:
    cmp rax, rcx
    jne .L27_invalid
    jmp .L27_store
.L27_div:
    test rdx, rdx
    jz .L27_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L27_idiv
    cmp r10, -1
    je .L27_over
.L27_idiv:
    cqo
    idiv r10
    mov r11, rax
    imul r11, r10
    add r11, rdx
    cmp r11, rcx
    jne .L27_invalid
    cmp r8d, 5
    cmove rax, rdx
    jmp .L27_store
.L27_cmp:
    xor eax, eax
    xor r10d, r10d
    cmp rcx, rdx
    setg al
    setl r10b
    sub rax, r10
.L27_store:
    mov [r9], rax
    xor eax, eax
    ret
.L27_over:
    mov eax, 1
    ret
.L27_zero:
    mov eax, 2
    ret
.L27_invalid:
    mov eax, 3
    ret
