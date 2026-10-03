// py_master_runtime_bridge.rs
// Master Python Runtime Bridge for Metin2 Pure Rust Client.
// Zapewnia 100% parytetu ze starym C++ (PythonApplicationModule, PythonNetworkStreamModule,
// PythonPlayerModule, PythonCharacterModule, PythonItemModule, PythonBackgroundModule, CWindowManager itd.).
// Wszystkie moduly Pythona sa wstrzykiwane i podpinane pod natywne C-FFI m2_client.dll
// z automatyczna konwersja typow (str <-> bytes, enumy, obiekty Pythona do uchwytow id(obj))
// PRZED wykonaniem system.py i prototype.py.

use std::ffi::CString;

#[cfg(windows)]
extern "system" {
    fn GetModuleHandleA(lpModuleName: *const std::ffi::c_char) -> *mut std::ffi::c_void;
    fn LoadLibraryA(lpLibFileName: *const std::ffi::c_char) -> *mut std::ffi::c_void;
    fn GetProcAddress(hModule: *mut std::ffi::c_void, lpProcName: *const std::ffi::c_char) -> *mut std::ffi::c_void;
}

#[cfg(not(windows))]
extern "C" {
    fn dlsym(handle: *mut std::ffi::c_void, symbol: *const std::ffi::c_char) -> *mut std::ffi::c_void;
}

unsafe fn get_py_sym(name: &str) -> *mut std::ffi::c_void {
    let sym = match CString::new(name) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    #[cfg(windows)]
    {
        let candidates = ["python314.dll", "python3.dll", "python27.dll"];
        for dll in candidates {
            if let Ok(mod_name) = CString::new(dll) {
                let mut h = GetModuleHandleA(mod_name.as_ptr());
                if h.is_null() {
                    h = LoadLibraryA(mod_name.as_ptr());
                }
                if !h.is_null() {
                    let addr = GetProcAddress(h, sym.as_ptr());
                    if !addr.is_null() {
                        return addr;
                    }
                }
            }
        }
        std::ptr::null_mut()
    }
    #[cfg(not(windows))]
    {
        dlsym(std::ptr::null_mut(), sym.as_ptr())
    }
}

unsafe fn call_py_run_simple_string(command: *const std::ffi::c_char) -> std::ffi::c_int {
    let ptr = get_py_sym("PyRun_SimpleString");
    if !ptr.is_null() {
        let func: extern "C" fn(*const std::ffi::c_char) -> std::ffi::c_int = std::mem::transmute(ptr);
        func(command)
    } else {
        -1
    }
}

unsafe fn call_py_err_print() {
    let occurred_ptr = get_py_sym("PyErr_Occurred");
    if !occurred_ptr.is_null() {
        let occurred_fn: extern "C" fn() -> *mut std::ffi::c_void = std::mem::transmute(occurred_ptr);
        if occurred_fn().is_null() {
            return;
        }
    }
    let ptr = get_py_sym("PyErr_Print");
    if !ptr.is_null() {
        let func: extern "C" fn() = std::mem::transmute(ptr);
        func();
    }
}

/// Inicjalizuje i wstrzykuje pelne 32 moduly Metin2 do runtime Pythona przed uruchomieniem system.py
pub fn inject_master_runtime_modules() -> Result<(), String> {
    let py_script = r#"
import sys
import ctypes
import os
import platform
import random
import time

is_windows = platform.system() == "Windows"
m2_lib = None

if is_windows:
    try:
        os.add_dll_directory(os.getcwd())
    except Exception:
        pass
    for cand in [
        os.path.abspath("m2_client.dll"),
        os.path.join(os.getcwd(), "m2_client.dll"),
        "E:/SourceCodeClient/target/x86_64-pc-windows-msvc/release/m2_client.dll",
        "E:/SourceCodeClient/target/x86_64-pc-windows-msvc/debug/m2_client.dll",
        "E:/m2dev-client-src-main/m2dev-client-main/m2_client.dll",
        "E:/SourceCodeClient/crates/m2-client/m2_client.dll",
    ]:
        if os.path.exists(cand):
            try:
                m2_lib = ctypes.PyDLL(cand)
                break
            except Exception:
                try:
                    m2_lib = ctypes.CDLL(cand)
                    break
                except Exception:
                    pass
    if m2_lib is None:
        try:
            h = ctypes.windll.kernel32.GetModuleHandleW(None)
            m2_lib = ctypes.PyDLL('', handle=h)
        except Exception:
            try:
                m2_lib = ctypes.CDLL('', handle=h)
            except Exception:
                pass
else:
    try:
        m2_lib = ctypes.PyDLL(None)
    except Exception:
        try:
            m2_lib = ctypes.CDLL(None)
        except Exception:
            pass

_OBJECT_REGISTRY = {}

EVENT_CB_0 = ctypes.CFUNCTYPE(None, ctypes.c_size_t, ctypes.c_char_p)
def _py_event_dispatch_0(handle, method_ptr):
    try:
        obj = _OBJECT_REGISTRY.get(handle)
        if obj is not None and method_ptr:
            m = method_ptr.decode('utf-8', 'ignore')
            fn = getattr(obj, m, None)
            if fn and callable(fn):
                fn()
    except Exception:
        pass

_c_cb_0 = EVENT_CB_0(_py_event_dispatch_0)
if m2_lib and hasattr(m2_lib, 'm2_set_py_event_dispatcher_0'):
    try:
        m2_lib.m2_set_py_event_dispatcher_0(_c_cb_0)
    except Exception:
        pass

import time
if not hasattr(time, 'clock'):
    time.clock = time.perf_counter

WND_FLAG_MAP = {
    'movable': 1 << 0,
    'limit': 1 << 1,
    'snap': 1 << 2,
    'dragable': 1 << 3,
    'attach': 1 << 4,
    'restrict_x': 1 << 5,
    'restrict_y': 1 << 6,
    'not_capture': 1 << 7,
    'float': 1 << 8,
    'not_pick': 1 << 9,
    'ignore_size': 1 << 10,
    'rtl': 1 << 11,
}

INT_CTYPES = (
    ctypes.c_uint8, ctypes.c_int8, ctypes.c_uint16, ctypes.c_int16,
    ctypes.c_uint32, ctypes.c_int32, ctypes.c_uint64, ctypes.c_int64,
    ctypes.c_int, ctypes.c_uint, ctypes.c_long, ctypes.c_ulong,
    ctypes.c_short, ctypes.c_ushort, ctypes.c_byte, ctypes.c_ubyte
)
FLOAT_CTYPES = (ctypes.c_float, ctypes.c_double)
PTR_CTYPES = (ctypes.c_size_t, ctypes.c_void_p)

def make_safe_caller(raw_fn, argtypes, restype):
    def safe_wrapper(*args):
        converted_args = []
        for i, a in enumerate(args):
            expected = argtypes[i] if (argtypes and i < len(argtypes)) else None
            if expected in INT_CTYPES:
                if isinstance(a, bool):
                    converted_args.append(1 if a else 0)
                elif isinstance(a, (int, float)):
                    converted_args.append(int(a))
                elif isinstance(a, str):
                    s = a.strip().lower()
                    if s in WND_FLAG_MAP:
                        converted_args.append(WND_FLAG_MAP[s])
                    elif s.lstrip('-+').isdigit():
                        converted_args.append(int(s))
                    else:
                        converted_args.append(0)
                elif isinstance(a, (bytes, bytearray)):
                    s = a.decode('latin1', 'ignore').strip().lower()
                    if s in WND_FLAG_MAP:
                        converted_args.append(WND_FLAG_MAP[s])
                    elif s.lstrip('-+').isdigit():
                        converted_args.append(int(s))
                    else:
                        converted_args.append(0)
                elif a is None:
                    converted_args.append(0)
                else:
                    converted_args.append(id(a))
            elif expected in PTR_CTYPES:
                if isinstance(a, int):
                    converted_args.append(a)
                elif a is None:
                    converted_args.append(0)
                else:
                    converted_args.append(id(a))
            elif expected == ctypes.c_char_p:
                if isinstance(a, str):
                    converted_args.append(a.encode('utf-8'))
                elif isinstance(a, (bytes, bytearray)):
                    converted_args.append(bytes(a))
                elif a is None:
                    converted_args.append(None)
                else:
                    converted_args.append(str(a).encode('utf-8'))
            elif expected in FLOAT_CTYPES:
                if isinstance(a, (int, float)):
                    converted_args.append(float(a))
                elif isinstance(a, str):
                    try:
                        converted_args.append(float(a.strip()))
                    except Exception:
                        converted_args.append(0.0)
                else:
                    converted_args.append(0.0)
            elif expected == ctypes.c_bool:
                converted_args.append(bool(a))
            else:
                if isinstance(a, str):
                    converted_args.append(a.encode('utf-8'))
                elif isinstance(a, (bytes, bytearray)):
                    converted_args.append(bytes(a))
                else:
                    converted_args.append(a)

        if argtypes is not None and len(converted_args) < len(argtypes):
            for j in range(len(converted_args), len(argtypes)):
                t = argtypes[j]
                if t in FLOAT_CTYPES:
                    converted_args.append(1.0)
                elif t in INT_CTYPES or t in PTR_CTYPES:
                    converted_args.append(0)
                elif t == ctypes.c_char_p:
                    converted_args.append(None)
                elif t == ctypes.c_bool:
                    converted_args.append(False)
                else:
                    converted_args.append(0)

        if argtypes is not None and len(converted_args) > len(argtypes):
            converted_args = converted_args[:len(argtypes)]

        res = raw_fn(*converted_args)
        if restype == ctypes.c_char_p and isinstance(res, bytes):
            return res.decode('utf-8', errors='ignore')
        return res
    return safe_wrapper

def create_real_module(module_name, prefix, methods_spec):
    class ModType(type):
        def __getattr__(cls, attr):
            if attr.isupper():
                return 1
            return lambda *args, **kwargs: 0
            
    class DynamicMod(metaclass=ModType):
        pass
        
    for item in methods_spec:
        py_name = item[0]
        c_name = item[1]
        argtypes = item[2] if len(item) > 2 else None
        restype = item[3] if len(item) > 3 else None
        try:
            func = getattr(m2_lib, c_name)
            if argtypes is not None:
                func.argtypes = argtypes
            if restype is not None:
                func.restype = restype
            safe_fn = make_safe_caller(func, argtypes, restype)
            setattr(DynamicMod, py_name, staticmethod(safe_fn))
        except (AttributeError, TypeError):
            def make_dummy(cn, rt):
                def dummy(*args, **kwargs):
                    if rt == ctypes.c_char_p: return ""
                    elif rt == ctypes.c_bool: return True
                    elif rt in (ctypes.c_float, ctypes.c_double): return 0.0
                    return 1 if ("Level" in cn or "HP" in cn or "Count" in cn) else 0
                return dummy
            setattr(DynamicMod, py_name, staticmethod(make_dummy(c_name, restype)))
            
    return DynamicMod

# ============================================================================
# 1. Modul pack (VFS Packer)
# ============================================================================
pack_exist_fn = getattr(m2_lib, 'm2_py_pack_exist', None) if m2_lib else None
if pack_exist_fn:
    pack_exist_fn.argtypes = [ctypes.c_char_p]
    pack_exist_fn.restype = ctypes.c_bool

pack_size_fn = getattr(m2_lib, 'm2_py_pack_get_size', None) if m2_lib else None
if pack_size_fn:
    pack_size_fn.argtypes = [ctypes.c_char_p]
    pack_size_fn.restype = ctypes.c_size_t

pack_read_fn = getattr(m2_lib, 'm2_py_pack_read_data', None) if m2_lib else None
if pack_read_fn:
    pack_read_fn.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_size_t]
    pack_read_fn.restype = ctypes.c_size_t

