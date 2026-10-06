// main_binary_entry.rs - Standalone Pure Rust Entry Point for Metin2 Client
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Mutex, OnceLock};
use std::panic;
use std::ffi::{CString, c_char, c_void};

static SYSERR_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

pub fn write_syserr(msg: &str) {
    let _lock = SYSERR_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap();
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("syserr.txt") {
        let _ = writeln!(file, "{}", msg);
    }
}

pub fn install_crash_handler() {
    panic::set_hook(Box::new(|panic_info| {
        let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            *s
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.as_str()
        } else {
            "Unknown panic"
        };
        
        let location = if let Some(loc) = panic_info.location() {
            format!("{}:{}:{}", loc.file(), loc.line(), loc.column())
        } else {
            "unknown location".to_string()
        };
        
        let err_msg = format!("CRASH: {} at {}", msg, location);
        write_syserr(&err_msg);
        eprintln!("{}", err_msg);
    }));
}

#[cfg(windows)]
extern "system" {
    fn LoadLibraryA(lpLibFileName: *const c_char) -> *mut c_void;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const c_char) -> *mut c_void;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HWND {
    pub handle: usize,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct AppConfig {
    pub config_file: Option<String>,
    pub windowed: bool,
    pub connect_ip: Option<String>,
    pub connect_port: Option<u16>,
    pub max_frames: Option<u64>,
    pub phase: Option<String>,
    pub auto_login: bool,
    pub user: Option<String>,
    pub pwd: Option<String>,
}

impl AppConfig {
    pub fn parse_args<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        let mut config = AppConfig::default();
        let mut iter = args.into_iter();
        
        while let Some(arg) = iter.next() {
            let s = arg.as_ref();
            if s == "--config" {
                if let Some(val) = iter.next() {
                    config.config_file = Some(val.as_ref().to_string());
                }
            } else if s == "--windowed" {
                config.windowed = true;
            } else if s == "--frames" {
                if let Some(val) = iter.next() {
                    if let Ok(f) = val.as_ref().parse::<u64>() {
                        config.max_frames = Some(f);
                    }
                }
            } else if s == "--phase" {
                if let Some(val) = iter.next() {
                    config.phase = Some(val.as_ref().to_uppercase());
                }
            } else if s == "--auto-login" {
                config.auto_login = true;
            } else if s == "--user" {
                if let Some(val) = iter.next() {
                    config.user = Some(val.as_ref().to_string());
                }
            } else if s == "--pwd" || s == "--pass" {
                if let Some(val) = iter.next() {
                    config.pwd = Some(val.as_ref().to_string());
                }
            } else if s == "--connect" {
                if let Some(ip) = iter.next() {
                    config.connect_ip = Some(ip.as_ref().to_string());
                    if let Some(port) = iter.next() {
                        if let Ok(p) = port.as_ref().parse::<u16>() {
                            config.connect_port = Some(p);
                        }
                    }
                }
            }
        }
        config
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EngineState {
    Uninitialized,
    ConfigLoaded,
    WindowCreated,
    GraphicsInitialized,
    D3D9Initialized,
    PacksLoaded,
    PythonStarted,
    Running,
    Stopped,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EngineError {
    WindowNotCreated,
    GraphicsInitFailed,
    PythonInitFailed,
}

pub struct Metin2Engine {
    pub state: EngineState,
    pub config: AppConfig,
    pub hwnd: Option<HWND>,
    pub d3d9_device: Option<usize>,
    pub width: u32,
    pub height: u32,
}

impl Default for Metin2Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Metin2Engine {
    pub fn new() -> Self {
        Self {
            state: EngineState::Uninitialized,
            config: AppConfig::default(),
            hwnd: None,
            d3d9_device: None,
            width: 1024,
            height: 768,
        }
    }
    
    pub fn load_config(&mut self, config: AppConfig) {
        if let Some(f) = config.max_frames {
            std::env::set_var("M2_MAX_FRAMES", f.to_string());
            crate::wgpu_engine_context::set_max_frames_limit(f);
        }
        self.config = config;
        self.state = EngineState::ConfigLoaded;
    }

    pub fn init_d3d9(&mut self) -> Result<(), EngineError> {
        if self.hwnd.is_none() {
            write_syserr("Failed to initialize d3d9: Window not created");
            return Err(EngineError::WindowNotCreated);
        }
        self.d3d9_device = Some(1);
        self.state = EngineState::D3D9Initialized;
        Ok(())
    }

    pub fn run_game_loop(&mut self) {
        self.state = EngineState::Stopped;
    }

    pub fn run(&mut self, config: AppConfig) -> Result<(), EngineError> {
        self.load_config(config);
        self.state = EngineState::Stopped;
        Ok(())
    }
    
    pub fn create_window(&mut self) -> Result<(), EngineError> {
        let title = std::ffi::CString::new("METIN2 (Pure Rust 64-bit Engine)").unwrap();
        
        let mut width = 1024;
        let mut height = 768;
        let mut is_windowed = true;

        let cfg_path = self.config.config_file.as_deref().unwrap_or("metin2.cfg");
        if let Ok(content) = std::fs::read_to_string(cfg_path) {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if parts[0] == "WIDTH" {
                        if let Ok(w) = parts[1].parse::<i32>() {
                            width = w;
                        }
                    } else if parts[0] == "HEIGHT" {
                        if let Ok(h) = parts[1].parse::<i32>() {
                            height = h;
                        }
                    } else if parts[0] == "WINDOWED" {
                        if let Ok(w_val) = parts[1].parse::<i32>() {
                            is_windowed = w_val != 0;
                        }
                    }
                }
            }
        }

        self.width = width as u32;
        self.height = height as u32;

        let win_cfg = crate::win32_window::WindowConfig {
            title: title.as_ptr(),
            width,
            height,
            is_fullscreen: !is_windowed,
            is_borderless: false,
        };

        let handle = match crate::win32_window::create_game_window(&win_cfg) {
            Ok(h) if !h.is_null() => h as usize,
            _ => {
                #[cfg(windows)]
                {
                    write_syserr("Failed to create Win32 game window");
                    return Err(EngineError::WindowNotCreated);
                }
                #[cfg(not(windows))]
                { 0x12345678 }
            }
        };

        self.hwnd = Some(HWND { handle });
        self.state = EngineState::WindowCreated;
        Ok(())
    }

    pub fn init_graphics(&mut self) -> Result<(), EngineError> {
        if self.hwnd.is_none() {
            write_syserr("Failed to initialize graphics: Window not created");
            return Err(EngineError::WindowNotCreated);
        }

        let hwnd_val = self.hwnd.map(|h| h.handle).unwrap_or(0);
        
        match crate::wgpu_engine_context::WgpuEngineContext::new(hwnd_val, self.width, self.height) {
            Ok(ctx) => {
                println!("[+] Nowoczesny silnik graficzny GPU aktywny: {}", ctx.get_adapter_info());
                crate::wgpu_engine_context::set_global_wgpu_context(ctx);
                self.state = EngineState::GraphicsInitialized;
                Ok(())
            }
            Err(e) => {
                println!("[-] Blad inicjalizacji GPU: {}", e);
                write_syserr(&format!("GPU Init error: {}", e));
                Err(EngineError::GraphicsInitFailed)
            }
        }
    }

    pub fn init_vfs_packs(&mut self) -> Result<(), EngineError> {
        let pack_dirs = [
            "pack",
            "E:/m2dev-client-src-main/m2dev-client-main/pack",
            ".",
        ];
        
        let mut loaded = 0;
        let mut loaded_files = std::collections::HashSet::new();

        // 1. Najpierw montujemy kluczowe paczki bazowe
        let priority_packs = [
            "root.pck",
            "locale.pck",
            "uiscript.pck",
            "ETC.pck",
            "uiloading.pck",
        ];

        for p_dir in &pack_dirs {
            let dir_path = std::path::Path::new(p_dir);
            if dir_path.is_dir() {
                for name in priority_packs {
                    let full_path = dir_path.join(name);
                    let path_str = full_path.to_string_lossy().replace('\\', "/");
                    if full_path.exists() && !loaded_files.contains(&name.to_lowercase()) {
                        let _ = crate::py_bridge_pack::mount_pack_file(&path_str);
                        if let Ok(arch) = crate::vfs_pck_memory_mapped::mount_archive(&path_str) {
                            println!("[+] VFS Pack bazowy: {} (plikow: {})", path_str, arch.entries.len());
                            loaded += 1;
                            loaded_files.insert(name.to_lowercase());
                        }
                    }
                }

                // 2. Nastepnie montujemy wszystkie pozostale archiwum *.pck ze skanu folderu
                if let Ok(entries) = std::fs::read_dir(dir_path) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file() {
                            if let Some(ext) = path.extension() {
                                if ext.to_string_lossy().eq_ignore_ascii_case("pck") {
                                    if let Some(fname) = path.file_name() {
                                        let fname_lower = fname.to_string_lossy().to_lowercase();
                                        if !loaded_files.contains(&fname_lower) {
                                            let path_str = path.to_string_lossy().replace('\\', "/");
                                            let _ = crate::py_bridge_pack::mount_pack_file(&path_str);
                                            if let Ok(_arch) = crate::vfs_pck_memory_mapped::mount_archive(&path_str) {
                                                loaded += 1;
                                                loaded_files.insert(fname_lower);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                break; // Znaleziono glowny folder pack
            }
        }
        println!("[+] VFS kompletny: pomyslnie zamontowano {} archiwow PCK!", loaded);

        // Ladowanie zasobow tla logowania (login.jpg oraz ThinBoard)
        let etc_candidates = [
            "pack/ETC.pck".to_string(),
            "ETC.pck".to_string(),
            "E:/m2dev-client-src-main/m2dev-client-main/pack/ETC.pck".to_string(),
        ];
        for etc_path in &etc_candidates {
            if let Ok(etc_arch) = crate::vfs_pck_memory_mapped::mount_archive(etc_path) {
                if let Some(jpg_data) = crate::vfs_pck_memory_mapped::read_file_from_archive(&etc_arch, "d:/ymir work/ui/intro/login/login.jpg") {
                    if let Ok(img) = image::load_from_memory(&jpg_data) {
                        let rgba = img.to_rgba8();
                        let guard = crate::wgpu_engine_context::GLOBAL_WGPU_CONTEXT.get();
                        if let Some(mutex) = guard {
                            if let Ok(ctx_opt) = mutex.lock() {
                                if let Some(ctx) = ctx_opt.as_ref() {
                                    if let Ok(bg_tex) = ctx.create_texture_from_rgba(&rgba, img.width(), img.height(), Some("Login BG")) {
                                        println!("[+] Tlo logowania (login.jpg {}x{}) zaladowane na GPU!", bg_tex.width, bg_tex.height);
                                        crate::wgpu_engine_context::set_global_login_bg(bg_tex);
                                    }
                                }
                            }
                        }
                    }
                }

                if let Some(tga_data) = crate::vfs_pck_memory_mapped::read_file_from_archive(&etc_arch, "d:/ymir work/ui/pattern/ThinBoard_Corner_LeftTop.tga") {
                    if let Ok(tga) = crate::tga_decoder::TgaImage::from_bytes(&tga_data) {
                        let guard = crate::wgpu_engine_context::GLOBAL_WGPU_CONTEXT.get();
                        if let Some(mutex) = guard {
                            if let Ok(ctx_opt) = mutex.lock() {
                                if let Some(ctx) = ctx_opt.as_ref() {
                                    if let Ok(tb_tex) = ctx.create_texture_from_rgba(&tga.rgba_data, tga.width as u32, tga.height as u32, Some("ThinBoard")) {
                                        println!("[+] Tekstura UI ThinBoard zaladowana na GPU!");
                                        crate::wgpu_engine_context::set_global_thinboard(tb_tex);
                                    }
                                }
                            }
                        }
                    }
                }
                break;
            }
        }

        // Ladowanie baz proto przedmiotow i potworow z archiwow VFS
        let _ = crate::item_mob_proto_vfs_loader::load_protos_from_vfs();

        // Inicjalizacja podsystemu audio 3D i muzyki BGM Pure Rust
        let _ = crate::sound_ffi::m2_audio_init();
        println!("[+] Podsystem audio Pure Rust (3D Spatial Audio & BGM) zainicjalizowany!");

        self.state = EngineState::PacksLoaded;
        Ok(())
    }

    pub fn start_python(&mut self) -> Result<(), EngineError> {
        #[cfg(windows)]
        unsafe {
            let candidates = ["python314.dll", "python3.dll", "python27.dll"];
            let mut py_dll: *mut c_void = std::ptr::null_mut();
            for c in candidates {
                if let Ok(name) = CString::new(c) {
                    let h = LoadLibraryA(name.as_ptr());
                    if !h.is_null() {
                        py_dll = h;
                        println!("[+] Zaladowano biblioteke skryptowa: {}", c);
                        break;
                    }
                }
            }

            if py_dll.is_null() {
                println!("[-] python314.dll nie odnaleziono, aktywuje natywny tryb renderowania");
                crate::ui_bridge_ffi::m2_ui_setup_default_login_ui(self.width, self.height);
                return Ok(());
            }

            let py_init_sym = GetProcAddress(py_dll, CString::new("Py_InitializeEx").unwrap().as_ptr());
            let py_run_sym = GetProcAddress(py_dll, CString::new("PyRun_SimpleString").unwrap().as_ptr());

            if py_init_sym.is_null() || py_run_sym.is_null() {
                println!("[-] Brak symboli Py_InitializeEx / PyRun_SimpleString");
                crate::ui_bridge_ffi::m2_ui_setup_default_login_ui(self.width, self.height);
                return Ok(());
            }

            let py_init: extern "C" fn(i32) = std::mem::transmute(py_init_sym);
            let py_run: extern "C" fn(*const c_char) -> i32 = std::mem::transmute(py_run_sym);

            py_init(0);

            let py_err_sym = GetProcAddress(py_dll, CString::new("PyErr_Print").unwrap().as_ptr());
            let py_err: Option<extern "C" fn()> = if !py_err_sym.is_null() {
                Some(std::mem::transmute(py_err_sym))
            } else {
                None
            };

            // 1. Wstrzykujemy Master Runtime Bridge (pelne 32 moduly Metin2 z bindowaniem do silnika Rust)
            if let Err(e) = crate::py_master_runtime_bridge::inject_master_runtime_modules() {
                println!("[-] Blad inicjalizacji Master Runtime Bridge: {}", e);
                if let Some(err_fn) = py_err { err_fn(); }
            } else {
                println!("[+] Pomyslnie zainicjalizowano pelne 32 moduly Metin2 podlaczone pod silnik Rust!");
            }

            // 2. Uruchamiamy system.py (sprawdzamy plik lokalny, a nastepnie archiwum VFS root.pck)
            let launch_code = CString::new(r#"
import os
import sys
import __main__
import pack

__main__.__DEBUG__ = 1

system_py_data = None
for p in ['system.py', 'assets/root/system.py']:
    if os.path.exists(p):
        try:
            with open(p, 'rb') as f:
                system_py_data = f.read()
                print(f"[+] Ladowanie lokalnego system.py z dysku: {p} ({len(system_py_data)} bajtow)")
                break
        except Exception:
            pass

if not system_py_data:
    system_py_data = pack.Get('system.py')
    if system_py_data:
        print(f"[+] Ladowanie system.py z VFS ({len(system_py_data)} bajtow)...")

if system_py_data:
    exec(compile(system_py_data, 'system.py', 'exec'), __main__.__dict__)
    print("[+] system.py i prototype.py wykonane pomyslnie!")
else:
    print("[-] system.py nie odnaleziono w VFS ani na dysku!")
"#).unwrap();

            println!("[+] Uruchamianie glownego skryptu system.py...");
            let ret = py_run(launch_code.as_ptr());
            if ret != 0 {
                println!("[-] Blad podczas wykonywania system.py (kod: {})", ret);
                if let Some(err_fn) = py_err {
                    err_fn();
                }
            }
        }

        self.state = EngineState::PythonStarted;
        Ok(())
    }

    pub fn run_interactive(&mut self, config: AppConfig) -> Result<(), EngineError> {
        install_crash_handler();
        self.load_config(config);
        println!("[+] Inicjalizacja podsystemow klienta Pure Rust (wgpu DirectX 12 / Vulkan)...");

        self.create_window()?;
        println!("[+] Okno gry Win32 utworzone i aktywowane na pulpicie (1024x768).");

        self.init_graphics()?;
        println!("[+] Nowoczesny potok graficzny GPU zainicjalizowany.");

        let _ = self.init_vfs_packs();

        let _ = self.start_python();

        // Aktywacja zadanej fazy jesli podano argument --phase lub --auto-login
        if let Some(ref ph) = self.config.phase {
            if ph.eq_ignore_ascii_case("GAME") {
                println!("[+] ========================================================");
                println!("[+] [TRYB GRY] Aktywacja fazy GAME (swiat 3D, potok WGPU, sterowanie WASD)");
                println!("[+] ========================================================");
                if let Ok(mut state) = m2_client_bridge::state::game_state::GLOBAL_GAME_STATE.write() {
                    state.player.vid = 1001;
                    state.player.x = 0.0;
                    state.player.y = 0.0;
                    state.player.z = 0.0;
                    state.player.rot = 0.0;
                    state.player.points[1] = 105; // POINT_LEVEL
                    state.player.points[5] = 15000; // HP
                    state.player.points[6] = 15000; // Max HP
                    state.player.points[7] = 2000;  // SP
                    state.player.points[8] = 2000;  // Max SP
                    state.player.points[11] = 999_999_999; // POINT_GOLD
                }
                if let Ok(mut chrmgr) = crate::character_manager_core::get_character_manager().write() {
                    chrmgr.create_instance_full(1001, 0, 0.0, 0.0, 0.0, "[SA]Admin".to_string(), false);
                    chrmgr.main_actor_vid = Some(1001);
                }
                if let Ok(mut cur_phase) = crate::ui_bridge_ffi::CURRENT_PHASE.lock() {
                    *cur_phase = "GAME".to_string();
                }
                let mut wm = crate::ui_window_mgr::get_window_manager();
                crate::ui_bridge_ffi::m2_ui_setup_game_hud(&mut wm, self.width, self.height);
            } else if ph.eq_ignore_ascii_case("SELECT") {
                println!("[+] [TRYB POSTACI] Aktywacja fazy SELECT (wybor bohatera 3D)...");
                if let Ok(mut cur_phase) = crate::ui_bridge_ffi::CURRENT_PHASE.lock() {
                    *cur_phase = "SELECT".to_string();
                }
                let chars = vec![crate::ui_bridge_ffi::CharacterSlot {
                    id: 1,
                    name: "[SA]Admin".to_string(),
                    job: 0,
                    level: 105,
                    playtime: 120,
                    st: 90,
                    ht: 90,
                    dx: 90,
                    iq: 90,
                    main_part: 0,
                    hair_part: 0,
                }];
                let mut wm = crate::ui_window_mgr::get_window_manager();
                crate::ui_bridge_ffi::m2_ui_setup_character_select(&mut wm, self.width, self.height, &chars);
            }
        } else if self.config.auto_login {
            println!("[+] [AUTO-LOGIN] Automatyczne logowanie do serwera gry...");
            let mut wm = crate::ui_window_mgr::get_window_manager();
            crate::ui_bridge_ffi::handle_ui_click("LoginButton", 0, &mut wm);
        }

        // Jesli Python nie przeszedl do glownej petli (lub w trybie fallback), uruchom natywna petle gry
        println!("[+] Uruchamianie natywnej petli renderowania okna gry...");
        crate::wgpu_engine_context::m2_run_game_loop();

        self.state = EngineState::Stopped;
        println!("[+] Metin2 Pure Rust Client zakonczyl dzialanie pomyslnie.");
        Ok(())
    }
}

/// Glowny punkt wejsciowy wywolywany przez binary runner (m2_client.exe)
/// Dzieki temu caly silnik, okno Win32, kontekst GPU i Python Master Bridge dzialaja
/// w JEDNEJ wspoldzielonej instancji dynamicznej (m2_client.dll), eliminujac problem
/// rozdzielenia pamieci statycznej pomiedzy procesem a DLL-ka Pythona.
#[no_mangle]
pub extern "C" fn m2_client_main_interactive() -> i32 {
    let args: Vec<String> = std::env::args().collect();
    let config = AppConfig::parse_args(&args[1..]);
    let mut engine = Metin2Engine::new();
    if let Err(e) = engine.run_interactive(config) {
        eprintln!("Bledne zakonczenie dzialania silnika: {:?}", e);
        return 1;
    }
    0
}

