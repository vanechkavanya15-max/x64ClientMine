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

        // Inicjalizacja natywnego interfejsu logowania (ThinBoard, lista serwerow, kanaly, flagi, przyciski)
        crate::ui_bridge_ffi::m2_ui_setup_default_login_ui(self.width, self.height);

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
                return Ok(());
            }

            let py_init_sym = GetProcAddress(py_dll, CString::new("Py_InitializeEx").unwrap().as_ptr());
            let py_run_sym = GetProcAddress(py_dll, CString::new("PyRun_SimpleString").unwrap().as_ptr());

            if py_init_sym.is_null() || py_run_sym.is_null() {
                println!("[-] Brak symboli Py_InitializeEx / PyRun_SimpleString");
                return Ok(());
            }

            let py_init: extern "C" fn(i32) = std::mem::transmute(py_init_sym);
            let py_run: extern "C" fn(*const c_char) -> i32 = std::mem::transmute(py_run_sym);

            py_init(0);

            let py_code = CString::new(r#"
import sys, types, ctypes, os
try:
    os.add_dll_directory(os.getcwd())
except Exception:
    pass

dll = None
dll_candidates = [
    os.path.abspath("m2_client.dll"),
    os.path.join(os.getcwd(), "m2_client.dll"),
    "E:/SourceCodeClient/target/x86_64-pc-windows-msvc/release/m2_client.dll",
    "E:/SourceCodeClient/target/x86_64-pc-windows-msvc/debug/m2_client.dll",
    "E:/m2dev-client-src-main/m2dev-client-main/m2_client.dll",
    "E:/SourceCodeClient/crates/m2-client/m2_client.dll",
]
for c in dll_candidates:
    if os.path.exists(c):
        try:
            dll = ctypes.CDLL(c)
            break
        except Exception:
            pass

if not dll:
    try:
        h = ctypes.windll.kernel32.GetModuleHandleW(None)
        dll = ctypes.CDLL('', handle=h)
    except Exception:
        dll = None

pack_exist_fn = getattr(dll, 'm2_py_pack_exist', None) if dll else None
if pack_exist_fn:
    pack_exist_fn.argtypes = [ctypes.c_char_p]
    pack_exist_fn.restype = ctypes.c_bool

pack_size_fn = getattr(dll, 'm2_py_pack_get_size', None) if dll else None
if pack_size_fn:
    pack_size_fn.argtypes = [ctypes.c_char_p]
    pack_size_fn.restype = ctypes.c_size_t

pack_read_fn = getattr(dll, 'm2_py_pack_read_data', None) if dll else None
if pack_read_fn:
    pack_read_fn.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_size_t]
    pack_read_fn.restype = ctypes.c_size_t

if dll and hasattr(dll, 'm2_py_pack_mount'):
    for p in ["root.pck", "locale.pck", "uiscript.pck", "ETC.pck", "uiloading.pck"]:
        dll.m2_py_pack_mount(f"pack/{p}".encode('utf-8'))

class PackModule:
    @staticmethod
    def Exist(filename):
        if not pack_exist_fn:
            return False
        return pack_exist_fn(filename.encode('utf-8'))
    @staticmethod
    def Get(filename):
        if not pack_size_fn or not pack_read_fn:
            return b''
        f_bytes = filename.encode('utf-8')
        size = pack_size_fn(f_bytes)
        if size == 0:
            return b''
        buf = ctypes.create_string_buffer(size)
        read_len = pack_read_fn(f_bytes, buf, size)
        return buf.raw[:read_len]

sys.modules['pack'] = PackModule

class DbgModule:
    @staticmethod
    def Trace(msg):
        sys.__stdout__.write(str(msg))
        sys.__stdout__.flush()
    @staticmethod
    def TraceError(msg):
        sys.__stderr__.write(f"[DBG ERROR] {msg}\n")
        sys.__stderr__.flush()
    @staticmethod
    def RegisterExceptionString(msg):
        pass
    @staticmethod
    def LogBox(msg, title="Error"):
        sys.__stderr__.write(f"[LOGBOX {title}] {msg}\n")
        sys.__stderr__.flush()

sys.modules['dbg'] = DbgModule

class AppModuleMeta(type):
    def __getattr__(cls, name):
        if name.isupper():
            return 0
        return lambda *args, **kwargs: 0