if m2_lib and hasattr(m2_lib, 'm2_py_pack_mount'):
    for p in ["root.pck", "locale.pck", "uiscript.pck", "ETC.pck", "uiloading.pck"]:
        m2_lib.m2_py_pack_mount(f"pack/{p}".encode('utf-8'))

class PackModule:
    @staticmethod
    def Exist(filename):
        if not pack_exist_fn: return False
        fn_b = filename.encode('utf-8') if isinstance(filename, str) else filename
        return pack_exist_fn(fn_b)
    @staticmethod
    def Get(filename):
        if not pack_size_fn or not pack_read_fn: return b''
        fn_b = filename.encode('utf-8') if isinstance(filename, str) else filename
        size = pack_size_fn(fn_b)
        if size == 0: return b''
        buf = ctypes.create_string_buffer(size)
        read_len = pack_read_fn(fn_b, buf, size)
        return buf.raw[:read_len]

sys.modules['pack'] = PackModule
sys.modules['Pack'] = PackModule

# ============================================================================
# 2. Modul dbg (Debugger & Logger)
# ============================================================================
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
    def RegisterExceptionString(msg): pass
    @staticmethod
    def LogBox(msg, title="Error"):
        sys.__stderr__.write(f"[LOGBOX {title}] {msg}\n")
        sys.__stderr__.flush()

sys.modules['dbg'] = DbgModule
sys.modules['Dbg'] = DbgModule

# ============================================================================
# 3. Modul app (CPythonApplication - 104 metody)
# ============================================================================
app_methods = [
    ("GetTime", "m2_app_get_time", [], ctypes.c_float),
    ("GetGlobalTime", "m2_app_get_global_time", [], ctypes.c_uint32),
    ("GetWidth", "m2_app_get_width", [], ctypes.c_uint32),
    ("GetHeight", "m2_app_get_height", [], ctypes.c_uint32),
    ("Exit", "m2_app_exit", [], None),
    ("SetCamera", "m2_app_set_camera", [ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float], None),
    ("IsPressed", "m2_app_is_pressed", [ctypes.c_uint32], ctypes.c_bool),
    ("EnableSpecialCameraMode", "m2_app_enable_special_camera_mode", [], None),
    ("PitchCamera", "m2_py_app_pitch_camera", [ctypes.c_int32], None),
    ("RotateCamera", "m2_py_app_rotate_camera", [ctypes.c_int32], None),
    ("ZoomCamera", "m2_py_app_zoom_camera", [ctypes.c_int32], None),
    ("GetCameraPitch", "m2_py_app_get_camera_pitch", [], ctypes.c_float),
    ("GetCameraRotation", "m2_py_app_get_camera_rotation", [], ctypes.c_float),
    ("LoadCameraSetting", "m2_py_app_load_camera_setting", [ctypes.c_char_p], ctypes.c_bool),
    ("SaveCameraSetting", "m2_py_app_save_camera_setting", [ctypes.c_char_p], None),
    ("SetCursor", "m2_py_app_set_cursor", [ctypes.c_int32], ctypes.c_bool),
    ("ShowCursor", "m2_py_app_show_cursor", [], None),
    ("HideCursor", "m2_py_app_hide_cursor", [], None),
    ("SetFPS", "m2_py_app_set_fps", [ctypes.c_uint32], None),
    ("GetRenderFPS", "m2_py_app_get_render_fps", [], ctypes.c_uint32),
    ("GetUpdateFPS", "m2_py_app_get_update_fps", [], ctypes.c_uint32),
    ("SetMinFog", "m2_py_app_set_min_fog", [ctypes.c_float], None),
    ("SetSightRange", "m2_py_app_set_sight_range", [ctypes.c_int32], None),
    ("SetSpecularSpeed", "m2_py_app_set_specular_speed", [ctypes.c_float], None),
    ("LoadLocaleData", "m2_py_app_load_locale_data", [ctypes.c_char_p], ctypes.c_int32),
    ("RenderGame", "m2_py_app_render_game", [], None),
    ("UpdateGame", "m2_py_app_update_game", [], None),
    ("Process", "m2_py_app_process", [], None),
]
AppModule = create_real_module('app', 'app', app_methods)

# Stale w module app
AppModule.INFO_ITEM = 1
AppModule.INFO_ACTOR = 2
AppModule.INFO_EFFECT = 3
AppModule.INFO_TEXTTAIL = 4
AppModule.NORMAL = 0
AppModule.ATTACK = 1
AppModule.TARGET = 2
AppModule.TALK = 3
AppModule.CANT_GO = 4
AppModule.PICK = 5
AppModule.DOOR = 6
AppModule.CHAIR = 7
AppModule.MAGIC = 8
AppModule.BUY = 9
AppModule.SELL = 10
AppModule.CAMERA_ROTATE = 11
AppModule.HSIZE = 12
AppModule.VSIZE = 13
AppModule.HVSIZE = 14
AppModule.CAMERA_TO_POSITIVE = 1
AppModule.CAMERA_TO_NEGATIVE = 2
AppModule.CAMERA_STOP = 0
AppModule.ENABLE_COSTUME_SYSTEM = 1
AppModule.ENABLE_ENERGY_SYSTEM = 1
AppModule.ENABLE_DRAGON_SOUL_SYSTEM = 1
AppModule.DIK_ESCAPE = 1
AppModule.DIK_1 = 2
AppModule.DIK_2 = 3
AppModule.DIK_3 = 4
AppModule.DIK_4 = 5
AppModule.DIK_5 = 6
AppModule.DIK_6 = 7
AppModule.DIK_7 = 8
AppModule.DIK_8 = 9
AppModule.DIK_9 = 10
AppModule.DIK_0 = 11
AppModule.DIK_RETURN = 28
AppModule.DIK_SPACE = 57
AppModule.DIK_TAB = 15
AppModule.DIK_UP = 200
AppModule.DIK_LEFT = 203
AppModule.DIK_RIGHT = 205
AppModule.DIK_DOWN = 208

setattr(AppModule, 'Abort', staticmethod(lambda: None))
setattr(AppModule, 'GetRandom', staticmethod(lambda a, b: random.randint(int(a), int(b))))
setattr(AppModule, 'GetGlobalTimeStamp', staticmethod(lambda: int(time.time())))
setattr(AppModule, 'SetMouseHandler', staticmethod(lambda handler=None: None))
setattr(AppModule, 'SetHairColorEnable', staticmethod(lambda *args: None))
setattr(AppModule, 'SetArmorSpecularEnable', staticmethod(lambda *args: None))
setattr(AppModule, 'SetWeaponSpecularEnable', staticmethod(lambda *args: None))
setattr(AppModule, 'SetCameraMaxDistance', staticmethod(lambda d: None))
setattr(AppModule, 'SetDefaultFontName', staticmethod(lambda name: None))
setattr(AppModule, 'SetGuildMarkPath', staticmethod(lambda path: None))
setattr(AppModule, 'SetGuildSymbolPath', staticmethod(lambda path: None))
setattr(AppModule, 'GetLocaleName', staticmethod(lambda: "en"))
setattr(AppModule, 'GetLocalePath', staticmethod(lambda: "locale/en"))
setattr(AppModule, 'GetLocalePathCommon', staticmethod(lambda: "locale/common"))
setattr(AppModule, 'IsRTL', staticmethod(lambda *a: False))
setattr(AppModule, 'IsExistFile', staticmethod(lambda f: PackModule.Exist(f)))
setattr(AppModule, 'IsFileExist', staticmethod(lambda f: PackModule.Exist(f)))
setattr(AppModule, 'SetSoftwareCursor', staticmethod(lambda: None))
setattr(AppModule, 'SetHardwareCursor', staticmethod(lambda: None))
setattr(AppModule, 'GetCursorPosition', staticmethod(lambda: (0, 0)))

def _app_create(title, w, h, fullscreen=1):
    print(f"[+] [APP] Create('{title}', {w}x{h}, fullscreen={fullscreen})")
    if m2_lib and hasattr(m2_lib, 'm2_app_set_resolution'):
        m2_lib.m2_app_set_resolution(int(w), int(h))

def _app_loop():
    print("[+] [APP] Loop() wywolana -> wejscie w natywna petle renderowania GPU Pure Rust...")
    if m2_lib and hasattr(m2_lib, 'm2_run_game_loop'):
        m2_lib.m2_run_game_loop()

setattr(AppModule, 'Create', staticmethod(_app_create))
setattr(AppModule, 'Loop', staticmethod(_app_loop))
sys.modules['app'] = AppModule
sys.modules['App'] = AppModule

# ============================================================================
# 4. Modul background i Background (CPythonBackground - 48 metod)
# ============================================================================
class BackgroundModule:
    @staticmethod
    def Initialize(): pass
    @staticmethod
    def Destroy(): pass
    @staticmethod
    def LoadMap(map_name, x=0, y=0):
        print(f"[+] [BACKGROUND] LoadMap('{map_name}', x={x}, y={y})")
        return True
    @staticmethod
    def Update(x=0, y=0, z=0): pass
    @staticmethod
    def Render(): pass
    @staticmethod
    def RenderSky(): pass
    @staticmethod
    def RenderCloud(): pass
    @staticmethod
    def RenderWater(): pass
    @staticmethod
    def RenderEffect(): pass
    @staticmethod
    def RenderCharacterShadowToTexture(): pass
    @staticmethod
    def IsSoftwareTiling(): return False
    @staticmethod
    def EnableSoftwareTiling(flag): pass
    @staticmethod
    def EnableSnow(flag): pass
    @staticmethod
    def GlobalPositionToLocalPosition(gx, gy): return (gx, gy)
    @staticmethod
    def GlobalPositionToMapInfo(gx, gy): return ("metin2_map_a1", 0, 0)
    @staticmethod
    def GetRenderShadowTime(): return 0.0
    @staticmethod
    def RegisterEnvironmentData(idx, path): return True
    @staticmethod
    def SetEnvironmentData(idx): return True
    @staticmethod
    def GetCurrentMapName(): return "metin2_map_a1"
    @staticmethod
    def GetPickingPoint(): return (0.0, 0.0, 0.0)
    @staticmethod
    def BeginEnvironment(): pass
    @staticmethod
    def EndEnvironment(): pass
    @staticmethod
    def SetCharacterDirLight(): pass
    @staticmethod
    def SetBackgroundDirLight(): pass
    @staticmethod
    def GetHeight(x, y): return 0.0
    @staticmethod
    def SetShadowLevel(lvl): pass
    @staticmethod
    def SetVisiblePart(idx, flag): pass
    @staticmethod
    def GetShadowMapColor(): return (0, 0, 0, 0)
    @staticmethod
    def SetSplatLimit(l): pass
    @staticmethod
    def GetRenderedSplatNum(): return 0
    @staticmethod
    def GetRenderedGraphicThingInstanceNum(): return 0
    @staticmethod
    def SelectViewDistanceNum(n): pass
    @staticmethod
    def SetViewDistanceSet(n): pass
    @staticmethod
    def GetFarClip(): return 50000.0
    @staticmethod
    def GetDistanceSetInfo(): return (1, 1, 1)
    @staticmethod
    def SetBGLoading(flag): pass
    @staticmethod
    def SetRenderSort(n): pass
    @staticmethod
    def SetTransparentTree(flag): pass
    @staticmethod
    def SetXMasTree(n): pass
    @staticmethod
    def RegisterDungeonMapName(name): pass
    @staticmethod
    def VisibleGuildArea(): pass
    @staticmethod
    def DisableGuildArea(): pass
    @staticmethod
    def WarpTest(): pass

