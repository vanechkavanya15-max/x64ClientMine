#[derive(Debug, Default, Clone)]
#[repr(C)]
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
            "mov [{ptr} + 0x00], rax",
            "mov [{ptr} + 0x08], rbx",
            "mov [{ptr} + 0x10], rcx",
            "mov [{ptr} + 0x18], rdx",
            "mov [{ptr} + 0x20], rsi",
            "mov [{ptr} + 0x28], rdi",
            "mov [{ptr} + 0x30], rbp",
            "mov [{ptr} + 0x38], rsp",
            "mov [{ptr} + 0x40], r8",
            "mov [{ptr} + 0x48], r9",
            "mov [{ptr} + 0x50], r10",
            "mov [{ptr} + 0x58], r11",
            "mov [{ptr} + 0x60], r12",
            "mov [{ptr} + 0x68], r13",
            "mov [{ptr} + 0x70], r14",
            "mov [{ptr} + 0x78], r15",
            "lea rax, [rip]",
            "mov [{ptr} + 0x80], rax",
            ptr = in(reg) &mut regs as *mut Registers,
            out("rax") _,
        );
    }
    regs
}

fn main() {
    println!("{:?}", capture_registers());
}