class AppModule(metaclass=AppModuleMeta):
    INFO_ITEM = 1
    INFO_ACTOR = 2
    INFO_EFFECT = 3
    INFO_TEXTTAIL = 4
    NORMAL = 0
    ATTACK = 1
    TARGET = 2
    TALK = 3
    CANT_GO = 4
    PICK = 5
    DOOR = 6
    CHAIR = 7
    MAGIC = 8
    BUY = 9
    SELL = 10
    CAMERA_ROTATE = 11
    HSIZE = 12
    VSIZE = 13
    HVSIZE = 14
    CAMERA_TO_POSITIVE = 1
    CAMERA_TO_NEGATIVE = 2
    CAMERA_STOP = 0
    ENABLE_COSTUME_SYSTEM = 1
    ENABLE_ENERGY_SYSTEM = 1
    ENABLE_DRAGON_SOUL_SYSTEM = 1
    DIK_ESCAPE = 1
    DIK_1 = 2
    DIK_2 = 3
    DIK_3 = 4
    DIK_4 = 5
    DIK_5 = 6
    DIK_6 = 7
    DIK_7 = 8
    DIK_8 = 9
    DIK_9 = 10
    DIK_0 = 11
    DIK_RETURN = 28
    DIK_SPACE = 57
    DIK_TAB = 15
    DIK_UP = 200
    DIK_LEFT = 203
    DIK_RIGHT = 205
    DIK_DOWN = 208

    @staticmethod
    def Abort():
        print("[APP] Abort")
    @staticmethod
    def Exit():
        print("[APP] Exit")
    @staticmethod
    def GetWidth():
        return 1024
    @staticmethod
    def GetHeight():
        return 768
    @staticmethod
    def GetRandom(a, b):
        import random
        return random.randint(int(a), int(b))
    @staticmethod
    def SetMouseHandler(*args): pass
    @staticmethod
    def SetHairColorEnable(*args): pass
    @staticmethod
    def SetArmorSpecularEnable(*args): pass
    @staticmethod
    def SetWeaponSpecularEnable(*args): pass
    @staticmethod
    def GetTime():
        import time
        return time.time()
    @staticmethod
    def GetGlobalTime():
        import time
        return int(time.time())
    @staticmethod
    def GetGlobalTimeStamp():
        import time
        return int(time.time())
    @staticmethod
    def Create(title, w, h, fullscreen):
        print(f"[APP] Create('{title}', {w}x{h})")
    @staticmethod
    def Loop():
        print("[+] [APP] Loop() wywolana -> uruchamianie natywnej petli renderowania GPU Pure Rust...")
        if dll and hasattr(dll, 'm2_run_game_loop'):
            dll.m2_run_game_loop()
    @staticmethod
    def GetLocaleName():
        return "en"
    @staticmethod
    def GetLocalePath():
        return "locale/en"
    loggined = False
    @staticmethod
    def SetDefaultFontName(font_name): pass
    @staticmethod
    def SetGuildMarkPath(path): pass
    @staticmethod
    def SetGuildSymbolPath(path): pass
    @staticmethod
    def GetLocalePathCommon():
        return "locale/common"
    @staticmethod
    def IsRTL(*args):
        return False

sys.modules['app'] = AppModule

class ServerStateCheckerModule:
    @staticmethod
    def Create(handler): pass
    @staticmethod
    def Initialize(handler=None): pass
    @staticmethod
    def Update(): pass
    @staticmethod
    def AddChannel(key, ip, udp_port): pass
    @staticmethod
    def Request(): pass

sys.modules['ServerStateChecker'] = ServerStateCheckerModule
sys.modules['serverStateChecker'] = ServerStateCheckerModule

class SystemSettingModuleMeta(type):
    def __getattr__(cls, name):
        if name.isupper():
            return 0
        return lambda *args, **kwargs: 0

class SystemSettingModule(metaclass=SystemSettingModuleMeta):
    @staticmethod
    def GetWidth():
        return 1024
    @staticmethod
    def GetHeight():
        return 768
    @staticmethod
    def GetBPP():
        return 32
    @staticmethod
    def IsSoftwareCursor():
        return 0
    @staticmethod
    def IsAlwaysShowName():
        return 1
    @staticmethod
    def IsShowDamage():
        return 1
    @staticmethod
    def IsShowSalesText():
        return 1

sys.modules['systemSetting'] = SystemSettingModule

_py_windows = {}
_window_texts = {}

def _py_dispatcher_0(handle, method_bytes):
    win = _py_windows.get(handle)
    if win:
        m = method_bytes.decode('utf-8', errors='ignore')
        fn = getattr(win, m, None)
        if fn:
            try:
                fn()
            except Exception as e:
                import traceback
                print(f"[EVENT ERR 0] {m}: {e}")
                traceback.print_exc()

def _py_dispatcher_1_str(handle, method_bytes, arg_bytes):
    win = _py_windows.get(handle)
    if win:
        m = method_bytes.decode('utf-8', errors='ignore')
        a = arg_bytes.decode('utf-8', errors='ignore') if arg_bytes else ""
        fn = getattr(win, m, None)
        if fn:
            try:
                fn(a)
            except Exception as e:
                print(f"[EVENT ERR 1_str] {m}({a}): {e}")

def _py_dispatcher_1_i32(handle, method_bytes, val):
    win = _py_windows.get(handle)
    if win:
        m = method_bytes.decode('utf-8', errors='ignore')
        fn = getattr(win, m, None)
        if fn:
            try:
                fn(val)
            except Exception as e:
                print(f"[EVENT ERR 1_i32] {m}({val}): {e}")

def _py_dispatcher_2_i32(handle, method_bytes, x, y):
    win = _py_windows.get(handle)
    if win:
        m = method_bytes.decode('utf-8', errors='ignore')
        fn = getattr(win, m, None)
        if fn:
            try:
                fn(x, y)
            except Exception as e:
                print(f"[EVENT ERR 2_i32] {m}({x}, {y}): {e}")

def _py_mod_dispatcher_0(mod_bytes, method_bytes):
    mod_name = mod_bytes.decode('utf-8', errors='ignore')
    mod = sys.modules.get(mod_name)
    if mod:
        m = method_bytes.decode('utf-8', errors='ignore')
        fn = getattr(mod, m, None)
        if fn:
            try:
                fn()
            except Exception as e:
                print(f"[MOD ERR 0] {mod_name}.{m}: {e}")

def _py_mod_dispatcher_1_str(mod_bytes, method_bytes, arg_bytes):
    mod_name = mod_bytes.decode('utf-8', errors='ignore')
    mod = sys.modules.get(mod_name)
    if mod:
        m = method_bytes.decode('utf-8', errors='ignore')
        a = arg_bytes.decode('utf-8', errors='ignore') if arg_bytes else ""
        fn = getattr(mod, m, None)
        if fn:
            try:
                fn(a)
            except Exception as e:
                print(f"[MOD ERR 1_str] {mod_name}.{m}({a}): {e}")