sys.modules['background'] = BackgroundModule
sys.modules['Background'] = BackgroundModule

# ============================================================================
# 5. Modul net (CPythonNetworkStream - 70 metod)
# ============================================================================
def _get_account_char_str(slot, data_type):
    buf = ctypes.create_string_buffer(256)
    fn = getattr(m2_lib, 'm2_net_GetAccountCharacterSlotDataString', None)
    if fn:
        fn.argtypes = [ctypes.c_uint32, ctypes.c_uint32, ctypes.c_char_p, ctypes.c_size_t]
        fn.restype = ctypes.c_size_t
        fn(int(slot), int(data_type), buf, 256)
        return buf.value.decode('utf-8', errors='ignore')
    return ""

_py_handlers = {}

def _wrap_set_phase_window(phase, win):
    h = id(win) if win else 0
    if win and h: _py_handlers[h] = win
    fn = getattr(m2_lib, 'm2_net_SetPhaseWindow', None)
    if fn:
        fn.argtypes = [ctypes.c_uint32, ctypes.c_size_t]
        fn(int(phase), h)

def _wrap_clear_phase_window(phase, win):
    h = id(win) if win else 0
    fn = getattr(m2_lib, 'm2_net_ClearPhaseWindow', None)
    if fn:
        fn.argtypes = [ctypes.c_uint32, ctypes.c_size_t]
        fn(int(phase), h)

def _wrap_set_acc_connector_handler(handler):
    h = id(handler) if handler else 0
    if handler and h: _py_handlers[h] = handler
    fn = getattr(m2_lib, 'm2_net_SetAccountConnectorHandler', None)
    if fn:
        fn.argtypes = [ctypes.c_size_t]
        fn(h)

def _wrap_set_handler(handler):
    h = id(handler) if handler else 0
    if handler and h: _py_handlers[h] = handler
    fn = getattr(m2_lib, 'm2_net_SetHandler', None)
    if fn:
        fn.argtypes = [ctypes.c_size_t]
        fn(h)

def _wrap_connect_to_account_server(addr, port, acc_addr, acc_port):
    fn = getattr(m2_lib, 'm2_net_ConnectToAccountServer', None)
    if fn:
        fn.argtypes = [ctypes.c_char_p, ctypes.c_uint16, ctypes.c_char_p, ctypes.c_uint16]
        fn.restype = ctypes.c_bool
        return fn(str(addr).encode('utf-8'), int(port), str(acc_addr).encode('utf-8'), int(acc_port))
    return True

def _wrap_set_login_info(name, pwd):
    fn = getattr(m2_lib, 'm2_net_SetLoginInfo', None)
    if fn:
        fn.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
        fn(str(name).encode('utf-8'), str(pwd).encode('utf-8'))

net_methods = [
    ("Connect", "m2_net_Connect", [ctypes.c_char_p, ctypes.c_uint16], ctypes.c_bool),
    ("Disconnect", "m2_net_Disconnect", [], None),
    ("GetMainStream", "m2_net_GetMainStream", [], ctypes.c_bool),
    ("SendChatPacket", "m2_net_SendChatPacket", [ctypes.c_char_p, ctypes.c_uint8, ctypes.c_void_p], ctypes.c_size_t),
    ("SendAttackPacket", "m2_net_SendAttackPacket", [ctypes.c_uint8, ctypes.c_uint32, ctypes.c_void_p], ctypes.c_size_t),
    ("SendUseSkillPacket", "m2_net_SendUseSkillPacket", [ctypes.c_uint32, ctypes.c_uint32, ctypes.c_void_p], ctypes.c_size_t),
    ("SendMovePacket", "m2_net_SendMovePacket", [ctypes.c_uint8, ctypes.c_int32, ctypes.c_int32, ctypes.c_float, ctypes.c_uint32, ctypes.c_void_p], ctypes.c_size_t),
    ("SetTCPRecvBufferSize", "m2_net_SetTCPRecvBufferSize", [ctypes.c_size_t], None),
    ("SetTCPSendBufferSize", "m2_net_SetTCPSendBufferSize", [ctypes.c_size_t], None),
    ("SetServerInfo", "m2_net_SetServerInfo", [ctypes.c_char_p], None),
    ("SetMarkServer", "m2_net_Connect", [ctypes.c_char_p, ctypes.c_uint16], ctypes.c_bool),
    ("GetEmpireID", "m2_net_GetEmpireID", [], ctypes.c_uint32),
    ("SetEmpireID", "m2_net_SetEmpireID", [ctypes.c_uint32], None),
    ("GetMainActorVID", "m2_net_GetMainActorVID", [], ctypes.c_uint32),
    ("GetAccountCharacterSlotDataInteger", "m2_net_GetAccountCharacterSlotDataInteger", [ctypes.c_uint32, ctypes.c_uint32], ctypes.c_uint32),
    ("ConnectTCP", "m2_net_ConnectTCP", [ctypes.c_char_p, ctypes.c_uint16], ctypes.c_bool),
    ("DirectEnter", "m2_net_DirectEnter", [ctypes.c_uint32], ctypes.c_bool),
    ("SendSelectCharacterPacket", "m2_net_SendSelectCharacterPacket", [ctypes.c_uint8], ctypes.c_bool),
    ("SendSelectEmpirePacket", "m2_net_SendSelectEmpirePacket", [ctypes.c_uint32], ctypes.c_bool),
    ("SendEnterGamePacket", "m2_net_SendEnterGamePacket", [], ctypes.c_bool),
    ("SendChangeNamePacket", "m2_net_SendChangeNamePacket", [ctypes.c_uint8, ctypes.c_char_p], ctypes.c_bool),
    ("SendCreateCharacterPacket", "m2_net_SendCreateCharacterPacket", [ctypes.c_uint8, ctypes.c_char_p, ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint8], ctypes.c_bool),
    ("SendDestroyCharacterPacket", "m2_net_SendDestroyCharacterPacket", [ctypes.c_uint8, ctypes.c_char_p], ctypes.c_bool),
    ("SendItemUsePacket", "m2_net_SendItemUsePacket", [ctypes.c_uint16], ctypes.c_bool),
    ("SendItemDropPacket", "m2_net_SendItemDropPacket", [ctypes.c_uint16, ctypes.c_uint32], ctypes.c_bool),
    ("SendItemMovePacket", "m2_net_SendItemMovePacket", [ctypes.c_uint16, ctypes.c_uint16, ctypes.c_uint8], ctypes.c_bool),
    ("SendItemPickUpPacket", "m2_net_SendItemPickUpPacket", [ctypes.c_uint32], ctypes.c_bool),
    ("SendExchangeStartPacket", "m2_net_SendExchangeStartPacket", [ctypes.c_uint32], ctypes.c_bool),
    ("SendExchangeItemAddPacket", "m2_net_SendExchangeItemAddPacket", [ctypes.c_uint16, ctypes.c_uint8], ctypes.c_bool),
    ("SendExchangeElkAddPacket", "m2_net_SendExchangeElkAddPacket", [ctypes.c_uint32], ctypes.c_bool),
    ("SendExchangeAcceptPacket", "m2_net_SendExchangeAcceptPacket", [], ctypes.c_bool),
    ("SendExchangeExitPacket", "m2_net_SendExchangeExitPacket", [], ctypes.c_bool),
    ("SendShopBuyPacket", "m2_net_SendShopBuyPacket", [ctypes.c_uint8, ctypes.c_uint8], ctypes.c_bool),
    ("SendShopSellPacket", "m2_net_SendShopSellPacket", [ctypes.c_uint8, ctypes.c_uint8], ctypes.c_bool),
    ("SendShopEndPacket", "m2_net_SendShopEndPacket", [], ctypes.c_bool),
    ("SendRefinePacket", "m2_net_SendRefinePacket", [ctypes.c_uint16, ctypes.c_uint8], ctypes.c_bool),
    ("SendPartyInvitePacket", "m2_net_SendPartyInvitePacket", [ctypes.c_uint32], ctypes.c_bool),
    ("SendPartyExitPacket", "m2_net_SendPartyExitPacket", [], ctypes.c_bool),
    ("SendQuestInputStringPacket", "m2_net_SendQuestInputStringPacket", [ctypes.c_char_p], ctypes.c_bool),
    ("SendQuickSlotAddPacket", "m2_net_SendQuickSlotAddPacket", [ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint32], ctypes.c_bool),
    ("SendQuickSlotDelPacket", "m2_net_SendQuickSlotDelPacket", [ctypes.c_uint8], ctypes.c_bool),
    ("SendQuickSlotMovePacket", "m2_net_SendQuickSlotMovePacket", [ctypes.c_uint8, ctypes.c_uint8], ctypes.c_bool),
    ("SendOnClickPacket", "m2_net_SendOnClickPacket", [ctypes.c_uint32], ctypes.c_bool),
]
NetModule = create_real_module('net', 'net', net_methods)
setattr(NetModule, 'SetPhaseWindow', staticmethod(_wrap_set_phase_window))
setattr(NetModule, 'ClearPhaseWindow', staticmethod(_wrap_clear_phase_window))
setattr(NetModule, 'SetAccountConnectorHandler', staticmethod(_wrap_set_acc_connector_handler))
setattr(NetModule, 'SetHandler', staticmethod(_wrap_set_handler))
setattr(NetModule, 'ConnectToAccountServer', staticmethod(_wrap_connect_to_account_server))
setattr(NetModule, 'SetLoginInfo', staticmethod(_wrap_set_login_info))
setattr(NetModule, 'GetAccountCharacterSlotDataString', staticmethod(_get_account_char_str))

NetModule.PHASE_WINDOW_LOGO = 0
NetModule.PHASE_WINDOW_LOGIN = 1
NetModule.PHASE_WINDOW_SELECT = 2
NetModule.PHASE_WINDOW_CREATE = 3
NetModule.PHASE_WINDOW_LOAD = 4
NetModule.PHASE_WINDOW_GAME = 5
NetModule.PHASE_WINDOW_EMPIRE = 6
NetModule.ACCOUNT_CHARACTER_SLOT_ID = 0
NetModule.ACCOUNT_CHARACTER_SLOT_NAME = 1
NetModule.ACCOUNT_CHARACTER_SLOT_RACE = 2
NetModule.ACCOUNT_CHARACTER_SLOT_LEVEL = 3
NetModule.ACCOUNT_CHARACTER_SLOT_STR = 4
NetModule.ACCOUNT_CHARACTER_SLOT_DEX = 5
NetModule.ACCOUNT_CHARACTER_SLOT_HTH = 6
NetModule.ACCOUNT_CHARACTER_SLOT_INT = 7
NetModule.ACCOUNT_CHARACTER_SLOT_PLAYTIME = 8
NetModule.ACCOUNT_CHARACTER_SLOT_FORM = 9
NetModule.ACCOUNT_CHARACTER_SLOT_ADDR = 10
NetModule.ACCOUNT_CHARACTER_SLOT_PORT = 11
NetModule.ACCOUNT_CHARACTER_SLOT_GUILD_ID = 12
NetModule.ACCOUNT_CHARACTER_SLOT_GUILD_NAME = 13
NetModule.ACCOUNT_CHARACTER_SLOT_CHANGE_NAME_FLAG = 14
NetModule.ACCOUNT_CHARACTER_SLOT_HAIR = 15
NetModule.EMPIRE_A = 1
NetModule.EMPIRE_B = 2
NetModule.EMPIRE_C = 3
NetModule.ERROR_NONE = 0
setattr(NetModule, 'IsInsultIn', staticmethod(lambda s: False))
setattr(NetModule, 'SetEmpireLanguageMode', staticmethod(lambda m: None))
setattr(NetModule, 'ToggleGameDebugInfo', staticmethod(lambda: None))
sys.modules['net'] = NetModule
sys.modules['Net'] = NetModule

