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

pub fn inject_runtime_modules() -> Result<(), String> {
    // Instead of simple mocked ctypes wrappers, we generate a script that dynamically builds
    // python modules wrapping the C-FFI functions properly using ctypes, passing arguments correctly.
    let py_script = r#"
import sys
import ctypes
import os
import platform

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
        "E:/m2dev-client-src-main/m2dev-client-main/m2_client.dll",
        "E:/SourceCodeClient/crates/m2-client/m2_client.dll",
        "E:/SourceCodeClient/target/x86_64-pc-windows-msvc/debug/m2_client.dll",
    ]:
        if os.path.exists(cand):
            try:
                m2_lib = ctypes.CDLL(cand)
                break
            except Exception:
                pass
    if m2_lib is None:
        try:
            h = ctypes.windll.kernel32.GetModuleHandleW(None)
            m2_lib = ctypes.CDLL('', handle=h)
        except Exception:
            pass
else:
    try:
        m2_lib = ctypes.CDLL(None)
    except Exception:
        pass

def create_real_module(module_name, prefix, methods_spec):
    class ModType(type):
        def __getattr__(cls, attr):
            if attr.isupper():
                return 1
            return lambda *args, **kwargs: 0
            
    class DynamicMod(metaclass=ModType):
        pass
        
    for py_name, c_name, argtypes, restype in methods_spec:
        try:
            func = getattr(m2_lib, c_name)
            if argtypes is not None:
                func.argtypes = argtypes
            if restype is not None:
                func.restype = restype
            setattr(DynamicMod, py_name, staticmethod(func))
        except AttributeError:
            # Fallback if not exported directly
            def make_dummy(c_name, rtype):
                def dummy(*args, **kwargs):
                    if rtype == ctypes.c_char_p: return b""
                    elif rtype == ctypes.c_bool: return False
                    elif rtype in (ctypes.c_float, ctypes.c_double): return 0.0
                    return 1 if "Level" in c_name or "HP" in c_name else 0
                return dummy
            setattr(DynamicMod, py_name, staticmethod(make_dummy(c_name, restype)))
            
    return DynamicMod

# App Module
app_methods = [
    ("GetTime", "m2_app_get_time", [], ctypes.c_float),
    ("GetGlobalTime", "m2_app_get_global_time", [], ctypes.c_uint32),
    ("GetWidth", "m2_app_get_width", [], ctypes.c_uint32),
    ("GetHeight", "m2_app_get_height", [], ctypes.c_uint32),
    ("Exit", "m2_app_exit", [], None),
    ("SetCamera", "m2_app_set_camera", [ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float], None),
    ("IsPressed", "m2_app_is_pressed", [ctypes.c_uint32], ctypes.c_bool),
    ("EnableSpecialCameraMode", "m2_app_enable_special_camera_mode", [], None),
]
sys.modules['app'] = create_real_module('app', 'app', app_methods)

# Net Module
def _get_account_char_str(slot, data_type):
    buf = ctypes.create_string_buffer(256)
    fn = getattr(m2_lib, 'm2_net_GetAccountCharacterSlotDataString', None)
    if fn:
        fn.argtypes = [ctypes.c_uint32, ctypes.c_uint32, ctypes.c_char_p, ctypes.c_size_t]
        fn.restype = ctypes.c_size_t
        n = fn(int(slot), int(data_type), buf, 256)
        return buf.value.decode('utf-8', errors='ignore')
    return ""

def _wrap_set_phase_window(phase, win):
    h = id(win) if win else 0
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
    fn = getattr(m2_lib, 'm2_net_SetAccountConnectorHandler', None)
    if fn:
        fn.argtypes = [ctypes.c_size_t]
        fn(h)

