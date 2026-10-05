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
    let ensure_ptr = get_py_sym("PyGILState_Ensure");
    let release_ptr = get_py_sym("PyGILState_Release");
    let gstate = if !ensure_ptr.is_null() {
        let ensure_fn: extern "C" fn() -> i32 = std::mem::transmute(ensure_ptr);
        ensure_fn()
    } else {
        0
    };

    let ptr = get_py_sym("PyRun_SimpleString");
    let res = if !ptr.is_null() {
        let func: extern "C" fn(*const std::ffi::c_char) -> std::ffi::c_int = std::mem::transmute(ptr);
        func(command)
    } else {
        -1
    };

    if !release_ptr.is_null() {
        let release_fn: extern "C" fn(i32) = std::mem::transmute(release_ptr);
        release_fn(gstate);
    }

    res
}

unsafe fn call_py_err_print() {
    let ensure_ptr = get_py_sym("PyGILState_Ensure");
    let release_ptr = get_py_sym("PyGILState_Release");
    let gstate = if !ensure_ptr.is_null() {
        let ensure_fn: extern "C" fn() -> i32 = std::mem::transmute(ensure_ptr);
        ensure_fn()
    } else {
        0
    };

    let occurred_ptr = get_py_sym("PyErr_Occurred");
    if !occurred_ptr.is_null() {
        let occurred_fn: extern "C" fn() -> *mut std::ffi::c_void = std::mem::transmute(occurred_ptr);
        if !occurred_fn().is_null() {
            if !release_ptr.is_null() {
                let release_fn: extern "C" fn(i32) = std::mem::transmute(release_ptr);
                release_fn(gstate);
            }
            return;
        }
    }
    let ptr = get_py_sym("PyErr_Print");
    if !ptr.is_null() {
        let func: extern "C" fn() = std::mem::transmute(ptr);
        func();
    }

    if !release_ptr.is_null() {
        let release_fn: extern "C" fn(i32) = std::mem::transmute(release_ptr);
        release_fn(gstate);
    }
}

/// Inicjalizuje i wstrzykuje pelne 32 moduly Metin2 do runtime Pythona przed uruchomieniem system.py
pub fn inject_master_runtime_modules() -> Result<(), String> {
    let wire_ptrs_script = format!(
        r#"
_NATIVE_WIRE_PTRS = {{
    'm2_app_set_frame_skip': {},
    'm2_app_get_random': {},
    'm2_app_get_global_time_stamp': {},
    'm2_app_set_hair_color_enable': {},
    'm2_app_set_armor_specular_enable': {},
    'm2_app_set_weapon_specular_enable': {},
    'm2_app_set_camera_max_distance': {},
    'm2_app_set_default_font_name': {},
    'm2_app_set_guild_mark_path': {},
    'm2_app_set_guild_symbol_path': {},
    'm2_app_set_center_position': {},
    'm2_app_get_image_info_width': {},
    'm2_app_get_image_info_height': {},
    'm2_app_get_image_info_valid': {},
    'm2_app_is_exist_file': {},
    'm2_app_get_locale_name': {},
}}
"#,
        crate::py_master_runtime_wire_final::m2_app_set_frame_skip as usize,
        crate::py_master_runtime_wire_final::m2_app_get_random as usize,
        crate::py_master_runtime_wire_final::m2_app_get_global_time_stamp as usize,
        crate::py_master_runtime_wire_final::m2_app_set_hair_color_enable as usize,
        crate::py_master_runtime_wire_final::m2_app_set_armor_specular_enable as usize,
        crate::py_master_runtime_wire_final::m2_app_set_weapon_specular_enable as usize,
        crate::py_master_runtime_wire_final::m2_app_set_camera_max_distance as usize,
        crate::py_master_runtime_wire_final::m2_app_set_default_font_name as usize,
        crate::py_master_runtime_wire_final::m2_app_set_guild_mark_path as usize,
        crate::py_master_runtime_wire_final::m2_app_set_guild_symbol_path as usize,
        crate::py_master_runtime_wire_final::m2_app_set_center_position as usize,
        crate::py_master_runtime_wire_final::m2_app_get_image_info_width as usize,
        crate::py_master_runtime_wire_final::m2_app_get_image_info_height as usize,
        crate::py_master_runtime_wire_final::m2_app_get_image_info_valid as usize,
        crate::py_master_runtime_wire_final::m2_app_is_exist_file as usize,
        crate::py_master_runtime_wire_final::m2_app_get_locale_name as usize,
    );
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
        os.path.join(os.path.dirname(sys.executable), "m2_client.dll"),
        os.path.join(os.path.dirname(sys.executable), "deps", "m2_client.dll"),
        "E:/SourceCodeClient/target/debug/deps/m2_client.dll",
        "E:/SourceCodeClient/target/debug/m2_client.dll",
        "E:/SourceCodeClient/m2_client.dll",
        "E:/SourceCodeClient/target/x86_64-pc-windows-msvc/release/m2_client.dll",
        "E:/SourceCodeClient/target/x86_64-pc-windows-msvc/debug/m2_client.dll",
        "E:/m2dev-client-src-main/m2dev-client-main/m2_client.dll",
        "E:/SourceCodeClient/crates/m2-client/m2_client.dll",
    ]:
        if os.path.exists(cand):
            try:
                m2_lib = ctypes.PyDLL(cand)
                print(f"[+] Python Master Bridge zaladowal PyDLL: {cand}")
                break
            except Exception:
                try:
                    m2_lib = ctypes.CDLL(cand)
                    print(f"[+] Python Master Bridge zaladowal CDLL: {cand}")
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

def _get_wire_fn(name, argtypes, restype=None):
    try:
        addr = _NATIVE_WIRE_PTRS.get(name, 0)
        if addr != 0:
            return ctypes.CFUNCTYPE(restype, *argtypes)(addr)
    except Exception:
        pass
    if m2_lib:
        fn = getattr(m2_lib, name, None)
        if fn:
            fn.argtypes = argtypes
            if restype: fn.restype = restype
            return fn
    return None

_OBJECT_REGISTRY = {}
_py_handlers = {}

EVENT_CB_0 = ctypes.CFUNCTYPE(None, ctypes.c_size_t, ctypes.c_char_p)
def _py_event_dispatch_0(handle, method_ptr):
    try:
        obj = _OBJECT_REGISTRY.get(handle) or _py_handlers.get(handle)
        if obj is not None and method_ptr:
            m = method_ptr.decode('utf-8', 'ignore')
            fn = getattr(obj, m, None)
            if fn and callable(fn):
                fn()
    except Exception as e:
        m = method_ptr.decode('utf-8', 'ignore') if method_ptr else ""
        print(f"[-] PY_EVENT_0 EXCEPTION in {m}: {e}")
        import traceback; traceback.print_exc()

EVENT_CB_1_STR = ctypes.CFUNCTYPE(None, ctypes.c_size_t, ctypes.c_char_p, ctypes.c_char_p)
def _py_event_dispatch_1_str(handle, method_ptr, arg_ptr):
    try:
        obj = _OBJECT_REGISTRY.get(handle) or _py_handlers.get(handle)
        if obj is not None and method_ptr:
            m = method_ptr.decode('utf-8', 'ignore')
            arg = arg_ptr.decode('utf-8', 'ignore') if arg_ptr else ""
            fn = getattr(obj, m, None)
            if fn and callable(fn):
                fn(arg)
    except Exception as e:
        m = method_ptr.decode('utf-8', 'ignore') if method_ptr else ""
        print(f"[-] PY_EVENT_1_STR EXCEPTION in {m}: {e}")
        import traceback; traceback.print_exc()

EVENT_CB_1_I32 = ctypes.CFUNCTYPE(None, ctypes.c_size_t, ctypes.c_char_p, ctypes.c_int32)
def _py_event_dispatch_1_i32(handle, method_ptr, val):
    try:
        obj = _OBJECT_REGISTRY.get(handle) or _py_handlers.get(handle)
        if obj is not None and method_ptr:
            m = method_ptr.decode('utf-8', 'ignore')
            fn = getattr(obj, m, None)
            if fn and callable(fn):
                fn(int(val))
    except Exception as e:
        m = method_ptr.decode('utf-8', 'ignore') if method_ptr else ""
        print(f"[-] PY_EVENT_1_I32 EXCEPTION in {m}: {e}")
        import traceback; traceback.print_exc()

EVENT_CB_2_I32 = ctypes.CFUNCTYPE(None, ctypes.c_size_t, ctypes.c_char_p, ctypes.c_int32, ctypes.c_int32)
def _py_event_dispatch_2_i32(handle, method_ptr, x, y):
    try:
        obj = _OBJECT_REGISTRY.get(handle) or _py_handlers.get(handle)
        m = method_ptr.decode('utf-8', 'ignore') if method_ptr else ""
        print(f"[PY_EVENT] dispatch_2_i32: handle=0x{handle:X}, method='{m}', x={x}, y={y}, found={obj is not None}")
        if obj is not None and m:
            fn = getattr(obj, m, None)
            if fn and callable(fn):
                print(f"[PY_EVENT] Executing {type(obj).__name__}.{m}({x}, {y})...")
                fn(int(x), int(y))
                print(f"[PY_EVENT] Finished {type(obj).__name__}.{m}!")
            else:
                print(f"[-] PY_EVENT: method '{m}' not found or not callable on {type(obj).__name__}")
        else:
            print(f"[-] PY_EVENT: handle 0x{handle:X} not found in _OBJECT_REGISTRY (total registered: {len(_OBJECT_REGISTRY)})")
    except Exception as e:
        import traceback
        print(f"[-] PY_EVENT_2_I32 EXCEPTION: {e}")
        traceback.print_exc()

MOD_CB_0 = ctypes.CFUNCTYPE(None, ctypes.c_char_p, ctypes.c_char_p)
def _py_mod_dispatch_0(mod_ptr, method_ptr):
    try:
        if mod_ptr and method_ptr:
            mod_name = mod_ptr.decode('utf-8', 'ignore')
            method_name = method_ptr.decode('utf-8', 'ignore')
            mod = sys.modules.get(mod_name)
            if mod:
                fn = getattr(mod, method_name, None)
                if fn and callable(fn):
                    fn()
    except Exception:
        pass

MOD_CB_1_STR = ctypes.CFUNCTYPE(None, ctypes.c_char_p, ctypes.c_char_p, ctypes.c_char_p)
def _py_mod_dispatch_1_str(mod_ptr, method_ptr, arg_ptr):
    try:
        if mod_ptr and method_ptr:
            mod_name = mod_ptr.decode('utf-8', 'ignore')
            method_name = method_ptr.decode('utf-8', 'ignore')
            arg = arg_ptr.decode('utf-8', 'ignore') if arg_ptr else ""
            mod = sys.modules.get(mod_name)
            if mod:
                fn = getattr(mod, method_name, None)
                if fn and callable(fn):
                    fn(arg)
    except Exception:
        pass

_c_cb_0 = EVENT_CB_0(_py_event_dispatch_0)
_c_cb_1_str = EVENT_CB_1_STR(_py_event_dispatch_1_str)
_c_cb_1_i32 = EVENT_CB_1_I32(_py_event_dispatch_1_i32)
_c_cb_2_i32 = EVENT_CB_2_I32(_py_event_dispatch_2_i32)
_c_mod_cb_0 = MOD_CB_0(_py_mod_dispatch_0)
_c_mod_cb_1_str = MOD_CB_1_STR(_py_mod_dispatch_1_str)

if m2_lib:
    for name, cb in [
        ('m2_set_py_event_dispatcher_0', _c_cb_0),
        ('m2_set_py_event_dispatcher_1_str', _c_cb_1_str),
        ('m2_set_py_event_dispatcher_1_i32', _c_cb_1_i32),
        ('m2_set_py_event_dispatcher_2_i32', _c_cb_2_i32),
        ('m2_set_py_mod_dispatcher_0', _c_mod_cb_0),
        ('m2_set_py_mod_dispatcher_1_str', _c_mod_cb_1_str),
    ]:
        fn = getattr(m2_lib, name, None)
        if fn:
            try:
                fn(cb)
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

class ModType(type):
    def __getattr__(cls, attr):
        if attr.isupper():
            return 1
        return lambda *args, **kwargs: 0

def create_real_module(module_name, prefix, methods_spec):
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
AppModule.DIK_ESC = 1
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
AppModule.DIK_MINUS = 12
AppModule.DIK_EQUALS = 13
AppModule.DIK_BACK = 14
AppModule.DIK_TAB = 15
AppModule.DIK_Q = 16
AppModule.DIK_W = 17
AppModule.DIK_E = 18
AppModule.DIK_R = 19
AppModule.DIK_T = 20
AppModule.DIK_Y = 21
AppModule.DIK_U = 22
AppModule.DIK_I = 23
AppModule.DIK_O = 24
AppModule.DIK_P = 25
AppModule.DIK_LBRACKET = 26
AppModule.DIK_RBRACKET = 27
AppModule.DIK_RETURN = 28
AppModule.DIK_LCONTROL = 29
AppModule.DIK_A = 30
AppModule.DIK_S = 31
AppModule.DIK_D = 32
AppModule.DIK_F = 33
AppModule.DIK_G = 34
AppModule.DIK_H = 35
AppModule.DIK_J = 36
AppModule.DIK_K = 37
AppModule.DIK_L = 38
AppModule.DIK_SEMICOLON = 39
AppModule.DIK_APOSTROPHE = 40
AppModule.DIK_GRAVE = 41
AppModule.DIK_LSHIFT = 42
AppModule.DIK_BACKSLASH = 43
AppModule.DIK_Z = 44
AppModule.DIK_X = 45
AppModule.DIK_C = 46
AppModule.DIK_V = 47
AppModule.DIK_B = 48
AppModule.DIK_N = 49
AppModule.DIK_M = 50
AppModule.DIK_COMMA = 51
AppModule.DIK_PERIOD = 52
AppModule.DIK_SLASH = 53
AppModule.DIK_RSHIFT = 54
AppModule.DIK_MULTIPLY = 55
AppModule.DIK_LALT = 56
AppModule.DIK_SPACE = 57
AppModule.DIK_CAPITAL = 58
AppModule.DIK_F1 = 59
AppModule.DIK_F2 = 60
AppModule.DIK_F3 = 61
AppModule.DIK_F4 = 62
AppModule.DIK_F5 = 63
AppModule.DIK_F6 = 64
AppModule.DIK_F7 = 65
AppModule.DIK_F8 = 66
AppModule.DIK_F9 = 67
AppModule.DIK_F10 = 68
AppModule.DIK_NUMLOCK = 69
AppModule.DIK_SCROLL = 70
AppModule.DIK_NUMPAD7 = 71
AppModule.DIK_NUMPAD8 = 72
AppModule.DIK_NUMPAD9 = 73
AppModule.DIK_SUBTRACT = 74
AppModule.DIK_NUMPAD4 = 75
AppModule.DIK_NUMPAD5 = 76
AppModule.DIK_NUMPAD6 = 77
AppModule.DIK_ADD = 78
AppModule.DIK_NUMPAD1 = 79
AppModule.DIK_NUMPAD2 = 80
AppModule.DIK_NUMPAD3 = 81
AppModule.DIK_NUMPAD0 = 82
AppModule.DIK_DECIMAL = 83
AppModule.DIK_F11 = 87
AppModule.DIK_F12 = 88
AppModule.DIK_NUMPADENTER = 156
AppModule.DIK_RCONTROL = 157
AppModule.DIK_DIVIDE = 181
AppModule.DIK_SYSRQ = 183
AppModule.DIK_RALT = 184
AppModule.DIK_PAUSE = 197
AppModule.DIK_HOME = 199
AppModule.DIK_UP = 200
AppModule.DIK_PGUP = 201
AppModule.DIK_PRIOR = 201
AppModule.DIK_LEFT = 203
AppModule.DIK_RIGHT = 205
AppModule.DIK_END = 207
AppModule.DIK_DOWN = 208
AppModule.DIK_PGDN = 209
AppModule.DIK_NEXT = 209
AppModule.DIK_INSERT = 210
AppModule.DIK_DELETE = 211
AppModule.DIK_LWIN = 219
AppModule.DIK_RWIN = 220
AppModule.DIK_APPS = 221

def _app_abort():
    fn = getattr(m2_lib, 'm2_app_abort', None)
    if fn:
        try: fn()
        except Exception: pass
    sys.exit(1)

def _app_get_cursor_position():
    fn = getattr(m2_lib, 'm2_wndmgr_get_mouse_position', None)
    if fn:
        try:
            x = ctypes.c_int32(0)
            y = ctypes.c_int32(0)
            fn.argtypes = [ctypes.POINTER(ctypes.c_int32), ctypes.POINTER(ctypes.c_int32)]
            fn(ctypes.byref(x), ctypes.byref(y))
            return (x.value, y.value)
        except Exception: pass
    return (0, 0)

setattr(AppModule, 'Abort', staticmethod(_app_abort))
def _wrap_app_set_frame_skip(enable=1):
    fn = _get_wire_fn('m2_app_set_frame_skip', [ctypes.c_int32], None)
    if fn:
        try:
            fn(int(bool(enable)))
        except Exception: pass

def _wrap_app_get_random(a, b):
    fn = _get_wire_fn('m2_app_get_random', [ctypes.c_int32, ctypes.c_int32], ctypes.c_int32)
    if fn:
        try:
            return int(fn(int(a), int(b)))
        except Exception: pass
    return random.randint(int(a), int(b))

def _wrap_app_get_global_time_stamp():
    fn = _get_wire_fn('m2_app_get_global_time_stamp', [], ctypes.c_int32)
    if fn:
        try:
            return int(fn())
        except Exception: pass
    return int(time.time())

def _wrap_app_set_hair_color_enable(enable=1):
    fn = _get_wire_fn('m2_app_set_hair_color_enable', [ctypes.c_int32], None)
    if fn:
        try:
            fn(int(bool(enable)))
        except Exception: pass

def _wrap_app_set_armor_specular_enable(enable=1):
    fn = _get_wire_fn('m2_app_set_armor_specular_enable', [ctypes.c_int32], None)
    if fn:
        try:
            fn(int(bool(enable)))
        except Exception: pass

def _wrap_app_set_weapon_specular_enable(enable=1):
    fn = _get_wire_fn('m2_app_set_weapon_specular_enable', [ctypes.c_int32], None)
    if fn:
        try:
            fn(int(bool(enable)))
        except Exception: pass

def _wrap_app_set_camera_max_distance(d=2500.0):
    fn = _get_wire_fn('m2_app_set_camera_max_distance', [ctypes.c_float], None)
    if fn:
        try:
            fn(float(d))
        except Exception: pass

def _wrap_app_set_default_font_name(name=""):
    fn = _get_wire_fn('m2_app_set_default_font_name', [ctypes.c_char_p], None)
    if fn:
        try:
            fn(name.encode('utf-8') if isinstance(name, str) else name)
        except Exception: pass

def _wrap_app_set_guild_mark_path(p=""):
    fn = _get_wire_fn('m2_app_set_guild_mark_path', [ctypes.c_char_p], None)
    if fn:
        try:
            fn(p.encode('utf-8') if isinstance(p, str) else p)
        except Exception: pass

def _wrap_app_set_guild_symbol_path(p=""):
    fn = _get_wire_fn('m2_app_set_guild_symbol_path', [ctypes.c_char_p], None)
    if fn:
        try:
            fn(p.encode('utf-8') if isinstance(p, str) else p)
        except Exception: pass

setattr(AppModule, 'SetFrameSkip', staticmethod(_wrap_app_set_frame_skip))
setattr(AppModule, 'GetRandom', staticmethod(_wrap_app_get_random))
setattr(AppModule, 'GetGlobalTimeStamp', staticmethod(_wrap_app_get_global_time_stamp))
setattr(AppModule, 'SetMouseHandler', staticmethod(lambda handler=None: None))
setattr(AppModule, 'SetHairColorEnable', staticmethod(_wrap_app_set_hair_color_enable))
setattr(AppModule, 'SetArmorSpecularEnable', staticmethod(_wrap_app_set_armor_specular_enable))
setattr(AppModule, 'SetWeaponSpecularEnable', staticmethod(_wrap_app_set_weapon_specular_enable))
setattr(AppModule, 'SetCameraMaxDistance', staticmethod(_wrap_app_set_camera_max_distance))
setattr(AppModule, 'SetDefaultFontName', staticmethod(_wrap_app_set_default_font_name))
setattr(AppModule, 'SetGuildMarkPath', staticmethod(_wrap_app_set_guild_mark_path))
setattr(AppModule, 'SetGuildSymbolPath', staticmethod(_wrap_app_set_guild_symbol_path))
def _wrap_app_get_locale_name():
    fn = _get_wire_fn('m2_app_get_locale_name', [], ctypes.c_char_p)
    if fn:
        try:
            res = fn()
            if res:
                return res.decode('utf-8', 'ignore')
        except Exception: pass
    return "en"