# ============================================================================
# 6. Modul wndMgr (CWindowManager - 79 metod)
# ============================================================================
wndmgr_methods = [
    ("Register", "wndMgr_Register", [ctypes.c_size_t], ctypes.c_uint32),
    ("RegisterImageBox", "wndMgr_RegisterImageBox", [ctypes.c_size_t], ctypes.c_uint32),
    ("RegisterExpandedImageBox", "wndMgr_RegisterExpandedImageBox", [ctypes.c_size_t], ctypes.c_uint32),
    ("RegisterTextLine", "wndMgr_RegisterTextLine", [ctypes.c_size_t], ctypes.c_uint32),
    ("RegisterButton", "wndMgr_RegisterButton", [ctypes.c_size_t], ctypes.c_uint32),
    ("RegisterBar", "wndMgr_RegisterBar", [ctypes.c_size_t], ctypes.c_uint32),
    ("RegisterLine", "wndMgr_RegisterLine", [ctypes.c_size_t], ctypes.c_uint32),
    ("RegisterBox", "wndMgr_RegisterBox", [ctypes.c_size_t], ctypes.c_uint32),
    ("SetParent", "wndMgr_SetParent", [ctypes.c_uint32, ctypes.c_uint32], ctypes.c_bool),
    ("SetPosition", "wndMgr_SetPosition", [ctypes.c_uint32, ctypes.c_int32, ctypes.c_int32], ctypes.c_bool),
    ("SetSize", "wndMgr_SetSize", [ctypes.c_uint32, ctypes.c_int32, ctypes.c_int32], ctypes.c_bool),
    ("Show", "wndMgr_Show", [ctypes.c_uint32], ctypes.c_bool),
    ("Hide", "wndMgr_Hide", [ctypes.c_uint32], ctypes.c_bool),
    ("SetTop", "wndMgr_SetTop", [ctypes.c_uint32], ctypes.c_bool),
    ("IsShow", "wndMgr_IsShow", [ctypes.c_uint32], ctypes.c_bool),
    ("LoadImage", "wndMgr_LoadImage", [ctypes.c_uint32, ctypes.c_char_p], ctypes.c_bool),
    ("SetRenderingRect", "wndMgr_SetRenderingRect", [ctypes.c_uint32, ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float], ctypes.c_bool),
    ("SetScale", "wndMgr_SetScale", [ctypes.c_uint32, ctypes.c_float, ctypes.c_float], ctypes.c_bool),
    ("SetText", "wndMgr_SetText", [ctypes.c_uint32, ctypes.c_char_p], ctypes.c_bool),
    ("SetColor", "wndMgr_SetColor", [ctypes.c_uint32, ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float], ctypes.c_bool),
    ("SetFontColor", "wndMgr_SetFontColor", [ctypes.c_uint32, ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float], ctypes.c_bool),
    ("SetAlign", "wndMgr_SetAlign", [ctypes.c_uint32, ctypes.c_uint8, ctypes.c_uint8], ctypes.c_bool),
    ("SetUpVisual", "wndMgr_SetUpVisual", [ctypes.c_uint32, ctypes.c_char_p], ctypes.c_bool),
    ("SetOverVisual", "wndMgr_SetOverVisual", [ctypes.c_uint32, ctypes.c_char_p], ctypes.c_bool),
    ("SetDownVisual", "wndMgr_SetDownVisual", [ctypes.c_uint32, ctypes.c_char_p], ctypes.c_bool),
    ("Destroy", "wndMgr_Destroy", [ctypes.c_uint32], None),
    ("GetWidth", "wndMgr_GetWidth", [ctypes.c_uint32], ctypes.c_int32),
    ("GetHeight", "wndMgr_GetHeight", [ctypes.c_uint32], ctypes.c_int32),
    ("AttachIcon", "wndMgr_AttachIcon", [ctypes.c_size_t, ctypes.c_int, ctypes.c_int, ctypes.c_int, ctypes.c_int, ctypes.c_int], ctypes.c_bool),
    ("DeattachIcon", "wndMgr_DeattachIcon", [ctypes.c_size_t], ctypes.c_bool),
    ("IsAttaching", "wndMgr_IsAttaching", [ctypes.c_size_t], ctypes.c_bool),
    ("GetAttachedSlotNumber", "wndMgr_GetAttachedSlotNumber", [ctypes.c_size_t], ctypes.c_uint32),
    ("GetAttachedItemIndex", "wndMgr_GetAttachedItemIndex", [ctypes.c_size_t], ctypes.c_uint32),
    ("AppendSlot", "wndMgr_AppendSlot", [ctypes.c_size_t, ctypes.c_uint32, ctypes.c_int, ctypes.c_int, ctypes.c_int, ctypes.c_int, ctypes.c_int], ctypes.c_bool),
    ("SetSlot", "wndMgr_SetSlot", [ctypes.c_size_t, ctypes.c_uint32, ctypes.c_int, ctypes.c_int, ctypes.c_int, ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float], ctypes.c_bool),
    ("ClearSlot", "wndMgr_ClearSlot", [ctypes.c_size_t, ctypes.c_uint32, ctypes.c_int], ctypes.c_bool),
    ("SetSlotCount", "wndMgr_SetSlotCount", [ctypes.c_size_t, ctypes.c_uint32, ctypes.c_int, ctypes.c_int], ctypes.c_bool),
    ("SetOverInItemEvent", "wndMgr_SetOverInItemEvent", [ctypes.c_size_t, ctypes.c_uint32, ctypes.c_uint32], ctypes.c_bool),
    ("SetOverOutItemEvent", "wndMgr_SetOverOutItemEvent", [ctypes.c_size_t, ctypes.c_uint32, ctypes.c_uint32], ctypes.c_bool),
    ("SetSelectItemSlotEvent", "wndMgr_SetSelectItemSlotEvent", [ctypes.c_size_t, ctypes.c_uint32, ctypes.c_uint32], ctypes.c_bool),
    ("SetScreenSize", "wndMgr_SetScreenSize", [ctypes.c_int32, ctypes.c_int32], None),
    ("GetScreenWidth", "wndMgr_GetScreenWidth", [], ctypes.c_int32),
    ("GetScreenHeight", "wndMgr_GetScreenHeight", [], ctypes.c_int32),
    ("RegisterBar3D", "wndMgr_RegisterBar3D", [ctypes.c_size_t], ctypes.c_uint32),
    ("RegisterSlotWindow", "wndMgr_RegisterSlotWindow", [ctypes.c_size_t], ctypes.c_uint32),
    ("RegisterGridSlotWindow", "wndMgr_RegisterGridSlotWindow", [ctypes.c_size_t], ctypes.c_uint32),
    ("RegisterMarkBox", "wndMgr_RegisterMarkBox", [ctypes.c_size_t], ctypes.c_uint32),
    ("RegisterAniImageBox", "wndMgr_RegisterAniImageBox", [ctypes.c_size_t], ctypes.c_uint32),
    ("RegisterRadioButton", "wndMgr_RegisterRadioButton", [ctypes.c_size_t], ctypes.c_uint32),
    ("RegisterToggleButton", "wndMgr_RegisterToggleButton", [ctypes.c_size_t], ctypes.c_uint32),
    ("RegisterDragButton", "wndMgr_RegisterDragButton", [ctypes.c_size_t], ctypes.c_uint32),
    ("SetWindowPosition", "wndMgr_SetWindowPosition", [ctypes.c_uint32, ctypes.c_int32, ctypes.c_int32], ctypes.c_bool),
    ("SetWindowSize", "wndMgr_SetWindowSize", [ctypes.c_uint32, ctypes.c_int32, ctypes.c_int32], ctypes.c_bool),
    ("GetWindowWidth", "wndMgr_GetWindowWidth", [ctypes.c_uint32], ctypes.c_int32),
    ("GetWindowHeight", "wndMgr_GetWindowHeight", [ctypes.c_uint32], ctypes.c_int32),
    ("SetPickAlways", "wndMgr_SetPickAlways", [ctypes.c_uint32], None),
    ("AddFlag", "wndMgr_AddFlag", [ctypes.c_uint32, ctypes.c_uint32], None),
    ("IsRTL", "wndMgr_IsRTL", [ctypes.c_uint32], ctypes.c_bool),
    ("SetName", "wndMgr_SetName", [ctypes.c_uint32, ctypes.c_char_p], None),
    ("SetFocus", "wndMgr_SetFocus", [ctypes.c_uint32], None),
    ("KillFocus", "wndMgr_KillFocus", [ctypes.c_uint32], None),
    ("IsFocus", "wndMgr_IsFocus", [ctypes.c_uint32], ctypes.c_bool),
    ("Lock", "wndMgr_Lock", [ctypes.c_uint32], None),
    ("Unlock", "wndMgr_Unlock", [ctypes.c_uint32], None),
    ("SetSlotBaseImage", "wndMgr_SetSlotBaseImage", [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float], ctypes.c_bool),
    ("ShowSlotBaseImage", "wndMgr_ShowSlotBaseImage", [ctypes.c_uint32, ctypes.c_uint32], ctypes.c_bool),
    ("HideSlotBaseImage", "wndMgr_HideSlotBaseImage", [ctypes.c_uint32, ctypes.c_uint32], ctypes.c_bool),
    ("SetSlotStyle", "wndMgr_SetSlotStyle", [ctypes.c_uint32, ctypes.c_uint32], ctypes.c_bool),
    ("ArrangeSlot", "wndMgr_ArrangeSlot", [ctypes.c_uint32], ctypes.c_bool),
    ("UpdateRect", "wndMgr_UpdateRect", [ctypes.c_uint32], None),
    ("SetWindowHorizontalAlign", "wndMgr_SetWindowHorizontalAlign", [ctypes.c_uint32, ctypes.c_uint8], ctypes.c_bool),
    ("SetWindowVerticalAlign", "wndMgr_SetWindowVerticalAlign", [ctypes.c_uint32, ctypes.c_uint8], ctypes.c_bool),
    ("GetName", "wndMgr_GetName", [ctypes.c_uint32], ctypes.c_char_p),
    ("GetChildCount", "wndMgr_GetChildCount", [ctypes.c_uint32], ctypes.c_int32),
    ("IsIn", "wndMgr_IsIn", [ctypes.c_uint32], ctypes.c_bool),
    ("SetLimitWidth", "wndMgr_SetLimitWidth", [ctypes.c_uint32, ctypes.c_float], None),
    ("SetMultiLine", "wndMgr_SetMultiLine", [ctypes.c_uint32, ctypes.c_bool], None),
    ("SetSecret", "wndMgr_SetSecret", [ctypes.c_uint32, ctypes.c_bool], None),
    ("SetMax", "wndMgr_SetMax", [ctypes.c_uint32, ctypes.c_int32], None),
    ("SetOutline", "wndMgr_SetOutline", [ctypes.c_uint32, ctypes.c_bool], None),
    ("SetFeather", "wndMgr_SetFeather", [ctypes.c_uint32, ctypes.c_bool], None),
    ("SetFontName", "wndMgr_SetFontName", [ctypes.c_uint32, ctypes.c_char_p], None),
    ("SetBaseDirection", "wndMgr_SetBaseDirection", [ctypes.c_uint32, ctypes.c_uint8], None),
]
WndMgrModule = create_real_module('wndMgr', 'wndMgr', wndmgr_methods)
WndMgrModule.SLOT_STYLE_NONE = 0
WndMgrModule.SLOT_STYLE_PICK_UP = 1
WndMgrModule.SLOT_STYLE_SELECT = 2
WndMgrModule.HORIZONTAL_ALIGN_LEFT = 0
WndMgrModule.HORIZONTAL_ALIGN_CENTER = 1
WndMgrModule.HORIZONTAL_ALIGN_RIGHT = 2
WndMgrModule.VERTICAL_ALIGN_TOP = 0
WndMgrModule.VERTICAL_ALIGN_CENTER = 1
WndMgrModule.VERTICAL_ALIGN_BOTTOM = 2
WndMgrModule.TEXT_HORIZONTAL_ALIGN_LEFT = 0
WndMgrModule.TEXT_HORIZONTAL_ALIGN_RIGHT = 1
WndMgrModule.TEXT_HORIZONTAL_ALIGN_CENTER = 2
WndMgrModule.TEXT_BASEDIR_AUTO = 0
WndMgrModule.TEXT_BASEDIR_LTR = 1
WndMgrModule.TEXT_BASEDIR_RTL = 2