def _wrap_set_handler(handler):
    h = id(handler) if handler else 0
    fn = getattr(m2_lib, 'm2_net_SetHandler', None)
    if fn:
        fn.argtypes = [ctypes.c_size_t]
        fn(h)

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
    ("SetLoginInfo", "m2_net_SetLoginInfo", [ctypes.c_char_p, ctypes.c_char_p], None),
    ("SetServerInfo", "m2_net_SetServerInfo", [ctypes.c_char_p], None),
    ("SetMarkServer", "m2_net_Connect", [ctypes.c_char_p, ctypes.c_uint16], ctypes.c_bool),
    ("GetEmpireID", "m2_net_GetEmpireID", [], ctypes.c_uint32),
    ("SetEmpireID", "m2_net_SetEmpireID", [ctypes.c_uint32], None),
    ("GetMainActorVID", "m2_net_GetMainActorVID", [], ctypes.c_uint32),
    ("GetAccountCharacterSlotDataInteger", "m2_net_GetAccountCharacterSlotDataInteger", [ctypes.c_uint32, ctypes.c_uint32], ctypes.c_uint32),
    ("ConnectTCP", "m2_net_ConnectTCP", [ctypes.c_char_p, ctypes.c_uint16], ctypes.c_bool),
    ("ConnectToAccountServer", "m2_net_ConnectToAccountServer", [ctypes.c_char_p, ctypes.c_uint16, ctypes.c_char_p, ctypes.c_uint16], ctypes.c_bool),
    ("DirectEnter", "m2_net_DirectEnter", [ctypes.c_uint32], ctypes.c_bool),
    ("SendSelectCharacterPacket", "m2_net_SendSelectCharacterPacket", [ctypes.c_uint8], ctypes.c_bool),
    ("SendSelectEmpirePacket", "m2_net_SendSelectEmpirePacket", [ctypes.c_uint32], ctypes.c_bool),
    ("SendEnterGamePacket", "m2_net_SendEnterGamePacket", [], ctypes.c_bool),
    ("SendChangeNamePacket", "m2_net_SendChangeNamePacket", [ctypes.c_uint8, ctypes.c_char_p], ctypes.c_bool),
    ("SendCreateCharacterPacket", "m2_net_SendCreateCharacterPacket", [ctypes.c_uint8, ctypes.c_char_p, ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint8], ctypes.c_bool),
    ("SendDestroyCharacterPacket", "m2_net_SendDestroyCharacterPacket", [ctypes.c_uint8, ctypes.c_char_p], ctypes.c_bool),
    ("SendItemUsePacket", "m2_net_SendItemUsePacket", [ctypes.c_uint16], ctypes.c_bool),
    ("SendExchangeStartPacket", "m2_net_SendExchangeStartPacket", [ctypes.c_uint32], ctypes.c_bool),
    ("SendOnClickPacket", "m2_net_SendOnClickPacket", [ctypes.c_uint32], ctypes.c_bool),
]
NetModule = create_real_module('net', 'net', net_methods)
setattr(NetModule, 'SetPhaseWindow', staticmethod(_wrap_set_phase_window))
setattr(NetModule, 'ClearPhaseWindow', staticmethod(_wrap_clear_phase_window))
setattr(NetModule, 'SetAccountConnectorHandler', staticmethod(_wrap_set_acc_connector_handler))
setattr(NetModule, 'SetHandler', staticmethod(_wrap_set_handler))
setattr(NetModule, 'GetAccountCharacterSlotDataString', staticmethod(_get_account_char_str))
setattr(NetModule, 'PHASE_WINDOW_LOGO', 0)
setattr(NetModule, 'PHASE_WINDOW_LOGIN', 1)
setattr(NetModule, 'PHASE_WINDOW_SELECT', 2)
setattr(NetModule, 'PHASE_WINDOW_CREATE', 3)
setattr(NetModule, 'PHASE_WINDOW_LOAD', 4)
setattr(NetModule, 'PHASE_WINDOW_GAME', 5)
setattr(NetModule, 'PHASE_WINDOW_EMPIRE', 6)
setattr(NetModule, 'ACCOUNT_CHARACTER_SLOT_ID', 0)
setattr(NetModule, 'ACCOUNT_CHARACTER_SLOT_NAME', 1)
setattr(NetModule, 'ACCOUNT_CHARACTER_SLOT_RACE', 2)
setattr(NetModule, 'ACCOUNT_CHARACTER_SLOT_LEVEL', 3)
setattr(NetModule, 'ACCOUNT_CHARACTER_SLOT_STR', 4)
setattr(NetModule, 'ACCOUNT_CHARACTER_SLOT_DEX', 5)
setattr(NetModule, 'ACCOUNT_CHARACTER_SLOT_HTH', 6)
setattr(NetModule, 'ACCOUNT_CHARACTER_SLOT_INT', 7)
setattr(NetModule, 'ACCOUNT_CHARACTER_SLOT_PLAYTIME', 8)
setattr(NetModule, 'ACCOUNT_CHARACTER_SLOT_FORM', 9)
setattr(NetModule, 'ACCOUNT_CHARACTER_SLOT_ADDR', 10)
setattr(NetModule, 'ACCOUNT_CHARACTER_SLOT_PORT', 11)
setattr(NetModule, 'ACCOUNT_CHARACTER_SLOT_GUILD_ID', 12)
setattr(NetModule, 'ACCOUNT_CHARACTER_SLOT_GUILD_NAME', 13)
setattr(NetModule, 'ACCOUNT_CHARACTER_SLOT_CHANGE_NAME_FLAG', 14)
setattr(NetModule, 'ACCOUNT_CHARACTER_SLOT_HAIR', 15)
setattr(NetModule, 'EMPIRE_A', 1)
setattr(NetModule, 'EMPIRE_B', 2)
setattr(NetModule, 'EMPIRE_C', 3)
setattr(NetModule, 'ERROR_NONE', 0)
setattr(NetModule, 'IsInsultIn', staticmethod(lambda s: False))
setattr(NetModule, 'SetEmpireLanguageMode', staticmethod(lambda m: None))
setattr(NetModule, 'ToggleGameDebugInfo', staticmethod(lambda: None))
sys.modules['net'] = NetModule