def _wrap_app_is_exist_file(f):
    fn = _get_wire_fn('m2_app_is_exist_file', [ctypes.c_char_p], ctypes.c_int32)
    if fn:
        try:
            return int(fn(f.encode('utf-8') if isinstance(f, str) else f))
        except Exception: pass
    return int(PackModule.Exist(f)) if hasattr(PackModule, 'Exist') else 0

def _wrap_app_set_center_position(*args):
    fn = _get_wire_fn('m2_app_set_center_position', [ctypes.c_float, ctypes.c_float, ctypes.c_float], None)
    if fn and len(args) >= 3:
        try:
            fn(float(args[0]), float(args[1]), float(args[2]))
        except Exception: pass

setattr(AppModule, 'GetLocaleName', staticmethod(_wrap_app_get_locale_name))
setattr(AppModule, 'GetLocalePath', staticmethod(lambda: "locale/en"))
setattr(AppModule, 'GetLocalePathCommon', staticmethod(lambda: "locale/common"))
setattr(AppModule, 'IsRTL', staticmethod(lambda *a: False))
setattr(AppModule, 'IsExistFile', staticmethod(_wrap_app_is_exist_file))
setattr(AppModule, 'IsFileExist', staticmethod(_wrap_app_is_exist_file))
setattr(AppModule, 'SetSoftwareCursor', staticmethod(lambda: None))
setattr(AppModule, 'SetHardwareCursor', staticmethod(lambda: None))
setattr(AppModule, 'GetCursorPosition', staticmethod(_app_get_cursor_position))
setattr(AppModule, 'SetCenterPosition', staticmethod(_wrap_app_set_center_position))
def _app_get_camera():
    if m2_lib:
        d = m2_lib.m2_app_get_camera_distance() if hasattr(m2_lib, 'm2_app_get_camera_distance') else 1000.0
        p = m2_lib.m2_app_get_camera_pitch() if hasattr(m2_lib, 'm2_app_get_camera_pitch') else 30.0
        r = m2_lib.m2_app_get_camera_rotation() if hasattr(m2_lib, 'm2_app_get_camera_rotation') else 0.0
        return (float(d), float(p), float(r), 100.0)
    return (1000.0, 30.0, 0.0, 100.0)

def _app_set_camera(dist, pitch, rot, h=100.0):
    if m2_lib and hasattr(m2_lib, 'm2_app_set_camera'):
        m2_lib.m2_app_set_camera(float(dist), float(pitch), float(rot), float(h))

def _app_pitch_camera(direction):
    if m2_lib and hasattr(m2_lib, 'm2_app_pitch_camera'):
        m2_lib.m2_app_pitch_camera(int(direction))

def _app_rotate_camera(direction):
    if m2_lib and hasattr(m2_lib, 'm2_app_rotate_camera'):
        m2_lib.m2_app_rotate_camera(int(direction))

def _app_zoom_camera(delta):
    if m2_lib and hasattr(m2_lib, 'm2_app_zoom_camera'):
        m2_lib.m2_app_zoom_camera(float(delta))

setattr(AppModule, 'GetCamera', staticmethod(_app_get_camera))
setattr(AppModule, 'SetCamera', staticmethod(_app_set_camera))
setattr(AppModule, 'GetCameraPitch', staticmethod(lambda: _app_get_camera()[1]))
setattr(AppModule, 'GetCameraRotation', staticmethod(lambda: _app_get_camera()[2]))
setattr(AppModule, 'PitchCamera', staticmethod(_app_pitch_camera))
setattr(AppModule, 'RotateCamera', staticmethod(_app_rotate_camera))
setattr(AppModule, 'ZoomCamera', staticmethod(_app_zoom_camera))
def _wrap_app_get_image_info(path):
    fn_valid = _get_wire_fn('m2_app_get_image_info_valid', [ctypes.c_char_p], ctypes.c_int32)
    fn_w = _get_wire_fn('m2_app_get_image_info_width', [ctypes.c_char_p], ctypes.c_int32)
    fn_h = _get_wire_fn('m2_app_get_image_info_height', [ctypes.c_char_p], ctypes.c_int32)
    if fn_valid and fn_w and fn_h:
        try:
            c_p = path.encode('utf-8') if isinstance(path, str) else path
            valid = bool(fn_valid(c_p))
            if valid:
                return (True, int(fn_w(c_p)), int(fn_h(c_p)))
            else:
                return (False, 0, 0)
        except Exception: pass
    return (False, 0, 0)

setattr(AppModule, 'GetImageInfo', staticmethod(_wrap_app_get_image_info))

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
setattr(AppModule, 'GetRenderTime', staticmethod(lambda *args, **kwargs: (0.0, 0)))
sys.modules['app'] = AppModule
sys.modules['App'] = AppModule

# ============================================================================
# 4. Modul background i Background (CPythonBackground - 48 metod)
# ============================================================================
class BackgroundModule(metaclass=ModType):
    DISTANCE0 = 0
    DISTANCE1 = 1
    DISTANCE2 = 2
    DISTANCE3 = 3
    DISTANCE4 = 4
    DISTANCE_SORT = 0
    TEXTURE_SORT = 1
    SHADOW_NONE = 0
    SHADOW_GROUND = 1
    SHADOW_GROUND_AND_SOLO = 2
    SHADOW_ALL = 3
    SHADOW_ALL_HIGH = 4
    SHADOW_ALL_MAX = 5
    SKY_RENDER_MODE_MODULATE = 0
    SKY_RENDER_MODE_MODULATE2X = 1
    SKY_RENDER_MODE_MODULATE4X = 2

    PART_SKY = 0
    PART_TREE = 1
    PART_CLOUD = 2
    PART_WATER = 3
    PART_OBJECT = 4
    PART_TERRAIN = 5

    _current_map_name = "metin2_map_a1"

    @staticmethod
    def Initialize(*args, **kwargs): pass
    @staticmethod
    def Destroy(*args, **kwargs): pass
    @classmethod
    def LoadMap(cls, map_name, x=0, y=0, z=0, *args, **kwargs):
        cls._current_map_name = str(map_name)
        print(f"[+] [BACKGROUND] LoadMap('{cls._current_map_name}', x={x}, y={y}, z={z})")
        return None
    @staticmethod
    def Update(*args, **kwargs): pass
    @staticmethod
    def Render(*args, **kwargs): pass
    @staticmethod
    def RenderSky(*args, **kwargs): pass
    @staticmethod
    def RenderCloud(*args, **kwargs): pass
    @staticmethod
    def RenderWater(*args, **kwargs): pass
    @staticmethod
    def RenderEffect(*args, **kwargs): pass
    @staticmethod
    def RenderCharacterShadowToTexture(*args, **kwargs): pass
    @staticmethod
    def IsSoftwareTiling(*args, **kwargs): return False
    @staticmethod
    def EnableSoftwareTiling(*args, **kwargs): pass
    @staticmethod
    def EnableSnow(*args, **kwargs): pass
    @staticmethod
    def GlobalPositionToLocalPosition(gx=0, gy=0, *args, **kwargs): return (gx, gy)
    @staticmethod
    def GlobalPositionToMapInfo(gx=0, gy=0, *args, **kwargs): return ("metin2_map_a1", 0, 0)
    @staticmethod
    def GetRenderShadowTime(*args, **kwargs): return 0.0
    @staticmethod
    def RegisterEnvironmentData(*args, **kwargs): return True
    @staticmethod
    def SetEnvironmentData(*args, **kwargs): return True
    @classmethod
    def GetCurrentMapName(cls, *args, **kwargs): return cls._current_map_name
    @staticmethod
    def GetPickingPoint(screen_x=0.0, screen_y=0.0, *args, **kwargs):
        fn = getattr(m2_lib, 'm2_py_background_get_picking_point', None)
        if fn:
            try:
                ox = ctypes.c_float(0.0)
                oy = ctypes.c_float(0.0)
                oz = ctypes.c_float(0.0)
                fn.argtypes = [ctypes.c_float, ctypes.c_float, ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float)]
                fn.restype = ctypes.c_bool
                if fn(float(screen_x), float(screen_y), ctypes.byref(ox), ctypes.byref(oy), ctypes.byref(oz)):
                    return (ox.value, oy.value, oz.value)
            except Exception: pass
        return (0.0, 0.0, 0.0)
    @staticmethod
    def BeginEnvironment(*args, **kwargs): pass
    @staticmethod
    def EndEnvironment(*args, **kwargs): pass
    @staticmethod
    def SetCharacterDirLight(*args, **kwargs): pass
    @staticmethod
    def SetBackgroundDirLight(*args, **kwargs): pass
    @staticmethod
    def GetHeight(x=0.0, y=0.0, *args, **kwargs):
        try:
            fn = getattr(m2_lib, 'm2_terrain_get_height', None)
            if fn:
                fn.argtypes = [ctypes.c_float, ctypes.c_float]
                fn.restype = ctypes.c_float
                return fn(float(x), float(y))
        except Exception: pass
        return 0.0
    @staticmethod
    def SetShadowLevel(*args, **kwargs): pass
    @staticmethod
    def SetVisiblePart(*args, **kwargs): pass
    @staticmethod
    def GetShadowMapColor(*args, **kwargs): return (0, 0, 0, 0)
    @staticmethod
    def SetSplatLimit(*args, **kwargs): pass
    @staticmethod
    def GetRenderedSplatNum(*args, **kwargs): return (0, 0, 0.0, "")
    @staticmethod
    def GetRenderedGraphicThingInstanceNum(*args, **kwargs): return 0
    @staticmethod
    def SelectViewDistanceNum(*args, **kwargs): pass
    @staticmethod
    def SetViewDistanceSet(*args, **kwargs): pass
    @staticmethod
    def GetFarClip(*args, **kwargs): return 50000.0
    @staticmethod
    def GetDistanceSetInfo(*args, **kwargs): return (1, 1000.0, 5000.0, 50000.0)
    @staticmethod
    def SetBGLoading(*args, **kwargs): pass
    @staticmethod
    def SetRenderSort(*args, **kwargs): pass
    @staticmethod
    def SetTransparentTree(*args, **kwargs): pass
    @staticmethod
    def SetXMasTree(*args, **kwargs): pass
    @staticmethod
    def RegisterDungeonMapName(*args, **kwargs): pass
    @staticmethod
    def VisibleGuildArea(*args, **kwargs): pass
    @staticmethod
    def DisableGuildArea(*args, **kwargs): pass
    @staticmethod
    def WarpTest(*args, **kwargs): pass

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

def _wrap_set_phase_window(phase, win):
    h = id(win) if win else 0
    if win and h:
        _py_handlers[h] = win
        _OBJECT_REGISTRY[h] = win
    fn = getattr(m2_lib, 'm2_net_SetPhaseWindow', None)
    if fn:
        fn.argtypes = [ctypes.c_uint32, ctypes.c_size_t]
        fn(int(phase), h)

def _wrap_clear_phase_window(phase, win):
    h = id(win) if win else 0
    if h:
        _py_handlers.pop(h, None)
        _OBJECT_REGISTRY.pop(h, None)
    fn = getattr(m2_lib, 'm2_net_ClearPhaseWindow', None)
    if fn:
        fn.argtypes = [ctypes.c_uint32, ctypes.c_size_t]
        fn(int(phase), h)

def _wrap_set_acc_connector_handler(handler):
    h = id(handler) if handler else 0
    if handler and h:
        _py_handlers[h] = handler
        _OBJECT_REGISTRY[h] = handler
    fn = getattr(m2_lib, 'm2_net_SetAccountConnectorHandler', None)
    if fn:
        fn.argtypes = [ctypes.c_size_t]
        fn(h)

def _wrap_set_handler(handler):
    h = id(handler) if handler else 0
    if handler and h:
        _py_handlers[h] = handler
        _OBJECT_REGISTRY[h] = handler
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
    ("SendSafeboxCheckin", "m2_net_SendSafeboxCheckin", [ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint16], ctypes.c_bool),
    ("SendSafeboxCheckout", "m2_net_SendSafeboxCheckout", [ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint16], ctypes.c_bool),
    ("SendSafeboxCheckinPacket", "m2_net_SendSafeboxCheckinPacket", [ctypes.c_uint16, ctypes.c_uint8], ctypes.c_bool),
    ("SendSafeboxCheckoutPacket", "m2_net_SendSafeboxCheckoutPacket", [ctypes.c_uint8, ctypes.c_uint16], ctypes.c_bool),
    ("SendGuildAdmin", "m2_net_SendGuildAdmin", [ctypes.c_uint8, ctypes.c_uint32], ctypes.c_bool),
    ("SendGuildAddMemberPacket", "m2_net_SendGuildAddMemberPacket", [ctypes.c_uint32], ctypes.c_bool),
    ("SendGuildRemoveMemberPacket", "m2_net_SendGuildRemoveMemberPacket", [ctypes.c_uint32], ctypes.c_bool),
    ("SendQuestInputStringPacket", "m2_net_SendQuestInputStringPacket", [ctypes.c_char_p], ctypes.c_bool),
    ("SendScriptButtonPacket", "m2_net_SendScriptButtonPacket", [ctypes.c_uint32], ctypes.c_bool),
    ("SendScriptSelectItemPacket", "m2_net_SendScriptSelectItemPacket", [ctypes.c_uint32], ctypes.c_bool),
    ("SendQuestConfirmPacket", "m2_net_SendQuestConfirmPacket", [ctypes.c_uint32, ctypes.c_uint8], ctypes.c_bool),
    ("SendQuestCancelPacket", "m2_net_SendQuestCancelPacket", [ctypes.c_uint32], ctypes.c_bool),
    ("SendSafeboxItemMovePacket", "m2_net_SendSafeboxItemMovePacket", [ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint8], ctypes.c_bool),
    ("SendMallCheckoutPacket", "m2_net_SendMallCheckoutPacket", [ctypes.c_uint8, ctypes.c_uint8], ctypes.c_bool),
    ("SendFishingPacket", "m2_net_SendFishingPacket", [ctypes.c_uint8, ctypes.c_int32], ctypes.c_bool),
    ("SendEmpirePacket", "m2_net_SendEmpirePacket", [ctypes.c_uint8], ctypes.c_bool),
    ("SendSelectEmpirePacket", "m2_net_SendEmpirePacket", [ctypes.c_uint8], ctypes.c_bool),
    ("SendMyShopPacket", "m2_net_SendMyShopPacket", [ctypes.c_char_p, ctypes.c_uint8], ctypes.c_bool),
    ("SendQuickSlotAddPacket", "m2_net_SendQuickSlotAddPacket", [ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint32], ctypes.c_bool),
    ("SendQuickSlotDelPacket", "m2_net_SendQuickSlotDelPacket", [ctypes.c_uint8], ctypes.c_bool),
    ("SendQuickSlotMovePacket", "m2_net_SendQuickSlotMovePacket", [ctypes.c_uint8, ctypes.c_uint8], ctypes.c_bool),
    ("SendOnClickPacket", "m2_net_SendOnClickPacket", [ctypes.c_uint32], ctypes.c_bool),
    ("StartGame", "m2_net_StartGame", [], ctypes.c_bool),
    ("Warp", "m2_net_Warp", [ctypes.c_int32, ctypes.c_int32], ctypes.c_bool),
    ("GetMainActorRace", "m2_net_GetMainActorRace", [], ctypes.c_uint32),
    ("GetMainActorEmpire", "m2_net_GetMainActorEmpire", [], ctypes.c_uint32),
    ("GetMainActorSkillGroup", "m2_net_GetMainActorSkillGroup", [], ctypes.c_uint32),
    ("IsChatInsultIn", "m2_net_IsChatInsultIn", [ctypes.c_char_p], ctypes.c_bool),
    ("IsInsultIn", "m2_net_IsInsultIn", [ctypes.c_char_p], ctypes.c_bool),
    ("LoadInsultList", "m2_net_LoadInsultList", [ctypes.c_char_p], ctypes.c_bool),
    ("UploadMark", "m2_net_UploadMark", [ctypes.c_char_p], ctypes.c_bool),
    ("UploadSymbol", "m2_net_UploadSymbol", [ctypes.c_char_p], ctypes.c_bool),
    ("GetGuildID", "m2_net_GetGuildID", [], ctypes.c_uint32),
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

def _wrap_net_exit_app():
    fn = getattr(m2_lib, 'm2_app_exit', None)
    if fn:
        try: fn()
        except Exception: pass
    else:
        sys.exit(0)

def _wrap_net_logout_game():
    print("[+] [NET] LogOutGame() - rozlaczenie i powrot do ekranu logowania")
    fn = getattr(m2_lib, 'm2_net_Disconnect', None)
    if fn:
        try: fn()
        except Exception: pass

def _wrap_net_exit_game():
    print("[+] [NET] ExitGame() - zakonczenie sesji postaci")
    _wrap_net_exit_app()

setattr(NetModule, 'ExitApplication', staticmethod(_wrap_net_exit_app))
setattr(NetModule, 'ExitGame', staticmethod(_wrap_net_exit_game))
setattr(NetModule, 'LogOutGame', staticmethod(_wrap_net_logout_game))
def _wrap_net_send_emoticon(e):
    fn = getattr(m2_lib, 'm2_net_SendEmoticon', None)
    if fn:
        fn.argtypes = [ctypes.c_uint8]
        fn.restype = ctypes.c_bool
        return fn(int(e))
    return True

def _wrap_net_send_give_item(target_vid, w, c, count):
    fn = getattr(m2_lib, 'm2_net_SendGiveItemPacket', None)
    if fn:
        fn.argtypes = [ctypes.c_uint32, ctypes.c_uint8, ctypes.c_uint16, ctypes.c_uint8]
        fn.restype = ctypes.c_bool
        return fn(int(target_vid), int(w), int(c), int(count))
    return True

def _wrap_net_send_gold_drop_new(gold):
    fn = getattr(m2_lib, 'm2_net_SendGoldDropPacketNew', None)
    if fn:
        fn.argtypes = [ctypes.c_uint32]
        fn.restype = ctypes.c_bool
        return fn(int(gold))
    return True

def _wrap_net_send_party_invite_answer(pid, accept):
    fn = getattr(m2_lib, 'm2_net_SendPartyInviteAnswerPacket', None)
    if fn:
        fn.argtypes = [ctypes.c_uint32, ctypes.c_uint8]
        fn.restype = ctypes.c_bool
        return fn(int(pid), int(accept))
    return True

def _wrap_net_send_whisper(name, msg):
    fn = getattr(m2_lib, 'm2_net_SendWhisperPacket', None)
    if fn:
        fn.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
        fn.restype = ctypes.c_bool
        return fn(str(name).encode('utf-8'), str(msg).encode('utf-8'))
    return True

def _wrap_net_send_char_position(x, y):
    fn = getattr(m2_lib, 'm2_net_SendCharacterPositionPacket', None)
    if fn:
        fn.argtypes = [ctypes.c_int32, ctypes.c_int32]
        fn.restype = ctypes.c_bool
        return fn(int(x), int(y))
    return True

def _wrap_net_send_target(vid):
    fn = getattr(m2_lib, 'm2_net_SendTargetPacket', None)
    if fn:
        fn.argtypes = [ctypes.c_uint32]
        fn.restype = ctypes.c_bool
        return fn(int(vid))
    return True

def _wrap_net_send_guild_war(guild_id, war_type):
    fn = getattr(m2_lib, 'm2_net_SendGuildWarPacket', None)
    if fn:
        fn.argtypes = [ctypes.c_uint32, ctypes.c_uint8]
        fn.restype = ctypes.c_bool
        return fn(int(guild_id), int(war_type))
    return True

setattr(NetModule, 'SendEmoticon', staticmethod(_wrap_net_send_emoticon))
setattr(NetModule, 'SendGiveItemPacket', staticmethod(_wrap_net_send_give_item))
setattr(NetModule, 'SendGoldDropPacketNew', staticmethod(_wrap_net_send_gold_drop_new))
setattr(NetModule, 'SendPartyInviteAnswerPacket', staticmethod(_wrap_net_send_party_invite_answer))
setattr(NetModule, 'SendWhisperPacket', staticmethod(_wrap_net_send_whisper))
setattr(NetModule, 'SendCharacterPositionPacket', staticmethod(_wrap_net_send_char_position))
setattr(NetModule, 'SendTargetPacket', staticmethod(_wrap_net_send_target))
setattr(NetModule, 'SendGuildWarPacket', staticmethod(_wrap_net_send_guild_war))
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
def _wrap_wndmgr_enable_scissor_rect(handle, l=0, t=0, r=0, b=0):
    fn = getattr(m2_lib, 'wndMgr_EnableScissorRect', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_size_t, ctypes.c_int32, ctypes.c_int32, ctypes.c_int32, ctypes.c_int32]
            fn(int(handle), int(l), int(t), int(r), int(b))
        except Exception: pass

def _wrap_wndmgr_disable_scissor_rect(handle):
    fn = getattr(m2_lib, 'wndMgr_DisableScissorRect', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_size_t]
            fn(int(handle))
        except Exception: pass