_CB_0 = ctypes.CFUNCTYPE(None, ctypes.c_size_t, ctypes.c_char_p)(_py_dispatcher_0)
_CB_1_STR = ctypes.CFUNCTYPE(None, ctypes.c_size_t, ctypes.c_char_p, ctypes.c_char_p)(_py_dispatcher_1_str)
_CB_1_I32 = ctypes.CFUNCTYPE(None, ctypes.c_size_t, ctypes.c_char_p, ctypes.c_int32)(_py_dispatcher_1_i32)
_CB_2_I32 = ctypes.CFUNCTYPE(None, ctypes.c_size_t, ctypes.c_char_p, ctypes.c_int32, ctypes.c_int32)(_py_dispatcher_2_i32)
_CB_MOD_0 = ctypes.CFUNCTYPE(None, ctypes.c_char_p, ctypes.c_char_p)(_py_mod_dispatcher_0)
_CB_MOD_1_STR = ctypes.CFUNCTYPE(None, ctypes.c_char_p, ctypes.c_char_p, ctypes.c_char_p)(_py_mod_dispatcher_1_str)

if dll:
    if hasattr(dll, 'm2_set_py_event_dispatcher_0'):
        dll.m2_set_py_event_dispatcher_0(_CB_0)
    if hasattr(dll, 'm2_set_py_event_dispatcher_1_str'):
        dll.m2_set_py_event_dispatcher_1_str(_CB_1_STR)
    if hasattr(dll, 'm2_set_py_event_dispatcher_1_i32'):
        dll.m2_set_py_event_dispatcher_1_i32(_CB_1_I32)
    if hasattr(dll, 'm2_set_py_event_dispatcher_2_i32'):
        dll.m2_set_py_event_dispatcher_2_i32(_CB_2_I32)
    if hasattr(dll, 'm2_set_py_mod_dispatcher_0'):
        dll.m2_set_py_mod_dispatcher_0(_CB_MOD_0)
    if hasattr(dll, 'm2_set_py_mod_dispatcher_1_str'):
        dll.m2_set_py_mod_dispatcher_1_str(_CB_MOD_1_STR)

    for reg in [
        'wndMgr_Register', 'wndMgr_RegisterImageBox', 'wndMgr_RegisterExpandedImageBox',
        'wndMgr_RegisterTextLine', 'wndMgr_RegisterButton', 'wndMgr_RegisterBar',
        'wndMgr_RegisterLine', 'wndMgr_RegisterBox', 'wndMgr_RegisterSlotWindow',
        'wndMgr_RegisterGridSlotWindow', 'wndMgr_RegisterMarkBox', 'wndMgr_RegisterAniImageBox',
        'wndMgr_RegisterRadioButton', 'wndMgr_RegisterToggleButton', 'wndMgr_RegisterDragButton',
        'wndMgr_SetMouseHandler',
    ]:
        if hasattr(dll, reg):
            getattr(dll, reg).argtypes = [ctypes.c_size_t]
            getattr(dll, reg).restype = ctypes.c_uint32

    if hasattr(dll, 'wndMgr_GetText'):
        dll.wndMgr_GetText.argtypes = [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_size_t]
        dll.wndMgr_GetText.restype = ctypes.c_size_t

    if hasattr(dll, 'm2_net_SetPhaseWindow'):
        dll.m2_net_SetPhaseWindow.argtypes = [ctypes.c_uint32, ctypes.c_size_t]
    if hasattr(dll, 'm2_net_ClearPhaseWindow'):
        dll.m2_net_ClearPhaseWindow.argtypes = [ctypes.c_uint32, ctypes.c_size_t]
    if hasattr(dll, 'm2_net_SetAccountConnectorHandler'):
        dll.m2_net_SetAccountConnectorHandler.argtypes = [ctypes.c_size_t]
    if hasattr(dll, 'm2_net_SetHandler'):
        dll.m2_net_SetHandler.argtypes = [ctypes.c_size_t]
    if hasattr(dll, 'm2_net_SetTCPRecvBufferSize'):
        dll.m2_net_SetTCPRecvBufferSize.argtypes = [ctypes.c_size_t]
    if hasattr(dll, 'm2_net_SetTCPSendBufferSize'):
        dll.m2_net_SetTCPSendBufferSize.argtypes = [ctypes.c_size_t]

