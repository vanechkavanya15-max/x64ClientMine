use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::{Mutex, OnceLock};

// We don't import m2_client_bridge here because we shouldn't modify Cargo.toml or lib.rs.
// But we need safe_ffi_call. The prompt says "KATEGORYCZNY ZAKAZ edytowania pliku crates/m2-client/src/lib.rs ani crates/m2-client/Cargo.toml ani zadnych innych istniejacych plikow wspoldzielonych".
// Since we can't edit shared files, we just use std::panic::catch_unwind or simply implement it safely.

fn safe_ffi_call<F, R>(default: R, f: F) -> R
where
    F: FnOnce() -> R + std::panic::UnwindSafe,
{
    std::panic::catch_unwind(f).unwrap_or(default)
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct D3DXCOLOR {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl D3DXCOLOR {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

pub const C_TEXTTAIL_PLAYER_COLOR: D3DXCOLOR = D3DXCOLOR::new(1.0, 1.0, 1.0, 1.0);
pub const C_TEXTTAIL_MONSTER_COLOR: D3DXCOLOR = D3DXCOLOR::new(1.0, 0.0, 0.0, 1.0);
pub const C_TEXTTAIL_ITEM_COLOR: D3DXCOLOR = D3DXCOLOR::new(1.0, 1.0, 1.0, 1.0);
pub const C_TEXTTAIL_CHAT_COLOR: D3DXCOLOR = D3DXCOLOR::new(1.0, 1.0, 1.0, 1.0);
pub const C_TEXTTAIL_INFO_COLOR: D3DXCOLOR = D3DXCOLOR::new(1.0, 0.785, 0.785, 1.0);
// 0xFFEFD3FF in ARGB -> A=FF, R=EF, G=D3, B=FF
pub const C_TEXTTAIL_GUILD_NAME_COLOR: D3DXCOLOR = D3DXCOLOR::new(
    0xEF as f32 / 255.0,
    0xD3 as f32 / 255.0,
    0xFF as f32 / 255.0,
    1.0,
);
pub const C_TEXTTAIL_NAME_POSITION: f32 = -10.0;
pub const C_FX_MARK_POSITION: f32 = 1.5;
pub const C_FY_GUILD_NAME_POSITION: f32 = 15.0;
pub const C_FY_MARK_POSITION: f32 = 15.0 + 11.0;

#[derive(Debug, Clone)]
pub struct TTextTail {
    pub dwVirtualID: u32,
    pub stText: String,
    pub fX: f32,
    pub fY: f32,
    pub fZ: f32,
    pub fDistanceFromPlayer: f32,
    pub Color: D3DXCOLOR,
    pub bNameFlag: bool,
    pub fHeight: f32,
    pub xStart: f32,
    pub yStart: f32,
    pub xEnd: f32,
    pub yEnd: f32,
    
    // We use usize to represent pointers to mock 1:1 C++ behavior since we don't have the original struct types here
    // In C++ these are: CGraphicTextInstance * pTextInstance, CGraphicObjectInstance * pOwnerTextInstance, etc.
    // For pure UI/FFI tests we map them to raw integer equivalents.
    pub pOwnerTextInstance: usize,
    pub pMarkInstance: usize,
    pub pGuildNameTextInstance: usize,
    pub pTitleTextInstance: usize,
    pub pLevelTextInstance: usize,
    pub pTextInstance: usize,
    
    // Additional fields mapped from C++
    pub stGuildName: Option<String>,
    pub stTitle: Option<String>,
    pub title_color: Option<D3DXCOLOR>,
    pub stLevel: Option<String>,
}

impl TTextTail {
    pub fn new(virtual_id: u32, text: String, color: D3DXCOLOR, f_height: f32) -> Self {
        Self {
            dwVirtualID: virtual_id,
            stText: text,
            fX: 0.0,
            fY: 0.0,
            fZ: 0.0,
            fDistanceFromPlayer: 0.0,
            Color: color,
            bNameFlag: false,
            fHeight: f_height,
            xStart: 0.0,
            yStart: 0.0,
            xEnd: 0.0,
            yEnd: 0.0,
            pOwnerTextInstance: 0,
            pMarkInstance: 0,
            pGuildNameTextInstance: 0,
            pTitleTextInstance: 0,
            pLevelTextInstance: 0,
            pTextInstance: 0,
            stGuildName: None,
            stTitle: None,
            title_color: None,
            stLevel: None,
        }
    }
}

pub struct CPythonTextTail {
    pub m_CharacterTextTailMap: HashMap<u32, TTextTail>,
    pub m_ItemTextTailMap: HashMap<u32, TTextTail>,
    pub m_ChatTailMap: HashMap<u32, TTextTail>,

    pub m_CharacterTextTailList: Vec<u32>,
    pub m_ItemTextTailList: Vec<u32>,
}

impl CPythonTextTail {
    pub fn new() -> Self {
        Self {
            m_CharacterTextTailMap: HashMap::new(),
            m_ItemTextTailMap: HashMap::new(),
            m_ChatTailMap: HashMap::new(),
            m_CharacterTextTailList: Vec::new(),
            m_ItemTextTailList: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.m_CharacterTextTailMap.clear();
        self.m_ItemTextTailMap.clear();
        self.m_ChatTailMap.clear();
        self.m_CharacterTextTailList.clear();
        self.m_ItemTextTailList.clear();
    }
}

impl Default for CPythonTextTail {
    fn default() -> Self {
        Self::new()
    }
}

static TEXT_TAIL_STATE: OnceLock<Mutex<CPythonTextTail>> = OnceLock::new();

pub fn get_text_tail_state() -> std::sync::MutexGuard<'static, CPythonTextTail> {
    TEXT_TAIL_STATE
        .get_or_init(|| Mutex::new(CPythonTextTail::new()))
        .lock()
        .unwrap()
}

#[no_mangle]
pub extern "C" fn m2_py_texttail_register_character(vid: u32) {
    safe_ffi_call((), || {
        let mut state = get_text_tail_state();
        if !state.m_CharacterTextTailMap.contains_key(&vid) {
            let tail = TTextTail::new(vid, String::new(), C_TEXTTAIL_PLAYER_COLOR, 10.0);
            state.m_CharacterTextTailMap.insert(vid, tail);
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_texttail_register_item(vid: u32, text_ptr: *const c_char) {
    safe_ffi_call((), || {
        if text_ptr.is_null() {
            return;
        }
        
        let c_str = unsafe { CStr::from_ptr(text_ptr) };
        if let Ok(text) = c_str.to_str() {
            let mut state = get_text_tail_state();
            if !state.m_ItemTextTailMap.contains_key(&vid) {
                let tail = TTextTail::new(vid, text.to_string(), C_TEXTTAIL_ITEM_COLOR, C_TEXTTAIL_NAME_POSITION);
                state.m_ItemTextTailMap.insert(vid, tail);
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_texttail_show_character_text_tail(vid: u32) {
    safe_ffi_call((), || {
        let mut state = get_text_tail_state();
        if state.m_CharacterTextTailMap.contains_key(&vid) {
            if !state.m_CharacterTextTailList.contains(&vid) {
                state.m_CharacterTextTailList.push(vid);
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_texttail_hide_character_text_tail(vid: u32) {
    safe_ffi_call((), || {
        let mut state = get_text_tail_state();
        if let Some(pos) = state.m_CharacterTextTailList.iter().position(|&x| x == vid) {
            state.m_CharacterTextTailList.remove(pos);
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_texttail_show_all_text_tail() {
    safe_ffi_call((), || {
        let mut state = get_text_tail_state();
        
        let mut chars_to_show = Vec::new();
        for (vid, tail) in state.m_CharacterTextTailMap.iter() {
            if tail.fDistanceFromPlayer < 3500.0 {
                chars_to_show.push(*vid);
            }
        }
        
        let mut items_to_show = Vec::new();
        for (vid, tail) in state.m_ItemTextTailMap.iter() {
            if tail.fDistanceFromPlayer < 3500.0 {
                items_to_show.push(*vid);
            }
        }

        for vid in chars_to_show {
            if !state.m_CharacterTextTailList.contains(&vid) {
                 state.m_CharacterTextTailList.push(vid);
            }
        }
        
        for vid in items_to_show {
            if !state.m_ItemTextTailList.contains(&vid) {
                 state.m_ItemTextTailList.push(vid);
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_texttail_hide_all_text_tail() {
    safe_ffi_call((), || {
        let mut state = get_text_tail_state();
        state.m_CharacterTextTailList.clear();
        state.m_ItemTextTailList.clear();
    });
}

#[no_mangle]
pub extern "C" fn m2_py_texttail_attach_title(
    vid: u32,
    title_ptr: *const c_char,
    r: f32,
    g: f32,
    b: f32,
) {
    safe_ffi_call((), || {
        if title_ptr.is_null() {
            return;
        }
        let c_str = unsafe { CStr::from_ptr(title_ptr) };
        if let Ok(title) = c_str.to_str() {
            let mut state = get_text_tail_state();
            if let Some(tail) = state.m_CharacterTextTailMap.get_mut(&vid) {
                tail.stTitle = Some(title.to_string());
                tail.title_color = Some(D3DXCOLOR::new(r, g, b, 1.0));
            }
        }
    });
}