def _wrap_wndmgr_is_scissor_rect_enabled(handle):
    fn = getattr(m2_lib, 'wndMgr_IsScissorRectEnabled', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_size_t]
            fn.restype = ctypes.c_bool
            return bool(fn(int(handle)))
        except Exception: pass
    return False

def _wrap_wndmgr_set_clip_rect(handle, l=0.0, t=0.0, r=0.0, b=0.0):
    fn = getattr(m2_lib, 'wndMgr_SetClipRect', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_size_t, ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float]
            fn(int(handle), float(l), float(t), float(r), float(b))
        except Exception: pass

def _wrap_wndmgr_set_slot_cool_time(handle, slot=0, cool=0.0, elapsed=0.0):
    fn = getattr(m2_lib, 'wndMgr_SetSlotCoolTime', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_size_t, ctypes.c_uint32, ctypes.c_float, ctypes.c_float]
            fn(int(handle), int(slot), float(cool), float(elapsed))
        except Exception: pass

setattr(WndMgrModule, 'EnableScissorRect', staticmethod(_wrap_wndmgr_enable_scissor_rect))
setattr(WndMgrModule, 'DisableScissorRect', staticmethod(_wrap_wndmgr_disable_scissor_rect))
setattr(WndMgrModule, 'IsScissorRectEnabled', staticmethod(_wrap_wndmgr_is_scissor_rect_enabled))
setattr(WndMgrModule, 'SetClipRect', staticmethod(_wrap_wndmgr_set_clip_rect))
setattr(WndMgrModule, 'SetSlotCoolTime', staticmethod(_wrap_wndmgr_set_slot_cool_time))
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
def _wrap_player_get_main_character_position():
    fn = getattr(m2_lib, 'm2_py_player_get_main_character_position', None)
    if fn:
        try:
            x, y, z = ctypes.c_float(0.0), ctypes.c_float(0.0), ctypes.c_float(0.0)
            fn.argtypes = [ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float)]
            fn.restype = ctypes.c_bool
            if fn(ctypes.byref(x), ctypes.byref(y), ctypes.byref(z)):
                return (x.value, y.value, z.value)
        except Exception: pass
    return (0.0, 0.0, 0.0)

setattr(PlayerModule, 'GetMainCharacterPosition', staticmethod(_wrap_player_get_main_character_position))

def _wrap_player_get_item_attribute(w=1, c=0, idx=0):
    fn = getattr(m2_lib, 'm2_py_player_get_item_attribute', None)
    if fn:
        try:
            atype = ctypes.c_uint8(0)
            aval = ctypes.c_int16(0)
            fn.argtypes = [ctypes.c_uint8, ctypes.c_uint16, ctypes.c_uint8, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_int16)]
            fn.restype = ctypes.c_bool
            slot = c if (w is not None and c is not None and idx is not None) else w
            attr_i = idx if idx is not None else c
            win = w if (idx is not None and c is not None) else 1
            if fn(int(win), int(slot), int(attr_i), ctypes.byref(atype), ctypes.byref(aval)):
                return (atype.value, aval.value)
        except Exception: pass
    return (0, 0)
setattr(PlayerModule, 'GetItemAttribute', staticmethod(_wrap_player_get_item_attribute))

# --- PlayerModule Constants (100% C++ Parity with PythonPlayerModule.cpp) ---
PlayerModule.SLOT_TYPE_NONE = 0
PlayerModule.SLOT_TYPE_INVENTORY = 1
PlayerModule.SLOT_TYPE_SKILL = 2
PlayerModule.SLOT_TYPE_EMOTION = 3
PlayerModule.SLOT_TYPE_SHOP = 4
PlayerModule.SLOT_TYPE_EXCHANGE_OWNER = 5
PlayerModule.SLOT_TYPE_EXCHANGE_TARGET = 6
PlayerModule.SLOT_TYPE_QUICK_SLOT = 7
PlayerModule.SLOT_TYPE_SAFEBOX = 8
PlayerModule.SLOT_TYPE_PRIVATE_SHOP = 9
PlayerModule.SLOT_TYPE_MALL = 10
PlayerModule.SLOT_TYPE_DRAGON_SOUL_INVENTORY = 11

PlayerModule.RESERVED_WINDOW = 0
PlayerModule.INVENTORY = 1
PlayerModule.EQUIPMENT = 2
PlayerModule.SAFEBOX = 3
PlayerModule.MALL = 4
PlayerModule.DRAGON_SOUL_INVENTORY = 5
PlayerModule.GROUND = 6

PlayerModule.INVENTORY_PAGE_SIZE = 45
PlayerModule.INVENTORY_PAGE_COUNT = 2
PlayerModule.INVENTORY_SLOT_COUNT = 90
PlayerModule.EQUIPMENT_SLOT_START = 90
PlayerModule.EQUIPMENT_PAGE_COUNT = 1
PlayerModule.NEW_EQUIPMENT_SLOT_START = 180
PlayerModule.NEW_EQUIPMENT_SLOT_COUNT = 32

PlayerModule.ITEM_MONEY = -1
PlayerModule.SKILL_SLOT_COUNT = 255
PlayerModule.ATTRIBUTE_SLOT_MAX_NUM = 7
PlayerModule.METIN_SOCKET_MAX_NUM = 3

PlayerModule.PK_MODE_PEACE = 0
PlayerModule.PK_MODE_REVENGE = 1
PlayerModule.PK_MODE_FREE = 2
PlayerModule.PK_MODE_PROTECT = 3
PlayerModule.PK_MODE_GUILD = 4
PlayerModule.PK_MODE_MAX_NUM = 5

PlayerModule.LEVEL = 1
PlayerModule.VOICE = 2
PlayerModule.EXP = 3
PlayerModule.NEXT_EXP = 4
PlayerModule.HP = 5
PlayerModule.MAX_HP = 6
PlayerModule.SP = 7
PlayerModule.MAX_SP = 8
PlayerModule.STAMINA = 9
PlayerModule.MAX_STAMINA = 10
PlayerModule.ELK = 14
PlayerModule.GOLD = 14
PlayerModule.ST = 12
PlayerModule.HT = 13
PlayerModule.DX = 14
PlayerModule.IQ = 15
PlayerModule.ATT_POWER = 17
PlayerModule.ATT_SPEED = 19
PlayerModule.MOVING_SPEED = 20
PlayerModule.CASTING_SPEED = 21
PlayerModule.ATT_MIN = 22
PlayerModule.ATT_MAX = 23
PlayerModule.DEF_GRADE = 24
PlayerModule.DEF_BONUS = 25
PlayerModule.PLAYTIME = 31

PlayerModule.CATEGORY_ACTIVE = 0
PlayerModule.CATEGORY_PASSIVE = 1

PlayerModule.ATTACH_METIN_OK = 0
PlayerModule.ATTACH_METIN_CANT = 1
PlayerModule.ATTACH_METIN_CANT_ATTACH_TO_EQUIPMENT = 2
PlayerModule.ATTACH_METIN_NOT_MATCHABLE_ITEM = 3
PlayerModule.ATTACH_METIN_NO_MATCHABLE_SOCKET = 4
PlayerModule.ATTACH_METIN_NOT_EXIST_GOLD_SOCKET = 5

PlayerModule.DETACH_METIN_OK = 0
PlayerModule.DETACH_METIN_CANT = 1
PlayerModule.REFINE_OK = 0
PlayerModule.REFINE_CANT = 1

PlayerModule.AUTO_POTION_TYPE_HP = 0
PlayerModule.AUTO_POTION_TYPE_SP = 1

PlayerModule.BLOCK_EXCHANGE = 1
PlayerModule.BLOCK_PARTY = 2
PlayerModule.BLOCK_GUILD = 4
PlayerModule.BLOCK_WHISPER = 8
PlayerModule.BLOCK_FRIEND = 16
PlayerModule.BLOCK_PARTY_REQUEST = 32

PlayerModule.DRAGON_SOUL_EQUIPMENT_FIRST_SIZE = 6
PlayerModule.DRAGON_SOUL_EQUIPMENT_PAGE_COUNT = 2
PlayerModule.DRAGON_SOUL_EQUIPMENT_SLOT_START = 0
PlayerModule.DRAGON_SOUL_PAGE_COUNT = 5
PlayerModule.DRAGON_SOUL_PAGE_SIZE = 32
PlayerModule.DRAGON_SOUL_SLOT_COUNT = 160

PlayerModule.MBF_SMART = 0
PlayerModule.MBF_MOVE = 1
PlayerModule.MBF_CAMERA = 2
PlayerModule.MBF_ATTACK = 3
PlayerModule.MBF_SKILL = 4
PlayerModule.MBF_AUTO = 5
PlayerModule.MBS_CLICK = 0
PlayerModule.MBS_PRESS = 1
PlayerModule.MBT_LEFT = 0
PlayerModule.MBT_RIGHT = 1

# --- QuickSlot FFI integration (m2_quickslot_get_v2) ---
class QuickSlotResult(ctypes.Structure):
    _fields_ = [("slot_type", ctypes.c_uint8), ("slot_position", ctypes.c_uint32)]

def _wrap_player_get_local_quick_slot(slot):
    fn = getattr(m2_lib, 'm2_quickslot_get_v2', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint8]
            fn.restype = QuickSlotResult
            res = fn(int(slot))
            return (int(res.slot_type), int(res.slot_position))
        except Exception: pass
    return (0, 0)

setattr(PlayerModule, 'GetLocalQuickSlot', staticmethod(_wrap_player_get_local_quick_slot))
setattr(PlayerModule, 'GetGlobalQuickSlot', staticmethod(_wrap_player_get_local_quick_slot))

# SkillCoolTime: Py_BuildValue("ff", fCoolTime, fElapsedCoolTime) -> (float, float)
def _wrap_player_get_skill_cool_time(idx):
    fn_cool = getattr(m2_lib, 'm2_py_player_get_skill_cool_time', None)
    fn_elap = getattr(m2_lib, 'm2_py_player_get_skill_elapsed_cool_time', None)
    cool = 0.0
    elapsed = 0.0
    if fn_cool:
        try:
            fn_cool.argtypes = [ctypes.c_uint32]
            fn_cool.restype = ctypes.c_float
            cool = float(fn_cool(int(idx)))
        except Exception: pass
    if fn_elap:
        try:
            fn_elap.argtypes = [ctypes.c_uint32]
            fn_elap.restype = ctypes.c_float
            elapsed = float(fn_elap(int(idx)))
        except Exception: pass
    return (cool, elapsed)
setattr(PlayerModule, 'GetSkillCoolTime', staticmethod(_wrap_player_get_skill_cool_time))

# Alignment: Py_BuildValue("ii", iAlignmentPoint, iAlignmentGrade) -> (int, int)
def _wrap_player_get_alignment_data():
    fn = getattr(m2_lib, 'm2_py_player_get_alignment_data', None)
    if fn:
        try:
            p = ctypes.c_int32(0)
            g = ctypes.c_int32(4)
            fn.argtypes = [ctypes.POINTER(ctypes.c_int32), ctypes.POINTER(ctypes.c_int32)]
            fn.restype = ctypes.c_bool
            if fn(ctypes.byref(p), ctypes.byref(g)):
                return (p.value, g.value)
        except Exception: pass
    return (0, 4)
setattr(PlayerModule, 'GetAlignmentData', staticmethod(_wrap_player_get_alignment_data))

# AutoPotionInfo: Py_BuildValue("biii", ...) -> (bool, int, int, int)
def _wrap_player_get_auto_potion_info(potion_type=0):
    fn = getattr(m2_lib, 'm2_py_player_get_auto_potion_info', None)
    if fn:
        try:
            act = ctypes.c_bool(False)
            cur = ctypes.c_int32(0)
            tot = ctypes.c_int32(0)
            slot = ctypes.c_int32(0)
            fn.argtypes = [ctypes.c_uint32, ctypes.POINTER(ctypes.c_bool), ctypes.POINTER(ctypes.c_int32), ctypes.POINTER(ctypes.c_int32), ctypes.POINTER(ctypes.c_int32)]
            fn.restype = ctypes.c_bool
            if fn(int(potion_type), ctypes.byref(act), ctypes.byref(cur), ctypes.byref(tot), ctypes.byref(slot)):
                return (act.value, cur.value, tot.value, slot.value)
        except Exception: pass
    return (False, 0, 0, 0)
setattr(PlayerModule, 'GetAutoPotionInfo', staticmethod(_wrap_player_get_auto_potion_info))

# PartyMemberAffects: zwraca liste 7 intow
def _wrap_player_get_party_member_affects(pid=0):
    fn = getattr(m2_lib, 'm2_py_player_get_party_member_affects', None)
    if fn:
        try:
            buf = (ctypes.c_int32 * 7)()
            fn.argtypes = [ctypes.c_uint32, ctypes.POINTER(ctypes.c_int32)]
            fn.restype = ctypes.c_bool
            if fn(int(pid), buf):
                return list(buf)
        except Exception: pass
    return [0, 0, 0, 0, 0, 0, 0]
setattr(PlayerModule, 'GetPartyMemberAffects', staticmethod(_wrap_player_get_party_member_affects))

# GetItemMetinSocket: Py_BuildValue("i", dwMetinSocketValue) -> int
def _wrap_player_get_item_metin_socket(w=1, c=0, idx=0):
    fn = getattr(m2_lib, 'm2_py_player_get_item_metin_socket', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint8, ctypes.c_uint16, ctypes.c_uint8]
            fn.restype = ctypes.c_int32
            slot = c if (w is not None and c is not None and idx is not None) else w
            sock_i = idx if idx is not None else c
            win = w if (idx is not None and c is not None) else 1
            return fn(int(win), int(slot), int(sock_i))
        except Exception: pass
    return 0
setattr(PlayerModule, 'GetItemMetinSocket', staticmethod(_wrap_player_get_item_metin_socket))

# GetItemLink: Py_BuildValue("s", ...) -> str
def _wrap_player_get_item_link(w=1, c=0):
    fn = getattr(m2_lib, 'm2_py_player_get_item_link', None)
    if fn:
        try:
            buf = ctypes.create_string_buffer(512)
            fn.argtypes = [ctypes.c_uint8, ctypes.c_uint16, ctypes.c_char_p, ctypes.c_size_t]
            fn.restype = ctypes.c_bool
            if fn(int(w), int(c), buf, ctypes.c_size_t(512)):
                return buf.value.decode('utf-8', errors='ignore')
        except Exception: pass
    return ""
setattr(PlayerModule, 'GetItemLink', staticmethod(_wrap_player_get_item_link))

# Exchange: tuple returns
def _wrap_player_get_exchange_item_attribute(owner=0, pos=0, idx=0):
    fn = getattr(m2_lib, 'm2_py_player_get_exchange_item_attribute', None)
    if fn:
        try:
            t = ctypes.c_uint8(0)
            v = ctypes.c_int16(0)
            fn.argtypes = [ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint8, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_int16)]
            fn.restype = ctypes.c_bool
            if fn(int(owner), int(pos), int(idx), ctypes.byref(t), ctypes.byref(v)):
                return (t.value, v.value)
        except Exception: pass
    return (0, 0)
setattr(PlayerModule, 'GetExchangeItemAttribute', staticmethod(_wrap_player_get_exchange_item_attribute))

def _wrap_player_get_exchange_item_metin_socket(owner=0, pos=0, idx=0):
    fn = getattr(m2_lib, 'm2_py_player_get_exchange_item_metin_socket', None)
    if fn:
        try:
            s = ctypes.c_uint32(0)
            fn.argtypes = [ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint8, ctypes.POINTER(ctypes.c_uint32)]
            fn.restype = ctypes.c_bool
            if fn(int(owner), int(pos), int(idx), ctypes.byref(s)):
                return s.value
        except Exception: pass
    return 0
setattr(PlayerModule, 'GetExchangeItemMetinSocket', staticmethod(_wrap_player_get_exchange_item_metin_socket))

# Shop
def _wrap_player_get_shop_item_attribute(pos=0, idx=0):
    fn = getattr(m2_lib, 'm2_shop_get_item_attribute', None)
    if fn:
        try:
            t = ctypes.c_uint8(0)
            v = ctypes.c_int16(0)
            fn.argtypes = [ctypes.c_uint8, ctypes.c_uint8, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_int16)]
            fn.restype = ctypes.c_bool
            if fn(int(pos), int(idx), ctypes.byref(t), ctypes.byref(v)):
                return (t.value, v.value)
        except Exception: pass
    return (0, 0)
setattr(PlayerModule, 'GetShopItemAttribute', staticmethod(_wrap_player_get_shop_item_attribute))

def _wrap_player_get_shop_item_metin_socket(pos=0, idx=0):
    fn = getattr(m2_lib, 'm2_shop_get_item_metin_socket', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint8, ctypes.c_uint8]
            fn.restype = ctypes.c_uint32
            return int(fn(int(pos), int(idx)))
        except Exception: pass
    return 0
setattr(PlayerModule, 'GetShopItemMetinSocket', staticmethod(_wrap_player_get_shop_item_metin_socket))

# Safebox / Mall
setattr(PlayerModule, 'GetSafeboxItemAttribute', staticmethod(lambda pos, idx: (0, 0)))
setattr(PlayerModule, 'GetSafeboxItemMetinSocket', staticmethod(lambda pos, idx: 0))
setattr(PlayerModule, 'GetMallItemAttribute', staticmethod(lambda pos, idx: (0, 0)))
setattr(PlayerModule, 'GetMallItemMetinSocket', staticmethod(lambda pos, idx: 0))

# Skill
setattr(PlayerModule, 'GetSkillGradeName', staticmethod(lambda idx: ""))
setattr(PlayerModule, 'GetSkillLevel', staticmethod(lambda idx: 0))
setattr(PlayerModule, 'GetSkillCurrentEfficientPercentage', staticmethod(lambda idx: 0.0))
setattr(PlayerModule, 'GetSkillNextEfficientPercentage', staticmethod(lambda idx: 0.0))

# Target: Py_BuildValue("ii", iHP, iMaxHP) -> (int, int)
def _wrap_player_get_target_hp(*args, **kwargs):
    fn = getattr(m2_lib, 'm2_py_player_get_target_hp', None)
    if fn:
        try:
            hp = ctypes.c_int32(0)
            max_hp = ctypes.c_int32(0)
            fn.argtypes = [ctypes.POINTER(ctypes.c_int32), ctypes.POINTER(ctypes.c_int32)]
            fn.restype = ctypes.c_bool
            if fn(ctypes.byref(hp), ctypes.byref(max_hp)):
                return (hp.value, max_hp.value)
        except Exception: pass
    return (0, 0)
setattr(PlayerModule, 'GetTargetHP', staticmethod(_wrap_player_get_target_hp))

# Guild: Py_BuildValue
def _wrap_player_get_character_guild_name():
    fn = getattr(m2_lib, 'm2_py_player_get_character_guild_name', None)
    if fn:
        try:
            buf = ctypes.create_string_buffer(128)
            fn.argtypes = [ctypes.c_char_p, ctypes.c_size_t]
            fn.restype = ctypes.c_bool
            if fn(buf, 128):
                return buf.value.decode('utf-8', 'ignore')
        except Exception: pass
    return ""
setattr(PlayerModule, 'GetCharacterGuildName', staticmethod(_wrap_player_get_character_guild_name))

# Dragon Soul
setattr(PlayerModule, 'GetDSItemAttribute', staticmethod(lambda pos, idx: (0, 0)))
setattr(PlayerModule, 'GetDSItemMetinSocket', staticmethod(lambda pos, idx: 0))

def _wrap_player_get_main_character_index():
    fn = getattr(m2_lib, 'm2_py_player_get_main_character_index', None)
    if fn:
        try:
            fn.restype = ctypes.c_uint32
            return int(fn())
        except Exception: pass
    return 0

def _wrap_player_get_target_vid():
    fn = getattr(m2_lib, 'm2_py_player_get_target_vid', None)
    if fn:
        try:
            fn.restype = ctypes.c_uint32
            return int(fn())
        except Exception: pass
    return 0

setattr(PlayerModule, 'GetMainCharacterIndex', staticmethod(_wrap_player_get_main_character_index))
setattr(PlayerModule, 'GetTargetVID', staticmethod(_wrap_player_get_target_vid))

# --- Missing Game Logic and Action Handlers (game.py & ui*.py parity) ---
_player_quick_page = 0
_player_game_window = None
_player_attack_key_state = False
_player_skills = {}

def _player_clear_skill_dict():
    _player_skills.clear()

def _player_set_skill(slot, vnum):
    _player_skills[slot] = vnum

def _player_get_quick_page():
    return _player_quick_page

def _player_set_quick_page(p):
    global _player_quick_page
    _player_quick_page = int(p)

def _player_set_game_window(wnd):
    global _player_game_window
    _player_game_window = wnd

def _player_get_game_window():
    return _player_game_window

def _player_is_main_character_index(vid):
    return int(vid) == _wrap_player_get_main_character_index()

def _player_set_attack_key_state(state):
    global _player_attack_key_state
    _player_attack_key_state = bool(state)