dll.wndMgr_SetParent.argtypes = [ctypes.c_uint32, ctypes.c_uint32]
dll.wndMgr_SetParent.restype = ctypes.c_bool
dll.wndMgr_SetPosition.argtypes = [ctypes.c_uint32, ctypes.c_int32, ctypes.c_int32]
dll.wndMgr_SetPosition.restype = ctypes.c_bool
dll.wndMgr_SetSize.argtypes = [ctypes.c_uint32, ctypes.c_int32, ctypes.c_int32]
dll.wndMgr_SetSize.restype = ctypes.c_bool
dll.wndMgr_Show.argtypes = [ctypes.c_uint32]
dll.wndMgr_Show.restype = ctypes.c_bool
dll.wndMgr_Hide.argtypes = [ctypes.c_uint32]
dll.wndMgr_Hide.restype = ctypes.c_bool
dll.wndMgr_SetTop.argtypes = [ctypes.c_uint32]
dll.wndMgr_SetTop.restype = ctypes.c_bool
dll.wndMgr_IsShow.argtypes = [ctypes.c_uint32]
dll.wndMgr_IsShow.restype = ctypes.c_bool
dll.wndMgr_LoadImage.argtypes = [ctypes.c_uint32, ctypes.c_char_p]
dll.wndMgr_LoadImage.restype = ctypes.c_bool
dll.wndMgr_SetRenderingRect.argtypes = [ctypes.c_uint32, ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float]
dll.wndMgr_SetRenderingRect.restype = ctypes.c_bool
dll.wndMgr_SetScale.argtypes = [ctypes.c_uint32, ctypes.c_float, ctypes.c_float]
dll.wndMgr_SetScale.restype = ctypes.c_bool
dll.wndMgr_SetText.argtypes = [ctypes.c_uint32, ctypes.c_char_p]
dll.wndMgr_SetText.restype = ctypes.c_bool
dll.wndMgr_SetColor.argtypes = [ctypes.c_uint32, ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float]
dll.wndMgr_SetColor.restype = ctypes.c_bool
dll.wndMgr_SetFontColor.argtypes = [ctypes.c_uint32, ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float]
dll.wndMgr_SetFontColor.restype = ctypes.c_bool
dll.wndMgr_SetAlign.argtypes = [ctypes.c_uint32, ctypes.c_uint8, ctypes.c_uint8]
dll.wndMgr_SetAlign.restype = ctypes.c_bool
dll.wndMgr_SetUpVisual.argtypes = [ctypes.c_uint32, ctypes.c_char_p]
dll.wndMgr_SetUpVisual.restype = ctypes.c_bool
dll.wndMgr_SetOverVisual.argtypes = [ctypes.c_uint32, ctypes.c_char_p]
dll.wndMgr_SetOverVisual.restype = ctypes.c_bool
dll.wndMgr_SetDownVisual.argtypes = [ctypes.c_uint32, ctypes.c_char_p]
dll.wndMgr_SetDownVisual.restype = ctypes.c_bool
dll.wndMgr_GetWidth.argtypes = [ctypes.c_uint32]
dll.wndMgr_GetWidth.restype = ctypes.c_int32
dll.wndMgr_GetHeight.argtypes = [ctypes.c_uint32]
dll.wndMgr_GetHeight.restype = ctypes.c_int32

_next_handle = 1000
def _alloc_handle():
    global _next_handle
    _next_handle += 1
    return _next_handle

class WndMgrModuleMeta(type):
    def __getattr__(cls, name):
        if name.isupper():
            return 1
        if 'Position' in name or 'Pos' in name:
            return lambda *a, **k: (0, 0)
        if 'Rect' in name:
            return lambda *a, **k: (0, 0, 0, 0)
        if 'Size' in name:
            return lambda *a, **k: (0, 0)
        if name.startswith('Register'):
            return lambda *a, **k: _alloc_handle()
        return lambda *args, **kwargs: 0