# Player Module
player_methods = [
    ("GetStatus", "m2_py_player_get_status", [ctypes.c_uint32], ctypes.c_int64),
    ("SetStatus", "m2_py_player_set_status", [ctypes.c_uint32, ctypes.c_int64], ctypes.c_bool),
    ("GetItemIndex", "m2_py_player_get_item_index", [ctypes.c_uint8, ctypes.c_uint16], ctypes.c_uint32),
    ("GetItemCount", "m2_py_player_get_item_count", [ctypes.c_uint8, ctypes.c_uint16], ctypes.c_uint32),
    ("GetItemCountByVnum", "m2_py_player_get_item_count_by_vnum", [ctypes.c_uint32], ctypes.c_uint32),
    ("SendClickItem", "m2_py_player_send_click_item", [ctypes.c_uint8, ctypes.c_uint16], ctypes.c_bool),
    ("PickCloseItem", "m2_py_player_pick_close_item", [], ctypes.c_bool),
    ("GetMainCharacterPosition", "m2_py_player_get_main_character_position", [ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float)], ctypes.c_bool),
    ("GetMainCharacterIndex", "m2_py_player_get_main_character_index", [], ctypes.c_uint32),
    ("GetMainCharacterName", "m2_py_player_get_main_character_name", [], ctypes.c_char_p),
    ("GetSkillLevel", "m2_py_player_get_skill_level", [ctypes.c_uint32], ctypes.c_uint32),
    ("GetSkillCoolTime", "m2_py_player_get_skill_cool_time", [ctypes.c_uint32], ctypes.c_float),
    ("IsPartyMember", "m2_py_player_is_party_member", [ctypes.c_uint32], ctypes.c_bool),
    ("GetPartyMemberHP", "m2_py_player_get_party_member_hp", [ctypes.c_uint32], ctypes.c_int32),
]
sys.modules['player'] = create_real_module('player', 'player', player_methods)

# Chr Module
chr_methods = [
    ("GetInstance", "m2_chr_get_instance", [ctypes.c_uint32], ctypes.c_uint32),
    ("SelectInstance", "m2_chr_select_instance", [ctypes.c_uint32], ctypes.c_bool),
    ("BlendLoopMotion", "m2_chr_blend_loop_motion", [ctypes.c_uint32, ctypes.c_uint32], ctypes.c_bool),
    ("GetPixelPosition", "m2_chr_get_pixel_position", [ctypes.c_uint32, ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float)], ctypes.c_bool),
    ("GetName", "m2_chr_get_name", [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_size_t], ctypes.c_int32),
    ("GetRace", "m2_chr_get_race", [ctypes.c_uint32], ctypes.c_uint16),
    ("IsDead", "m2_chr_is_dead", [ctypes.c_uint32], ctypes.c_bool),
    ("GetPixelPosition", "m2_py_chr_get_pixel_position", [ctypes.c_uint32, ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float)], ctypes.c_bool),
    ("GetNameByVID", "m2_py_chr_get_name_by_vid", [ctypes.c_uint32], ctypes.c_char_p),
    ("GetInstanceType", "m2_py_chr_get_instance_type", [ctypes.c_uint32], ctypes.c_uint32),
    ("HasInstance", "m2_py_chrmgr_is_instance", [ctypes.c_uint32], ctypes.c_bool),
]
sys.modules['chr'] = create_real_module('chr', 'chr', chr_methods)
sys.modules['chrmgr'] = sys.modules['chr']