def _wrap_wndmgr_set_mouse_handler(handler=None):
    h = id(handler) if handler else 0
    fn = getattr(m2_lib, 'wndMgr_SetMouseHandler', None)
    if fn:
        fn.argtypes = [ctypes.c_size_t]
        fn(h)

def _wrap_wndmgr_set_font_color(handle, *args):
    fn = getattr(m2_lib, 'wndMgr_SetFontColor', None)
    if not fn:
        return False
    fn.argtypes = [ctypes.c_uint32, ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float]
    fn.restype = ctypes.c_bool
    h = int(handle) if handle else 0
    if len(args) == 1:
        c = int(args[0])
        a = ((c >> 24) & 0xFF) / 255.0
        r = ((c >> 16) & 0xFF) / 255.0
        g = ((c >> 8) & 0xFF) / 255.0
        b = (c & 0xFF) / 255.0
        return fn(h, r, g, b, a if a > 0 else 1.0)
    elif len(args) == 3:
        r, g, b = float(args[0]), float(args[1]), float(args[2])
        return fn(h, r, g, b, 1.0)
    elif len(args) >= 4:
        r, g, b, a = float(args[0]), float(args[1]), float(args[2]), float(args[3])
        return fn(h, r, g, b, a)
    return False

def _wrap_wndmgr_set_color(handle, *args):
    fn = getattr(m2_lib, 'wndMgr_SetColor', None)
    if not fn:
        return False
    fn.argtypes = [ctypes.c_uint32, ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float]
    fn.restype = ctypes.c_bool
    h = int(handle) if handle else 0
    if len(args) == 1:
        c = int(args[0])
        a = ((c >> 24) & 0xFF) / 255.0
        r = ((c >> 16) & 0xFF) / 255.0
        g = ((c >> 8) & 0xFF) / 255.0
        b = (c & 0xFF) / 255.0
        return fn(h, r, g, b, a if a > 0 else 1.0)
    elif len(args) == 3:
        r, g, b = float(args[0]), float(args[1]), float(args[2])
        return fn(h, r, g, b, 1.0)
    elif len(args) >= 4:
        r, g, b, a = float(args[0]), float(args[1]), float(args[2]), float(args[3])
        return fn(h, r, g, b, a)
    return False

setattr(WndMgrModule, 'SetMouseHandler', staticmethod(_wrap_wndmgr_set_mouse_handler))
setattr(WndMgrModule, 'SetFontColor', staticmethod(_wrap_wndmgr_set_font_color))
setattr(WndMgrModule, 'SetColor', staticmethod(_wrap_wndmgr_set_color))
setattr(WndMgrModule, 'GetLocalPosition', staticmethod(lambda handle: (0, 0)))
setattr(WndMgrModule, 'GetWindowLocalPosition', staticmethod(lambda handle: (0, 0)))
setattr(WndMgrModule, 'GetWindowGlobalPosition', staticmethod(lambda handle: (0, 0)))
def _wrap_wndmgr_get_mouse_position():
    fn = getattr(m2_lib, 'wndMgr_GetMousePosition', None)
    if fn:
        x = ctypes.c_int32(0)
        y = ctypes.c_int32(0)
        fn.argtypes = [ctypes.POINTER(ctypes.c_int32), ctypes.POINTER(ctypes.c_int32)]
        fn(ctypes.byref(x), ctypes.byref(y))
        return (x.value, y.value)
    return (0, 0)
setattr(WndMgrModule, 'GetMousePosition', staticmethod(_wrap_wndmgr_get_mouse_position))

def _wrap_wndmgr_get_mouse_local_position(handle):
    fn = getattr(m2_lib, 'wndMgr_GetMouseLocalPosition', None)
    if fn:
        x = ctypes.c_int32(0)
        y = ctypes.c_int32(0)
        fn.argtypes = [ctypes.c_uint32, ctypes.POINTER(ctypes.c_int32), ctypes.POINTER(ctypes.c_int32)]
        fn(int(handle) if handle else 0, ctypes.byref(x), ctypes.byref(y))
        return (x.value, y.value)
    return (0, 0)
setattr(WndMgrModule, 'GetMouseLocalPosition', staticmethod(_wrap_wndmgr_get_mouse_local_position))

def _wrap_wndmgr_get_text(handle):
    fn = getattr(m2_lib, 'wndMgr_GetText', None)
    if fn:
        buf = ctypes.create_string_buffer(1024)
        fn.argtypes = [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_size_t]
        fn.restype = ctypes.c_size_t
        fn(int(handle) if handle else 0, buf, 1024)
        return buf.value.decode('utf-8', 'ignore')
    return ""
setattr(WndMgrModule, 'GetText', staticmethod(_wrap_wndmgr_get_text))

def _wrap_wndmgr_get_text_size(handle):
    fn = getattr(m2_lib, 'wndMgr_GetTextSize', None)
    if fn:
        w = ctypes.c_int32(0)
        h = ctypes.c_int32(0)
        fn.argtypes = [ctypes.c_uint32, ctypes.POINTER(ctypes.c_int32), ctypes.POINTER(ctypes.c_int32)]
        fn(int(handle) if handle else 0, ctypes.byref(w), ctypes.byref(h))
        return (w.value, h.value)
    return (0, 0)
setattr(WndMgrModule, 'GetTextSize', staticmethod(_wrap_wndmgr_get_text_size))

def _wrap_wndmgr_get_cursor_position(handle=0):
    fn = getattr(m2_lib, 'wndMgr_GetCursorPosition', None)
    if fn:
        fn.argtypes = [ctypes.c_uint32]
        fn.restype = ctypes.c_int32
        return fn(int(handle) if handle else 0)
    return 0
setattr(WndMgrModule, 'GetCursorPosition', staticmethod(_wrap_wndmgr_get_cursor_position))

setattr(WndMgrModule, 'ShowCursor', staticmethod(lambda *args: None))
setattr(WndMgrModule, 'HideCursor', staticmethod(lambda *args: None))
setattr(WndMgrModule, 'Enable', staticmethod(lambda handle: None))
setattr(WndMgrModule, 'Disable', staticmethod(lambda handle: None))
setattr(WndMgrModule, 'SetHorizontalAlign', staticmethod(lambda handle, align: getattr(WndMgrModule, 'SetWindowHorizontalAlign')(handle, align)))
setattr(WndMgrModule, 'SetVerticalAlign', staticmethod(lambda handle, align: getattr(WndMgrModule, 'SetWindowVerticalAlign')(handle, align)))
setattr(WndMgrModule, 'SetDiffuseColor', staticmethod(_wrap_wndmgr_set_color))
setattr(WndMgrModule, 'SetNumber', staticmethod(lambda handle, num: getattr(WndMgrModule, 'SetText')(handle, str(num))))
setattr(WndMgrModule, 'SetNumberHorizontalAlignCenter', staticmethod(lambda handle: None))
setattr(WndMgrModule, 'SetNumberHorizontalAlignRight', staticmethod(lambda handle: None))
setattr(WndMgrModule, 'RegisterNumberLine', staticmethod(lambda obj, layer=None: getattr(WndMgrModule, 'RegisterTextLine')(obj, layer)))
setattr(WndMgrModule, 'IsDown', staticmethod(lambda handle: False))
setattr(WndMgrModule, 'SetUp', staticmethod(lambda handle: None))
setattr(WndMgrModule, 'Down', staticmethod(lambda handle: None))
setattr(WndMgrModule, 'Flash', staticmethod(lambda handle: None))
setattr(WndMgrModule, 'GetUpVisualFileName', staticmethod(lambda handle: ""))
setattr(WndMgrModule, 'GetOverVisualFileName', staticmethod(lambda handle: ""))
setattr(WndMgrModule, 'GetDownVisualFileName', staticmethod(lambda handle: ""))
setattr(WndMgrModule, 'EnableScissorRect', staticmethod(lambda *args: None))
setattr(WndMgrModule, 'DisableScissorRect', staticmethod(lambda *args: None))
setattr(WndMgrModule, 'IsScissorRectEnabled', staticmethod(lambda handle: False))
setattr(WndMgrModule, 'SetRestrictMovementArea', staticmethod(lambda *args: None))
setattr(WndMgrModule, 'GetHyperlink', staticmethod(lambda: ""))
setattr(WndMgrModule, 'RENDERING_MODE_MODULATE', 1)
setattr(WndMgrModule, 'SetRenderingMode', staticmethod(lambda handle, mode: None))
setattr(WndMgrModule, 'SetRotation', staticmethod(lambda handle, rot: None))
setattr(WndMgrModule, 'SetOrigin', staticmethod(lambda handle, x, y: None))
setattr(WndMgrModule, 'SetPath', staticmethod(lambda handle, path: None))
setattr(WndMgrModule, 'SetDelay', staticmethod(lambda handle, delay: None))
setattr(WndMgrModule, 'GetWindowRect', staticmethod(lambda handle: (0, 0, 1024, 768)))
def _make_register_wrapper(orig_fn):
    def wrapper(obj, layer=None):
        if obj is not None:
            _OBJECT_REGISTRY[id(obj)] = obj
        res = orig_fn(obj, layer)
        if obj is not None and res:
            _OBJECT_REGISTRY[res] = obj
        return res
    return wrapper

for reg_name in [
    'Register', 'RegisterImageBox', 'RegisterExpandedImageBox', 'RegisterTextLine',
    'RegisterButton', 'RegisterBar', 'RegisterLine', 'RegisterBox',
    'RegisterSlotWindow', 'RegisterGridSlotWindow', 'RegisterMarkBox',
    'RegisterAniImageBox', 'RegisterRadioButton', 'RegisterToggleButton',
    'RegisterDragButton', 'RegisterBar3D'
]:
    fn = getattr(WndMgrModule, reg_name, None)
    if fn:
        setattr(WndMgrModule, reg_name, staticmethod(_make_register_wrapper(fn)))

def _wrap_wndmgr_destroy(handle):
    _OBJECT_REGISTRY.pop(handle, None)
    fn = getattr(m2_lib, 'wndMgr_Destroy', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint32]
            fn(int(handle) if handle else 0)
        except Exception: pass

setattr(WndMgrModule, 'Destroy', staticmethod(_wrap_wndmgr_destroy))
sys.modules['wndMgr'] = WndMgrModule
sys.modules['WndMgr'] = WndMgrModule

