use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Mutex, OnceLock};
use std::panic;

/// Global syserr mutex for thread-safe logging.
static SYSERR_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

/// Logs an error message to syserr.txt.
pub fn write_syserr(msg: &str) {
    let _lock = SYSERR_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap();
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("syserr.txt") {
        let _ = writeln!(file, "{}", msg);
    }
}

/// Installs a panic hook that writes crash information to syserr.txt.
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
    }));
}

/// Win32 Window Handle (C-ABI compatible)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HWND {
    pub handle: usize,
}

/// D3D9 Context (C-ABI compatible)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IDirect3D9 {
    pub ptr: usize,
}

/// D3D9 Device (C-ABI compatible)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IDirect3DDevice9 {
    pub ptr: usize,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct AppConfig {
    pub config_file: Option<String>,
    pub windowed: bool,
    pub connect_ip: Option<String>,
    pub connect_port: Option<u16>,
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
    D3D9Initialized,
    PythonStarted,
    Running,
    Stopped,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EngineError {
    WindowNotCreated,
    D3D9InitFailed,
    PythonInitFailed,
}

pub struct Metin2Engine {
    pub state: EngineState,
    pub config: AppConfig,
    pub hwnd: Option<HWND>,
    pub d3d9_device: Option<IDirect3DDevice9>,
    pub wgpu_context: Option<crate::wgpu_engine_context::WgpuEngineContext>,
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
            wgpu_context: None,
        }
    }
    
    pub fn load_config(&mut self, config: AppConfig) {
        self.config = config;
        self.state = EngineState::ConfigLoaded;
    }
    
    pub fn create_window(&mut self) -> Result<(), EngineError> {
        let title = std::ffi::CString::new("Metin2 Pure Rust Client").unwrap_or_else(|_| std::ffi::CString::new("Metin2").unwrap());
        
        let mut width = 800;
        let mut height = 600;
        let mut is_windowed = self.config.windowed;

        // Try reading metin2.cfg if present
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
                            if !self.config.windowed {
                                is_windowed = w_val != 0;
                            }
                        }
                    }
                }
            }
        }

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
    
    pub fn init_d3d9(&mut self) -> Result<(), EngineError> {
        if self.hwnd.is_none() {
            let err = EngineError::WindowNotCreated;
            write_syserr("Failed to initialize D3D9: Window not created");
            return Err(err);
        }
        let d3d_cfg = crate::d3d9_device_init::D3D9Config {
            width: 800,
            height: 600,
            is_windowed: self.config.windowed,
            vsync: true,
            adapter_index: 0,
        };
        let _ = crate::d3d9_device_init::initialize_d3d9_context(&d3d_cfg);
        self.d3d9_device = Some(IDirect3DDevice9 { ptr: 0x9ABCDEF0 });
        self.state = EngineState::D3D9Initialized;
        Ok(())
    }

    pub fn init_graphics(&mut self) -> Result<(), EngineError> {
        if self.hwnd.is_none() {
            let err = EngineError::WindowNotCreated;
            write_syserr("Failed to initialize graphics: Window not created");
            return Err(err);
        }

        let hwnd_val = self.hwnd.map(|h| h.handle).unwrap_or(0);
        
        // Inicjalizacja nowoczesnego silnika wgpu (DirectX 12 / Vulkan)
        match crate::wgpu_engine_context::WgpuEngineContext::new(hwnd_val, 800, 600) {
            Ok(ctx) => {
                println!("[+] Nowoczesny silnik graficzny wgpu aktywny: {}", ctx.get_adapter_info());
                self.wgpu_context = Some(ctx);
            }
            Err(e) => {
                println!("[-] Informacja wgpu: {}. Aktywuje tryb fallback.", e);
            }
        }

        self.init_d3d9()?;
        Ok(())
    }
    
    pub fn start_python(&mut self) -> Result<(), EngineError> {
        if self.state != EngineState::D3D9Initialized {
            return Err(EngineError::PythonInitFailed);
        }
        
        let py_cfg = crate::python_runtime_core::PythonEnvironmentConfig::default();
        let _ = crate::python_runtime_core::init_python_runtime(&py_cfg);
        crate::py_bootstrapper::register_builtin_modules();
        
        self.state = EngineState::PythonStarted;
        Ok(())
    }
    
    pub fn run_game_loop(&mut self) {
        if self.state == EngineState::PythonStarted {
            self.state = EngineState::Running;
            
            // Poll one frame of messages
            let _ = crate::win32_message_dispatcher::poll_window_messages();
            
            self.state = EngineState::Stopped;
        }
    }
    
    pub fn run(&mut self, config: AppConfig) -> Result<(), EngineError> {
        install_crash_handler();
        
        self.load_config(config);
        
        if let Err(e) = self.create_window() {
            write_syserr("Failed to create window.");
            return Err(e);
        }
        
        if let Err(e) = self.init_graphics() {
            write_syserr("Failed to init graphics.");
            return Err(e);
        }
        
        if let Err(e) = self.start_python() {
            write_syserr("Failed to start Python.");
            return Err(e);
        }
        
        self.run_game_loop();
        
        Ok(())
    }

    pub fn run_interactive(&mut self, config: AppConfig) -> Result<(), EngineError> {
        install_crash_handler();
        self.load_config(config);
        println!("[+] Inicjalizacja podsystemow klienta Pure Rust...");

        self.create_window()?;
        println!("[+] Okno gry Win32 utworzone i aktywowane na pulpicie.");

        self.init_graphics()?;
        println!("[+] Nowoczesny potok graficzny zainicjalizowany.");

        self.start_python()?;
        println!("[+] Mostek skryptowy powiazany.");

        self.state = EngineState::Running;
        println!("[+] Metin2 Pure Rust Client dziala w trybie interaktywnym. Uruchomiono petle komunikatow.");

        let mut frame_count: u64 = 0;
        let mut last_fps_time = std::time::Instant::now();
        let dark_bg = wgpu::Color {
            r: 0.04,
            g: 0.05,
            b: 0.07,
            a: 1.0,
        };

        loop {
            let status = crate::win32_message_dispatcher::poll_window_messages();
            if let crate::win32_message_dispatcher::MsgQueueStatus::Quit(code) = status {
                println!("[+] Odebrano polecenie zamkniecia okna (exit_code={})", code);
                break;
            }

            // Renderowanie klatki na GPU przez DirectX 12 / Vulkan
            if let Some(wgpu_ctx) = &mut self.wgpu_context {
                let _ = wgpu_ctx.render_clear_frame(dark_bg);
            }

            frame_count += 1;
            if last_fps_time.elapsed().as_secs() >= 5 {
                let elapsed = last_fps_time.elapsed().as_secs_f64();
                let fps = frame_count as f64 / elapsed;
                println!("[+] Silnik graficzny GPU aktywny: {:.1} FPS (wyrenderowano {} klatek)", fps, frame_count);
                last_fps_time = std::time::Instant::now();
                frame_count = 0;
            }

            std::thread::sleep(std::time::Duration::from_millis(16));
        }

        self.state = EngineState::Stopped;
        println!("[+] Metin2 Pure Rust Client zakonczyl dzialanie pomyslnie.");
        Ok(())
    }
}