class WndMgrModule(metaclass=WndMgrModuleMeta):
    SLOT_STYLE_NONE = 0
    SLOT_STYLE_PICK_UP = 1
    SLOT_STYLE_SELECT = 2
    HORIZONTAL_ALIGN_LEFT = 0
    HORIZONTAL_ALIGN_CENTER = 1
    HORIZONTAL_ALIGN_RIGHT = 2
    VERTICAL_ALIGN_TOP = 0
    VERTICAL_ALIGN_CENTER = 1
    VERTICAL_ALIGN_BOTTOM = 2
    TEXT_HORIZONTAL_ALIGN_LEFT = 0
    TEXT_HORIZONTAL_ALIGN_RIGHT = 1
    TEXT_HORIZONTAL_ALIGN_CENTER = 2

    @staticmethod
    def SetScreenSize(w, h): pass
    @staticmethod
    def GetScreenWidth(): return 1024
    @staticmethod
    def GetScreenHeight(): return 768

    @classmethod
    def Register(cls, win, layer="UI"):
        h = id(win) if win else 0
        if win and h: _py_windows[h] = win
        if dll and hasattr(dll, 'wndMgr_Register'):
            return dll.wndMgr_Register(h)
        return _alloc_handle()
    @classmethod
    def RegisterImageBox(cls, win, layer="UI"):
        h = id(win) if win else 0
        if win and h: _py_windows[h] = win
        if dll and hasattr(dll, 'wndMgr_RegisterImageBox'):
            return dll.wndMgr_RegisterImageBox(h)
        return _alloc_handle()
    @classmethod
    def RegisterExpandedImageBox(cls, win, layer="UI"):
        h = id(win) if win else 0
        if win and h: _py_windows[h] = win
        if dll and hasattr(dll, 'wndMgr_RegisterExpandedImageBox'):
            return dll.wndMgr_RegisterExpandedImageBox(h)
        return _alloc_handle()
    @classmethod
    def RegisterTextLine(cls, win, layer="UI"):
        h = id(win) if win else 0
        if win and h: _py_windows[h] = win
        if dll and hasattr(dll, 'wndMgr_RegisterTextLine'):
            return dll.wndMgr_RegisterTextLine(h)
        return _alloc_handle()
    @classmethod
    def RegisterButton(cls, win, layer="UI"):
        h = id(win) if win else 0
        if win and h: _py_windows[h] = win
        if dll and hasattr(dll, 'wndMgr_RegisterButton'):
            return dll.wndMgr_RegisterButton(h)
        return _alloc_handle()
    @classmethod
    def RegisterBar(cls, win, layer="UI"):
        h = id(win) if win else 0
        if win and h: _py_windows[h] = win
        if dll and hasattr(dll, 'wndMgr_RegisterBar'):
            return dll.wndMgr_RegisterBar(h)
        return _alloc_handle()
    @classmethod
    def RegisterLine(cls, win, layer="UI"):
        h = id(win) if win else 0
        if win and h: _py_windows[h] = win
        if dll and hasattr(dll, 'wndMgr_RegisterLine'):
            return dll.wndMgr_RegisterLine(h)
        return _alloc_handle()
    @classmethod
    def RegisterBox(cls, win, layer="UI"):
        h = id(win) if win else 0
        if win and h: _py_windows[h] = win
        if dll and hasattr(dll, 'wndMgr_RegisterBox'):
            return dll.wndMgr_RegisterBox(h)
        return _alloc_handle()
    @classmethod
    def RegisterWindow(cls, win, layer="UI"):
        h = id(win) if win else 0
        if win and h: _py_windows[h] = win
        if dll and hasattr(dll, 'wndMgr_Register'):
            return dll.wndMgr_Register(h)
        return _alloc_handle()
    @classmethod
    def RegisterSlotWindow(cls, win, layer="UI"):
        h = id(win) if win else 0
        if win and h: _py_windows[h] = win
        if dll and hasattr(dll, 'wndMgr_RegisterSlotWindow'):
            return dll.wndMgr_RegisterSlotWindow(h)
        return _alloc_handle()
    @classmethod
    def RegisterGridSlotWindow(cls, win, layer="UI"):
        h = id(win) if win else 0
        if win and h: _py_windows[h] = win
        if dll and hasattr(dll, 'wndMgr_RegisterGridSlotWindow'):
            return dll.wndMgr_RegisterGridSlotWindow(h)
        return _alloc_handle()
    @classmethod
    def RegisterMarkBox(cls, win, layer="UI"):
        h = id(win) if win else 0
        if win and h: _py_windows[h] = win
        if dll and hasattr(dll, 'wndMgr_RegisterMarkBox'):
            return dll.wndMgr_RegisterMarkBox(h)
        return _alloc_handle()
    @classmethod
    def RegisterAniImageBox(cls, win, layer="UI"):
        h = id(win) if win else 0
        if win and h: _py_windows[h] = win
        if dll and hasattr(dll, 'wndMgr_RegisterAniImageBox'):
            return dll.wndMgr_RegisterAniImageBox(h)
        return _alloc_handle()
    @classmethod
    def RegisterRadioButton(cls, win, layer="UI"):
        h = id(win) if win else 0
        if win and h: _py_windows[h] = win
        if dll and hasattr(dll, 'wndMgr_RegisterRadioButton'):
            return dll.wndMgr_RegisterRadioButton(h)
        return _alloc_handle()
    @classmethod
    def RegisterToggleButton(cls, win, layer="UI"):
        h = id(win) if win else 0
        if win and h: _py_windows[h] = win
        if dll and hasattr(dll, 'wndMgr_RegisterToggleButton'):
            return dll.wndMgr_RegisterToggleButton(h)
        return _alloc_handle()
    @classmethod
    def RegisterDragButton(cls, win, layer="UI"):
        h = id(win) if win else 0
        if win and h: _py_windows[h] = win
        if dll and hasattr(dll, 'wndMgr_RegisterDragButton'):
            return dll.wndMgr_RegisterDragButton(h)
        return _alloc_handle()

    @staticmethod
    def SetParent(child, parent):
        return dll.wndMgr_SetParent(int(child), int(parent))
    @staticmethod
    def SetPosition(handle, x, y):
        return dll.wndMgr_SetPosition(int(handle), int(x), int(y))
    @staticmethod
    def SetWindowPosition(handle, x, y):
        return dll.wndMgr_SetPosition(int(handle), int(x), int(y))
    @staticmethod
    def SetSize(handle, w, h):
        return dll.wndMgr_SetSize(int(handle), int(w), int(h))
    @staticmethod
    def SetWindowSize(handle, w, h):
        return dll.wndMgr_SetSize(int(handle), int(w), int(h))
    @staticmethod
    def Show(handle):
        return dll.wndMgr_Show(int(handle))
    @staticmethod
    def Hide(handle):
        return dll.wndMgr_Hide(int(handle))
    @staticmethod
    def SetTop(handle):
        return dll.wndMgr_SetTop(int(handle))
    @staticmethod
    def IsShow(handle):
        return dll.wndMgr_IsShow(int(handle))
    @staticmethod
    def LoadImage(handle, path):
        return dll.wndMgr_LoadImage(int(handle), str(path).encode('utf-8'))
    @staticmethod
    def LoadImageFromFile(handle, path):
        return dll.wndMgr_LoadImage(int(handle), str(path).encode('utf-8'))
    @staticmethod
    def SetRenderingRect(handle, l, t, r, b):
        return dll.wndMgr_SetRenderingRect(int(handle), float(l), float(t), float(r), float(b))
    @staticmethod
    def SetScale(handle, sx, sy):
        return dll.wndMgr_SetScale(int(handle), float(sx), float(sy))
    @staticmethod
    def SetText(handle, text):
        _window_texts[int(handle)] = str(text)
        return dll.wndMgr_SetText(int(handle), str(text).encode('utf-8'))
    @staticmethod
    def GetText(handle):
        if dll and hasattr(dll, 'wndMgr_GetText'):
            buf = ctypes.create_string_buffer(512)
            res = dll.wndMgr_GetText(int(handle), buf, 512)
            if res > 0:
                return buf.value.decode('utf-8', errors='ignore')
        return _window_texts.get(int(handle), '')
    @staticmethod
    def GetLocalPosition(handle):
        x = ctypes.c_int32(0)
        y = ctypes.c_int32(0)
        if dll and hasattr(dll, 'wndMgr_GetLocalPosition'):
            try:
                dll.wndMgr_GetLocalPosition(int(handle), ctypes.byref(x), ctypes.byref(y))
                return (x.value, y.value)
            except Exception: pass
        return (0, 0)
    @staticmethod
    def GetWindowLocalPosition(handle):
        return WndMgrModule.GetLocalPosition(handle)
    @staticmethod
    def GetWindowGlobalPosition(handle):
        return WndMgrModule.GetLocalPosition(handle)
    @staticmethod
    def GetMousePosition(): return (0, 0)
    @staticmethod
    def GetMouseLocalPosition(handle): return (0, 0)
    @staticmethod
    def GetTextSize(handle): return (0, 0)
    @staticmethod
    def GetName(handle): return ''
    @staticmethod
    def IsFocus(handle): return False
    @staticmethod
    def IsDown(handle): return False
    @staticmethod
    def IsIn(handle): return False
    @staticmethod
    def SetColor(handle, color, *args):
        if isinstance(color, int):
            a = ((color >> 24) & 0xff) / 255.0
            r = ((color >> 16) & 0xff) / 255.0
            g = ((color >> 8) & 0xff) / 255.0
            b = (color & 0xff) / 255.0
            return dll.wndMgr_SetColor(int(handle), r, g, b, a)
        return True
    @staticmethod
    def SetFontColor(handle, r, g=None, b=None):
        if g is not None and b is not None:
            red = float(r)/255.0 if r > 1.0 else float(r)
            green = float(g)/255.0 if g > 1.0 else float(g)
            blue = float(b)/255.0 if b > 1.0 else float(b)
            return dll.wndMgr_SetFontColor(int(handle), red, green, blue, 1.0)
        elif isinstance(r, int):
            c = r
            a = ((c >> 24) & 0xff) / 255.0
            red = ((c >> 16) & 0xff) / 255.0
            green = ((c >> 8) & 0xff) / 255.0
            blue = (c & 0xff) / 255.0
            return dll.wndMgr_SetFontColor(int(handle), red, green, blue, a)
        return True
    @staticmethod
    def SetWindowHorizontalAlign(handle, align):
        return dll.wndMgr_SetAlign(int(handle), int(align), 0)
    @staticmethod
    def SetWindowVerticalAlign(handle, align):
        return dll.wndMgr_SetAlign(int(handle), 0, int(align))
    @staticmethod
    def SetUpVisual(handle, path):
        return dll.wndMgr_SetUpVisual(int(handle), str(path).encode('utf-8'))
    @staticmethod
    def SetOverVisual(handle, path):
        return dll.wndMgr_SetOverVisual(int(handle), str(path).encode('utf-8'))
    @staticmethod
    def SetDownVisual(handle, path):
        return dll.wndMgr_SetDownVisual(int(handle), str(path).encode('utf-8'))
    @staticmethod
    def GetWidth(handle):
        return dll.wndMgr_GetWidth(int(handle))
    @staticmethod
    def GetHeight(handle):
        return dll.wndMgr_GetHeight(int(handle))
    @staticmethod
    def GetWindowWidth(handle):
        return dll.wndMgr_GetWidth(int(handle))
    @staticmethod
    def GetWindowHeight(handle):
        return dll.wndMgr_GetHeight(int(handle))
    @staticmethod
    def GetWindowLocalPosition(handle):
        return (0, 0)
    @staticmethod
    def GetWindowGlobalPosition(handle):
        return (0, 0)
    @staticmethod
    def GetMouseLocalPosition(handle):
        return (0, 0)
    @staticmethod
    def GetWindowRect(handle):
        w = dll.wndMgr_GetWidth(int(handle))
        h = dll.wndMgr_GetHeight(int(handle))
        return (0, 0, w, h)
    @staticmethod
    def Destroy(handle):
        return dll.wndMgr_Destroy(int(handle))
    @staticmethod
    def IsRTL(*args):
        return False

