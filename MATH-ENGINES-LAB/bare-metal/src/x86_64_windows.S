/* x86-64 Windows ABI, leaf routines; no OS calls, no external math code.
 * rcx = pointer to lhs limbs, rdx = pointer to rhs limbs,
 * r8 = output pointer (two limbs, or four for divrem).
 * eax: 0 success, 1 signed overflow, 2 divide by zero.
 * Inputs/outputs are two's-complement little-endian 128-bit limbs.
 * Only volatile registers and caller-provided 32-byte shadow space.
 * Output is untouched on failure. No nonvolatile registers or stack adjustment.
 */
.text
.globl vivim_native_add128
vivim_native_add128:
    mov rax, [rcx]
    mov r9, [rcx+8]
    add rax, [rdx]
    adc r9, [rdx+8]
    jo .Loverflow
    mov [r8], rax
    mov [r8+8], r9
    xor eax, eax
    ret

.globl vivim_native_sub128
vivim_native_sub128:
    mov rax, [rcx]
    mov r9, [rcx+8]
    sub rax, [rdx]
    sbb r9, [rdx+8]
    jo .Loverflow
    mov [r8], rax
    mov [r8+8], r9
    xor eax, eax
    ret

.globl vivim_native_neg128
vivim_native_neg128:
    xor eax, eax
    xor r9d, r9d
    sub rax, [rcx]
    sbb r9, [rcx+8]
    jo .Loverflow
    mov [r8], rax
    mov [r8+8], r9
    xor eax, eax
    ret

.globl vivim_native_mul128
vivim_native_mul128:
    mov rax, [rcx]
    mov r9, [rcx+8]
    mov r10, [rdx]
    mov r11, [rdx+8]
    mov rcx, r9
    xor rcx, r11
    mov [rsp+8], rcx
    test r9, r9
    jns .Lmul_a_positive
    neg rax
    adc r9, 0
    neg r9
.Lmul_a_positive:
    test r11, r11
    jns .Lmul_b_positive
    neg r10
    adc r11, 0
    neg r11
.Lmul_b_positive:
    test r9, r9
    jz .Lmul_base
    test r11, r11
    jnz .Loverflow
.Lmul_base:
    mov [rsp+16], rax
    mul r10
    mov [rsp+24], rax
    mov rcx, rdx
    test r9, r9
    jnz .Lmul_cross_a
    test r11, r11
    jz .Lmul_sign
    mov rax, [rsp+16]
    mul r11
    jmp .Lmul_cross_done
.Lmul_cross_a:
    mov rax, r9
    mul r10
.Lmul_cross_done:
    test rdx, rdx
    jnz .Loverflow
    add rcx, rax
    jc .Loverflow
.Lmul_sign:
    mov rax, [rsp+24]
    mov r9, rcx
    cmp qword ptr [rsp+8], 0
    js .Lmul_negative
    test r9, r9
    js .Loverflow
    jmp .Lmul_store
.Lmul_negative:
    test r9, r9
    jns .Lmul_negate
    mov r11, 0x8000000000000000
    cmp r9, r11
    jne .Loverflow
    test rax, rax
    jnz .Loverflow
.Lmul_negate:
    neg rax
    adc r9, 0
    neg r9
.Lmul_store:
    mov [r8], rax
    mov [r8+8], r9
    xor eax, eax
    ret

/* Exact signed truncating div/rem. 128 rounds of unsigned restoring division.
 * rax:r9 = low:high shifting numerator/quotient.
 * rcx:rdx = low:high remainder, r10:r11 = low:high divisor magnitude.
 * Shadow [rsp+8]: quotient sign; [rsp+16]: dividend sign;
 * [rsp+24]: loop counter. Signed MIN / -1 rejected after magnitude division.
 */
.globl vivim_native_divrem128
vivim_native_divrem128:
    mov rax, [rcx]
    mov r9, [rcx+8]
    mov r10, [rdx]
    mov r11, [rdx+8]
    mov rcx, r10
    or rcx, r11
    jz .Lzero
    mov [rsp+16], r9
    mov rcx, r9
    xor rcx, r11
    mov [rsp+8], rcx
    test r9, r9
    jns .Ldiv_a_positive
    neg rax
    adc r9, 0
    neg r9
.Ldiv_a_positive:
    test r11, r11
    jns .Ldiv_b_positive
    neg r10
    adc r11, 0
    neg r11
.Ldiv_b_positive:
    xor ecx, ecx
    xor edx, edx
    mov qword ptr [rsp+24], 128
.Ldiv_loop:
    shl rax, 1
    rcl r9, 1
    rcl rcx, 1
    rcl rdx, 1
    cmp rdx, r11
    jb .Ldiv_next
    ja .Ldiv_sub
    cmp rcx, r10
    jb .Ldiv_next
.Ldiv_sub:
    sub rcx, r10
    sbb rdx, r11
    or rax, 1
.Ldiv_next:
    dec qword ptr [rsp+24]
    jnz .Ldiv_loop
    cmp qword ptr [rsp+8], 0
    js .Ldiv_q_negative
    test r9, r9
    js .Loverflow
    jmp .Ldiv_q_done
.Ldiv_q_negative:
    neg rax
    adc r9, 0
    neg r9
.Ldiv_q_done:
    cmp qword ptr [rsp+16], 0
    jns .Ldiv_store
    neg rcx
    adc rdx, 0
    neg rdx
.Ldiv_store:
    mov [r8], rax
    mov [r8+8], r9
    mov [r8+16], rcx
    mov [r8+24], rdx
    xor eax, eax
    ret
.Loverflow:
    mov eax, 1
    ret
.Lzero:
    mov eax, 2
    ret

/* Challenger: use hardware DIV when divisor magnitude fits one 64-bit limb.
 * Otherwise tail-jump to the original restoring division. This changes no ABI.
 */
.globl vivim_native_divrem128_fast
vivim_native_divrem128_fast:
    mov r10, [rdx]
    mov r11, [rdx+8]
    test r11, r11
    jz .Lfast_positive_divisor
    cmp r11, -1
    jne vivim_native_divrem128
    test r10, r10
    jz vivim_native_divrem128
    neg r10
    jmp .Lfast_divisor_ready
.Lfast_positive_divisor:
    test r10, r10
    jz .Lzero
.Lfast_divisor_ready:
    mov rax, [rcx]
    mov r9, [rcx+8]
    mov [rsp+8], rax
    mov [rsp+16], r9
    mov rcx, r9
    xor rcx, r11
    mov [rsp+24], rcx
    test r9, r9
    jns .Lfast_n_positive
    neg rax
    adc r9, 0
    neg r9
.Lfast_n_positive:
    mov r11, rax
    mov rax, r9
    xor edx, edx
    div r10
    mov r9, rax
    mov rax, r11
    div r10
    mov rcx, rdx
    xor edx, edx
    cmp qword ptr [rsp+24], 0
    js .Lfast_q_negative
    test r9, r9
    js .Loverflow
    jmp .Lfast_q_done
.Lfast_q_negative:
    neg rax
    adc r9, 0
    neg r9
.Lfast_q_done:
    cmp qword ptr [rsp+16], 0
    jns .Ldiv_store
    neg rcx
    adc rdx, 0
    neg rdx
    jmp .Ldiv_store
