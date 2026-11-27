/* Embedded: native dispatch table, one actual arithmetic entrypoint.
 * Uses signed checked ALU instructions with a position-independent table.
 */
.text
.globl p011_seed
p011_seed:
    cmp r8d, 6
    ja .L11_invalid
    lea r10, [rip+.L11_table]
    movsxd r11, dword ptr [r10+r8*4]
    add r10, r11
    mov rax, rcx
    jmp r10
.L11_table:
    .long .L11_add-.L11_table
    .long .L11_sub-.L11_table
    .long .L11_mul-.L11_table
    .long .L11_div-.L11_table
    .long .L11_neg-.L11_table
    .long .L11_div-.L11_table
    .long .L11_cmp-.L11_table
.L11_add:
    add rax, rdx
    jo .L11_over
    jmp .L11_write
.L11_sub:
    sub rax, rdx
    jo .L11_over
    jmp .L11_write
.L11_mul:
    imul rax, rdx
    jo .L11_over
    jmp .L11_write
.L11_neg:
    neg rax
    jo .L11_over
    jmp .L11_write
.L11_div:
    test rdx, rdx
    jz .L11_zero
    mov r10, rdx
    mov r11, 0x8000000000000000
    cmp rax, r11
    jne .L11_idiv
    cmp r10, -1
    je .L11_over
.L11_idiv:
    cqo
    idiv r10
    cmp r8d, 5
    cmove rax, rdx
    jmp .L11_write
.L11_cmp:
    cmp rcx, rdx
    mov rax, -1
    jl .L11_write
    mov eax, 1
    jg .L11_write
    xor eax, eax
.L11_write:
    mov [r9], rax
    xor eax, eax
    ret
.L11_over:
    mov eax, 1
    ret
.L11_zero:
    mov eax, 2
    ret
.L11_invalid:
    mov eax, 3
    ret
