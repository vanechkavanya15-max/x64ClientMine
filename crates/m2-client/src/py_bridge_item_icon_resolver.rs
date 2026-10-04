//! Item icon resolution (.sub, .tga), UV texture coordinates, and slot size calculation.

use std::collections::HashMap;
use std::ffi::{CStr, c_char};
use std::sync::{Mutex, OnceLock};
use m2_client_bridge::ffi_guard::safe_ffi_call;

#[derive(Debug, Clone)]
pub struct ItemIconDescriptor {
    pub vnum: u32,
    pub image_path: String,
    pub uv_coords: [f32; 4], // [u_min, v_min, u_max, v_max]
    pub slot_width: u8,
    pub slot_height: u8,
}

impl Default for ItemIconDescriptor {
    fn default() -> Self {
        Self {
            vnum: 0,
            image_path: String::new(),
            uv_coords: [0.0, 0.0, 1.0, 1.0],
            slot_width: 1,
            slot_height: 1,
        }
    }
}

pub struct ItemIconResolver {
    descriptors: HashMap<u32, ItemIconDescriptor>,
    base_icon_dir: String,
}

impl Default for ItemIconResolver {
    fn default() -> Self {
        Self {
            descriptors: HashMap::new(),
            base_icon_dir: "icon/item/".to_string(),
        }
    }
}

impl ItemIconResolver {
    pub fn get_icon_path(&self, vnum: u32) -> String {
        if let Some(desc) = self.descriptors.get(&vnum) {
            if !desc.image_path.is_empty() {
                return desc.image_path.clone();
            }
        }
        // Standard Metin2 naming: 00010.tga (5-digit padded)
        format!("{}{:05}.tga", self.base_icon_dir, vnum)
    }

    pub fn get_size(&self, vnum: u32) -> (u8, u8) {
        if let Some(desc) = self.descriptors.get(&vnum) {
            return (desc.slot_width, desc.slot_height);
        }
        // Weapons (10-3199) usually 1x2 or 1x3, Armors (11000-12099) 1x2, others 1x1
        if (10..=3199).contains(&vnum) {
            (1, 2)
        } else if (11000..=12099).contains(&vnum) {
            (1, 2)
        } else {
            (1, 1)
        }
    }

    pub fn register_override(&mut self, vnum: u32, path: String, w: u8, h: u8, uv: [f32; 4]) {
        self.descriptors.insert(
            vnum,
            ItemIconDescriptor {
                vnum,
                image_path: path,
                uv_coords: uv,
                slot_width: w.clamp(1, 3),
                slot_height: h.clamp(1, 3),
            },
        );
    }
}

static ICON_RESOLVER: OnceLock<Mutex<ItemIconResolver>> = OnceLock::new();

pub fn get_item_icon_resolver() -> &'static Mutex<ItemIconResolver> {
    ICON_RESOLVER.get_or_init(|| Mutex::new(ItemIconResolver::default()))
}

// C-FFI exports
#[no_mangle]
pub extern "C" fn m2_py_item_get_icon_image_path(vnum: u32, out_buf: *mut c_char, max_len: usize) -> bool {
    safe_ffi_call(false, || {
        if out_buf.is_null() || max_len == 0 {
            return false;
        }
        let resolver = get_item_icon_resolver().lock().unwrap();
        let path = resolver.get_icon_path(vnum);
        let bytes = path.as_bytes();
        let copy_len = bytes.len().min(max_len - 1);
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, out_buf, copy_len);
            *out_buf.add(copy_len) = 0;
        }
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_item_get_size_in_slots(vnum: u32, out_w: *mut u8, out_h: *mut u8) -> bool {
    safe_ffi_call(false, || {
        if out_w.is_null() || out_h.is_null() {
            return false;
        }
        let resolver = get_item_icon_resolver().lock().unwrap();
        let (w, h) = resolver.get_size(vnum);
        unsafe {
            *out_w = w;
            *out_h = h;
        }
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_item_register_icon_override(vnum: u32, path: *const c_char, w: u8, h: u8) -> bool {
    safe_ffi_call(false, || {
        if path.is_null() {
            return false;
        }
        let path_str = unsafe { CStr::from_ptr(path) }.to_string_lossy().into_owned();
        let mut resolver = get_item_icon_resolver().lock().unwrap();
        resolver.register_override(vnum, path_str, w, h, [0.0, 0.0, 1.0, 1.0]);
        true
    })
}
