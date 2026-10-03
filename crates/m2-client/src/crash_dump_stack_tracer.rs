use std::backtrace::Backtrace;
use std::fs::OpenOptions;
use std::io::Write;
use std::panic;
use std::sync::Once;

static INIT: Once = Once::new();

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

#[cfg(not(target_arch = "x86_64"))]
pub fn capture_registers() -> Registers {
    Registers::default()
}

pub fn setup_panic_hook() {
    INIT.call_once(|| {
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            let regs = capture_registers();
            let backtrace = Backtrace::force_capture();
            
            let mut report = String::new();
            report.push_str("=== CRASH REPORT ===\n");
            
            if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                report.push_str(&format!("Panic occurred: {}\n", s));
            } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                report.push_str(&format!("Panic occurred: {}\n", s));
            } else {
                report.push_str("Panic occurred: Unknown payload\n");
            }
            
            if let Some(location) = panic_info.location() {
                report.push_str(&format!("Location: {}:{}\n", location.file(), location.line()));
            }
            
            report.push_str("\n--- CPU Registers (x86_64) ---\n");
            report.push_str(&format!("RAX: {:#018x}\n", regs.rax));
            report.push_str(&format!("RBX: {:#018x}\n", regs.rbx));
            report.push_str(&format!("RCX: {:#018x}\n", regs.rcx));
            report.push_str(&format!("RDX: {:#018x}\n", regs.rdx));
            report.push_str(&format!("RSI: {:#018x}\n", regs.rsi));
            report.push_str(&format!("RDI: {:#018x}\n", regs.rdi));
            report.push_str(&format!("RBP: {:#018x}\n", regs.rbp));
            report.push_str(&format!("RSP: {:#018x}\n", regs.rsp));
            report.push_str(&format!("R8:  {:#018x}\n", regs.r8));
            report.push_str(&format!("R9:  {:#018x}\n", regs.r9));
            report.push_str(&format!("R10: {:#018x}\n", regs.r10));
            report.push_str(&format!("R11: {:#018x}\n", regs.r11));
            report.push_str(&format!("R12: {:#018x}\n", regs.r12));
            report.push_str(&format!("R13: {:#018x}\n", regs.r13));
            report.push_str(&format!("R14: {:#018x}\n", regs.r14));
            report.push_str(&format!("R15: {:#018x}\n", regs.r15));
            report.push_str(&format!("RIP: {:#018x}\n", regs.rip));
            
            report.push_str("\n--- Stack Trace ---\n");
            report.push_str(&format!("{}", backtrace));
            report.push_str("====================\n\n");
            
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("crash_report.txt") {
                let _ = file.write_all(report.as_bytes());
            }
            
            original_hook(panic_info);
        }));
    });
}