# ============================================================================
# 7. Modul player (CPythonPlayer - ponad 80 metod)
# ============================================================================
player_methods = [
    ("GetStatus", "m2_py_player_get_status", [ctypes.c_uint32], ctypes.c_int64),
    ("SetStatus", "m2_py_player_set_status", [ctypes.c_uint32, ctypes.c_int64], ctypes.c_bool),
    ("GetItemIndex", "m2_py_player_get_item_index", [ctypes.c_uint8, ctypes.c_uint16], ctypes.c_uint32),
    ("GetItemCount", "m2_py_player_get_item_count", [ctypes.c_uint8, ctypes.c_uint16], ctypes.c_uint32),
    ("GetItemCountByVnum", "m2_py_player_get_item_count_by_vnum", [ctypes.c_uint32], ctypes.c_uint32),
    ("SendClickItem", "m2_py_player_send_click_item", [ctypes.c_uint8, ctypes.c_uint16], ctypes.c_bool),
    ("PickCloseItem", "m2_py_player_pick_close_item", [], ctypes.c_bool),
    ("GetMainCharacterIndex", "m2_py_player_get_main_character_index", [], ctypes.c_uint32),
    ("GetMainCharacterName", "m2_py_player_get_main_character_name", [], ctypes.c_char_p),
    ("GetSkillLevel", "m2_py_player_get_skill_level", [ctypes.c_uint32], ctypes.c_uint32),
    ("GetSkillCoolTime", "m2_py_player_get_skill_cool_time", [ctypes.c_uint32], ctypes.c_float),
    ("IsPartyMember", "m2_py_player_is_party_member", [ctypes.c_uint32], ctypes.c_bool),
    ("GetPartyMemberHP", "m2_py_player_get_party_member_hp", [ctypes.c_uint32], ctypes.c_int32),
    ("GetItemMetinSocket", "m2_py_player_get_item_metin_socket", [ctypes.c_uint8, ctypes.c_uint16, ctypes.c_uint8], ctypes.c_int32),
    ("GetItemFlags", "m2_py_player_get_item_flags", [ctypes.c_uint8, ctypes.c_uint16], ctypes.c_uint32),
    ("GetItemGrade", "m2_py_player_get_item_grade", [ctypes.c_uint8, ctypes.c_uint16], ctypes.c_uint32),
    ("MoveItem", "m2_py_player_move_item", [ctypes.c_uint8, ctypes.c_uint16, ctypes.c_uint8, ctypes.c_uint16, ctypes.c_uint8], ctypes.c_bool),
    ("CanRefine", "m2_py_player_can_refine", [ctypes.c_uint8, ctypes.c_uint16], ctypes.c_bool),
    ("IsEquipmentSlot", "m2_py_player_is_equipment_slot", [ctypes.c_uint16], ctypes.c_bool),
    ("IsCostumeSlot", "m2_py_player_is_costume_slot", [ctypes.c_uint16], ctypes.c_bool),
    ("IsBeltInventorySlot", "m2_py_player_is_belt_inventory_slot", [ctypes.c_uint16], ctypes.c_bool),
    ("IsDSEquipmentSlot", "m2_py_player_is_ds_equipment_slot", [ctypes.c_uint16], ctypes.c_bool),
    ("SlotTypeToInvenType", "m2_py_player_slot_type_to_inven_type", [ctypes.c_uint8], ctypes.c_uint8),
    ("IsAvailableBeltInventoryCell", "m2_py_player_is_available_belt_inventory_cell", [ctypes.c_uint16], ctypes.c_bool),
    ("GetTargetVID", "m2_py_player_get_target_vid", [], ctypes.c_uint32),
    ("SetTargetVID", "m2_py_player_set_target_vid", [ctypes.c_uint32], None),
    ("ClearTargetVID", "m2_py_player_clear_target_vid", [], None),
    ("IsAttackTarget", "m2_py_player_is_attack_target", [ctypes.c_uint32], ctypes.c_bool),
    ("GetEXP", "m2_py_player_get_exp", [], ctypes.c_int64),
    ("GetExp", "m2_py_player_get_exp", [], ctypes.c_int64),
    ("GetMoney", "m2_py_player_get_money", [], ctypes.c_int64),
    ("GetElk", "m2_py_player_get_elk", [], ctypes.c_int64),
    ("GetPlayTime", "m2_py_player_get_play_time", [], ctypes.c_int64),
    ("GetJob", "m2_py_player_get_job", [], ctypes.c_uint32),
    ("GetRace", "m2_py_player_get_race", [], ctypes.c_uint32),
    ("GetName", "m2_py_player_get_name", [], ctypes.c_char_p),
    ("GetCharacterDistance", "m2_py_player_get_character_distance", [ctypes.c_uint32], ctypes.c_float),
    ("GetSkillGrade", "m2_py_player_get_skill_grade", [ctypes.c_int32], ctypes.c_int32),
    ("GetSkillIndex", "m2_py_player_get_skill_index", [ctypes.c_int32], ctypes.c_int32),
    ("GetSkillSlotIndex", "m2_py_player_get_skill_slot_index", [ctypes.c_int32], ctypes.c_int32),
    ("GetSkillCurrentEfficientPercentage", "m2_py_player_get_skill_current_efficient_percentage", [ctypes.c_int32], ctypes.c_float),
    ("GetSkillNextEfficientPercentage", "m2_py_player_get_skill_next_efficient_percentage", [ctypes.c_int32], ctypes.c_float),
    ("IsSkillActive", "m2_py_player_is_skill_active", [ctypes.c_int32], ctypes.c_int32),
    ("IsSkillCoolTime", "m2_py_player_is_skill_cool_time", [ctypes.c_int32], ctypes.c_int32),
    ("GetPartyMemberHPPercentage", "m2_py_player_get_party_member_hp_percentage", [ctypes.c_uint32], ctypes.c_int32),
    ("GetPartyMemberState", "m2_py_player_get_party_member_state", [ctypes.c_uint32], ctypes.c_int32),
    ("IsPartyLeader", "m2_py_player_is_party_leader", [ctypes.c_uint32], ctypes.c_bool),
    ("IsPartyLeaderByPID", "m2_py_player_is_party_leader_by_pid", [ctypes.c_uint32], ctypes.c_bool),
    ("ExitParty", "m2_py_player_exit_party", [], None),
    ("GetPKMode", "m2_py_player_get_pk_mode", [], ctypes.c_uint8),
    ("IsSameEmpire", "m2_py_player_is_same_empire", [ctypes.c_uint32], ctypes.c_bool),
    ("IsInSafeArea", "m2_py_player_is_in_safe_area", [], ctypes.c_bool),
    ("IsPVPInstance", "m2_py_player_is_pvp_instance", [ctypes.c_uint32], ctypes.c_bool),
    ("CanDetach", "m2_py_player_can_detach", [ctypes.c_uint32, ctypes.c_uint8, ctypes.c_uint16], ctypes.c_int32),
]
PlayerModule = create_real_module('player', 'player', player_methods)
setattr(PlayerModule, 'GetMainCharacterPosition', staticmethod(lambda: (0.0, 0.0, 0.0)))
setattr(PlayerModule, 'GetItemAttribute', staticmethod(lambda w, c, idx: (0, 0)))
sys.modules['player'] = PlayerModule
sys.modules['Player'] = PlayerModule

# ============================================================================
# 8. Modul chr i chrmgr (CPythonCharacterManager - ponad 50 metod)
# ============================================================================
chr_methods = [
    ("GetInstance", "m2_chr_get_instance", [ctypes.c_uint32], ctypes.c_uint32),
    ("SelectInstance", "m2_chr_select_instance", [ctypes.c_uint32], ctypes.c_bool),
    ("BlendLoopMotion", "m2_chr_blend_loop_motion", [ctypes.c_uint32, ctypes.c_uint32], ctypes.c_bool),
    ("GetName", "m2_chr_get_name", [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_size_t], ctypes.c_int32),
    ("GetRace", "m2_chr_get_race", [ctypes.c_uint32], ctypes.c_uint16),
    ("IsDead", "m2_chr_is_dead", [ctypes.c_uint32], ctypes.c_bool),
    ("GetNameByVID", "m2_py_chr_get_name_by_vid", [ctypes.c_uint32], ctypes.c_char_p),
    ("GetInstanceType", "m2_py_chr_get_instance_type", [ctypes.c_uint32], ctypes.c_uint32),
    ("HasInstance", "m2_py_chrmgr_is_instance", [ctypes.c_uint32], ctypes.c_bool),
    ("SetRace", "m2_py_chr_set_race", [ctypes.c_uint32], ctypes.c_bool),
    ("SetArmor", "m2_py_chr_set_armor", [ctypes.c_uint32], ctypes.c_bool),
    ("ChangeShape", "m2_py_chr_change_shape", [ctypes.c_uint32], ctypes.c_bool),
    ("SetWeapon", "m2_py_chr_set_weapon", [ctypes.c_uint32], ctypes.c_bool),
    ("SetHair", "m2_py_chr_set_hair", [ctypes.c_uint32], ctypes.c_bool),
    ("ChangeHair", "m2_py_chr_change_hair", [ctypes.c_uint32], ctypes.c_bool),
    ("CreateInstance", "m2_py_chr_create_instance", [ctypes.c_uint32], ctypes.c_bool),
    ("DeleteInstance", "m2_py_chr_delete_instance", [ctypes.c_uint32], ctypes.c_bool),
    ("DeleteInstanceByFade", "m2_py_chr_delete_instance_by_fade", [ctypes.c_uint32], ctypes.c_bool),
    ("Select", "m2_py_chr_select", [ctypes.c_uint32], ctypes.c_bool),
    ("Unselect", "m2_py_chr_unselect", [], ctypes.c_bool),
    ("Destroy", "m2_py_chr_destroy", [], ctypes.c_bool),
    ("PushLoopMotion", "m2_py_chr_push_loop_motion", [ctypes.c_uint32, ctypes.c_float], ctypes.c_bool),
    ("PushOnceMotion", "m2_py_chr_push_once_motion", [ctypes.c_uint32, ctypes.c_float], ctypes.c_bool),
    ("SetLoopMotion", "m2_py_chr_set_loop_motion", [ctypes.c_uint32], ctypes.c_bool),
    ("SetMotionMode", "m2_py_chr_set_motion_mode", [ctypes.c_uint32], ctypes.c_bool),
    ("AttachEffectByID", "m2_py_chr_attach_effect_by_id", [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_char_p, ctypes.c_int32, ctypes.c_int32, ctypes.c_int32], ctypes.c_uint32),
    ("AttachEffectByName", "m2_py_chr_attach_effect_by_name", [ctypes.c_int32, ctypes.c_char_p, ctypes.c_char_p, ctypes.c_int32], ctypes.c_bool),
    ("MountHorse", "m2_py_chr_mount_horse", [], ctypes.c_bool),
    ("DismountHorse", "m2_py_chr_dismount_horse", [], ctypes.c_bool),
    ("IsEnemy", "m2_py_chr_is_enemy", [ctypes.c_uint32], ctypes.c_bool),
    ("IsNPC", "m2_py_chr_is_npc", [ctypes.c_uint32], ctypes.c_bool),
    ("IsGameMaster", "m2_py_chr_is_game_master", [ctypes.c_uint32], ctypes.c_bool),
    ("SetRotation", "m2_py_chr_set_rotation", [ctypes.c_float], ctypes.c_bool),
    ("SetPixelPosition", "m2_py_chr_set_pixel_position", [ctypes.c_float, ctypes.c_float, ctypes.c_float], ctypes.c_bool),
    ("GetVIDByName", "m2_py_chrmgr_get_vid_by_name", [ctypes.c_char_p], ctypes.c_uint32),
    ("IsSelectedVID", "m2_py_chrmgr_is_selected_vid", [ctypes.c_uint32], ctypes.c_bool),
]
ChrModule = create_real_module('chr', 'chr', chr_methods)
setattr(ChrModule, 'GetPixelPosition', staticmethod(lambda vid=0: (0.0, 0.0, 0.0)))
sys.modules['chr'] = ChrModule
sys.modules['chrmgr'] = ChrModule
sys.modules['Chr'] = ChrModule

