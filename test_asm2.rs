#[derive(Debug, Default)]
pub struct Registers {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
}

#[cfg(target_arch = "x86_64")]
pub fn capture_registers() -> Registers {
    let mut regs = Registers::default();
    unsafe {
        std::arch::asm!(
            "mov [rdi + 0x00], rax",
            "mov [rdi + 0x08], rbx",
            "mov [rdi + 0x10], rcx",
            "mov [rdi + 0x18], rdx",
            "mov [rdi + 0x20], rsi",
            "mov [rdi + 0x28], rbp",
            "mov [rdi + 0x30], rsp",
            "mov [rdi + 0x38], r8",
            "mov [rdi + 0x40], r9",
            "mov [rdi + 0x48], r10",
            "mov [rdi + 0x50], r11",
            "mov [rdi + 0x58], r12",
            "mov [rdi + 0x60], r13",
            "mov [rdi + 0x68], r14",
            "mov [rdi + 0x70], r15",
            "lea rax, [rip]",
            "mov [rdi + 0x78], rax",
            in("rdi") &mut regs as *mut Registers,
            out("rax") _,
        );
    }
    regs
}

fn main() {
    println!("{:?}", capture_registers());
}