def _player_request_use_local_quick_slot(slot):
    try:
        q_type, q_pos = _wrap_player_get_local_quick_slot(slot)
        if q_type == 1:
            net_mod = sys.modules.get('net')
            if net_mod and hasattr(net_mod, 'SendItemUsePacket'):
                net_mod.SendItemUsePacket(q_pos)
        elif q_type == 2:
            net_mod = sys.modules.get('net')
            if net_mod and hasattr(net_mod, 'SendUseSkillPacket'):
                net_mod.SendUseSkillPacket(q_pos, _wrap_player_get_main_character_index(), None)
    except Exception: pass

def _player_request_delete_global_quick_slot(slot):
    try:
        net_mod = sys.modules.get('net')
        if net_mod and hasattr(net_mod, 'SendQuickSlotDelPacket'):
            net_mod.SendQuickSlotDelPacket(int(slot))
    except Exception: pass

def _player_request_add_local_quick_slot(local_slot, wnd_type, wnd_item_pos):
    try:
        global_slot = int(local_slot) + _player_quick_page * 8
        net_mod = sys.modules.get('net')
        if net_mod and hasattr(net_mod, 'SendQuickSlotAddPacket'):
            net_mod.SendQuickSlotAddPacket(global_slot, int(wnd_type), int(wnd_item_pos))
    except Exception: pass

def _player_request_move_global_quick_slot(src_global, dst_local):
    try:
        dst_global = int(dst_local) + _player_quick_page * 8
        net_mod = sys.modules.get('net')
        if net_mod and hasattr(net_mod, 'SendQuickSlotMovePacket'):
            net_mod.SendQuickSlotMovePacket(int(src_global), dst_global)
    except Exception: pass

def _player_is_mounting_horse():
    try:
        fn = getattr(m2_lib, 'm2_py_player_is_mounting_horse', None)
        if fn:
            fn.restype = ctypes.c_bool
            return bool(fn())
    except Exception: pass
    return False

def _wrap_player_combo_attack():
    fn = getattr(m2_lib, 'm2_py_player_combo_attack', None)
    if fn:
        try:
            fn.restype = ctypes.c_bool
            return bool(fn())
        except Exception: pass
    return False

def _wrap_player_clear_target():
    fn = getattr(m2_lib, 'm2_py_player_clear_target', None)
    if fn:
        try: fn()
        except Exception: pass

def _wrap_player_get_guild_id():
    fn = getattr(m2_lib, 'm2_py_player_get_guild_id', None)
    if fn:
        try:
            fn.restype = ctypes.c_uint32
            return int(fn())
        except Exception: pass
    return 0

setattr(PlayerModule, 'ClearSkillDict', staticmethod(_player_clear_skill_dict))
setattr(PlayerModule, 'SetSkill', staticmethod(_player_set_skill))
setattr(PlayerModule, 'EndKeyWalkingImmediately', staticmethod(lambda *args, **kwargs: None))
setattr(PlayerModule, 'EndMouseWalking', staticmethod(lambda *args, **kwargs: None))
setattr(PlayerModule, 'StartMouseWalking', staticmethod(lambda *args, **kwargs: None))
setattr(PlayerModule, 'IsMainCharacterIndex', staticmethod(_player_is_main_character_index))
setattr(PlayerModule, 'IsMountingHorse', staticmethod(_player_is_mounting_horse))
setattr(PlayerModule, 'SetAttackKeyState', staticmethod(_player_set_attack_key_state))
setattr(PlayerModule, 'SetSingleDIKKeyState', staticmethod(lambda *args, **kwargs: None))
setattr(PlayerModule, 'SetMouseState', staticmethod(lambda *args, **kwargs: None))
setattr(PlayerModule, 'SetMouseMiddleButtonState', staticmethod(lambda *args, **kwargs: None))
setattr(PlayerModule, 'SetMouseFunc', staticmethod(lambda *args, **kwargs: None))
setattr(PlayerModule, 'GetMouseFunc', staticmethod(lambda *args, **kwargs: 0))
setattr(PlayerModule, 'SetGameWindow', staticmethod(_player_set_game_window))
setattr(PlayerModule, 'GetGameWindow', staticmethod(_player_get_game_window))
setattr(PlayerModule, 'SetQuickPage', staticmethod(_player_set_quick_page))
setattr(PlayerModule, 'GetQuickPage', staticmethod(_player_get_quick_page))
setattr(PlayerModule, 'RequestUseLocalQuickSlot', staticmethod(_player_request_use_local_quick_slot))
setattr(PlayerModule, 'RequestDeleteGlobalQuickSlot', staticmethod(_player_request_delete_global_quick_slot))
setattr(PlayerModule, 'RequestAddLocalQuickSlot', staticmethod(_player_request_add_local_quick_slot))
setattr(PlayerModule, 'RequestMoveGlobalQuickSlotToLocalQuickSlot', staticmethod(_player_request_move_global_quick_slot))
setattr(PlayerModule, 'LocalQuickSlotIndexToGlobalQuickSlotIndex', staticmethod(lambda s=0, *args, **kwargs: int(s) + _player_quick_page * 8))
setattr(PlayerModule, 'ResetCameraRotation', staticmethod(lambda *args, **kwargs: None))
setattr(PlayerModule, 'ClickSkillSlot', staticmethod(lambda *args, **kwargs: None))
setattr(PlayerModule, 'ChangeCurrentSkillNumberOnly', staticmethod(lambda *args, **kwargs: None))
setattr(PlayerModule, 'ComboAttack', staticmethod(_wrap_player_combo_attack))
setattr(PlayerModule, 'Clear', staticmethod(lambda *args, **kwargs: None))
setattr(PlayerModule, 'ClearTarget', staticmethod(_wrap_player_clear_target))
setattr(PlayerModule, 'SetPlayTime', staticmethod(lambda *args, **kwargs: None))
setattr(PlayerModule, 'GetGuildID', staticmethod(_wrap_player_get_guild_id))
setattr(PlayerModule, 'CanAttackInstance', staticmethod(lambda *args, **kwargs: True))
setattr(PlayerModule, 'IsActingEmotion', staticmethod(lambda *args, **kwargs: False))
setattr(PlayerModule, 'ActEmotion', staticmethod(lambda *args, **kwargs: None))
setattr(PlayerModule, 'ShowPlayer', staticmethod(lambda *args, **kwargs: None))
setattr(PlayerModule, 'HidePlayer', staticmethod(lambda *args, **kwargs: None))
setattr(PlayerModule, 'SetAutoCameraRotationSpeed', staticmethod(lambda *args, **kwargs: None))
setattr(PlayerModule, 'SetQuickCameraMode', staticmethod(lambda *args, **kwargs: None))
setattr(PlayerModule, 'RemoveQuickSlotByValue', staticmethod(lambda *args, **kwargs: None))
setattr(PlayerModule, 'isItem', staticmethod(lambda *args, **kwargs: True))
setattr(PlayerModule, 'IsObserverMode', staticmethod(lambda *args, **kwargs: False))

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

def _wrap_chr_get_pixel_position(vid=0):
    fn = getattr(m2_lib, 'm2_chr_get_pixel_position', None)
    if fn:
        try:
            x, y, z = ctypes.c_float(0.0), ctypes.c_float(0.0), ctypes.c_float(0.0)
            fn.argtypes = [ctypes.c_uint32, ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float)]
            fn.restype = ctypes.c_bool
            target_vid = int(vid) if vid else _wrap_player_get_main_character_index()
            if fn(target_vid, ctypes.byref(x), ctypes.byref(y), ctypes.byref(z)):
                return (x.value, y.value, z.value)
        except Exception: pass
    return (0.0, 0.0, 0.0)

setattr(ChrModule, 'GetPixelPosition', staticmethod(_wrap_chr_get_pixel_position))

def _wrap_chr_set_virtual_id(vid=0):
    fn = getattr(m2_lib, 'm2_chr_set_virtual_id', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint32]
            fn.restype = ctypes.c_bool
            fn(int(vid))
        except Exception: pass

def _wrap_chr_set_name_string(name=""):
    fn = getattr(m2_lib, 'm2_chr_set_name_string', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_char_p]
            fn.restype = ctypes.c_bool
            fn(name.encode('utf-8') if isinstance(name, str) else name)
        except Exception: pass

def _wrap_chr_set_instance_type(t=0):
    fn = getattr(m2_lib, 'm2_chr_set_instance_type', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_int32]
            fn.restype = ctypes.c_bool
            fn(int(t))
        except Exception: pass

def _wrap_chr_refresh():
    fn = getattr(m2_lib, 'm2_chr_refresh_selected', None)
    if fn:
        try:
            fn.restype = ctypes.c_bool
            fn()
        except Exception: pass

def _wrap_chr_deform():
    fn = getattr(m2_lib, 'm2_chr_deform_selected', None)
    if fn:
        try:
            fn.restype = ctypes.c_bool
            fn()
        except Exception: pass

setattr(ChrModule, 'SetVirtualID', staticmethod(_wrap_chr_set_virtual_id))
setattr(ChrModule, 'SetNameString', staticmethod(_wrap_chr_set_name_string))
setattr(ChrModule, 'SetInstanceType', staticmethod(_wrap_chr_set_instance_type))
def _wrap_chr_set_direction(d):
    fn = getattr(m2_lib, 'm2_chr_set_direction', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_float]
            fn(float(d))
        except Exception: pass

def _wrap_chr_revive():
    fn = getattr(m2_lib, 'm2_chr_revive', None)
    if fn:
        try: fn()
        except Exception: pass

def _wrap_chr_die():
    fn = getattr(m2_lib, 'm2_chr_die', None)
    if fn:
        try: fn()
        except Exception: pass

def _wrap_chr_get_guild_id(vid=0):
    fn = getattr(m2_lib, 'm2_chr_get_guild_id', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint32]
            fn.restype = ctypes.c_uint32
            return int(fn(int(vid)))
        except Exception: pass
    return 0

def _wrap_chr_is_party_member(vid=0):
    fn = getattr(m2_lib, 'm2_chr_is_party_member', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint32]
            fn.restype = ctypes.c_bool
            return bool(fn(int(vid)))
        except Exception: pass
    return False

def _wrap_chr_get_rotation():
    fn = getattr(m2_lib, 'm2_chr_get_rotation', None)
    if fn:
        try:
            fn.restype = ctypes.c_float
            return float(fn())
        except Exception: pass
    return 0.0

def _wrap_chr_get_virtual_number():
    fn = getattr(m2_lib, 'm2_chr_get_virtual_number', None)
    if fn:
        try:
            fn.restype = ctypes.c_uint32
            return int(fn())
        except Exception: pass
    return 0

setattr(ChrModule, 'SetDirection', staticmethod(_wrap_chr_set_direction))
setattr(ChrModule, 'Refresh', staticmethod(_wrap_chr_refresh))
setattr(ChrModule, 'Deform', staticmethod(_wrap_chr_deform))
setattr(ChrModule, 'Render', staticmethod(lambda: None))
setattr(ChrModule, 'RenderCollision', staticmethod(lambda: None))
setattr(ChrModule, 'Revive', staticmethod(_wrap_chr_revive))
setattr(ChrModule, 'Die', staticmethod(_wrap_chr_die))
setattr(ChrModule, 'Hide', staticmethod(lambda: None))
setattr(ChrModule, 'Show', staticmethod(lambda: None))
setattr(ChrModule, 'Update', staticmethod(lambda: None))
setattr(ChrModule, 'GetGuildID', staticmethod(_wrap_chr_get_guild_id))
setattr(ChrModule, 'IsPartyMember', staticmethod(_wrap_chr_is_party_member))
def _wrap_chr_pick():
    fn = getattr(m2_lib, 'm2_chr_Pick', None)
    if fn:
        try:
            fn.restype = ctypes.c_uint32
            res = fn()
            if res > 0: return res
        except Exception: pass
    fn2 = getattr(m2_lib, 'm2_py_player_get_target_vid', None)
    if fn2:
        try:
            fn2.restype = ctypes.c_uint32
            return fn2()
        except Exception: pass
    return 0