sys.modules['wndMgr'] = WndMgrModule

class GrpModuleMeta(type):
    def __getattr__(cls, name):
        if name.isupper():
            return 0
        return lambda *args, **kwargs: 0

class GrpModule(metaclass=GrpModuleMeta):
    @staticmethod
    def GenerateColor(r, g, b, a=1.0):
        ia = int(a * 255.0) & 0xff
        ir = int(r * 255.0) & 0xff
        ig = int(g * 255.0) & 0xff
        ib = int(b * 255.0) & 0xff
        return (ia << 24) | (ir << 16) | (ig << 8) | ib

sys.modules['grp'] = GrpModule
sys.modules['grpText'] = GrpModule
sys.modules['grpImage'] = GrpModule

class NetModuleMeta(type):
    def __getattr__(cls, name):
        if name.isupper():
            return 0
        return lambda *args, **kwargs: 0

class NetModule(metaclass=NetModuleMeta):
    PHASE_WINDOW_LOGO = 0
    PHASE_WINDOW_LOGIN = 1
    PHASE_WINDOW_SELECT = 2
    PHASE_WINDOW_CREATE = 3
    PHASE_WINDOW_LOAD = 4
    PHASE_WINDOW_GAME = 5
    PHASE_WINDOW_EMPIRE = 6
    ACCOUNT_CHARACTER_SLOT_ID = 0
    ACCOUNT_CHARACTER_SLOT_NAME = 1
    ACCOUNT_CHARACTER_SLOT_RACE = 2
    ACCOUNT_CHARACTER_SLOT_LEVEL = 3
    ACCOUNT_CHARACTER_SLOT_STR = 4
    ACCOUNT_CHARACTER_SLOT_DEX = 5
    ACCOUNT_CHARACTER_SLOT_HTH = 6
    ACCOUNT_CHARACTER_SLOT_INT = 7
    ACCOUNT_CHARACTER_SLOT_PLAYTIME = 8
    ACCOUNT_CHARACTER_SLOT_FORM = 9
    ACCOUNT_CHARACTER_SLOT_ADDR = 10
    ACCOUNT_CHARACTER_SLOT_PORT = 11
    ACCOUNT_CHARACTER_SLOT_GUILD_ID = 12
    ACCOUNT_CHARACTER_SLOT_GUILD_NAME = 13
    ACCOUNT_CHARACTER_SLOT_CHANGE_NAME_FLAG = 14
    ACCOUNT_CHARACTER_SLOT_HAIR = 15
    EMPIRE_A = 1
    EMPIRE_B = 2
    EMPIRE_C = 3
    ERROR_NONE = 0

    @staticmethod
    def Connect(ip, port):
        if dll and hasattr(dll, 'm2_net_Connect'):
            return dll.m2_net_Connect(str(ip).encode('utf-8'), int(port))
        return True
    @staticmethod
    def SetPhaseWindow(phase, win):
        h = id(win) if win else 0
        if win and h: _py_windows[h] = win
        if dll and hasattr(dll, 'm2_net_SetPhaseWindow'):
            dll.m2_net_SetPhaseWindow(int(phase), h)
    @staticmethod
    def ClearPhaseWindow(phase, win):
        h = id(win) if win else 0
        if win and h: _py_windows.pop(h, None)
        if dll and hasattr(dll, 'm2_net_ClearPhaseWindow'):
            dll.m2_net_ClearPhaseWindow(int(phase), h)
    @staticmethod
    def SetAccountConnectorHandler(handler):
        h = id(handler) if handler else 0
        if handler and h: _py_windows[h] = handler
        if dll and hasattr(dll, 'm2_net_SetAccountConnectorHandler'):
            dll.m2_net_SetAccountConnectorHandler(h)
    @staticmethod
    def SetHandler(handler):
        h = id(handler) if handler else 0
        if handler and h: _py_windows[h] = handler
        if dll and hasattr(dll, 'm2_net_SetHandler'):
            dll.m2_net_SetHandler(h)
    @staticmethod
    def ConnectToAccountServer(addr, port, acc_addr, acc_port):
        if dll and hasattr(dll, 'm2_net_ConnectToAccountServer'):
            return dll.m2_net_ConnectToAccountServer(str(addr).encode('utf-8'), int(port), str(acc_addr).encode('utf-8'), int(acc_port))
        return True
    @staticmethod
    def SetLoginInfo(name, pwd):
        if dll and hasattr(dll, 'm2_net_SetLoginInfo'):
            dll.m2_net_SetLoginInfo(str(name).encode('utf-8'), str(pwd).encode('utf-8'))
    @staticmethod
    def GetAccountCharacterSlotDataInteger(slot, data_type):
        if dll and hasattr(dll, 'm2_net_GetAccountCharacterSlotDataInteger'):
            return dll.m2_net_GetAccountCharacterSlotDataInteger(int(slot), int(data_type))
        return 0
    @staticmethod
    def GetAccountCharacterSlotDataString(slot, data_type):
        if dll and hasattr(dll, 'm2_net_GetAccountCharacterSlotDataString'):
            buf = ctypes.create_string_buffer(256)
            dll.m2_net_GetAccountCharacterSlotDataString(int(slot), int(data_type), buf, 256)
            return buf.value.decode('utf-8', errors='ignore')
        return ""
    @staticmethod
    def GetEmpireID():
        if dll and hasattr(dll, 'm2_net_GetEmpireID'):
            return dll.m2_net_GetEmpireID()
        return 1
    @staticmethod
    def SetEmpireLanguageMode(m): pass
    @staticmethod
    def SetServerInfo(info): pass
    @staticmethod
    def SetMarkServer(addr, port): pass
    @staticmethod
    def ToggleGameDebugInfo(): pass
    @staticmethod
    def IsInsultIn(s): return False