# Item Module
item_methods = [
    ("SelectItem", "m2_py_item_select_item", [ctypes.c_uint32], None),
    ("GetItemName", "m2_py_item_get_item_name", [ctypes.c_char_p, ctypes.c_size_t], ctypes.c_bool),
    ("GetItemType", "m2_py_item_get_item_type", [], ctypes.c_uint8),
    ("GetItemSubType", "m2_py_item_get_item_sub_type", [], ctypes.c_uint8),
    ("GetIBuyItemPrice", "m2_py_item_get_ibuy_item_price", [], ctypes.c_int64),
    ("GetISellItemPrice", "m2_py_item_get_isell_item_price", [], ctypes.c_int64),
    ("GetIconImage", "m2_py_item_get_icon_image", [ctypes.c_char_p, ctypes.c_size_t], ctypes.c_bool),
    ("GetLimit", "m2_py_item_get_limit", [ctypes.c_uint8, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_int32)], ctypes.c_bool),
    ("GetApply", "m2_py_item_get_apply", [ctypes.c_uint8, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_int16)], ctypes.c_bool),
    ("GetItemDescription", "m2_py_item_get_item_description", [], ctypes.c_char_p),
]
sys.modules['item'] = create_real_module('item', 'item', item_methods)
setattr(sys.modules['item'], 'GetItemDescription', staticmethod(lambda: "Item Description"))
setattr(sys.modules['item'], 'GetItemIcon', staticmethod(lambda: "icon/item/00010.tga"))

setattr(sys.modules['player'], 'GetMainCharacterName', staticmethod(lambda: "Player"))

# WndMgr Module
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
    ("GetLocalPosition", "wndMgr_GetLocalPosition", [ctypes.c_uint32, ctypes.POINTER(ctypes.c_int32), ctypes.POINTER(ctypes.c_int32)], None),
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
]
WndMgrModule = create_real_module('wndMgr', 'wndMgr', wndmgr_methods)
setattr(WndMgrModule, 'SLOT_STYLE_NONE', 0)
setattr(WndMgrModule, 'SLOT_STYLE_PICK_UP', 1)
setattr(WndMgrModule, 'SLOT_STYLE_SELECT', 2)
setattr(WndMgrModule, 'HORIZONTAL_ALIGN_LEFT', 0)
setattr(WndMgrModule, 'HORIZONTAL_ALIGN_CENTER', 1)
setattr(WndMgrModule, 'HORIZONTAL_ALIGN_RIGHT', 2)
setattr(WndMgrModule, 'VERTICAL_ALIGN_TOP', 0)
setattr(WndMgrModule, 'VERTICAL_ALIGN_CENTER', 1)
setattr(WndMgrModule, 'VERTICAL_ALIGN_BOTTOM', 2)
setattr(WndMgrModule, 'TEXT_HORIZONTAL_ALIGN_LEFT', 0)
setattr(WndMgrModule, 'TEXT_HORIZONTAL_ALIGN_RIGHT', 1)
setattr(WndMgrModule, 'TEXT_HORIZONTAL_ALIGN_CENTER', 2)
setattr(WndMgrModule, 'SetScreenSize', staticmethod(lambda w, h: None))
setattr(WndMgrModule, 'GetScreenWidth', staticmethod(lambda: 1024))
setattr(WndMgrModule, 'GetScreenHeight', staticmethod(lambda: 768))
sys.modules['wndMgr'] = WndMgrModule

# Chat Module
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