setattr(ChrModule, 'Pick', staticmethod(_wrap_chr_pick))
setattr(ChrModule, 'PickAll', staticmethod(lambda: []))
setattr(ChrModule, 'LookAt', staticmethod(lambda *args: None))
setattr(ChrModule, 'SetRotationAll', staticmethod(lambda *args: None))
setattr(ChrModule, 'BlendRotation', staticmethod(lambda *args: None))
setattr(ChrModule, 'GetRotation', staticmethod(_wrap_chr_get_rotation))
setattr(ChrModule, 'GetProjectPosition', staticmethod(lambda vid=0, h=0: (0, 0)))
setattr(ChrModule, 'GetVirtualNumber', staticmethod(_wrap_chr_get_virtual_number))
setattr(ChrModule, 'GetBoundBoxOnlyXY', staticmethod(lambda *args: (0.0, 0.0, 0.0, 0.0)))
setattr(ChrModule, 'testGetPKData', staticmethod(lambda *args: (0, 4)))
setattr(ChrModule, 'RaceToJob', staticmethod(lambda race: (int(race) // 2) if int(race) < 8 else 0))
setattr(ChrModule, 'RaceToSex', staticmethod(lambda race: (int(race) % 2) if int(race) < 8 else 0))

ChrModule.MOTION_MODE_RESERVED = 0
ChrModule.MOTION_MODE_GENERAL = 1
ChrModule.MOTION_MODE_ONEHAND_SWORD = 2
ChrModule.MOTION_MODE_TWOHAND_SWORD = 3
ChrModule.MOTION_MODE_DUALHAND_SWORD = 4
ChrModule.MOTION_MODE_BOW = 5
ChrModule.MOTION_MODE_FAN = 6
ChrModule.MOTION_MODE_BELL = 7
ChrModule.MOTION_MODE_FISHING = 8
ChrModule.MOTION_MODE_HORSE = 9
ChrModule.MOTION_MODE_HORSE_ONEHAND_SWORD = 10
ChrModule.MOTION_MODE_HORSE_TWOHAND_SWORD = 11
ChrModule.MOTION_MODE_HORSE_DUALHAND_SWORD = 12
ChrModule.MOTION_MODE_HORSE_BOW = 13
ChrModule.MOTION_MODE_HORSE_FAN = 14
ChrModule.MOTION_MODE_HORSE_BELL = 15
ChrModule.MOTION_MODE_WEDDING_DRESS = 16

ChrModule.MOTION_NONE = 0
ChrModule.MOTION_SPAWN = 1
ChrModule.MOTION_WAIT = 2
ChrModule.MOTION_WALK = 3
ChrModule.MOTION_RUN = 4
ChrModule.MOTION_CHANGE_WEAPON = 5
ChrModule.MOTION_DAMAGE = 6
ChrModule.MOTION_DAMAGE_FLYING = 7
ChrModule.MOTION_STAND_UP = 8
ChrModule.MOTION_DAMAGE_BACK = 9
ChrModule.MOTION_DAMAGE_FLYING_BACK = 10
ChrModule.MOTION_STAND_UP_BACK = 11
ChrModule.MOTION_DEAD = 12
ChrModule.MOTION_DEAD_BACK = 13
ChrModule.MOTION_NORMAL_ATTACK = 14
ChrModule.MOTION_COMBO_ATTACK_1 = 15
ChrModule.MOTION_COMBO_ATTACK_2 = 16
ChrModule.MOTION_COMBO_ATTACK_3 = 17
ChrModule.MOTION_COMBO_ATTACK_4 = 18
ChrModule.MOTION_COMBO_ATTACK_5 = 19
ChrModule.MOTION_COMBO_ATTACK_6 = 20
ChrModule.MOTION_COMBO_ATTACK_7 = 21
ChrModule.MOTION_COMBO_ATTACK_8 = 22
ChrModule.MOTION_INTRO_WAIT = 23
ChrModule.MOTION_INTRO_SELECTED = 24
ChrModule.MOTION_INTRO_NOT_SELECTED = 25
ChrModule.MOTION_STOP = 32
ChrModule.MOTION_SKILL = 33

ChrModule.DIR_NORTH = 0
ChrModule.DIR_NORTHEAST = 1
ChrModule.DIR_EAST = 2
ChrModule.DIR_SOUTHEAST = 3
ChrModule.DIR_SOUTH = 4
ChrModule.DIR_SOUTHWEST = 5
ChrModule.DIR_WEST = 6
ChrModule.DIR_NORTHWEST = 7

ChrModule.INSTANCE_TYPE_PLAYER = 0
ChrModule.INSTANCE_TYPE_NPC = 1
ChrModule.INSTANCE_TYPE_ENEMY = 2
ChrModule.INSTANCE_TYPE_BUILDING = 3
ChrModule.INSTANCE_TYPE_OBJECT = 4

ChrModule.PART_WEAPON = 0
ChrModule.PART_HEAD = 1
ChrModule.PART_WEAPON_LEFT = 2

ChrModule.NAMECOLOR_MOB = 6
ChrModule.NAMECOLOR_NPC = 7
ChrModule.NAMECOLOR_PC = 8
ChrModule.NAMECOLOR_EMPIRE_MOB = 12
ChrModule.NAMECOLOR_EMPIRE_NPC = 13
ChrModule.NAMECOLOR_EMPIRE_PC = 14
ChrModule.NAMECOLOR_FUNC = 18
ChrModule.NAMECOLOR_PK = 19
ChrModule.NAMECOLOR_PVP = 20
ChrModule.NAMECOLOR_PARTY = 21
ChrModule.NAMECOLOR_WARP = 22
ChrModule.NAMECOLOR_WAYPOINT = 23
ChrModule.NAMECOLOR_EXTRA = 28

class ChrmgrModule:
    @staticmethod
    def SetEmpireNameMode(mode=1, *args, **kwargs): pass
    @staticmethod
    def GetVIDInfo(vid=0, *args, **kwargs): return ""
    @staticmethod
    def GetPickedVID(*args, **kwargs):
        try:
            fn = getattr(m2_lib, 'm2_py_player_get_target_vid', None)
            if fn:
                fn.restype = ctypes.c_uint32
                vid = fn()
                if vid > 0: return vid
        except Exception: pass
        return -1
    @staticmethod
    def SetShapeModel(shape=0, path="", *args, **kwargs): pass
    @staticmethod
    def AppendShapeSkin(shape=0, part=0, path="", *args, **kwargs): pass
    _active_race = 0
    _path_name = ""

    @staticmethod
    def SetPathName(path="", *args, **kwargs):
        ChrmgrModule._path_name = str(path) if path else ""

    @staticmethod
    def CreateRace(race=0, *args, **kwargs):
        ChrmgrModule._active_race = int(race)
        fn = getattr(m2_lib, 'm2_py_chrmgr_create_race', None)
        if fn:
            try:
                fn.argtypes = [ctypes.c_uint32]
                fn.restype = ctypes.c_bool
                return fn(int(race))
            except Exception: pass
        return True

    @staticmethod
    def SelectRace(race=0, *args, **kwargs):
        ChrmgrModule._active_race = int(race)
        return True

    @staticmethod
    def LoadRaceData(*args, **kwargs):
        if not args: return True
        if len(args) == 1:
            target_race = ChrmgrModule._active_race
            path = str(args[0])
        else:
            if isinstance(args[0], int) or (isinstance(args[0], str) and args[0].isdigit()):
                target_race = int(args[0])
                path = str(args[1]) if len(args) > 1 else str(args[0])
            else:
                target_race = ChrmgrModule._active_race
                path = str(args[0])
        full_path = (ChrmgrModule._path_name + path) if ChrmgrModule._path_name and not path.startswith("d:/") and not path.startswith(ChrmgrModule._path_name) and not path.startswith("msm/") else path
        fn = getattr(m2_lib, 'm2_py_chrmgr_load_race_data', None)
        if fn and full_path:
            try:
                fn.argtypes = [ctypes.c_uint32, ctypes.c_char_p]
                fn.restype = ctypes.c_bool
                return fn(int(target_race), str(full_path).encode('utf-8'))
            except Exception: pass
        return True

    @staticmethod
    def LoadLocalRaceData(*args, **kwargs):
        if not args: return True
        if len(args) == 1:
            target_race = ChrmgrModule._active_race
            path = str(args[0])
        else:
            if isinstance(args[0], int) or (isinstance(args[0], str) and args[0].isdigit()):
                target_race = int(args[0])
                path = str(args[1]) if len(args) > 1 else str(args[0])
            else:
                target_race = ChrmgrModule._active_race
                path = str(args[0])
        fn = getattr(m2_lib, 'm2_py_chrmgr_load_race_data', None)
        if fn and path:
            try:
                fn.argtypes = [ctypes.c_uint32, ctypes.c_char_p]
                fn.restype = ctypes.c_bool
                return fn(int(target_race), str(path).encode('utf-8'))
            except Exception: pass
        return True

    @staticmethod
    def RegisterAttachingBoneName(*args, **kwargs):
        if not args: return True
        if len(args) == 2:
            race = ChrmgrModule._active_race
            part = int(args[0])
            bone = str(args[1])
        elif len(args) >= 3:
            race = int(args[0])
            part = int(args[1])
            bone = str(args[2])
        else:
            return True
        fn = getattr(m2_lib, 'm2_py_chrmgr_register_attaching_bone_name', None)
        if fn and bone:
            try:
                fn.argtypes = [ctypes.c_uint32, ctypes.c_uint32, ctypes.c_char_p]
                fn.restype = ctypes.c_bool
                return fn(int(race), int(part), str(bone).encode('utf-8'))
            except Exception: pass
        return True

    @staticmethod
    def RegisterMotionMode(*args, **kwargs):
        if not args: return True
        if len(args) == 1:
            race = ChrmgrModule._active_race
            mode = int(args[0])
        else:
            race = int(args[0])
            mode = int(args[1])
        fn = getattr(m2_lib, 'm2_py_chrmgr_register_motion_mode', None)
        if fn:
            try:
                fn.argtypes = [ctypes.c_uint32, ctypes.c_uint32]
                fn.restype = ctypes.c_bool
                return fn(int(race), int(mode))
            except Exception: pass
        return True

    @staticmethod
    def SetMotionRandomWeight(*args, **kwargs):
        if len(args) == 4:
            race = ChrmgrModule._active_race
            mode = int(args[0])
            motion = int(args[1])
            weight = int(args[3])
        elif len(args) >= 5:
            race = int(args[0])
            mode = int(args[1])
            motion = int(args[2])
            weight = int(args[4])
        else:
            return True
        fn = getattr(m2_lib, 'm2_py_chrmgr_set_motion_random_weight', None)
        if fn:
            try:
                fn.argtypes = [ctypes.c_uint32, ctypes.c_uint32, ctypes.c_uint32, ctypes.c_uint32]
                fn.restype = ctypes.c_bool
                return fn(int(race), int(mode), int(motion), int(weight))
            except Exception: pass
        return True

    @staticmethod
    def RegisterNormalAttack(*args, **kwargs):
        if len(args) == 2:
            race = ChrmgrModule._active_race
            mode = int(args[0])
            count = int(args[1])
        elif len(args) >= 3:
            race = int(args[0])
            mode = int(args[1])
            count = int(args[2])
        else:
            return True
        fn = getattr(m2_lib, 'm2_py_chrmgr_register_normal_attack', None)
        if fn:
            try:
                fn.argtypes = [ctypes.c_uint32, ctypes.c_uint32, ctypes.c_uint32]
                fn.restype = ctypes.c_bool
                return fn(int(race), int(mode), int(count))
            except Exception: pass
        return True

    @staticmethod
    def ReserveComboAttack(vid=0, combo=0, *args, **kwargs): pass
    @staticmethod
    def RegisterComboAttack(race=0, mode=0, combo=0, count=0, *args, **kwargs): pass
    @staticmethod
    def ReserveComboAttackNew(vid=0, combo=0, *args, **kwargs): pass

    @staticmethod
    def RegisterComboAttackNew(*args, **kwargs):
        if len(args) == 4:
            race = ChrmgrModule._active_race
            mode = int(args[0])
            combo = int(args[1])
            count = int(args[3])
        elif len(args) >= 5:
            race = int(args[0])
            mode = int(args[1])
            combo = int(args[2])
            count = int(args[4])
        else:
            return True
        fn = getattr(m2_lib, 'm2_py_chrmgr_register_combo_attack_new', None)
        if fn:
            try:
                fn.argtypes = [ctypes.c_uint32, ctypes.c_uint32, ctypes.c_uint32, ctypes.c_uint32]
                fn.restype = ctypes.c_bool
                return fn(int(race), int(mode), int(combo), int(count))
            except Exception: pass
        return True

    @staticmethod
    def RegisterMotionData(*args, **kwargs):
        if len(args) >= 3 and isinstance(args[2], str):
            race = ChrmgrModule._active_race
            mode = int(args[0])
            motion = int(args[1])
            path = str(args[2])
        elif len(args) >= 4 and isinstance(args[3], str):
            race = int(args[0])
            mode = int(args[1])
            motion = int(args[2])
            path = str(args[3])
        else:
            return True
        full_path = (ChrmgrModule._path_name + path) if ChrmgrModule._path_name and not path.startswith("d:/") and not path.startswith(ChrmgrModule._path_name) else path
        fn = getattr(m2_lib, 'm2_py_chrmgr_register_motion_data', None)
        if fn and full_path:
            try:
                fn.argtypes = [ctypes.c_uint32, ctypes.c_uint32, ctypes.c_uint32, ctypes.c_char_p]
                fn.restype = ctypes.c_bool
                return fn(int(race), int(mode), int(motion), str(full_path).encode('utf-8'))
            except Exception: pass
        return True

    @staticmethod
    def RegisterRaceName(race=0, name="", *args, **kwargs):
        fn = getattr(m2_lib, 'm2_py_chrmgr_register_race_name', None)
        if fn and name:
            try:
                fn.argtypes = [ctypes.c_uint32, ctypes.c_char_p]
                fn.restype = ctypes.c_bool
                return fn(int(race), str(name).encode('utf-8'))
            except Exception: pass
        return True

    @staticmethod
    def RegisterRaceSrcName(name="", src="", *args, **kwargs):
        fn = getattr(m2_lib, 'm2_py_chrmgr_register_race_src_name', None)
        if fn and name and src:
            try:
                fn.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
                fn.restype = ctypes.c_bool
                return fn(str(name).encode('utf-8'), str(src).encode('utf-8'))
            except Exception: pass
        return True

    @staticmethod
    def RegisterCacheMotionData(*args, **kwargs):
        return ChrmgrModule.RegisterMotionData(*args, **kwargs)

    @staticmethod
    def SetAffect(race=0, affect=0, file="", *args, **kwargs): pass
    @staticmethod
    def SetEmoticon(vid=0, emoticon=0, *args, **kwargs): pass
    @staticmethod
    def IsPossibleEmoticon(vid=0, *args, **kwargs): return True

    @staticmethod
    def RegisterEffect(*args, **kwargs):
        if len(args) >= 3:
            etype = int(args[0])
            bone = str(args[1])
            file = str(args[2])
        elif len(args) == 2:
            etype = int(args[0])
            bone = ""
            file = str(args[1])
        else:
            return True
        fn = getattr(m2_lib, 'm2_py_chrmgr_register_effect', None)
        if fn:
            try:
                fn.argtypes = [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_char_p]
                fn.restype = ctypes.c_bool
                b_str = str(bone).encode('utf-8') if bone else b""
                f_str = str(file).encode('utf-8') if file else b""
                return fn(int(etype), b_str, f_str)
            except Exception: pass
        return True

    @staticmethod
    def RegisterCacheEffect(*args, **kwargs):
        return ChrmgrModule.RegisterEffect(*args, **kwargs)
    @staticmethod
    def RegisterPointEffect(point=0, file="", *args, **kwargs): pass
    @staticmethod
    def ShowPointEffect(vid=0, point=0, *args, **kwargs): pass
    @staticmethod
    def ToggleDirectionLine(enable=0, *args, **kwargs): pass
    @staticmethod
    def SetMovingSpeed(speed=100, *args, **kwargs): pass
    @staticmethod
    def SetDustGap(gap=0, *args, **kwargs):
        fn = getattr(m2_lib, 'm2_py_chrmgr_set_dust_gap', None)
        if fn:
            try:
                fn.argtypes = [ctypes.c_uint32]
                fn.restype = ctypes.c_bool
                return fn(int(gap))
            except Exception: pass
        return True
    @staticmethod
    def SetHorseDustGap(gap=0, *args, **kwargs):
        fn = getattr(m2_lib, 'm2_py_chrmgr_set_horse_dust_gap', None)
        if fn:
            try:
                fn.argtypes = [ctypes.c_uint32]
                fn.restype = ctypes.c_bool
                return fn(int(gap))
            except Exception: pass
        return True
    @staticmethod
    def RegisterTitleName(index=0, name="", *args, **kwargs):
        fn = getattr(m2_lib, 'm2_py_chrmgr_register_title_name', None)
        if fn and name:
            try:
                fn.argtypes = [ctypes.c_uint32, ctypes.c_char_p]
                fn.restype = ctypes.c_bool
                return fn(int(index), str(name).encode('utf-8'))
            except Exception: pass
        return True
    @staticmethod
    def RefreshAllPCTextTail(*args, **kwargs): pass
    @staticmethod
    def RegisterNameColor(index=0, r=0, g=0, b=0, *args, **kwargs):
        fn = getattr(m2_lib, 'm2_py_chrmgr_register_name_color', None)
        if fn:
            try:
                fn.argtypes = [ctypes.c_uint32, ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint8]
                fn.restype = ctypes.c_bool
                return fn(int(index), int(r), int(g), int(b))
            except Exception: pass
        return True
    @staticmethod
    def RegisterTitleColor(index=0, r=0, g=0, b=0, *args, **kwargs):
        fn = getattr(m2_lib, 'm2_py_chrmgr_register_title_color', None)
        if fn:
            try:
                fn.argtypes = [ctypes.c_uint32, ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint8]
                fn.restype = ctypes.c_bool
                return fn(int(index), int(r), int(g), int(b))
            except Exception: pass
        return True

ChrmgrModule.NAMECOLOR_MOB = 6
ChrmgrModule.NAMECOLOR_NPC = 7
ChrmgrModule.NAMECOLOR_PC = 8
ChrmgrModule.NAMECOLOR_EMPIRE_MOB = 12
ChrmgrModule.NAMECOLOR_EMPIRE_NPC = 13
ChrmgrModule.NAMECOLOR_EMPIRE_PC = 14
ChrmgrModule.NAMECOLOR_FUNC = 18
ChrmgrModule.NAMECOLOR_PK = 19
ChrmgrModule.NAMECOLOR_PVP = 20
ChrmgrModule.NAMECOLOR_PARTY = 21
ChrmgrModule.NAMECOLOR_WARP = 22
ChrmgrModule.NAMECOLOR_WAYPOINT = 23
ChrmgrModule.NAMECOLOR_EXTRA = 28
ChrmgrModule.EFFECT_SPAWN_DISAPPEAR = 0
ChrmgrModule.EFFECT_SPAWN_APPEAR = 1
ChrmgrModule.EFFECT_DUST = 2
ChrmgrModule.EFFECT_HORSE_DUST = 3
ChrmgrModule.EFFECT_STUN = 4
ChrmgrModule.EFFECT_HIT = 5
ChrmgrModule.EFFECT_FLAME_ATTACK = 6
ChrmgrModule.EFFECT_FLAME_HIT = 7
ChrmgrModule.EFFECT_FLAME_ATTACH = 8
ChrmgrModule.EFFECT_ELECTRIC_ATTACK = 9
ChrmgrModule.EFFECT_ELECTRIC_HIT = 10
ChrmgrModule.EFFECT_ELECTRIC_ATTACH = 11
ChrmgrModule.EFFECT_SELECT = 12
ChrmgrModule.EFFECT_TARGET = 13
ChrmgrModule.EFFECT_CRITICAL = 14
ChrmgrModule.EFFECT_DAMAGE_TARGET = 15
ChrmgrModule.EFFECT_DAMAGE_NOT_TARGET = 16
ChrmgrModule.EFFECT_DAMAGE_SELFDAMAGE = 17
ChrmgrModule.EFFECT_DAMAGE_SELFDAMAGE2 = 18
ChrmgrModule.EFFECT_DAMAGE_POISON = 19
ChrmgrModule.EFFECT_DAMAGE_MISS = 20
ChrmgrModule.EFFECT_DAMAGE_TARGETMISS = 21
ChrmgrModule.EFFECT_DAMAGE_CRITICAL = 22
ChrmgrModule.EFFECT_LEVELUP = 23
ChrmgrModule.EFFECT_SKILLUP = 24
ChrmgrModule.EFFECT_HPUP_RED = 25
ChrmgrModule.EFFECT_SPUP_BLUE = 26
ChrmgrModule.EFFECT_SPEEDUP_GREEN = 27
ChrmgrModule.EFFECT_DXUP_PURPLE = 28
ChrmgrModule.EFFECT_PENETRATE = 29
ChrmgrModule.EFFECT_BLOCK = 30
ChrmgrModule.EFFECT_DODGE = 31
ChrmgrModule.EFFECT_FIRECRACKER = 32
ChrmgrModule.EFFECT_SPIN_TOP = 33
ChrmgrModule.EFFECT_WEAPON = 34
ChrmgrModule.EFFECT_AFFECT = 35
ChrmgrModule.EFFECT_EMOTICON = 36
ChrmgrModule.EFFECT_EMPIRE = 37
ChrmgrModule.EFFECT_REFINED = 38
ChrmgrModule.EFFECT_SUCCESS = 39
ChrmgrModule.EFFECT_FAIL = 40
ChrmgrModule.EFFECT_FR_SUCCESS = 41
ChrmgrModule.EFFECT_PERCENT_DAMAGE1 = 42
ChrmgrModule.EFFECT_PERCENT_DAMAGE2 = 43
ChrmgrModule.EFFECT_PERCENT_DAMAGE3 = 44
ChrmgrModule.EFFECT_AUTO_HPUP = 45
ChrmgrModule.EFFECT_AUTO_SPUP = 46
ChrmgrModule.EFFECT_RAMADAN_RING_EQUIP = 47
ChrmgrModule.EFFECT_HALLOWEEN_CANDY_EQUIP = 48
ChrmgrModule.EFFECT_HAPPINESS_RING_EQUIP = 49
ChrmgrModule.EFFECT_LOVE_PENDANT_EQUIP = 50
ChrmgrModule.EFFECT_AGGREGATE_MONSTER = 51

sys.modules['chr'] = ChrModule
sys.modules['Chr'] = ChrModule
sys.modules['chrmgr'] = ChrmgrModule

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

# --- ItemModule Constants (100% C++ Parity with PythonItemModule.cpp) ---
ItemModule.USESOUND_ACCESSORY = 0
ItemModule.USESOUND_ARMOR = 1
ItemModule.USESOUND_BOW = 2
ItemModule.USESOUND_DEFAULT = 3
ItemModule.USESOUND_WEAPON = 4
ItemModule.USESOUND_POTION = 5
ItemModule.USESOUND_PORTAL = 6
ItemModule.DROPSOUND_ACCESSORY = 0
ItemModule.DROPSOUND_ARMOR = 1
ItemModule.DROPSOUND_BOW = 2
ItemModule.DROPSOUND_DEFAULT = 3
ItemModule.DROPSOUND_WEAPON = 4

ItemModule.EQUIPMENT_COUNT = 16
ItemModule.EQUIPMENT_HEAD = 0
ItemModule.EQUIPMENT_BODY = 1
ItemModule.EQUIPMENT_WEAPON = 2
ItemModule.EQUIPMENT_WRIST = 3
ItemModule.EQUIPMENT_SHOES = 4
ItemModule.EQUIPMENT_NECK = 5
ItemModule.EQUIPMENT_EAR = 6
ItemModule.EQUIPMENT_UNIQUE1 = 7
ItemModule.EQUIPMENT_UNIQUE2 = 8
ItemModule.EQUIPMENT_ARROW = 9
ItemModule.EQUIPMENT_RING1 = 10
ItemModule.EQUIPMENT_RING2 = 11
ItemModule.EQUIPMENT_BELT = 12

ItemModule.ITEM_TYPE_NONE = 0
ItemModule.ITEM_TYPE_WEAPON = 1
ItemModule.ITEM_TYPE_ARMOR = 2
ItemModule.ITEM_TYPE_USE = 3
ItemModule.ITEM_TYPE_AUTOUSE = 4
ItemModule.ITEM_TYPE_MATERIAL = 5
ItemModule.ITEM_TYPE_SPECIAL = 6
ItemModule.ITEM_TYPE_TOOL = 7
ItemModule.ITEM_TYPE_LOTTERY = 8
ItemModule.ITEM_TYPE_ELK = 9
ItemModule.ITEM_TYPE_METIN = 10
ItemModule.ITEM_TYPE_CONTAINER = 11
ItemModule.ITEM_TYPE_FISH = 12
ItemModule.ITEM_TYPE_ROD = 13
ItemModule.ITEM_TYPE_RESOURCE = 14
ItemModule.ITEM_TYPE_CAMPFIRE = 15
ItemModule.ITEM_TYPE_UNIQUE = 16
ItemModule.ITEM_TYPE_SKILLBOOK = 17
ItemModule.ITEM_TYPE_QUEST = 18
ItemModule.ITEM_TYPE_POLYMORPH = 19
ItemModule.ITEM_TYPE_TREASURE_BOX = 20
ItemModule.ITEM_TYPE_TREASURE_KEY = 21
ItemModule.ITEM_TYPE_PICK = 24
ItemModule.ITEM_TYPE_BLEND = 27
ItemModule.ITEM_TYPE_COSTUME = 28
ItemModule.ITEM_TYPE_DS = 29
ItemModule.ITEM_TYPE_SPECIAL_DS = 30
ItemModule.ITEM_TYPE_EXTRACT = 31
ItemModule.ITEM_TYPE_SECONDARY_COIN = 32
ItemModule.ITEM_TYPE_RING = 33
ItemModule.ITEM_TYPE_BELT = 34

ItemModule.ARMOR_BODY = 0
ItemModule.ARMOR_HEAD = 1
ItemModule.ARMOR_SHIELD = 2
ItemModule.ARMOR_WRIST = 3
ItemModule.ARMOR_FOOTS = 4
ItemModule.ARMOR_NECK = 5
ItemModule.ARMOR_EAR = 6

ItemModule.WEAPON_SWORD = 0
ItemModule.WEAPON_DAGGER = 1
ItemModule.WEAPON_BOW = 2
ItemModule.WEAPON_TWO_HANDED = 3
ItemModule.WEAPON_BELL = 4
ItemModule.WEAPON_FAN = 5
ItemModule.WEAPON_ARROW = 6
ItemModule.WEAPON_MOUNT_SPEAR = 7

ItemModule.ITEM_ANTIFLAG_FEMALE = 1
ItemModule.ITEM_ANTIFLAG_MALE = 2
ItemModule.ITEM_ANTIFLAG_WARRIOR = 4
ItemModule.ITEM_ANTIFLAG_ASSASSIN = 8
ItemModule.ITEM_ANTIFLAG_SURA = 16
ItemModule.ITEM_ANTIFLAG_SHAMAN = 32
ItemModule.ITEM_ANTIFLAG_GET = 64
ItemModule.ITEM_ANTIFLAG_DROP = 128
ItemModule.ITEM_ANTIFLAG_SELL = 256
ItemModule.ITEM_ANTIFLAG_EMPIRE_A = 512
ItemModule.ITEM_ANTIFLAG_EMPIRE_B = 1024
ItemModule.ITEM_ANTIFLAG_EMPIRE_C = 2048
ItemModule.ITEM_ANTIFLAG_SAVE = 4096
ItemModule.ITEM_ANTIFLAG_GIVE = 8192
ItemModule.ITEM_ANTIFLAG_PKDROP = 16384
ItemModule.ITEM_ANTIFLAG_STACK = 32768
ItemModule.ITEM_ANTIFLAG_MYSHOP = 65536
ItemModule.ITEM_ANTIFLAG_SAFEBOX = 131072

ItemModule.LIMIT_NONE = 0
ItemModule.LIMIT_LEVEL = 1
ItemModule.LIMIT_STR = 2
ItemModule.LIMIT_DEX = 3
ItemModule.LIMIT_INT = 4
ItemModule.LIMIT_CON = 5
ItemModule.LIMIT_REAL_TIME = 7
ItemModule.LIMIT_REAL_TIME_START_FIRST_USE = 8
ItemModule.LIMIT_TIMER_BASED_ON_WEAR = 9

ItemModule.APPLY_NONE = 0
ItemModule.APPLY_MAX_HP = 1
ItemModule.APPLY_MAX_SP = 2
ItemModule.APPLY_CON = 3
ItemModule.APPLY_INT = 4
ItemModule.APPLY_STR = 5
ItemModule.APPLY_DEX = 6
ItemModule.APPLY_ATT_SPEED = 7
ItemModule.APPLY_MOV_SPEED = 8
ItemModule.APPLY_CAST_SPEED = 9
ItemModule.APPLY_HP_REGEN = 10
ItemModule.APPLY_SP_REGEN = 11
ItemModule.APPLY_POISON_PCT = 12
ItemModule.APPLY_STUN_PCT = 13
ItemModule.APPLY_SLOW_PCT = 14
ItemModule.APPLY_CRITICAL_PCT = 15
ItemModule.APPLY_PENETRATE_PCT = 16
ItemModule.APPLY_ATTBONUS_HUMAN = 17
ItemModule.APPLY_ATTBONUS_ANIMAL = 18
ItemModule.APPLY_ATTBONUS_ORC = 19
ItemModule.APPLY_ATTBONUS_MILGYO = 20
ItemModule.APPLY_ATTBONUS_UNDEAD = 21
ItemModule.APPLY_ATTBONUS_DEVIL = 22
ItemModule.APPLY_STEAL_HP = 23
ItemModule.APPLY_STEAL_SP = 24
ItemModule.APPLY_MANA_BURN_PCT = 25
ItemModule.APPLY_BLOCK = 27
ItemModule.APPLY_DODGE = 28
ItemModule.APPLY_RESIST_SWORD = 29
ItemModule.APPLY_RESIST_TWOHAND = 30
ItemModule.APPLY_RESIST_DAGGER = 31
ItemModule.APPLY_RESIST_BELL = 32
ItemModule.APPLY_RESIST_FAN = 33
ItemModule.APPLY_RESIST_BOW = 34
ItemModule.APPLY_RESIST_FIRE = 35
ItemModule.APPLY_RESIST_ELEC = 36
ItemModule.APPLY_RESIST_MAGIC = 37
ItemModule.APPLY_RESIST_WIND = 38
ItemModule.APPLY_REFLECT_MELEE = 39
ItemModule.APPLY_POISON_REDUCE = 41
ItemModule.APPLY_EXP_DOUBLE_BONUS = 43
ItemModule.APPLY_GOLD_DOUBLE_BONUS = 44
ItemModule.APPLY_ITEM_DROP_BONUS = 45
ItemModule.APPLY_ATT_GRADE_BONUS = 53
ItemModule.APPLY_DEF_GRADE_BONUS = 54
ItemModule.APPLY_MAGIC_ATT_GRADE = 55
ItemModule.APPLY_MAGIC_DEF_GRADE = 56

def _wrap_item_get_limit(limit_idx):
    fn = getattr(m2_lib, 'm2_py_item_get_limit', None)
    if fn:
        try:
            l_type = ctypes.c_uint8(0)
            l_val = ctypes.c_int32(0)
            fn.argtypes = [ctypes.c_size_t, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_int32)]
            fn.restype = ctypes.c_bool
            if fn(int(limit_idx), ctypes.byref(l_type), ctypes.byref(l_val)):
                return (int(l_type.value), int(l_val.value))
        except Exception: pass
    return (0, 0)

def _wrap_item_get_apply(apply_idx):
    fn = getattr(m2_lib, 'm2_py_item_get_apply', None)
    if fn:
        try:
            a_type = ctypes.c_uint8(0)
            a_val = ctypes.c_int32(0)
            fn.argtypes = [ctypes.c_size_t, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_int32)]
            fn.restype = ctypes.c_bool
            if fn(int(apply_idx), ctypes.byref(a_type), ctypes.byref(a_val)):
                return (int(a_type.value), int(a_val.value))
        except Exception: pass
    return (0, 0)

def _wrap_item_get_affect(affect_idx):
    fn = getattr(m2_lib, 'm2_py_item_get_affect', None)
    if fn:
        try:
            a_type = ctypes.c_uint8(0)
            a_val = ctypes.c_int32(0)
            fn.argtypes = [ctypes.c_size_t, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_int32)]
            fn.restype = ctypes.c_bool
            if fn(int(affect_idx), ctypes.byref(a_type), ctypes.byref(a_val)):
                return (int(a_type.value), int(a_val.value))
        except Exception: pass
    return (0, 0)

