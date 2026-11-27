/* Independent Win64 ABI probe. Only for reviewed statically linked leaf seeds.
 * Args: rcx=seed, rdx=a, r8=b, r9d=opcode, [rsp+40]=out.
 * Return: low 32 bits native status; upper 32 bits violation mask.
 * Mask bit 0 nonvolatile GPR, bit 1 XMM6-15, bit 2 direction flag,
 * bit 3 MXCSR control bits, bit 4 x87 control word.
 * Restores caller's state. Stack corruption that prevents return is a crash,
 * not a detected bit; run only in crash-contained development processes. */
.text
.globl r003_abi_probe
r003_abi_probe:
    push rbx
    push rbp
    push rsi
    push rdi
    push r12
    push r13
    push r14
    push r15
    sub rsp, 248
    movdqu [rsp+32], xmm6
    movdqu [rsp+48], xmm7
    movdqu [rsp+64], xmm8
    movdqu [rsp+80], xmm9
    movdqu [rsp+96], xmm10
    movdqu [rsp+112], xmm11
    movdqu [rsp+128], xmm12
    movdqu [rsp+144], xmm13
    movdqu [rsp+160], xmm14
    movdqu [rsp+176], xmm15
    stmxcsr [rsp+192]
    fnstcw [rsp+200]
    mov r10, rcx
    mov rcx, rdx
    mov rdx, r8
    mov r8d, r9d
    mov r9, [rsp+352]
    mov rbx, 0x11111111
    mov rbp, 0x22222222
    mov rsi, 0x33333333
    mov rdi, 0x44444444
    mov r12, 0x55555555
    mov r13, 0x66666666
    mov r14, 0x77777777
    mov r15, 0x88888888
    pcmpeqd xmm6, xmm6
    pcmpeqd xmm7, xmm7
    pcmpeqd xmm8, xmm8
    pcmpeqd xmm9, xmm9
    pcmpeqd xmm10, xmm10
    pcmpeqd xmm11, xmm11
    pcmpeqd xmm12, xmm12
    pcmpeqd xmm13, xmm13
    pcmpeqd xmm14, xmm14
    pcmpeqd xmm15, xmm15
    call r10
    mov [rsp+208], eax
    xor r10d, r10d
    mov r11d, 0x11111111
    cmp rbx, r11
    jne .Labi_gpr_bad
    mov r11d, 0x22222222
    cmp rbp, r11
    jne .Labi_gpr_bad
    mov r11d, 0x33333333
    cmp rsi, r11
    jne .Labi_gpr_bad
    mov r11d, 0x44444444
    cmp rdi, r11
    jne .Labi_gpr_bad
    mov r11d, 0x55555555
    cmp r12, r11
    jne .Labi_gpr_bad
    mov r11d, 0x66666666
    cmp r13, r11
    jne .Labi_gpr_bad
    mov r11d, 0x77777777
    cmp r14, r11
    jne .Labi_gpr_bad
    mov r11d, 0x88888888
    cmp r15, r11
    jne .Labi_gpr_bad
    jmp .Labi_xmm
.Labi_gpr_bad:
    or r10d, 1
.Labi_xmm:
    pcmpeqd xmm0, xmm0
    movdqa xmm1, xmm6
    pcmpeqb xmm1, xmm0
    pmovmskb r11d, xmm1
    cmp r11d, 65535
    jne .Labi_xmm_bad
    movdqa xmm1, xmm7
    pcmpeqb xmm1, xmm0
    pmovmskb r11d, xmm1
    cmp r11d, 65535
    jne .Labi_xmm_bad
    movdqa xmm1, xmm8
    pcmpeqb xmm1, xmm0
    pmovmskb r11d, xmm1
    cmp r11d, 65535
    jne .Labi_xmm_bad
    movdqa xmm1, xmm9
    pcmpeqb xmm1, xmm0
    pmovmskb r11d, xmm1
    cmp r11d, 65535
    jne .Labi_xmm_bad
    movdqa xmm1, xmm10
    pcmpeqb xmm1, xmm0
    pmovmskb r11d, xmm1
    cmp r11d, 65535
    jne .Labi_xmm_bad
    movdqa xmm1, xmm11
    pcmpeqb xmm1, xmm0
    pmovmskb r11d, xmm1
    cmp r11d, 65535
    jne .Labi_xmm_bad
    movdqa xmm1, xmm12
    pcmpeqb xmm1, xmm0
    pmovmskb r11d, xmm1
    cmp r11d, 65535
    jne .Labi_xmm_bad
    movdqa xmm1, xmm13
    pcmpeqb xmm1, xmm0
    pmovmskb r11d, xmm1
    cmp r11d, 65535
    jne .Labi_xmm_bad
    movdqa xmm1, xmm14
    pcmpeqb xmm1, xmm0
    pmovmskb r11d, xmm1
    cmp r11d, 65535
    jne .Labi_xmm_bad
    movdqa xmm1, xmm15
    pcmpeqb xmm1, xmm0
    pmovmskb r11d, xmm1
    cmp r11d, 65535
    jne .Labi_xmm_bad
    jmp .Labi_flags
.Labi_xmm_bad:
    or r10d, 2
.Labi_flags:
    pushfq
    pop r11
    test r11d, 1024
    jz .Labi_mxcsr
    or r10d, 4
.Labi_mxcsr:
    cld
    stmxcsr [rsp+196]
    mov r11d, [rsp+192]
    xor r11d, [rsp+196]
    test r11d, 0xffc0
    jz .Labi_x87
    or r10d, 8
.Labi_x87:
    fnstcw [rsp+202]
    movzx r11d, word ptr [rsp+200]
    cmp r11w, [rsp+202]
    je .Labi_restore
    or r10d, 16
.Labi_restore:
    ldmxcsr [rsp+192]
    fldcw [rsp+200]
    movdqu xmm6, [rsp+32]
    movdqu xmm7, [rsp+48]
    movdqu xmm8, [rsp+64]
    movdqu xmm9, [rsp+80]
    movdqu xmm10, [rsp+96]
    movdqu xmm11, [rsp+112]
    movdqu xmm12, [rsp+128]
    movdqu xmm13, [rsp+144]
    movdqu xmm14, [rsp+160]
    movdqu xmm15, [rsp+176]
    mov eax, [rsp+208]
    shl r10, 32
    or rax, r10
    add rsp, 248
    pop r15
    pop r14
    pop r13
    pop r12
    pop rdi
    pop rsi
    pop rbp
    pop rbx
    ret
/* Deliberately incorrect, reviewed negative controls. Never candidates. */
.globl r003_bad_gpr
r003_bad_gpr:
    xor ebx, ebx
    xor eax, eax
    ret
.globl r003_bad_xmm
r003_bad_xmm:
    pxor xmm6, xmm6
    xor eax, eax
    ret
.globl r003_bad_df
r003_bad_df:
    std
    xor eax, eax
    ret
.globl r003_bad_mxcsr
r003_bad_mxcsr:
    stmxcsr [rsp+8]
    xor dword ptr [rsp+8], 0x2000
    ldmxcsr [rsp+8]
    xor eax, eax
    ret
.globl r003_bad_x87
r003_bad_x87:
    fnstcw [rsp+8]
    xor word ptr [rsp+8], 0x0400
    fldcw [rsp+8]
    xor eax, eax
    ret