sys.modules['net'] = NetModule

class IMEModuleMeta(type):
    def __getattr__(cls, name):
        if name.isupper():
            return 1
        return lambda *args, **kwargs: 0

class IMEModule(metaclass=IMEModuleMeta):
    _text = ""
    _max = 30
    _user_max = 30
    _cursor = 0
    _secret = False

    @classmethod
    def SetText(cls, text):
        cls._text = str(text) if text else ""
        cls._cursor = len(cls._text)
    @classmethod
    def GetText(cls, secret=False):
        if secret or cls._secret:
            return "*" * len(cls._text)
        return cls._text
    @classmethod
    def GetCodePage(cls): return 1250
    @classmethod
    def SetMax(cls, m): cls._max = int(m)
    @classmethod
    def SetUserMax(cls, m): cls._user_max = int(m)
    @classmethod
    def SetCursorPosition(cls, pos):
        if pos < 0: cls._cursor = len(cls._text)
        else: cls._cursor = min(len(cls._text), max(0, int(pos)))
    @classmethod
    def SetNumberMode(cls): pass
    @classmethod
    def SetStringMode(cls): pass
    @classmethod
    def SetSecretMode(cls, flag): cls._secret = bool(flag)
    @classmethod
    def EnableCaptureInput(cls): pass
    @classmethod
    def DisableCaptureInput(cls): pass
    @classmethod
    def EnableIME(cls): pass
    @classmethod
    def DisableIME(cls): pass
    @classmethod
    def AddExceptKey(cls, key): pass
    @classmethod
    def ClearExceptKey(cls): pass
    @classmethod
    def MoveEnd(cls): cls._cursor = len(cls._text)
    @classmethod
    def MoveHome(cls): cls._cursor = 0
    @classmethod
    def MoveLeft(cls): cls._cursor = max(0, cls._cursor - 1)
    @classmethod
    def MoveRight(cls): cls._cursor = min(len(cls._text), cls._cursor + 1)
    @classmethod
    def GetReading(cls): return ""
    @classmethod
    def GetReadingError(cls): return 0
    @classmethod
    def GetCandidate(cls, idx): return ""
    @classmethod
    def GetCandidateCount(cls): return 0
    @classmethod
    def GetCandidateSelection(cls): return 0