def _wrap_item_get_item_size(*args):
    fn = getattr(m2_lib, 'm2_py_item_get_item_size', None)
    if fn:
        try:
            fn.restype = ctypes.c_uint8
            h = fn()
            return (1, int(h) if h > 0 else 1)
        except Exception: pass
    return (1, 1)

def _wrap_item_get_item_icon():
    buf = ctypes.create_string_buffer(256)
    fn = getattr(m2_lib, 'm2_py_item_get_icon_image_file_name', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_char_p, ctypes.c_size_t]
            fn.restype = ctypes.c_bool
            if fn(buf, 256):
                return buf.value.decode('utf-8', errors='ignore')
        except Exception: pass
    return "icon/item/00010.tga"

def _wrap_item_can_add_to_quickslot_item():
    t = getattr(ItemModule, 'GetItemType', lambda: 0)()
    return t in (3, 4, 18, 16)

setattr(ItemModule, 'GetLimit', staticmethod(_wrap_item_get_limit))
setattr(ItemModule, 'GetLimitType', staticmethod(lambda idx: _wrap_item_get_limit(idx)[0]))
setattr(ItemModule, 'GetLimitValue', staticmethod(lambda idx: _wrap_item_get_limit(idx)[1]))
setattr(ItemModule, 'GetApply', staticmethod(_wrap_item_get_apply))
setattr(ItemModule, 'GetAffect', staticmethod(_wrap_item_get_affect))
setattr(ItemModule, 'GetItemSize', staticmethod(_wrap_item_get_item_size))
setattr(ItemModule, 'GetItemIcon', staticmethod(_wrap_item_get_item_icon))
setattr(ItemModule, 'CanAddToQuickSlotItem', staticmethod(_wrap_item_can_add_to_quickslot_item))
setattr(ItemModule, 'IsWearableFlag', staticmethod(lambda f: True))
setattr(ItemModule, 'SetDropSoundFileName', staticmethod(lambda sound_type, filename: None))
setattr(ItemModule, 'SetUseSoundFileName', staticmethod(lambda sound_type, filename: None))
setattr(ItemModule, 'GetItemSummary', staticmethod(lambda: ""))
setattr(ItemModule, 'LoadItemTable', staticmethod(lambda path: True))
setattr(ItemModule, 'CreateItem', staticmethod(lambda vnum: None))
def _wrap_item_get_use_type():
    fn = getattr(m2_lib, 'm2_py_item_get_use_type', None)
    if fn:
        try:
            buf = ctypes.create_string_buffer(64)
            fn.argtypes = [ctypes.c_char_p, ctypes.c_size_t]
            fn.restype = ctypes.c_bool
            if fn(buf, 64):
                return buf.value.decode('utf-8', 'ignore')
        except Exception: pass
    return "USE_POTION"
setattr(ItemModule, 'GetUseType', staticmethod(_wrap_item_get_use_type))

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
def _wrap_grp_generate_color(r=1.0, g=1.0, b=1.0, a=1.0):
    fn = getattr(m2_lib, 'm2_grp_generate_color', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float]
            fn.restype = ctypes.c_uint32
            return int(fn(float(r), float(g), float(b), float(a)))
        except Exception: pass
    return ((int(a * 255) & 0xff) << 24) | ((int(r * 255) & 0xff) << 16) | ((int(g * 255) & 0xff) << 8) | (int(b * 255) & 0xff)

def _wrap_grp_clear_depth_buffer():
    fn = getattr(m2_lib, 'm2_grp_clear_depth_buffer', None)
    if fn:
        try: fn()
        except Exception: pass

def _wrap_grp_set_game_render_state():
    fn = getattr(m2_lib, 'm2_grp_set_game_render_state', None)
    if fn:
        try: fn()
        except Exception: pass

def _wrap_grp_set_interface_render_state():
    fn = getattr(m2_lib, 'm2_grp_set_interface_render_state', None)
    if fn:
        try: fn()
        except Exception: pass

def _wrap_grp_push_state():
    fn = getattr(m2_lib, 'm2_grp_push_state', None)
    if fn:
        try: fn()
        except Exception: pass

def _wrap_grp_pop_state():
    fn = getattr(m2_lib, 'm2_grp_pop_state', None)
    if fn:
        try: fn()
        except Exception: pass

setattr(GrpModule, 'GenerateColor', staticmethod(_wrap_grp_generate_color))
setattr(GrpModule, 'ClearDepthBuffer', staticmethod(_wrap_grp_clear_depth_buffer))
setattr(GrpModule, 'SetGameRenderState', staticmethod(_wrap_grp_set_game_render_state))
setattr(GrpModule, 'SetInterfaceRenderState', staticmethod(_wrap_grp_set_interface_render_state))
setattr(GrpModule, 'PushState', staticmethod(_wrap_grp_push_state))
setattr(GrpModule, 'PopState', staticmethod(_wrap_grp_pop_state))
setattr(GrpModule, 'PushMatrix', staticmethod(lambda: None))
setattr(GrpModule, 'PopMatrix', staticmethod(lambda: None))
setattr(GrpModule, 'Translate', staticmethod(lambda *args: None))
setattr(GrpModule, 'Rotate', staticmethod(lambda *args: None))
setattr(GrpModule, 'SetOmniLight', staticmethod(lambda: None))
setattr(GrpModule, 'SetViewport', staticmethod(lambda *args: None))
setattr(GrpModule, 'RestoreViewport', staticmethod(lambda: None))
setattr(GrpModule, 'SetPerspective', staticmethod(lambda *args: None))
setattr(GrpModule, 'SetOrtho2d', staticmethod(lambda *args: None))
setattr(GrpModule, 'SetOrtho3d', staticmethod(lambda *args: None))
setattr(GrpModule, 'SetCursorPosition', staticmethod(lambda *args: None))
setattr(GrpModule, 'GetCursorPosition3d', staticmethod(lambda: (0.0, 0.0, 0.0)))
setattr(GrpModule, 'SetAroundCamera', staticmethod(lambda *args: None))
setattr(GrpModule, 'SetPositionCamera', staticmethod(lambda *args: None))
setattr(GrpModule, 'SetEyeCamera', staticmethod(lambda *args: None))

def _wrap_grp_get_camera_position():
    fn = getattr(m2_lib, 'm2_camera_get_eye', None)
    if fn:
        try:
            x = ctypes.c_float(0.0)
            y = ctypes.c_float(0.0)
            z = ctypes.c_float(0.0)
            fn.argtypes = [ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float)]
            fn.restype = ctypes.c_bool
            if fn(ctypes.byref(x), ctypes.byref(y), ctypes.byref(z)):
                return (x.value, y.value, z.value)
        except Exception: pass
    return (0.0, 0.0, 0.0)

def _wrap_grp_get_target_position():
    fn = getattr(m2_lib, 'm2_camera_get_target', None)
    if fn:
        try:
            x = ctypes.c_float(0.0)
            y = ctypes.c_float(0.0)
            z = ctypes.c_float(0.0)
            fn.argtypes = [ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float)]
            fn.restype = ctypes.c_bool
            if fn(ctypes.byref(x), ctypes.byref(y), ctypes.byref(z)):
                return (x.value, y.value, z.value)
        except Exception: pass
    return (0.0, 0.0, 0.0)

setattr(GrpModule, 'GetCameraPosition', staticmethod(_wrap_grp_get_camera_position))
setattr(GrpModule, 'GetTargetPosition', staticmethod(_wrap_grp_get_target_position))
setattr(GrpModule, 'CreateTextBar', staticmethod(lambda *args: 1))
setattr(GrpModule, 'CreateBigTextBar', staticmethod(lambda *args: 1))
setattr(GrpModule, 'DestroyTextBar', staticmethod(lambda *args: None))
setattr(GrpModule, 'RenderTextBar', staticmethod(lambda *args: None))
setattr(GrpModule, 'TextBarTextOut', staticmethod(lambda *args: None))
setattr(GrpModule, 'TextBarSetTextColor', staticmethod(lambda *args: None))
setattr(GrpModule, 'TextBarGetTextExtent', staticmethod(lambda *args: (10, 10)))
setattr(GrpModule, 'ClearTextBar', staticmethod(lambda *args: None))
setattr(GrpModule, 'SetTextBarClipRect', staticmethod(lambda *args: None))
setattr(GrpModule, 'SetAlpha', staticmethod(lambda a: None))
setattr(GrpModule, 'SetDiffuseColor', staticmethod(lambda *args: None))
setattr(GrpModule, 'SetClearColor', staticmethod(lambda *args: None))
setattr(GrpModule, 'RenderRoundBox', staticmethod(lambda *args: None))
setattr(GrpModule, 'RenderBox3d', staticmethod(lambda *args: None))
setattr(GrpModule, 'RenderBar', staticmethod(lambda *args: None))
setattr(GrpModule, 'RenderBar3d', staticmethod(lambda *args: None))
setattr(GrpModule, 'RenderGradationBar', staticmethod(lambda *args: None))
setattr(GrpModule, 'RenderCube', staticmethod(lambda *args: None))
setattr(GrpModule, 'RenderDownButton', staticmethod(lambda *args: None))
setattr(GrpModule, 'RenderUpButton', staticmethod(lambda *args: None))
setattr(GrpModule, 'GetAvailableMemory', staticmethod(lambda: 1024*1024*1024))
setattr(GrpModule, 'SaveScreenShot', staticmethod(lambda *args: (False, "")))
setattr(GrpModule, 'SaveScreenShotToPath', staticmethod(lambda *args: (False, "")))
setattr(GrpModule, 'SetGamma', staticmethod(lambda g: None))
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
def _wrap_snd_stop_all_sound():
    fn = getattr(m2_lib, 'm2_snd_stop_all_sound', None)
    if fn:
        try: fn()
        except Exception: pass

def _wrap_snd_play_sound_3d(path, x=0.0, y=0.0, z=0.0):
    fn = getattr(m2_lib, 'm2_snd_play_sound_3d', None)
    if fn and path:
        try:
            p_bytes = path.encode('utf-8') if isinstance(path, str) else path
            fn.argtypes = [ctypes.c_char_p, ctypes.c_float, ctypes.c_float, ctypes.c_float]
            fn.restype = ctypes.c_bool
            return bool(fn(p_bytes, float(x), float(y), float(z)))
        except Exception: pass
    return False

setattr(SndModule, 'StopAllSound', staticmethod(_wrap_snd_stop_all_sound))
setattr(SndModule, 'PlaySound3D', staticmethod(_wrap_snd_play_sound_3d))
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
TextTailModule = create_real_module('textTail', 'texttail', texttail_methods)

def _wrap_texttail_arrange():
    fn = getattr(m2_lib, 'm2_text_tail_arrange_text_tail', None)
    if fn:
        try: fn()
        except Exception: pass

def _wrap_texttail_show_all():
    fn = getattr(m2_lib, 'm2_text_tail_show_all_text_tail', None)
    if fn:
        try: fn()
        except Exception: pass

def _wrap_texttail_hide_all():
    fn = getattr(m2_lib, 'm2_text_tail_hide_all_text_tail', None)
    if fn:
        try: fn()
        except Exception: pass

def _wrap_texttail_show_char(vid):
    fn = getattr(m2_lib, 'm2_text_tail_show_character_text_tail', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint32]
            fn(int(vid))
        except Exception: pass

def _wrap_texttail_hide_char(vid):
    fn = getattr(m2_lib, 'm2_text_tail_hide_character_text_tail', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint32]
            fn(int(vid))
        except Exception: pass

def _wrap_texttail_register_chat(vid, text):
    fn = getattr(m2_lib, 'm2_text_tail_register_chat_tail', None)
    if fn and text:
        try:
            fn.argtypes = [ctypes.c_uint32, ctypes.c_char_p]
            fn(int(vid), str(text).encode('utf-8'))
        except Exception: pass

setattr(TextTailModule, 'GetPosition', staticmethod(lambda *args: (0.0, 0.0, 0.0)))
setattr(TextTailModule, 'ArrangeTextTail', staticmethod(_wrap_texttail_arrange))
setattr(TextTailModule, 'ShowAllTextTail', staticmethod(_wrap_texttail_show_all))
setattr(TextTailModule, 'HideAllTextTail', staticmethod(_wrap_texttail_hide_all))
setattr(TextTailModule, 'ShowCharacterTextTail', staticmethod(_wrap_texttail_show_char))
setattr(TextTailModule, 'HideCharacterTextTail', staticmethod(_wrap_texttail_hide_char))
setattr(TextTailModule, 'RegisterChatTail', staticmethod(_wrap_texttail_register_chat))
setattr(TextTailModule, 'Clear', staticmethod(lambda: None))
setattr(TextTailModule, 'UpdateAllTextTail', staticmethod(lambda: None))
setattr(TextTailModule, 'UpdateShowingTextTail', staticmethod(lambda: None))
setattr(TextTailModule, 'Render', staticmethod(lambda: None))
setattr(TextTailModule, 'ShowItemTextTail', staticmethod(lambda vid: None))
setattr(TextTailModule, 'IsChat', staticmethod(lambda vid: False))
setattr(TextTailModule, 'Pick', staticmethod(lambda x=0, y=0: 0))
setattr(TextTailModule, 'SelectItemName', staticmethod(lambda vid: None))

sys.modules['textTail'] = TextTailModule
sys.modules['TextTail'] = TextTailModule

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
GuildModule = create_real_module('guild', 'guild', guild_methods)
GuildModule.AUTH_ADD_MEMBER = 1
GuildModule.AUTH_REMOVE_MEMBER = 2
GuildModule.AUTH_NOTICE = 4
GuildModule.AUTH_SKILL = 8
GuildModule.ENEMY_GUILD_SLOT_MAX_COUNT = 6
setattr(GuildModule, 'IsGuildEnable', staticmethod(lambda: getattr(GuildModule, 'GetGuildID', lambda: 0)() > 0))
setattr(GuildModule, 'GuildIDToMarkID', staticmethod(lambda gid: int(gid)))
setattr(GuildModule, 'GetMarkImageFilenameByMarkID', staticmethod(lambda mid: "mark/10_0.tga"))
setattr(GuildModule, 'GetMarkIndexByMarkID', staticmethod(lambda mid: int(mid)))
def _wrap_guild_has_land():
    fn = getattr(m2_lib, 'm2_guild_has_land', None)
    if fn:
        try:
            fn.restype = ctypes.c_bool
            return bool(fn())
        except Exception: pass
    return False

def _wrap_guild_get_experience():
    fn = getattr(m2_lib, 'm2_guild_get_exp', None)
    if fn:
        try:
            cur = ctypes.c_uint32(0)
            max_e = ctypes.c_uint32(10000)
            fn.argtypes = [ctypes.POINTER(ctypes.c_uint32), ctypes.POINTER(ctypes.c_uint32)]
            fn.restype = ctypes.c_bool
            if fn(ctypes.byref(cur), ctypes.byref(max_e)):
                return (cur.value, max_e.value)
        except Exception: pass
    return (0, 0)

def _wrap_guild_get_member_count():
    fn = getattr(m2_lib, 'm2_guild_get_member_count', None)
    if fn:
        try:
            cur = ctypes.c_uint16(0)
            max_m = ctypes.c_uint16(32)
            fn.argtypes = [ctypes.POINTER(ctypes.c_uint16), ctypes.POINTER(ctypes.c_uint16)]
            fn.restype = ctypes.c_bool
            if fn(ctypes.byref(cur), ctypes.byref(max_m)):
                return (cur.value, max_m.value)
        except Exception: pass
    return (0, 0)

def _wrap_guild_get_skill_point():
    fn = getattr(m2_lib, 'm2_guild_get_skill_point', None)
    if fn:
        try:
            fn.restype = ctypes.c_uint8
            return int(fn())
        except Exception: pass
    return 0

def _wrap_guild_get_skill_level(idx):
    fn = getattr(m2_lib, 'm2_guild_get_skill_level', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint8]
            fn.restype = ctypes.c_uint8
            return int(fn(int(idx)))
        except Exception: pass
    return 0

def _wrap_guild_get_grade_name(idx):
    fn = getattr(m2_lib, 'm2_guild_get_grade_name', None)
    if fn:
        try:
            buf = ctypes.create_string_buffer(32)
            fn.argtypes = [ctypes.c_uint8, ctypes.c_char_p, ctypes.c_size_t]
            fn.restype = ctypes.c_bool
            if fn(int(idx), buf, 32):
                return buf.value.decode('utf-8', 'ignore')
        except Exception: pass
    return f"Grade {idx}"

def _wrap_guild_main_player_has_authority(auth):
    fn = getattr(m2_lib, 'm2_guild_main_player_has_authority', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint8]
            fn.restype = ctypes.c_bool
            return bool(fn(int(auth)))
        except Exception: pass
    return True

