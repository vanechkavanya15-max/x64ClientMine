// m2_client.rs - Main executable binary entry point for Metin2 Pure Rust Client

#[cfg(windows)]
fn main() {
    unsafe {
        use std::ffi::CString;
        extern "system" {
            fn LoadLibraryA(lpLibFileName: *const i8) -> usize;
            fn GetProcAddress(hModule: usize, lpProcName: *const i8) -> usize;
        }

        let dll_name = CString::new("m2_client.dll").unwrap();
        let h_dll = LoadLibraryA(dll_name.as_ptr());
        if h_dll != 0 {
            let fn_name = CString::new("m2_client_main_interactive").unwrap();
            let proc = GetProcAddress(h_dll, fn_name.as_ptr());
            if proc != 0 {
                let entry_fn: extern "C" fn() -> i32 = std::mem::transmute(proc);
                let code = entry_fn();
                std::process::exit(code);
            }
        }
    }

    // Fallback do bezposredniej instancji w exe
    let args: Vec<String> = std::env::args().collect();
    let config = m2_client::main_binary_entry::AppConfig::parse_args(&args[1..]);
    let mut engine = m2_client::main_binary_entry::Metin2Engine::new();
    if let Err(e) = engine.run_interactive(config) {
        eprintln!("Bledne zakonczenie dzialania silnika: {:?}", e);
    }
}

#[cfg(not(windows))]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = m2_client::main_binary_entry::AppConfig::parse_args(&args[1..]);
    let mut engine = m2_client::main_binary_entry::Metin2Engine::new();
    if let Err(e) = engine.run_interactive(config) {
        eprintln!("Bledne zakonczenie dzialania silnika: {:?}", e);
    }
}