sys.modules['ime'] = IMEModule
sys.modules['IME'] = IMEModule

class SndModuleMeta(type):
    def __getattr__(cls, name):
        if name.isupper():
            return 1
        return lambda *args, **kwargs: 0

class SndModule(metaclass=SndModuleMeta):
    @staticmethod
    def PlaySound(filename): return True
    @staticmethod
    def PlaySound3D(filename, x=0, y=0, z=0): return True
    @staticmethod
    def PlayBGM(filename): return True
    @staticmethod
    def StopBGM(): return True
    @staticmethod
    def SetSoundVolume(vol): pass
    @staticmethod
    def SetBGMVolume(vol): pass
    @staticmethod
    def SetMusicVolume(vol): pass
    @staticmethod
    def SetMasterVolume(vol): pass
    @staticmethod
    def FadeInMusic(filename): pass
    @staticmethod
    def FadeOutMusic(): pass
    @staticmethod
    def FadeOutAllMusic(): pass
    @staticmethod
    def FadeLimitOutMusic(): pass
    @staticmethod
    def StopAllSound(): pass

sys.modules['snd'] = SndModule
sys.modules['Snd'] = SndModule


class DynamicModuleMeta(type):
    def __getattr__(cls, name):
        if name.startswith('__'):
            raise AttributeError(name)
        if name.isupper():
            return 1
        return lambda *args, **kwargs: 0

class DynamicModule(metaclass=DynamicModuleMeta):
    pass

for m in [
    'player', 'Player', 'item', 'Item', 'chr', 'chrmgr',
    'guild', 'Guild', 'nonplayer', 'NonPlayer', 'quest', 'Quest',
    'safebox', 'Safebox', 'shop', 'Shop', 'skill', 'Skill',
    'target', 'Target', 'event', 'Event',
    'miniMap', 'MiniMap', 'profiler', 'Profiler',
    'background', 'Background', 'trade', 'Trade', 'chat', 'Chat',
    'textTail', 'TextTail', 'effect', 'Effect', 'fly', 'Fly',
    'eventmgr', 'EventMgr', 'messenger', 'Messenger',
    'serverStateChecker', 'ServerStateChecker'
]:
    sys.modules[m] = DynamicModule

import builtins
builtins.TRUE = 1
builtins.FALSE = 0

import __main__
__main__.__DEBUG__ = 1

# Uruchomienie system.py z root.pck
system_py_data = PackModule.Get('system.py')
if system_py_data:
    print(f"[+] Ladowanie system.py z VFS ({len(system_py_data)} bajtow)...")
    exec(compile(system_py_data, 'system.py', 'exec'), __main__.__dict__)
    print("[+] system.py wykonany pomyslnie!")
"#).unwrap();

            let py_err_sym = GetProcAddress(py_dll, CString::new("PyErr_Print").unwrap().as_ptr());
            let py_err: Option<extern "C" fn()> = if !py_err_sym.is_null() {
                Some(std::mem::transmute(py_err_sym))
            } else {
                None
            };

            println!("[+] Mostek Pythona 3.14 zainicjalizowany, uruchamianie system.py...");
            let ret = py_run(py_code.as_ptr());
            if ret != 0 {
                println!("[-] Blad podczas wykonywania py_code (kod: {})", ret);
                if let Some(err_fn) = py_err {
                    err_fn();
                }
            }

            // Wstrzykujemy pelne, natywne moduly C-API Pythona (player, chr, net, item, ui itp.)
            if let Err(e) = crate::py_bootstrap_runtime_injector::inject_runtime_modules() {
                println!("[-] Ostrzezenie wstrzykiwania modulow runtime Pythona: {}", e);
            } else {
                println!("[+] Pomyslnie wstrzyknieto pelne moduly runtime Pythona podlaczone pod GameState!");
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

        // Jesli Python nie przeszedl do glownej petli (lub w trybie fallback), uruchom natywna petle gry
        println!("[+] Uruchamianie natywnej petli renderowania okna gry...");
        crate::wgpu_engine_context::m2_run_game_loop();

        self.state = EngineState::Stopped;
        println!("[+] Metin2 Pure Rust Client zakonczyl dzialanie pomyslnie.");
        Ok(())
    }
}