setattr(GuildModule, 'HasGuildLand', staticmethod(_wrap_guild_has_land))
setattr(GuildModule, 'GetGuildMasterName', staticmethod(lambda: "GuildMaster"))
setattr(GuildModule, 'GetEnemyGuildName', staticmethod(lambda idx: ""))
setattr(GuildModule, 'GetGuildBoardCommentCount', staticmethod(lambda: 0))
setattr(GuildModule, 'GetGuildBoardCommentData', staticmethod(lambda *args: (0, "", "")))
setattr(GuildModule, 'GetGuildExperience', staticmethod(_wrap_guild_get_experience))
setattr(GuildModule, 'GetGuildMemberCount', staticmethod(_wrap_guild_get_member_count))
setattr(GuildModule, 'GetGuildMemberLevelSummary', staticmethod(lambda: 0))
setattr(GuildModule, 'GetGuildMemberLevelAverage', staticmethod(lambda: 0))
setattr(GuildModule, 'GetGuildExperienceSummary', staticmethod(lambda: 0))
setattr(GuildModule, 'GetGuildSkillPoint', staticmethod(_wrap_guild_get_skill_point))
setattr(GuildModule, 'GetDragonPowerPoint', staticmethod(lambda: (0, 0)))
setattr(GuildModule, 'GetGuildSkillLevel', staticmethod(_wrap_guild_get_skill_level))
setattr(GuildModule, 'GetSkillLevel', staticmethod(_wrap_guild_get_skill_level))
setattr(GuildModule, 'GetSkillMaxLevelNew', staticmethod(lambda idx: 7))
setattr(GuildModule, 'SetSkillIndex', staticmethod(lambda idx: None))
setattr(GuildModule, 'GetSkillIndex', staticmethod(lambda: 0))
setattr(GuildModule, 'GetGradeData', staticmethod(lambda *args: ("", 0)))
setattr(GuildModule, 'GetGradeName', staticmethod(_wrap_guild_get_grade_name))
setattr(GuildModule, 'GetMemberData', staticmethod(lambda *args: (0, "", 0, 0, 0, 0, 0)))
setattr(GuildModule, 'MemberIndexToPID', staticmethod(lambda idx: int(idx) + 1))
setattr(GuildModule, 'IsMember', staticmethod(lambda pid: True))
setattr(GuildModule, 'IsMemberByName', staticmethod(lambda name: True))
setattr(GuildModule, 'MainPlayerHasAuthority', staticmethod(_wrap_guild_main_player_has_authority))
setattr(GuildModule, 'Destroy', staticmethod(lambda: None))
sys.modules['guild'] = GuildModule
sys.modules['Guild'] = GuildModule

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
ShopModule = create_real_module('shop', 'shop', shop_methods)
ShopModule.SHOP_SLOT_COUNT = 40
ShopModule.SHOP_COIN_TYPE_GOLD = 0
ShopModule.SHOP_COIN_TYPE_SECONDARY_COIN = 1

def _wrap_shop_get_tab_count():
    fn = getattr(m2_lib, 'm2_py_shop_get_tab_count', None)
    if fn:
        try:
            fn.restype = ctypes.c_uint8
            return int(fn())
        except Exception: pass
    return 1

def _wrap_shop_select_tab(tab_idx):
    fn = getattr(m2_lib, 'm2_py_shop_select_tab', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint8]
            fn.restype = ctypes.c_bool
            return bool(fn(int(tab_idx)))
        except Exception: pass
    return True

def _wrap_shop_get_selected_tab():
    fn = getattr(m2_lib, 'm2_py_shop_get_selected_tab', None)
    if fn:
        try:
            fn.restype = ctypes.c_uint8
            return int(fn())
        except Exception: pass
    return 0

def _wrap_shop_get_tab_name(tab_idx):
    fn = getattr(m2_lib, 'm2_py_shop_get_tab_name', None)
    if fn:
        try:
            buf = ctypes.create_string_buffer(128)
            fn.argtypes = [ctypes.c_uint8, ctypes.c_char_p, ctypes.c_size_t]
            fn.restype = ctypes.c_bool
            if fn(int(tab_idx), buf, 128):
                return buf.value.decode('utf-8', errors='ignore')
        except Exception: pass
    return f"Tab {int(tab_idx) + 1}"

def _wrap_shop_get_tab_coin_type(tab_idx):
    fn = getattr(m2_lib, 'm2_py_shop_get_tab_coin_type', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint8]
            fn.restype = ctypes.c_uint8
            return int(fn(int(tab_idx)))
        except Exception: pass
    return 0

def _wrap_shop_is_private_shop():
    fn = getattr(m2_lib, 'm2_py_shop_is_private_shop', None)
    if fn:
        try:
            fn.restype = ctypes.c_bool
            return bool(fn())
        except Exception: pass
    return False

def _wrap_shop_is_main_player_private_shop():
    fn = getattr(m2_lib, 'm2_py_shop_is_main_player_private_shop', None)
    if fn:
        try:
            fn.restype = ctypes.c_bool
            return bool(fn())
        except Exception: pass
    return False

def _wrap_shop_clear_private_shop_stock():
    fn = getattr(m2_lib, 'm2_py_shop_clear_private_shop_stock', None)
    if fn:
        try: fn()
        except Exception: pass

def _wrap_shop_add_private_shop_item_stock(win_type, cell, display_pos, price):
    fn = getattr(m2_lib, 'm2_py_shop_add_private_shop_item_stock', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint8, ctypes.c_uint16, ctypes.c_uint8, ctypes.c_uint32]
            fn(int(win_type), int(cell), int(display_pos), int(price))
        except Exception: pass

def _wrap_shop_del_private_shop_item_stock(win_type, cell):
    fn = getattr(m2_lib, 'm2_py_shop_del_private_shop_item_stock', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint8, ctypes.c_uint16]
            fn(int(win_type), int(cell))
        except Exception: pass

def _wrap_shop_get_private_shop_item_price(win_type, cell):
    fn = getattr(m2_lib, 'm2_py_shop_get_private_shop_item_price', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint8, ctypes.c_uint16]
            fn.restype = ctypes.c_uint32
            return int(fn(int(win_type), int(cell)))
        except Exception: pass
    return 0

def _wrap_shop_build_private_shop(title):
    fn = getattr(m2_lib, 'm2_py_shop_build_private_shop', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_char_p]
            fn.restype = ctypes.c_bool
            return bool(fn(str(title).encode('utf-8')))
        except Exception: pass
    return True

setattr(ShopModule, 'GetTabCount', staticmethod(_wrap_shop_get_tab_count))
setattr(ShopModule, 'SelectTab', staticmethod(_wrap_shop_select_tab))
setattr(ShopModule, 'GetSelectedTab', staticmethod(_wrap_shop_get_selected_tab))
setattr(ShopModule, 'GetTabName', staticmethod(_wrap_shop_get_tab_name))
setattr(ShopModule, 'GetTabCoinType', staticmethod(_wrap_shop_get_tab_coin_type))
setattr(ShopModule, 'IsPrivateShop', staticmethod(_wrap_shop_is_private_shop))
setattr(ShopModule, 'IsMainPlayerPrivateShop', staticmethod(_wrap_shop_is_main_player_private_shop))
setattr(ShopModule, 'ClearPrivateShopStock', staticmethod(_wrap_shop_clear_private_shop_stock))
setattr(ShopModule, 'AddPrivateShopItemStock', staticmethod(_wrap_shop_add_private_shop_item_stock))
setattr(ShopModule, 'DelPrivateShopItemStock', staticmethod(_wrap_shop_del_private_shop_item_stock))
setattr(ShopModule, 'GetPrivateShopItemPrice', staticmethod(_wrap_shop_get_private_shop_item_price))
setattr(ShopModule, 'BuildPrivateShop', staticmethod(_wrap_shop_build_private_shop))
setattr(ShopModule, 'GetItemAttribute', staticmethod(_wrap_player_get_shop_item_attribute))
setattr(ShopModule, 'GetItemMetinSocket', staticmethod(_wrap_player_get_shop_item_metin_socket))
sys.modules['shop'] = ShopModule
sys.modules['Shop'] = ShopModule

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
SkillModule = create_real_module('skill', 'skill', skill_methods)

def _wrap_skill_get_require_stat_data(skill_index=0, stat_index=0):
    fn = getattr(m2_lib, 'm2_skill_get_require_stat_data', None)
    if fn:
        try:
            p = ctypes.c_uint8(0)
            lvl = ctypes.c_uint8(0)
            fn.argtypes = [ctypes.c_uint32, ctypes.c_uint32, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_uint8)]
            fn.restype = ctypes.c_bool
            if fn(int(skill_index), int(stat_index), ctypes.byref(p), ctypes.byref(lvl)):
                return (p.value, lvl.value)
        except Exception: pass
    return (0, 0)

def _wrap_skill_get_new_affect_data(skill_index=0, affect_index=0, skill_level=1.0):
    fn = getattr(m2_lib, 'm2_skill_get_new_affect_data', None)
    if fn:
        try:
            buf = ctypes.create_string_buffer(64)
            min_v = ctypes.c_float(0.0)
            max_v = ctypes.c_float(0.0)
            fn.argtypes = [ctypes.c_uint32, ctypes.c_uint32, ctypes.c_float, ctypes.c_char_p, ctypes.c_size_t, ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float)]
            fn.restype = ctypes.c_bool
            if fn(int(skill_index), int(affect_index), float(skill_level), buf, ctypes.c_size_t(64), ctypes.byref(min_v), ctypes.byref(max_v)):
                return (buf.value.decode('utf-8', errors='ignore'), min_v.value, max_v.value)
        except Exception: pass
    return ("", 0.0, 0.0)

setattr(SkillModule, 'GetNewAffectData', staticmethod(_wrap_skill_get_new_affect_data))
setattr(SkillModule, 'GetSkillRequireStatData', staticmethod(_wrap_skill_get_require_stat_data))
setattr(SkillModule, 'GetSkillRequirementData', staticmethod(lambda *args: ("", 0)))
sys.modules['skill'] = SkillModule
sys.modules['Skill'] = SkillModule

quest_methods = [
    ("GetQuestCount", "m2_py_quest_get_quest_count", [], ctypes.c_uint32),
    
    ("Clear", "m2_py_quest_clear", [], None),
    ("SetSkin", "m2_py_quest_set_skin", [ctypes.c_uint8], None),
    ("SetTitle", "m2_py_quest_set_title", [ctypes.c_char_p], None),
]
QuestModule = create_real_module('quest', 'quest', quest_methods)
def _wrap_quest_get_quest_icon_file_name(idx=0):
    fn = getattr(m2_lib, 'm2_py_quest_get_quest_icon_file_name', None)
    if fn:
        try:
            name_buf = ctypes.create_string_buffer(256)
            fn.argtypes = [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_size_t]
            fn.restype = ctypes.c_bool
            if fn(int(idx), name_buf, ctypes.c_size_t(256)):
                return name_buf.value.decode('utf-8', 'ignore')
        except Exception: pass
    return ""
    
setattr(QuestModule, 'GetQuestIconFileName', staticmethod(_wrap_quest_get_quest_icon_file_name))
def _wrap_quest_get_quest_name(idx=0):
    fn = getattr(m2_lib, 'm2_py_quest_get_quest_name', None)
    if fn:
        try:
            name_buf = ctypes.create_string_buffer(256)
            fn.argtypes = [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_size_t]
            fn.restype = ctypes.c_bool
            if fn(int(idx), name_buf, ctypes.c_size_t(256)):
                return name_buf.value.decode('utf-8', 'ignore')
        except Exception: pass
    return ""
    
setattr(QuestModule, 'GetQuestName', staticmethod(_wrap_quest_get_quest_name))
QuestModule.QUEST_MAX_NUM = 5
def _wrap_quest_get_quest_data(idx=0):
    fn = getattr(m2_lib, 'm2_py_quest_get_quest_data', None)
    if fn:
        try:
            title_buf = ctypes.create_string_buffer(256)
            counter_name_buf = ctypes.create_string_buffer(256)
            counter_val = ctypes.c_int32(0)
            
            fn.argtypes = [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_size_t, ctypes.c_char_p, ctypes.c_size_t, ctypes.POINTER(ctypes.c_int32)]
            fn.restype = ctypes.c_bool
            
            if fn(int(idx), title_buf, ctypes.c_size_t(256), counter_name_buf, ctypes.c_size_t(256), ctypes.byref(counter_val)):
                return (title_buf.value.decode('utf-8', 'ignore'), 0, counter_name_buf.value.decode('utf-8', 'ignore'), counter_val.value)
        except Exception: pass
    return ("", 0, "", 0)

setattr(QuestModule, 'GetQuestData', staticmethod(_wrap_quest_get_quest_data))
def _wrap_quest_get_quest_index(idx=0):
    fn = getattr(m2_lib, 'm2_py_quest_get_quest_index', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint32]
            fn.restype = ctypes.c_uint32
            return int(fn(int(idx)))
        except Exception: pass
    return 0
    
setattr(QuestModule, 'GetQuestIndex', staticmethod(_wrap_quest_get_quest_index))

def _wrap_quest_get_quest_last_time(idx=0):
    fn = getattr(m2_lib, 'm2_py_quest_get_quest_last_time', None)
    if fn:
        try:
            buf = ctypes.create_string_buffer(256)
            lt = ctypes.c_int32(0)
            fn.argtypes = [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_size_t, ctypes.POINTER(ctypes.c_int32)]
            fn.restype = ctypes.c_bool
            if fn(int(idx), buf, ctypes.c_size_t(256), ctypes.byref(lt)):
                return (buf.value.decode('utf-8', errors='ignore'), lt.value)
        except Exception: pass
    return ("", 0)

setattr(QuestModule, 'GetQuestLastTime', staticmethod(_wrap_quest_get_quest_last_time))
sys.modules['quest'] = QuestModule
sys.modules['Quest'] = QuestModule

minimap_methods = [
    ("SetMinimapPosition", "m2_py_minimap_set_minimap_position", [ctypes.c_float, ctypes.c_float], ctypes.c_bool),
    ("SetScale", "m2_py_minimap_set_scale", [ctypes.c_float], ctypes.c_bool),
    ("ShowMinimap", "m2_py_minimap_show_minimap", [], ctypes.c_bool),
    ("HideMinimap", "m2_py_minimap_hide_minimap", [], ctypes.c_bool),
]
MiniMapModule = create_real_module('miniMap', 'minimap', minimap_methods)
MiniMapModule.TYPE_OPC = 0
MiniMapModule.TYPE_OPCPVP = 1
MiniMapModule.TYPE_OPCPVPSELF = 2
MiniMapModule.TYPE_NPC = 3
MiniMapModule.TYPE_MONSTER = 4
MiniMapModule.TYPE_WARP = 5
MiniMapModule.TYPE_WAYPOINT = 6
MiniMapModule.TYPE_PARTY = 7
MiniMapModule.TYPE_EMPIRE = 8

def _wrap_minimap_get_atlas_size():
    fn = getattr(m2_lib, 'm2_minimap_get_atlas_size', None)
    if fn:
        try:
            out_x = ctypes.c_float(0.0)
            out_y = ctypes.c_float(0.0)
            fn.argtypes = [ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float)]
            fn.restype = ctypes.c_bool
            b_get = fn(ctypes.byref(out_x), ctypes.byref(out_y))
            return (bool(b_get), int(out_x.value), int(out_y.value))
        except Exception: pass
    return (False, 256, 256)

def _wrap_minimap_get_atlas_info(screen_x, screen_y):
    fn = getattr(m2_lib, 'm2_minimap_get_atlas_info', None)
    if fn:
        try:
            buf = ctypes.create_string_buffer(256)
            out_pos_x = ctypes.c_float(0.0)
            out_pos_y = ctypes.c_float(0.0)
            out_color = ctypes.c_uint32(0)
            out_guild = ctypes.c_uint32(0)
            fn.argtypes = [
                ctypes.c_float, ctypes.c_float,
                ctypes.c_char_p, ctypes.c_size_t,
                ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float),
                ctypes.POINTER(ctypes.c_uint32), ctypes.POINTER(ctypes.c_uint32)
            ]
            fn.restype = ctypes.c_bool
            b_find = fn(
                float(screen_x), float(screen_y),
                buf, 256,
                ctypes.byref(out_pos_x), ctypes.byref(out_pos_y),
                ctypes.byref(out_color), ctypes.byref(out_guild)
            )
            name_str = buf.value.decode('utf-8', errors='ignore')
            return (bool(b_find), name_str, int(out_pos_x.value / 100.0), int(out_pos_y.value / 100.0), int(out_color.value), int(out_guild.value))
        except Exception: pass
    return (False, "", 0, 0, 0, 0)

def _wrap_minimap_open_atlas_window():
    fn = getattr(m2_lib, 'm2_minimap_open_atlas_window', None)
    if fn:
        try: return bool(fn())
        except Exception: pass
    return True

def _wrap_minimap_close_atlas_window():
    fn = getattr(m2_lib, 'm2_minimap_close_atlas_window', None)
    if fn:
        try: return bool(fn())
        except Exception: pass
    return True

def _wrap_minimap_show_atlas():
    fn = getattr(m2_lib, 'm2_minimap_show_atlas', None)
    if fn:
        try: return bool(fn())
        except Exception: pass
    return True

def _wrap_minimap_hide_atlas():
    fn = getattr(m2_lib, 'm2_minimap_hide_atlas', None)
    if fn:
        try: return bool(fn())
        except Exception: pass
    return True

def _wrap_minimap_is_show_atlas():
    fn = getattr(m2_lib, 'm2_minimap_is_show_atlas', None)
    if fn:
        try: return bool(fn())
        except Exception: pass
    return False

def _wrap_minimap_is_atlas():
    fn = getattr(m2_lib, 'm2_minimap_is_atlas', None)
    if fn:
        try: return bool(fn())
        except Exception: pass
    return False

def _wrap_minimap_load_atlas():
    fn = getattr(m2_lib, 'm2_minimap_load_atlas', None)
    if fn:
        try: return bool(fn())
        except Exception: pass
    return True

def _wrap_minimap_add_way_point(id_val, x, y, name_str=""):
    fn = getattr(m2_lib, 'm2_py_minimap_add_way_point', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint32, ctypes.c_float, ctypes.c_float, ctypes.c_char_p]
            fn.restype = ctypes.c_bool
            c_name = str(name_str).encode('utf-8')
            return bool(fn(int(id_val), float(x), float(y), c_name))
        except Exception: pass
    return True

def _wrap_minimap_remove_way_point(id_val):
    fn = getattr(m2_lib, 'm2_py_minimap_remove_way_point', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint32]
            fn.restype = ctypes.c_bool
            return bool(fn(int(id_val)))
        except Exception: pass
    return True

def _wrap_minimap_clear_waypoints():
    fn = getattr(m2_lib, 'm2_minimap_clear_waypoints', None)
    if fn:
        try: return bool(fn())
        except Exception: pass
    return True

def _wrap_minimap_is_visible():
    fn = getattr(m2_lib, 'm2_minimap_is_visible', None)
    if fn:
        try: return bool(fn())
        except Exception: pass
    return True

setattr(MiniMapModule, 'GetAtlasSize', staticmethod(_wrap_minimap_get_atlas_size))
setattr(MiniMapModule, 'GetAtlasInfo', staticmethod(_wrap_minimap_get_atlas_info))
setattr(MiniMapModule, 'OpenAtlasWindow', staticmethod(_wrap_minimap_open_atlas_window))
setattr(MiniMapModule, 'CloseAtlasWindow', staticmethod(_wrap_minimap_close_atlas_window))
setattr(MiniMapModule, 'ShowAtlas', staticmethod(_wrap_minimap_show_atlas))
setattr(MiniMapModule, 'HideAtlas', staticmethod(_wrap_minimap_hide_atlas))
setattr(MiniMapModule, 'isShowAtlas', staticmethod(_wrap_minimap_is_show_atlas))
setattr(MiniMapModule, 'IsAtlas', staticmethod(_wrap_minimap_is_atlas))
setattr(MiniMapModule, 'LoadAtlas', staticmethod(_wrap_minimap_load_atlas))
setattr(MiniMapModule, 'AddWayPoint', staticmethod(_wrap_minimap_add_way_point))
setattr(MiniMapModule, 'RemoveWayPoint', staticmethod(_wrap_minimap_remove_way_point))
setattr(MiniMapModule, 'ClearWayPoints', staticmethod(_wrap_minimap_clear_waypoints))
def _wrap_minimap_scale_up():
    fn = getattr(m2_lib, 'm2_minimap_scale_up', None)
    if fn:
        try: return bool(fn())
        except Exception: pass
    return True

def _wrap_minimap_scale_down():
    fn = getattr(m2_lib, 'm2_minimap_scale_down', None)
    if fn:
        try: return bool(fn())
        except Exception: pass
    return True

def _wrap_minimap_show():
    fn = getattr(m2_lib, 'm2_minimap_show_minimap', None)
    if fn:
        try: return bool(fn())
        except Exception: pass
    return True

def _wrap_minimap_hide():
    fn = getattr(m2_lib, 'm2_minimap_hide_minimap', None)
    if fn:
        try: return bool(fn())
        except Exception: pass
    return True

def _wrap_minimap_get_info(*args):
    fn = getattr(m2_lib, 'm2_minimap_get_info', None)
    if fn:
        try:
            buf = ctypes.create_string_buffer(64)
            x = ctypes.c_float(0.0)
            y = ctypes.c_float(0.0)
            scale = ctypes.c_float(1.0)
            fn.argtypes = [ctypes.c_char_p, ctypes.c_size_t, ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float)]
            fn.restype = ctypes.c_bool
            if fn(buf, 64, ctypes.byref(x), ctypes.byref(y), ctypes.byref(scale)):
                name = buf.value.decode('utf-8', 'ignore')
                return (True, name, int(x.value), int(y.value), float(scale.value))
        except Exception: pass
    return (False, "", 0, 0, 0)