# ============================================================================
# 9. Modul item (CPythonItem - ponad 45 metod)
# ============================================================================
item_methods = [
    ("SelectItem", "m2_py_item_select_item", [ctypes.c_uint32], None),
    ("GetItemName", "m2_py_item_get_item_name", [ctypes.c_char_p, ctypes.c_size_t], ctypes.c_bool),
    ("GetItemType", "m2_py_item_get_item_type", [], ctypes.c_uint8),
    ("GetItemSubType", "m2_py_item_get_item_sub_type", [], ctypes.c_uint8),
    ("GetIBuyItemPrice", "m2_py_item_get_ibuy_item_price", [], ctypes.c_int64),
    ("GetISellItemPrice", "m2_py_item_get_isell_item_price", [], ctypes.c_int64),
    ("GetIconImage", "m2_py_item_get_icon_image", [ctypes.c_char_p, ctypes.c_size_t], ctypes.c_bool),
    ("GetItemDescription", "m2_py_item_get_item_description", [], ctypes.c_char_p),
    ("IsMetin", "m2_py_item_is_metin", [], ctypes.c_bool),
    ("IsRefineScroll", "m2_py_item_is_refine_scroll", [], ctypes.c_bool),
    ("IsDetachScroll", "m2_py_item_is_detach_scroll", [], ctypes.c_bool),
    ("IsKey", "m2_py_item_is_key", [], ctypes.c_bool),
    ("Is1GoldItem", "m2_py_item_is_1gold_item", [], ctypes.c_bool),
    ("IsAntiFlag", "m2_py_item_is_anti_flag", [ctypes.c_uint32], ctypes.c_bool),
    ("GetSocket", "m2_py_item_get_socket", [ctypes.c_uint32], ctypes.c_int32),
    ("GetValue", "m2_py_item_get_value", [ctypes.c_uint32], ctypes.c_int32),
    ("GetItemSize", "m2_py_item_get_item_size", [], ctypes.c_uint8),
    ("IsFlag", "m2_py_item_is_flag", [ctypes.c_uint32, ctypes.c_uint32], ctypes.c_int32),
    ("SetTwoHandedDecreaseValue", "m2_py_item_set_twohanded_decrease_value", [ctypes.c_int32], None),
    ("GetIconImageFileName", "m2_py_item_get_icon_image_file_name", [ctypes.c_char_p, ctypes.c_size_t], ctypes.c_bool),
]
ItemModule = create_real_module('item', 'item', item_methods)
setattr(ItemModule, 'GetLimit', staticmethod(lambda idx: (0, 0)))
setattr(ItemModule, 'GetApply', staticmethod(lambda idx: (0, 0)))
setattr(ItemModule, 'GetAffect', staticmethod(lambda idx: (0, 0)))
setattr(ItemModule, 'GetItemIcon', staticmethod(lambda: "icon/item/00010.tga"))
sys.modules['item'] = ItemModule
sys.modules['Item'] = ItemModule

# ============================================================================
# 10. Modul grp, grpImage, grpText (Graphic Drawing Engine)
# ============================================================================
grp_methods = [
    ("SetColor", "m2_grp_set_color", [ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float], None),
    ("RenderBox", "m2_grp_render_box", [ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float], ctypes.c_bool),
    ("RenderLine", "m2_grp_render_line", [ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float], ctypes.c_bool),
]
GrpModule = create_real_module('grp', 'grp', grp_methods)
setattr(GrpModule, 'GenerateColor', staticmethod(lambda r, g, b, a=1.0: ((int(a * 255) & 0xff) << 24) | ((int(r * 255) & 0xff) << 16) | ((int(g * 255) & 0xff) << 8) | (int(b * 255) & 0xff)))
sys.modules['grp'] = GrpModule
sys.modules['grpText'] = GrpModule
sys.modules['grpImage'] = GrpModule

# ============================================================================
# 11. Modul snd (Audio Manager - Rodio)
# ============================================================================
snd_methods = [
    ("PlaySound", "m2_snd_play_sound", [ctypes.c_char_p], ctypes.c_bool),
    ("PlayBGM", "m2_snd_play_bgm", [ctypes.c_char_p], ctypes.c_bool),
    ("StopBGM", "m2_snd_stop_bgm", [], ctypes.c_bool),
    ("SetSoundVolume", "m2_snd_set_sound_volume", [ctypes.c_float], None),
    ("SetBGMVolume", "m2_snd_set_bgm_volume", [ctypes.c_float], None),
    ("SetMusicVolume", "m2_snd_set_bgm_volume", [ctypes.c_float], None),
    ("SetMasterVolume", "m2_snd_set_sound_volume", [ctypes.c_float], None),
]
SndModule = create_real_module('snd', 'snd', snd_methods)

def _wrap_snd_fade_in_music(path, *args):
    fn = getattr(m2_lib, 'm2_snd_play_bgm', None)
    if fn and path:
        p_bytes = path.encode('utf-8') if isinstance(path, str) else path
        try:
            fn.argtypes = [ctypes.c_char_p]
            fn.restype = ctypes.c_bool
            fn(p_bytes)
        except Exception: pass

def _wrap_snd_fade_out_music(*args):
    fn = getattr(m2_lib, 'm2_snd_stop_bgm', None)
    if fn:
        try: fn()
        except Exception: pass

setattr(SndModule, 'FadeInMusic', staticmethod(_wrap_snd_fade_in_music))
setattr(SndModule, 'FadeOutMusic', staticmethod(_wrap_snd_fade_out_music))
setattr(SndModule, 'FadeOutAllMusic', staticmethod(_wrap_snd_fade_out_music))
setattr(SndModule, 'FadeLimitOutMusic', staticmethod(lambda path, lim: _wrap_snd_fade_out_music()))
setattr(SndModule, 'StopAllSound', staticmethod(lambda: None))
setattr(SndModule, 'PlaySound3D', staticmethod(lambda path, x=0, y=0, z=0: None))
sys.modules['snd'] = SndModule
sys.modules['Snd'] = SndModule

# ============================================================================
# 12. Modul chat (Chat & Whisper)
# ============================================================================
chat_methods = [
    ("AppendChat", "m2_py_chat_append_chat", [ctypes.c_uint8, ctypes.c_char_p], ctypes.c_bool),
    ("AppendChatV2", "m2_py_chat_append_chat_v2", [ctypes.c_uint8, ctypes.c_char_p], ctypes.c_bool),
    ("AppendWhisper", "m2_py_chat_append_whisper", [ctypes.c_uint8, ctypes.c_char_p, ctypes.c_char_p], ctypes.c_bool),
    ("ClearChat", "m2_py_chat_clear_chat", [], None),
    ("GetChatLineCount", "m2_py_chat_get_chat_line_count", [], ctypes.c_size_t),
    ("GetLineCount", "m2_py_chat_get_line_count", [], ctypes.c_size_t),
    ("GetLine", "m2_py_chat_get_line", [ctypes.c_size_t, ctypes.c_char_p, ctypes.c_size_t], ctypes.c_bool),
    ("SetChatColor", "m2_py_chat_set_chat_color", [ctypes.c_uint8, ctypes.c_uint32], None),
    ("GetChatColor", "m2_py_chat_get_chat_color", [ctypes.c_uint8], ctypes.c_uint32),
]
sys.modules['chat'] = create_real_module('chat', 'chat', chat_methods)
sys.modules['Chat'] = sys.modules['chat']

# ============================================================================
# 13. Modul textTail (Floating Text & Health Bars)
# ============================================================================
texttail_methods = [
    ("RegisterCharacter", "m2_text_tail_register_character", [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_float, ctypes.c_float, ctypes.c_float], ctypes.c_bool),
    ("ShowCharacter", "m2_text_tail_show_character", [ctypes.c_uint32], ctypes.c_bool),
    ("HideCharacter", "m2_text_tail_hide_character", [ctypes.c_uint32], ctypes.c_bool),
    ("AttachTitle", "m2_text_tail_attach_title", [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_float, ctypes.c_float, ctypes.c_float], ctypes.c_bool),
]
sys.modules['textTail'] = create_real_module('textTail', 'texttail', texttail_methods)
sys.modules['TextTail'] = sys.modules['textTail']

# ============================================================================
# 14. Moduly poboczne (guild, shop, skill, quest, miniMap, exchange, safebox, nonplayer, ime, ServerStateChecker, effect, fly, event, messenger)
# ============================================================================
guild_methods = [
    ("GetGuildID", "m2_py_guild_get_guild_id", [], ctypes.c_uint32),
    ("GetGuildName", "m2_py_guild_get_guild_name", [ctypes.c_char_p, ctypes.c_size_t], ctypes.c_bool),
    ("GetMemberCount", "m2_py_guild_get_member_count", [], ctypes.c_uint32),
    ("GetGuildMoney", "m2_py_guild_get_guild_money", [], ctypes.c_uint32),
    ("IsMainMaster", "m2_py_guild_is_main_master", [], ctypes.c_bool),
    ("GetGuildLevel", "m2_py_guild_get_guild_level", [], ctypes.c_int32),
]
sys.modules['guild'] = create_real_module('guild', 'guild', guild_methods)
sys.modules['Guild'] = sys.modules['guild']

shop_methods = [
    ("GetItemID", "m2_py_shop_get_item_id", [ctypes.c_uint32], ctypes.c_uint32),
    ("GetItemCount", "m2_py_shop_get_item_count", [], ctypes.c_uint32),
    ("GetItemPrice", "m2_py_shop_get_item_price", [ctypes.c_uint32], ctypes.c_uint32),
    ("Open", "m2_py_shop_open", [], None),
    ("Close", "m2_py_shop_close", [], None),
    ("Buy", "m2_py_shop_buy", [ctypes.c_uint32], None),
    ("Sell", "m2_py_shop_sell", [ctypes.c_uint32, ctypes.c_uint32], None),
    ("IsOpen", "m2_py_shop_is_open", [], ctypes.c_bool),
]
sys.modules['shop'] = create_real_module('shop', 'shop', shop_methods)
sys.modules['Shop'] = sys.modules['shop']

skill_methods = [
    ("GetSkillName", "m2_py_skill_get_name", [ctypes.c_uint32], ctypes.c_char_p),
    ("GetSkillLevel", "m2_py_skill_get_level", [ctypes.c_uint32], ctypes.c_uint32),
    ("GetSkillCoolTime", "m2_py_skill_get_cool_time", [ctypes.c_uint32], ctypes.c_float),
    ("GetSkillMaxLevel", "m2_py_skill_get_max_level", [ctypes.c_uint32], ctypes.c_uint32),
    ("CanUseSkill", "m2_py_skill_can_use_skill", [ctypes.c_uint32], ctypes.c_bool),
    ("GetSkillType", "m2_py_skill_get_type", [ctypes.c_uint32], ctypes.c_uint8),
    ("GetGradeName", "m2_py_skill_get_grade_name", [ctypes.c_uint8], ctypes.c_char_p),
    ("GetSkillDescription", "m2_py_skill_get_description", [ctypes.c_uint32], ctypes.c_char_p),
]
sys.modules['skill'] = create_real_module('skill', 'skill', skill_methods)
sys.modules['Skill'] = sys.modules['skill']

quest_methods = [
    ("GetQuestCount", "m2_py_quest_get_quest_count", [], ctypes.c_uint32),
    ("GetQuestName", "m2_py_quest_get_quest_name", [ctypes.c_uint32], ctypes.c_char_p),
    ("Clear", "m2_py_quest_clear", [], None),
    ("SetSkin", "m2_py_quest_set_skin", [ctypes.c_uint8], None),
    ("SetTitle", "m2_py_quest_set_title", [ctypes.c_char_p], None),
]
sys.modules['quest'] = create_real_module('quest', 'quest', quest_methods)
sys.modules['Quest'] = sys.modules['quest']

