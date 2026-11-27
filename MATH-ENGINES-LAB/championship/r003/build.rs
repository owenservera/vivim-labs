//! P024: bounded instruction grammar emits native source. No arithmetic in Rust.
use std::{env,fs,path::PathBuf};
fn main(){
    println!("cargo:rerun-if-changed=build.rs");
    let mut s=String::from(".text\n.globl p024_seed\np024_seed:\n    mov rax, rcx\n");
    let labels=["add","sub","mul","div","neg","rem","cmp"];
    for (n,label) in labels.iter().enumerate(){
        s.push_str(&format!("    cmp r8d, {n}\n    je .L24_{label}\n"));
    }
    s.push_str("    mov eax, 3\n    ret\n");
    // Legal operation templates. The grammar is separate from evaluator.
    for (name,insn) in [("add","add rax, rdx"),("sub","sub rax, rdx"),("mul","imul rax, rdx"),("neg","neg rax")]{
        s.push_str(&format!(".L24_{name}:\n    {insn}\n    jo .L24_over\n    jmp .L24_store\n"));
    }
    s.push_str(".L24_rem:\n.L24_div:\n    test rdx, rdx\n    jz .L24_zero\n    mov r10, rdx\n    mov r11, 0x8000000000000000\n    cmp rax, r11\n    jne .L24_execute\n    cmp r10, -1\n    je .L24_over\n.L24_execute:\n    cqo\n    idiv r10\n    cmp r8d, 5\n    cmove rax, rdx\n    jmp .L24_store\n.L24_cmp:\n    cmp rcx, rdx\n    mov rax, -1\n    jl .L24_store\n    mov eax, 1\n    jg .L24_store\n    xor eax, eax\n.L24_store:\n    mov [r9], rax\n    xor eax, eax\n    ret\n.L24_over:\n    mov eax, 1\n    ret\n.L24_zero:\n    mov eax, 2\n    ret\n");
    fs::write(PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("P024.S"),s).unwrap();
}
