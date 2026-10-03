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

pub fn capture_registers() -> Registers {
    let mut regs = Registers::default();
    unsafe {
        std::arch::asm!(
            "mov {0}, rax",
            "mov {1}, rbx",
            "mov {2}, rcx",
            "mov {3}, rdx",
            "mov {4}, rsi",
            "mov {5}, rdi",
            "mov {6}, rbp",
            "mov {7}, rsp",
            "mov {8}, r8",
            "mov {9}, r9",
            "mov {10}, r10",
            "mov {11}, r11",
            "mov {12}, r12",
            "mov {13}, r13",
            "mov {14}, r14",
            "mov {15}, r15",
            "lea {16}, [rip]",
            out(reg) regs.rax,
            out(reg) regs.rbx,
            out(reg) regs.rcx,
            out(reg) regs.rdx,
            out(reg) regs.rsi,
            out(reg) regs.rdi,
            out(reg) regs.rbp,
            out(reg) regs.rsp,
            out(reg) regs.r8,
            out(reg) regs.r9,
            out(reg) regs.r10,
            out(reg) regs.r11,
            out(reg) regs.r12,
            out(reg) regs.r13,
            out(reg) regs.r14,
            out(reg) regs.r15,
            out(reg) regs.rip,
        );
    }
    regs
}

fn main() {
    println!("{:?}", capture_registers());
}
