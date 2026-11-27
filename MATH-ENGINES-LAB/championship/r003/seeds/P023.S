/* Obligation Lattice: explicit unsigned carry and borrow results with
 * independent sign obligations; 16-bit cells are assembled before range
 * qualification. Multiply and division discharge hardware/status checks. */
.text
.globl p023_seed
p023_seed:
    mov rax, rcx
    cmp r8d, 0
    je .L23_add
    cmp r8d, 1
    je .L23_sub
    cmp r8d, 2
    je .L23_mul
    cmp r8d, 3
    je .L23_div
    cmp r8d, 4
    je .L23_neg
    cmp r8d, 5
    je .L23_div
    cmp r8d, 6
    je .L23_cmp
    mov eax, 3
    ret
.L23_add:
    mov [rsp+8], rcx
    mov [rsp+16], rdx
    mov r10, rcx
    mov r11, rdx
    /* Four 16-bit additions. Shifts do not preserve carry, so save carry
       as an explicit obligation between cells. */
    xor eax, eax
    xor ecx, ecx
    mov word ptr [rsp+24], 0
.L23_add_cell:
    movzx edx, r10w
    movzx r8d, r11w
    add edx, r8d
    movzx r8d, word ptr [rsp+24]
    add edx, r8d
    mov r8d, edx
    shr r8d, 16
    mov [rsp+24], r8w
    movzx edx, dx
    shl rdx, cl
    or rax, rdx
    shr r10, 16
    shr r11, 16
    add ecx, 16
    cmp ecx, 64
    jne .L23_add_cell
    mov r10, [rsp+8]
    mov r11, [rsp+16]
    xor r11, r10
    not r11
    xor r10, rax
    test r11, r10
    js .L23_over
    jmp .L23_store
.L23_sub:
    mov r10, rcx
    xor r10, rdx
    sub rax, rdx
    xor rcx, rax
    test rcx, r10
    js .L23_over
    jmp .L23_store
.L23_mul:
    imul rdx
    mov r10, rax
    sar r10, 63
    cmp r10, rdx
    jne .L23_over
    jmp .L23_store
.L23_neg:
    neg rax
    jo .L23_over
    jmp .L23_store
.L23_div:
    test rdx, rdx
    jz .L23_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L23_idiv
    cmp r10, -1
    je .L23_over
.L23_idiv:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L23_store
.L23_cmp:
    xor eax, eax
    xor r10d, r10d
    cmp rcx, rdx
    setg al
    setl r10b
    sub rax, r10
.L23_store:
    mov [r9], rax
    xor eax, eax
    ret
.L23_over:
    mov eax, 1
    ret
.L23_zero:
    mov eax, 2
    ret
