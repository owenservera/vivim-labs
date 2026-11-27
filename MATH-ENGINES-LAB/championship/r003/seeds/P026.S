/* Cellular fabric: 32-bit limb add/sub cells, signed overflow from high cell;
 * signed magnitude hardware multiplication/division for the other cells.
 * Win64 ABI, same independent contract as other entrants.
 */
.text
.globl p026_seed
p026_seed:
    cmp r8d, 0
    je .L26_add
    cmp r8d, 1
    je .L26_sub
    cmp r8d, 2
    je .L26_mul
    cmp r8d, 3
    je .L26_div
    cmp r8d, 4
    je .L26_neg
    cmp r8d, 5
    je .L26_div
    cmp r8d, 6
    je .L26_cmp
    mov eax, 3
    ret
.L26_add:
    mov r10, rcx
    shr r10, 32
    mov r11, rdx
    shr r11, 32
    mov eax, ecx
    add eax, edx
    adc r10d, r11d
    jo .L26_over
    jmp .L26_join
.L26_sub:
    mov r10, rcx
    shr r10, 32
    mov r11, rdx
    shr r11, 32
    mov eax, ecx
    sub eax, edx
    sbb r10d, r11d
    jo .L26_over
    jmp .L26_join
.L26_neg:
    mov r11, rcx
    shr r11, 32
    xor r10d, r10d
    xor eax, eax
    sub eax, ecx
    sbb r10d, r11d
    jo .L26_over
.L26_join:
    shl r10, 32
    or rax, r10
    jmp .L26_store
.L26_mul:
    mov rax, rcx
    imul rdx
    mov r10, rax
    sar r10, 63
    cmp rdx, r10
    jne .L26_over
    jmp .L26_store
.L26_div:
    test rdx, rdx
    jz .L26_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rcx, r11
    jne .L26_do_div
    cmp r10, -1
    je .L26_over
.L26_do_div:
    mov rax, rcx
    cqo
    idiv r10
    cmp r8d, 5
    jne .L26_store
    mov rax, rdx
    jmp .L26_store
.L26_cmp:
    xor r10d, r10d
    xor eax, eax
    cmp rcx, rdx
    setg al
    setl r10b
    sub rax, r10
.L26_store:
    mov [r9], rax
    xor eax, eax
    ret
.L26_over:
    mov eax, 1
    ret
.L26_zero:
    mov eax, 2
    ret
