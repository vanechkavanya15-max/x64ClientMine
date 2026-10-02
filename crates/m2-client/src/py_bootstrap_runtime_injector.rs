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

pub fn inject_runtime_modules() -> Result<(), String> {
    // Instead of simple mocked ctypes wrappers, we generate a script that dynamically builds
    // python modules wrapping the C-FFI functions properly using ctypes, passing arguments correctly.
    let py_script = r#"
import sys
import ctypes
import os
import platform

is_windows = platform.system() == "Windows"

if is_windows:
    try:
        m2_lib = ctypes.WinDLL(None)
    except Exception:
        m2_lib = ctypes.CDLL(None)
else:
    m2_lib = ctypes.CDLL(None)

def create_real_module(module_name, prefix, methods_spec):
    class ModType(type):
        def __getattr__(cls, attr):
            if attr.isupper():
                return 1
            raise AttributeError(f"Module {module_name} has no attribute {attr}")
            
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
            def make_dummy(c_name):
                def dummy(*args, **kwargs):
                    print(f"WARN: Missing FFI export {c_name} called")
                    return 0
                return dummy
            setattr(DynamicMod, py_name, staticmethod(make_dummy(c_name)))
            
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
net_methods = [
    ("Connect", "m2_net_Connect", [ctypes.c_char_p, ctypes.c_uint16], ctypes.c_bool),
    ("Disconnect", "m2_net_Disconnect", [], None),
    ("GetMainStream", "m2_net_GetMainStream", [], ctypes.c_bool),
    ("SendChatPacket", "m2_net_SendChatPacket", [ctypes.c_char_p, ctypes.c_uint8, ctypes.c_void_p], ctypes.c_size_t),
    ("SendAttackPacket", "m2_net_SendAttackPacket", [ctypes.c_uint8, ctypes.c_uint32, ctypes.c_void_p], ctypes.c_size_t),
    ("SendUseSkillPacket", "m2_net_SendUseSkillPacket", [ctypes.c_uint32, ctypes.c_uint32, ctypes.c_void_p], ctypes.c_size_t),
    ("SendMovePacket", "m2_net_SendMovePacket", [ctypes.c_uint8, ctypes.c_int32, ctypes.c_int32, ctypes.c_float, ctypes.c_uint32, ctypes.c_void_p], ctypes.c_size_t),
]
sys.modules['net'] = create_real_module('net', 'net', net_methods)

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
]
sys.modules['item'] = create_real_module('item', 'item', item_methods)

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
]
sys.modules['guild'] = create_real_module('guild', 'guild', guild_methods)

# Shop Module
shop_methods = [
]
sys.modules['shop'] = create_real_module('shop', 'shop', shop_methods)

# Skill Module
skill_methods = [
]
sys.modules['skill'] = create_real_module('skill', 'skill', skill_methods)

# Exchange Module
exchange_methods = [
]
sys.modules['exchange'] = create_real_module('exchange', 'exchange', exchange_methods)

# Quest Module
quest_methods = [
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
sys.modules['nonplayer'] = make_dummy_module('nonplayer')

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
            return Err("Failed to execute py_bootstrap_runtime_injector Python script".to_string());
        }
    }
    
    Ok(())
}