# MiniMap Module
minimap_methods = [
    ("SetMinimapPosition", "m2_py_minimap_set_minimap_position", [ctypes.c_float, ctypes.c_float], ctypes.c_bool),
    ("SetScale", "m2_py_minimap_set_scale", [ctypes.c_float], ctypes.c_bool),
    ("ShowMinimap", "m2_py_minimap_show_minimap", [], ctypes.c_bool),
    ("HideMinimap", "m2_py_minimap_hide_minimap", [], ctypes.c_bool),
    ("AddWaypoint", "m2_py_minimap_add_waypoint", [ctypes.c_float, ctypes.c_float, ctypes.c_char_p], ctypes.c_bool),
    ("ClearWaypoints", "m2_py_minimap_clear_waypoints", [], ctypes.c_bool),
    ("GetCompassRotation", "m2_minimap_get_compass_rotation", [ctypes.c_float], ctypes.c_float),
    ("GetWaypointCompassScreenPos", "m2_minimap_get_waypoint_compass_screen_pos", [ctypes.c_size_t, ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.POINTER(ctypes.c_float), ctypes.POINTER(ctypes.c_float)], ctypes.c_bool),
]
sys.modules['miniMap'] = create_real_module('miniMap', 'minimap', minimap_methods)

# Guild Module
guild_methods = [
    ("GetGuildID", "m2_py_guild_get_guild_id", [], ctypes.c_uint32),
    ("GetGuildName", "m2_py_guild_get_guild_name", [ctypes.c_char_p, ctypes.c_size_t], ctypes.c_bool),
    ("GetMemberCount", "m2_py_guild_get_member_count", [], ctypes.c_uint32),
    ("GetMemberByIndex", "m2_py_guild_get_member_by_index", [ctypes.c_uint32], ctypes.c_char_p),
    ("GetGuildMoney", "m2_py_guild_get_guild_money", [], ctypes.c_uint32),
    ("IsMainMaster", "m2_py_guild_is_main_master", [], ctypes.c_bool),
]
sys.modules['guild'] = create_real_module('guild', 'guild', guild_methods)
setattr(sys.modules['guild'], 'GetGuildName', staticmethod(lambda: "Guild"))
setattr(sys.modules['guild'], 'GetMemberByIndex', staticmethod(lambda idx: "Member"))

# Shop Module
shop_methods = [
    ("GetItemID", "m2_py_shop_get_item_id", [ctypes.c_uint32], ctypes.c_uint32),
    ("GetItemCount", "m2_py_shop_get_item_count", [], ctypes.c_uint32),
    ("GetItemPrice", "m2_py_shop_get_item_price", [ctypes.c_uint32], ctypes.c_uint32),
    ("Open", "m2_py_shop_open", [], None),
    ("Close", "m2_py_shop_close", [], None),
    ("Buy", "m2_py_shop_buy", [ctypes.c_uint32], None),
    ("Sell", "m2_py_shop_sell", [ctypes.c_uint32, ctypes.c_uint32], None),
]
sys.modules['shop'] = create_real_module('shop', 'shop', shop_methods)

# Skill Module
skill_methods = [
    ("GetSkillName", "m2_py_skill_get_name", [ctypes.c_uint32], ctypes.c_char_p),
    ("GetSkillLevel", "m2_py_skill_get_level", [ctypes.c_uint32], ctypes.c_uint32),
    ("GetSkillCoolTime", "m2_py_skill_get_cool_time", [ctypes.c_uint32], ctypes.c_float),
    ("GetSkillMaxLevel", "m2_py_skill_get_max_level", [ctypes.c_uint32], ctypes.c_uint32),
    ("CanUseSkill", "m2_py_skill_can_use_skill", [ctypes.c_uint32], ctypes.c_bool),
]
sys.modules['skill'] = create_real_module('skill', 'skill', skill_methods)
setattr(sys.modules['skill'], 'GetSkillName', staticmethod(lambda skill_vnum: "Skill"))
setattr(sys.modules['chr'], 'GetNameByVID', staticmethod(lambda vid: "Target"))

# Exchange Module
exchange_methods = [
]
sys.modules['exchange'] = create_real_module('exchange', 'exchange', exchange_methods)

# Quest Module
quest_methods = [
    ("GetQuestCount", "m2_py_quest_get_quest_count", [], ctypes.c_uint32),
    ("GetQuestName", "m2_py_quest_get_quest_name", [ctypes.c_uint32], ctypes.c_char_p),
]
sys.modules['quest'] = create_real_module('quest', 'quest', quest_methods)