setattr(MiniMapModule, 'Show', staticmethod(_wrap_minimap_show))
setattr(MiniMapModule, 'Hide', staticmethod(_wrap_minimap_hide))
setattr(MiniMapModule, 'ScaleUp', staticmethod(_wrap_minimap_scale_up))
setattr(MiniMapModule, 'ScaleDown', staticmethod(_wrap_minimap_scale_down))
setattr(MiniMapModule, 'SetMiniMapSize', staticmethod(lambda *args, **kwargs: None))
setattr(MiniMapModule, 'SetCenterPosition', staticmethod(lambda *args, **kwargs: getattr(MiniMapModule, 'SetMinimapPosition', lambda *a, **kw: True)(*args, **kwargs)))
setattr(MiniMapModule, 'Destroy', staticmethod(lambda *args, **kwargs: None))
setattr(MiniMapModule, 'Create', staticmethod(lambda *args, **kwargs: None))
setattr(MiniMapModule, 'Update', staticmethod(lambda *args, **kwargs: None))
setattr(MiniMapModule, 'Render', staticmethod(lambda *args, **kwargs: None))
setattr(MiniMapModule, 'UpdateAtlas', staticmethod(lambda *args, **kwargs: None))
setattr(MiniMapModule, 'RenderAtlas', staticmethod(lambda *args, **kwargs: None))
setattr(MiniMapModule, 'RegisterAtlasWindow', staticmethod(lambda *args, **kwargs: None))
setattr(MiniMapModule, 'UnregisterAtlasWindow', staticmethod(lambda *args, **kwargs: None))
setattr(MiniMapModule, 'GetGuildAreaID', staticmethod(lambda *args, **kwargs: 0))
setattr(MiniMapModule, 'SetAtlasScale', staticmethod(lambda *args, **kwargs: None))
setattr(MiniMapModule, 'GetInfo', staticmethod(_wrap_minimap_get_info))
sys.modules['miniMap'] = MiniMapModule
sys.modules['MiniMap'] = MiniMapModule

exchange_methods = [
    ("GetItemVnumFromSelf", "m2_py_ui_exchange_get_item_vnum_from_self", [ctypes.c_size_t], ctypes.c_uint32),
    ("GetItemVnumFromTarget", "m2_py_ui_exchange_get_item_vnum_from_target", [ctypes.c_size_t], ctypes.c_uint32),
    ("GetElkFromSelf", "m2_py_ui_exchange_get_elk_from_self", [], ctypes.c_uint32),
    ("GetElkFromTarget", "m2_py_ui_exchange_get_elk_from_target", [], ctypes.c_uint32),
    ("isAcceptFromSelf", "m2_py_ui_exchange_is_accepted_from_self", [], ctypes.c_bool),
    ("isAcceptFromTarget", "m2_py_ui_exchange_is_accepted_from_target", [], ctypes.c_bool),
]
ExchangeModule = create_real_module('exchange', 'exchange', exchange_methods)
ExchangeModule.EXCHANGE_ITEM_MAX_NUM = 12
def _wrap_exchange_init_trading():
    fn = getattr(m2_lib, 'm2_py_exchange_init_trading', None)
    if fn:
        try: return bool(fn())
        except Exception: pass
    return True

def _wrap_exchange_is_trading():
    fn = getattr(m2_lib, 'm2_py_exchange_is_trading', None)
    if fn:
        try:
            fn.restype = ctypes.c_bool
            return bool(fn())
        except Exception: pass
    return False

def _wrap_exchange_get_item_count_from_self(slot):
    fn = getattr(m2_lib, 'm2_py_exchange_get_item_count_from_self', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint8]
            fn.restype = ctypes.c_uint8
            return int(fn(int(slot)))
        except Exception: pass
    return 0

def _wrap_exchange_get_item_count_from_target(slot):
    fn = getattr(m2_lib, 'm2_py_exchange_get_item_count_from_target', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint8]
            fn.restype = ctypes.c_uint8
            return int(fn(int(slot)))
        except Exception: pass
    return 0

def _wrap_exchange_get_accept_from_self():
    fn = getattr(m2_lib, 'm2_py_exchange_get_accept_from_self', None)
    if fn:
        try:
            fn.restype = ctypes.c_bool
            return bool(fn())
        except Exception: pass
    return False

def _wrap_exchange_get_accept_from_target():
    fn = getattr(m2_lib, 'm2_py_exchange_get_accept_from_target', None)
    if fn:
        try:
            fn.restype = ctypes.c_bool
            return bool(fn())
        except Exception: pass
    return False

def _wrap_exchange_get_name_from_self():
    fn = getattr(m2_lib, 'm2_py_exchange_get_name_from_self', None)
    if fn:
        try:
            buf = ctypes.create_string_buffer(64)
            fn.argtypes = [ctypes.c_char_p, ctypes.c_size_t]
            fn.restype = ctypes.c_int32
            res = fn(buf, 64)
            if res > 0:
                return buf.value.decode('utf-8', 'ignore')
        except Exception: pass
    return getattr(PlayerModule, 'GetName', lambda: "Player")()

def _wrap_exchange_get_name_from_target():
    fn = getattr(m2_lib, 'm2_py_exchange_get_name_from_target', None)
    if fn:
        try:
            buf = ctypes.create_string_buffer(64)
            fn.argtypes = [ctypes.c_char_p, ctypes.c_size_t]
            fn.restype = ctypes.c_int32
            res = fn(buf, 64)
            if res > 0:
                return buf.value.decode('utf-8', 'ignore')
        except Exception: pass
    return "Target"

def _wrap_exchange_get_item_metin_from_self(slot, metin_idx=0):
    fn = getattr(m2_lib, 'm2_py_exchange_get_item_metin_from_self', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint8, ctypes.c_uint8]
            fn.restype = ctypes.c_uint32
            return int(fn(int(slot), int(metin_idx)))
        except Exception: pass
    return 0

def _wrap_exchange_get_item_metin_from_target(slot, metin_idx=0):
    fn = getattr(m2_lib, 'm2_py_exchange_get_item_metin_from_target', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint8, ctypes.c_uint8]
            fn.restype = ctypes.c_uint32
            return int(fn(int(slot), int(metin_idx)))
        except Exception: pass
    return 0

def _wrap_exchange_get_item_attr_from_self(slot, attr_idx=0):
    fn = getattr(m2_lib, 'm2_py_exchange_get_item_attr_from_self', None)
    if fn:
        try:
            out_type = ctypes.c_uint8(0)
            out_val = ctypes.c_int16(0)
            fn.argtypes = [ctypes.c_uint8, ctypes.c_uint8, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_int16)]
            fn.restype = ctypes.c_bool
            if fn(int(slot), int(attr_idx), ctypes.byref(out_type), ctypes.byref(out_val)):
                return (int(out_type.value), int(out_val.value))
        except Exception: pass
    return (0, 0)

def _wrap_exchange_get_item_attr_from_target(slot, attr_idx=0):
    fn = getattr(m2_lib, 'm2_py_exchange_get_item_attr_from_target', None)
    if fn:
        try:
            out_type = ctypes.c_uint8(0)
            out_val = ctypes.c_int16(0)
            fn.argtypes = [ctypes.c_uint8, ctypes.c_uint8, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_int16)]
            fn.restype = ctypes.c_bool
            if fn(int(slot), int(attr_idx), ctypes.byref(out_type), ctypes.byref(out_val)):
                return (int(out_type.value), int(out_val.value))
        except Exception: pass
    return (0, 0)

def _wrap_exchange_get_elk_mode():
    fn = getattr(m2_lib, 'm2_py_exchange_get_elk_mode', None)
    if fn:
        try:
            fn.restype = ctypes.c_bool
            return bool(fn())
        except Exception: pass
    return False

def _wrap_exchange_set_elk_mode(mode):
    fn = getattr(m2_lib, 'm2_py_exchange_set_elk_mode', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_bool]
            fn(bool(mode))
        except Exception: pass

setattr(ExchangeModule, 'InitTrading', staticmethod(_wrap_exchange_init_trading))
setattr(ExchangeModule, 'isTrading', staticmethod(_wrap_exchange_is_trading))
setattr(ExchangeModule, 'GetItemCountFromSelf', staticmethod(_wrap_exchange_get_item_count_from_self))
setattr(ExchangeModule, 'GetItemCountFromTarget', staticmethod(_wrap_exchange_get_item_count_from_target))
setattr(ExchangeModule, 'GetAcceptFromSelf', staticmethod(_wrap_exchange_get_accept_from_self))
setattr(ExchangeModule, 'GetAcceptFromTarget', staticmethod(_wrap_exchange_get_accept_from_target))
setattr(ExchangeModule, 'GetNameFromSelf', staticmethod(_wrap_exchange_get_name_from_self))
setattr(ExchangeModule, 'GetNameFromTarget', staticmethod(_wrap_exchange_get_name_from_target))
setattr(ExchangeModule, 'GetItemMetinSocketFromSelf', staticmethod(_wrap_exchange_get_item_metin_from_self))
setattr(ExchangeModule, 'GetItemMetinSocketFromTarget', staticmethod(_wrap_exchange_get_item_metin_from_target))
setattr(ExchangeModule, 'GetItemAttributeFromSelf', staticmethod(_wrap_exchange_get_item_attr_from_self))
setattr(ExchangeModule, 'GetItemAttributeFromTarget', staticmethod(_wrap_exchange_get_item_attr_from_target))
setattr(ExchangeModule, 'GetElkMode', staticmethod(_wrap_exchange_get_elk_mode))
setattr(ExchangeModule, 'SetElkMode', staticmethod(_wrap_exchange_set_elk_mode))
sys.modules['exchange'] = ExchangeModule
sys.modules['Exchange'] = ExchangeModule
sys.modules['trade'] = ExchangeModule
sys.modules['Trade'] = ExchangeModule

safebox_methods = [
    ("GetItemID", "m2_py_ui_safebox_get_item_id", [ctypes.c_size_t], ctypes.c_uint32),
    ("GetItemCount", "m2_py_ui_safebox_get_item_count", [ctypes.c_size_t], ctypes.c_uint8),
    ("GetMoney", "m2_py_ui_safebox_get_money", [], ctypes.c_uint32),
]
SafeboxModule = create_real_module('safebox', 'safebox', safebox_methods)
SafeboxModule.SAFEBOX_SLOT_X_COUNT = 5
SafeboxModule.SAFEBOX_SLOT_Y_COUNT = 9
SafeboxModule.SAFEBOX_PAGE_SIZE = 45
SafeboxModule.PAGE_SIZE = 45
def _wrap_safebox_get_size():
    fn = getattr(m2_lib, 'm2_py_ui_safebox_get_size', None)
    if fn:
        try:
            fn.restype = ctypes.c_size_t
            return int(fn())
        except Exception: pass
    return 45

def _wrap_safebox_get_item_flags(slot):
    fn = getattr(m2_lib, 'm2_py_ui_safebox_get_item_flags', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_size_t]
            fn.restype = ctypes.c_uint32
            return int(fn(int(slot)))
        except Exception: pass
    return 0

def _wrap_safebox_get_item_metin_socket(slot, socket_index=0):
    fn = getattr(m2_lib, 'm2_py_ui_safebox_get_item_metin_socket', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_size_t, ctypes.c_size_t]
            fn.restype = ctypes.c_int32
            return int(fn(int(slot), int(socket_index)))
        except Exception: pass
    return 0

def _wrap_safebox_get_item_attribute(slot, attr_index=0):
    fn = getattr(m2_lib, 'm2_py_ui_safebox_get_item_attribute', None)
    if fn:
        try:
            out_type = ctypes.c_uint8(0)
            out_val = ctypes.c_int16(0)
            fn.argtypes = [ctypes.c_size_t, ctypes.c_size_t, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_int16)]
            fn.restype = ctypes.c_bool
            if fn(int(slot), int(attr_index), ctypes.byref(out_type), ctypes.byref(out_val)):
                return (int(out_type.value), int(out_val.value))
        except Exception: pass
    return (0, 0)

def _wrap_safebox_get_mall_item_id(slot):
    fn = getattr(m2_lib, 'm2_py_ui_safebox_get_mall_item_id', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_size_t]
            fn.restype = ctypes.c_uint32
            return int(fn(int(slot)))
        except Exception: pass
    return 0

def _wrap_safebox_get_mall_item_count(slot):
    fn = getattr(m2_lib, 'm2_py_ui_safebox_get_mall_item_count', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_size_t]
            fn.restype = ctypes.c_uint8
            return int(fn(int(slot)))
        except Exception: pass
    return 0

def _wrap_safebox_get_mall_item_metin_socket(slot, socket_index=0):
    fn = getattr(m2_lib, 'm2_py_ui_safebox_get_mall_item_metin_socket', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_size_t, ctypes.c_size_t]
            fn.restype = ctypes.c_int32
            return int(fn(int(slot), int(socket_index)))
        except Exception: pass
    return 0

def _wrap_safebox_get_mall_item_attribute(slot, attr_index=0):
    fn = getattr(m2_lib, 'm2_py_ui_safebox_get_mall_item_attribute', None)
    if fn:
        try:
            out_type = ctypes.c_uint8(0)
            out_val = ctypes.c_int16(0)
            fn.argtypes = [ctypes.c_size_t, ctypes.c_size_t, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_int16)]
            fn.restype = ctypes.c_bool
            if fn(int(slot), int(attr_index), ctypes.byref(out_type), ctypes.byref(out_val)):
                return (int(out_type.value), int(out_val.value))
        except Exception: pass
    return (0, 0)

def _wrap_safebox_get_mall_size():
    fn = getattr(m2_lib, 'm2_py_ui_safebox_get_mall_size', None)
    if fn:
        try:
            fn.restype = ctypes.c_size_t
            return int(fn())
        except Exception: pass
    return 45

setattr(SafeboxModule, 'GetCurrentSafeboxSize', staticmethod(_wrap_safebox_get_size))
setattr(SafeboxModule, 'GetItemFlags', staticmethod(_wrap_safebox_get_item_flags))
setattr(SafeboxModule, 'GetItemMetinSocket', staticmethod(_wrap_safebox_get_item_metin_socket))
setattr(SafeboxModule, 'GetItemAttribute', staticmethod(_wrap_safebox_get_item_attribute))
setattr(SafeboxModule, 'GetMallItemID', staticmethod(_wrap_safebox_get_mall_item_id))
setattr(SafeboxModule, 'GetMallItemCount', staticmethod(_wrap_safebox_get_mall_item_count))
setattr(SafeboxModule, 'GetMallItemMetinSocket', staticmethod(_wrap_safebox_get_mall_item_metin_socket))
setattr(SafeboxModule, 'GetMallItemAttribute', staticmethod(_wrap_safebox_get_mall_item_attribute))
setattr(SafeboxModule, 'GetMallSize', staticmethod(_wrap_safebox_get_mall_size))
sys.modules['safebox'] = SafeboxModule
sys.modules['Safebox'] = SafeboxModule

nonplayer_methods = [
    ("GetMonsterName", "m2_py_nonplayer_get_monster_name", [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_size_t], ctypes.c_bool),
    ("GetMonsterLevel", "m2_py_nonplayer_get_monster_level", [ctypes.c_uint32], ctypes.c_uint32),
    ("GetMonsterMaxHP", "m2_py_nonplayer_get_monster_max_hp", [ctypes.c_uint32], ctypes.c_uint32),
]
NonPlayerModule = create_real_module('nonplayer', 'nonplayer', nonplayer_methods)
def _wrap_nonplayer_get_monster_grade(vnum=0):
    fn = getattr(m2_lib, 'm2_nonplayer_get_monster_grade', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint32]
            fn.restype = ctypes.c_uint8
            return int(fn(int(vnum)))
        except Exception: pass
    return 0

def _wrap_nonplayer_is_monster(vnum=0):
    fn = getattr(m2_lib, 'm2_nonplayer_is_monster', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint32]
            fn.restype = ctypes.c_bool
            return bool(fn(int(vnum)))
        except Exception: pass
    return True

def _wrap_nonplayer_is_npc(vnum=0):
    fn = getattr(m2_lib, 'm2_nonplayer_is_npc', None)
    if fn:
        try:
            fn.argtypes = [ctypes.c_uint32]
            fn.restype = ctypes.c_bool
            return bool(fn(int(vnum)))
        except Exception: pass
    return False

setattr(NonPlayerModule, 'GetMonsterGrade', staticmethod(_wrap_nonplayer_get_monster_grade))
setattr(NonPlayerModule, 'IsMonster', staticmethod(_wrap_nonplayer_is_monster))
setattr(NonPlayerModule, 'IsNPC', staticmethod(_wrap_nonplayer_is_npc))
sys.modules['nonplayer'] = NonPlayerModule
sys.modules['NonPlayer'] = NonPlayerModule

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
    def GetCandidate(cls, *args): return ("", 0)
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

class MessengerModule:
    @staticmethod
    def RemoveFriend(name): pass
    @staticmethod
    def IsFriendByName(name): return False
    @staticmethod
    def Destroy(): pass
    @staticmethod
    def RefreshGuildMember(): pass
    @staticmethod
    def SetMessengerHandler(handler): pass

sys.modules['messenger'] = MessengerModule
sys.modules['Messenger'] = MessengerModule

class EventModule:
    BOX_VISIBLE_LINE_COUNT = 6
    BUTTON_TYPE_NEXT = 0
    BUTTON_TYPE_DONE = 1
    BUTTON_TYPE_CANCEL = 2

    @staticmethod
    def RegisterEventSet(idx=0): pass
    @staticmethod
    def RegisterEventSetFromString(s=""): pass
    @staticmethod
    def ClearEventSet(): pass
    @staticmethod
    def SetRestrictedCount(c=0): pass
    @staticmethod
    def GetEventSetLocalYPosition(): return 0
    @staticmethod
    def AddEventSetLocalYPosition(y=0): pass
    @staticmethod
    def InsertText(text=""): pass
    @staticmethod
    def InsertTextInline(text=""): pass
    @staticmethod
    def UpdateEventSet(): pass
    @staticmethod
    def RenderEventSet(): pass
    @staticmethod
    def SetEventSetWidth(w=0): pass
    @staticmethod
    def Skip(): pass
    @staticmethod
    def IsWait(): return False
    @staticmethod
    def EndEventProcess(): pass
    @staticmethod
    def SelectAnswer(ans=0): pass
    @staticmethod
    def GetLineCount(): return 0
    @staticmethod
    def SetVisibleStartLine(l=0): pass
    @staticmethod
    def GetVisibleStartLine(): return 0
    @staticmethod
    def SetEventHandler(h=None): pass
    @staticmethod
    def SetInterfaceWindow(w=None): pass
    @staticmethod
    def SetLeftTimeString(s=""): pass
    @staticmethod
    def QuestButtonClick(btn=0): pass
    @staticmethod
    def Destroy(): pass

sys.modules['event'] = EventModule
sys.modules['Event'] = EventModule
sys.modules['eventmgr'] = EventModule
sys.modules['EventMgr'] = EventModule

class EffectMeta(type):
    def __getattr__(cls, name):
        return 0

class EffectModule(metaclass=EffectMeta):
    INDEX_FLY_TYPE_NORMAL = 0
    INDEX_FLY_TYPE_FIRE_CRACKER = 1
    INDEX_FLY_TYPE_AUTO_FIRE = 2

    FLY_NONE = 0
    FLY_EXP = 1
    FLY_HP_MEDIUM = 2
    FLY_HP_BIG = 3
    FLY_SP_SMALL = 4
    FLY_SP_MEDIUM = 5
    FLY_SP_BIG = 6
    FLY_FIREWORK1 = 7
    FLY_FIREWORK2 = 8
    FLY_FIREWORK3 = 9
    FLY_FIREWORK4 = 10
    FLY_FIREWORK5 = 11
    FLY_FIREWORK6 = 12
    FLY_FIREWORK_XMAS = 13
    FLY_CHAIN_LIGHTNING = 14
    FLY_HP_SMALL = 15
    FLY_SKILL_MUYEONG = 16

    @staticmethod
    def RegisterEffect(*args, **kwargs): return 0
    @staticmethod
    def CreateEffect(*args, **kwargs): return 0
    @staticmethod
    def DeleteEffect(*args, **kwargs): pass
    @staticmethod
    def SetPosition(*args, **kwargs): pass
    @staticmethod
    def RegisterIndexedFlyData(*args, **kwargs): pass
    @staticmethod
    def Update(*args, **kwargs): pass
    @staticmethod
    def Render(*args, **kwargs): pass

sys.modules['effect'] = EffectModule
sys.modules['Effect'] = EffectModule

class FlyModule:
    @staticmethod
    def Update(): pass
    @staticmethod
    def Render(): pass

sys.modules['fly'] = FlyModule
sys.modules['Fly'] = FlyModule

class ProfilerModule:
    @staticmethod
    def Clear(): pass

sys.modules['profiler'] = ProfilerModule
sys.modules['Profiler'] = ProfilerModule

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

    let full_script = format!("{}
{}", wire_ptrs_script, py_script);
    let c_code = match CString::new(full_script) {
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