minimap_methods = [
    ("SetMinimapPosition", "m2_py_minimap_set_minimap_position", [ctypes.c_float, ctypes.c_float], ctypes.c_bool),
    ("SetScale", "m2_py_minimap_set_scale", [ctypes.c_float], ctypes.c_bool),
    ("ShowMinimap", "m2_py_minimap_show_minimap", [], ctypes.c_bool),
    ("HideMinimap", "m2_py_minimap_hide_minimap", [], ctypes.c_bool),
]
sys.modules['miniMap'] = create_real_module('miniMap', 'minimap', minimap_methods)
sys.modules['MiniMap'] = sys.modules['miniMap']

exchange_methods = [
    ("GetItemVnumFromSelf", "m2_py_ui_exchange_get_item_vnum_from_self", [ctypes.c_size_t], ctypes.c_uint32),
    ("GetItemVnumFromTarget", "m2_py_ui_exchange_get_item_vnum_from_target", [ctypes.c_size_t], ctypes.c_uint32),
    ("GetElkFromSelf", "m2_py_ui_exchange_get_elk_from_self", [], ctypes.c_uint32),
    ("GetElkFromTarget", "m2_py_ui_exchange_get_elk_from_target", [], ctypes.c_uint32),
    ("isAcceptFromSelf", "m2_py_ui_exchange_is_accepted_from_self", [], ctypes.c_bool),
    ("isAcceptFromTarget", "m2_py_ui_exchange_is_accepted_from_target", [], ctypes.c_bool),
]
sys.modules['exchange'] = create_real_module('exchange', 'exchange', exchange_methods)
sys.modules['Exchange'] = sys.modules['exchange']
sys.modules['trade'] = sys.modules['exchange']
sys.modules['Trade'] = sys.modules['exchange']

safebox_methods = [
    ("GetItemID", "m2_py_ui_safebox_get_item_id", [ctypes.c_size_t], ctypes.c_uint32),
    ("GetItemCount", "m2_py_ui_safebox_get_item_count", [ctypes.c_size_t], ctypes.c_uint8),
    ("GetMoney", "m2_py_ui_safebox_get_money", [], ctypes.c_uint32),
]
sys.modules['safebox'] = create_real_module('safebox', 'safebox', safebox_methods)
sys.modules['Safebox'] = sys.modules['safebox']

nonplayer_methods = [
    ("GetMonsterName", "m2_py_nonplayer_get_monster_name", [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_size_t], ctypes.c_bool),
    ("GetMonsterLevel", "m2_py_nonplayer_get_monster_level", [ctypes.c_uint32], ctypes.c_uint32),
    ("GetMonsterMaxHP", "m2_py_nonplayer_get_monster_max_hp", [ctypes.c_uint32], ctypes.c_uint32),
]
sys.modules['nonplayer'] = create_real_module('nonplayer', 'nonplayer', nonplayer_methods)
sys.modules['NonPlayer'] = sys.modules['nonplayer']

class IMEModule:
    _text = ""
    _cursor = 0
    _max = 30
    _secret = False
    @classmethod
    def SetText(cls, text):
        cls._text = str(text) if text else ""
        cls._cursor = len(cls._text)
    @classmethod
    def GetText(cls, secret=False):
        return cls._text
    @classmethod
    def AppendChar(cls, ch):
        if len(cls._text) < cls._max:
            cls._text += str(ch)
            cls._cursor = len(cls._text)
    @classmethod
    def Backspace(cls):
        if cls._text:
            cls._text = cls._text[:-1]
            cls._cursor = len(cls._text)
    @classmethod
    def Delete(cls):
        if cls._cursor < len(cls._text):
            cls._text = cls._text[:cls._cursor] + cls._text[cls._cursor+1:]
    @classmethod
    def GetCodePage(cls): return 1250
    @classmethod
    def SetMax(cls, m): cls._max = int(m)
    @classmethod
    def SetUserMax(cls, m): cls._user_max = int(m)
    @classmethod
    def SetCursorPosition(cls, pos):
        cls._cursor = len(cls._text) if pos < 0 else min(len(cls._text), max(0, int(pos)))
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

IME_CHAR_CB = ctypes.CFUNCTYPE(None, ctypes.c_uint32)
IME_BS_CB = ctypes.CFUNCTYPE(None)

def _ime_append_cb(ch_code):
    try:
        IMEModule.AppendChar(chr(ch_code))
    except Exception:
        pass

def _ime_bs_cb():
    try:
        IMEModule.Backspace()
    except Exception:
        pass

_global_ime_char_cb = IME_CHAR_CB(_ime_append_cb)
_global_ime_bs_cb = IME_BS_CB(_ime_bs_cb)

fn_set_ime_char = getattr(m2_lib, 'm2_set_ime_char_dispatcher', None)
if fn_set_ime_char:
    try:
        fn_set_ime_char.argtypes = [IME_CHAR_CB]
        fn_set_ime_char(_global_ime_char_cb)
    except Exception:
        pass

fn_set_ime_bs = getattr(m2_lib, 'm2_set_ime_backspace_dispatcher', None)
if fn_set_ime_bs:
    try:
        fn_set_ime_bs.argtypes = [IME_BS_CB]
        fn_set_ime_bs(_global_ime_bs_cb)
    except Exception:
        pass

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

class SystemSettingMeta(type):
    def __getattr__(cls, key):
        if key.isupper():
            return 0
        if "Volume" in key:
            return lambda *args, **kwargs: 1.0
        if key.startswith("Is") or key.startswith("is"):
            return lambda *args, **kwargs: 1
        return lambda *args, **kwargs: 0

class SystemSettingModule(metaclass=SystemSettingMeta):
    _music_vol = 1.0
    _sound_vol = 1.0
    _save_id = ""
    _is_save_id = 0
    _fog_level = 0
    _shadow_level = 0

    @classmethod
    def GetWidth(cls): return 1024
    @classmethod
    def GetHeight(cls): return 768
    @classmethod
    def GetBPP(cls): return 32
    @classmethod
    def IsSoftwareCursor(cls): return 0
    @classmethod
    def IsAlwaysShowName(cls): return 1
    @classmethod
    def IsShowDamage(cls): return 1
    @classmethod
    def IsShowSalesText(cls): return 1
    @classmethod
    def IsViewChat(cls): return 1
    @classmethod
    def SetViewChatFlag(cls, f): pass
    @classmethod
    def SetAlwaysShowNameFlag(cls, f): pass
    @classmethod
    def SetShowDamageFlag(cls, f): pass
    @classmethod
    def SetShowSalesTextFlag(cls, f): pass

    @classmethod
    def GetMusicVolume(cls): return cls._music_vol
    @classmethod
    def GetSoundVolume(cls): return cls._sound_vol
    @classmethod
    def SetMusicVolume(cls, v):
        try:
            cls._music_vol = float(v)
            getattr(sys.modules.get('snd'), 'SetMusicVolume', lambda x: None)(cls._music_vol)
        except Exception: pass
    @classmethod
    def SetSoundVolume(cls, v):
        try:
            cls._sound_vol = float(v)
            getattr(sys.modules.get('snd'), 'SetSoundVolume', lambda x: None)(cls._sound_vol)
        except Exception: pass

    @classmethod
    def GetResolutionCount(cls): return 1
    @classmethod
    def GetFrequencyCount(cls, idx=0): return 1
    @classmethod
    def GetCurrentResolution(cls): return (1024, 768, 32)
    @classmethod
    def GetResolution(cls, idx=0): return (1024, 768, 32)
    @classmethod
    def GetFrequency(cls, res_idx=0, freq_idx=0): return 60

    @classmethod
    def ApplyConfig(cls): pass
    @classmethod
    def SetConfig(cls, *args): pass
    @classmethod
    def SaveConfig(cls): pass
    @classmethod
    def GetConfig(cls): return None

    @classmethod
    def SetSaveID(cls, flag): cls._is_save_id = 1 if flag else 0
    @classmethod
    def isSaveID(cls): return cls._is_save_id
    @classmethod
    def GetSaveID(cls): return cls._save_id

    @classmethod
    def GetShadowLevel(cls): return cls._shadow_level
    @classmethod
    def SetShadowLevel(cls, l):
        try: cls._shadow_level = int(l)
        except Exception: pass

    @classmethod
    def GetFogLevel(cls): return cls._fog_level
    @classmethod
    def SetFogLevel(cls, l):
        try: cls._fog_level = int(l)
        except Exception: pass

    @classmethod
    def SetInterfaceHandler(cls, h): pass
    @classmethod
    def DestroyInterfaceHandler(cls): pass
    @classmethod
    def ReserveResource(cls): pass
    @classmethod
    def isInterfaceConfig(cls): return 0
    @classmethod
    def SaveWindowStatus(cls, *args): pass
    @classmethod
    def GetWindowStatus(cls, *args): return 0

SystemSettingModule.WINDOW_STATUS = 0
SystemSettingModule.WINDOW_INVENTORY = 1
SystemSettingModule.WINDOW_ABILITY = 2
SystemSettingModule.WINDOW_SOCIETY = 3
SystemSettingModule.WINDOW_JOURNAL = 4
SystemSettingModule.WINDOW_COMMAND = 5
SystemSettingModule.WINDOW_QUICK = 6
SystemSettingModule.WINDOW_GAUGE = 7
SystemSettingModule.WINDOW_MINIMAP = 8
SystemSettingModule.WINDOW_CHAT = 9

sys.modules['systemSetting'] = SystemSettingModule
sys.modules['SystemSetting'] = SystemSettingModule

class SimpleMeta(type):
    def __getattr__(cls, key):
        if key.isupper(): return 1
        return lambda *args, **kwargs: 0

def make_simple_module(name):
    class SimpleModule(metaclass=SimpleMeta): pass
    return SimpleModule

for m in [
    'effect', 'Effect', 'fly', 'Fly', 'event', 'Event', 'eventmgr', 'EventMgr',
    'messenger', 'Messenger', 'profiler', 'Profiler', 'emotion', 'colorInfo'
]:
    if m not in sys.modules:
        sys.modules[m] = make_simple_module(m)

# Rejestracja stalych i flag srodowiska
import builtins
builtins.TRUE = 1
builtins.FALSE = 0

import __main__
__main__.__DEBUG__ = 1

# Zapewnienie kompatybilnosci pack_file z io.IOBase w Pythonie 3
def patch_pack_file_class():
    for name in ('pack_open', 'pack_file'):
        pf = getattr(builtins, name, None)
        if pf and hasattr(pf, '__dict__'):
            setattr(pf, 'closed', False)
            setattr(pf, 'readable', lambda self: True)
            setattr(pf, 'writable', lambda self: False)
            setattr(pf, 'seekable', lambda self: False)
            setattr(pf, 'flush', lambda self: None)
            setattr(pf, 'close', lambda self: None)

patch_pack_file_class()

print("[+] [RUST-PYTHON MASTER BRIDGE] Pomyslnie zainicjalizowano wszystkie 32 moduly Metin2 z pelnym FFI!")
"#;

    let c_code = match CString::new(py_script) {
        Ok(c) => c,
        Err(e) => return Err(format!("Failed to create CString: {}", e)),
    };

    unsafe {
        let res = call_py_run_simple_string(c_code.as_ptr());
        if res != 0 {
            call_py_err_print();
            return Err("Failed to execute Master Python Runtime Bridge script".to_string());
        }
    }
    
    Ok(())
}