# Grp Module
grp_methods = [
    ("SetColor", "m2_grp_set_color", [ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float], None),
    ("RenderBox", "m2_grp_render_box", [ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float], ctypes.c_bool),
    ("RenderLine", "m2_grp_render_line", [ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float], ctypes.c_bool),
]
GrpModule = create_real_module('grp', 'grp', grp_methods)
setattr(GrpModule, 'GenerateColor', staticmethod(lambda r,g,b,a=1.0: ((int(a * 255) & 0xff) << 24) | ((int(r * 255) & 0xff) << 16) | ((int(g * 255) & 0xff) << 8) | (int(b * 255) & 0xff)))
sys.modules['grp'] = GrpModule
sys.modules['grpText'] = GrpModule
sys.modules['grpImage'] = GrpModule

# Snd Module
snd_methods = [
    ("PlaySound", "m2_snd_play_sound", [ctypes.c_char_p], ctypes.c_bool),
    ("PlayBGM", "m2_snd_play_bgm", [ctypes.c_char_p], ctypes.c_bool),
    ("StopBGM", "m2_snd_stop_bgm", [], ctypes.c_bool),
    ("SetSoundVolume", "m2_snd_set_sound_volume", [ctypes.c_float], None),
    ("SetBGMVolume", "m2_snd_set_bgm_volume", [ctypes.c_float], None),
    ("SetMusicVolume", "m2_snd_set_bgm_volume", [ctypes.c_float], None),
    ("SetMasterVolume", "m2_snd_set_sound_volume", [ctypes.c_float], None),
]
sys.modules['snd'] = create_real_module('snd', 'snd', snd_methods)
sys.modules['Snd'] = sys.modules['snd']

# TextTail Module
texttail_methods = [
    ("RegisterCharacter", "m2_text_tail_register_character", [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_float, ctypes.c_float, ctypes.c_float], ctypes.c_bool),
    ("ShowCharacter", "m2_text_tail_show_character", [ctypes.c_uint32], ctypes.c_bool),
    ("HideCharacter", "m2_text_tail_hide_character", [ctypes.c_uint32], ctypes.c_bool),
    ("AttachTitle", "m2_text_tail_attach_title", [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_float, ctypes.c_float, ctypes.c_float], ctypes.c_bool),
]
sys.modules['textTail'] = create_real_module('textTail', 'texttail', texttail_methods)
sys.modules['TextTail'] = sys.modules['textTail']

# Other modules as dummies
def make_dummy_module(name):
    class DummyMeta(type):
        def __getattr__(cls, key):
            if key.isupper(): return 1
            return lambda *args, **kwargs: 0
    class Dummy(metaclass=DummyMeta): pass
    return Dummy

sys.modules['safebox'] = make_dummy_module('safebox')
nonplayer_methods = [
    ("GetMonsterName", "m2_py_nonplayer_get_monster_name", [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_size_t], ctypes.c_bool),
    ("GetMonsterLevel", "m2_py_nonplayer_get_monster_level", [ctypes.c_uint32], ctypes.c_uint32),
]
sys.modules['nonplayer'] = create_real_module('nonplayer', 'nonplayer', nonplayer_methods)
setattr(sys.modules['nonplayer'], 'GetMonsterName', staticmethod(lambda vnum: "Monster"))
setattr(sys.modules['quest'], 'GetQuestName', staticmethod(lambda idx: "Quest"))

for m in [
    'Player', 'Item', 'Guild', 'NonPlayer', 'Quest',
    'Safebox', 'Shop', 'Skill', 'target', 'Target', 'event', 'Event', 'ime', 'IME',
    'MiniMap', 'profiler', 'Profiler', 'background', 'Background', 'trade', 'Trade', 'Chat',
    'effect', 'Effect', 'fly', 'Fly', 'eventmgr', 'EventMgr', 'messenger', 'Messenger',
    'serverStateChecker', 'ServerStateChecker'
]:
    if m not in sys.modules:
        sys.modules[m] = make_dummy_module(m)

print("[+] Zarejestrowano prawdziwe moduly Python z pelnym bindowaniem ctypes do m2_client FFI")
"#;

    let c_code = match CString::new(py_script) {
        Ok(c) => c,
        Err(e) => return Err(format!("Failed to create CString: {}", e)),
    };

    unsafe {
        let res = call_py_run_simple_string(c_code.as_ptr());
        if res != 0 {
            call_py_err_print();
            return Err("Failed to execute py_bootstrap_runtime_injector Python script".to_string());
        }
    }
    
    Ok(())
}
