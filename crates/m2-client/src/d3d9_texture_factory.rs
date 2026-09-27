use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{OnceLock, RwLock};

use crate::dxt1_decoder::decode_dxt1;
use crate::dxt3_decoder::decode_dxt3;
use crate::dxt5_decoder::decode_dxt5;
use crate::tga_decoder::TgaImage;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    ARGB8888 = 0,
    XRGB8888 = 1,
    DXT1 = 2,
    DXT3 = 3,
    DXT5 = 4,
    A8R8G8B8 = 5,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TextureData {
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub mip_levels: u32,
    pub raw_bytes: Vec<u8>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureHandle(pub u32);

struct MipLevel {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

struct InternalTexture {
    format: TextureFormat,
    mipmaps: Vec<MipLevel>,
}

static TEXTURE_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

fn get_texture_store() -> &'static RwLock<HashMap<TextureHandle, InternalTexture>> {
    static STORE: OnceLock<RwLock<HashMap<TextureHandle, InternalTexture>>> = OnceLock::new();
    STORE.get_or_init(|| RwLock::new(HashMap::new()))
}

fn calculate_mip_size(width: u32, height: u32, format: TextureFormat) -> usize {
    let w = width as usize;
    let h = height as usize;
    match format {
        TextureFormat::ARGB8888 | TextureFormat::XRGB8888 | TextureFormat::A8R8G8B8 => {
            w * h * 4
        }
        TextureFormat::DXT1 => {
            let blocks_w = (w.max(1) + 3) / 4;
            let blocks_h = (h.max(1) + 3) / 4;
            blocks_w * blocks_h * 8
        }
        TextureFormat::DXT3 | TextureFormat::DXT5 => {
            let blocks_w = (w.max(1) + 3) / 4;
            let blocks_h = (h.max(1) + 3) / 4;
            blocks_w * blocks_h * 16
        }
    }
}

pub fn create_texture_from_memory(data: &TextureData) -> Result<TextureHandle, String> {
    if data.width == 0 || data.height == 0 {
        return Err("Invalid texture dimensions".to_string());
    }

    let mut mipmaps = Vec::new();
    let mut offset = 0;
    
    // Attempt TGA decoding if it looks like TGA format and it is A8R8G8B8/ARGB8888/XRGB8888
    // TGA often has 1 mip level and specific raw format, but we handle it via our internal decoder
    let mut raw_bytes = data.raw_bytes.clone();
    
    // Check if it might be a TGA file based on header (minimal check for magic/type)
    if (data.format == TextureFormat::ARGB8888 || data.format == TextureFormat::XRGB8888 || data.format == TextureFormat::A8R8G8B8) && raw_bytes.len() > 18 {
        let is_tga = raw_bytes[2] == 2 || raw_bytes[2] == 10;
        if is_tga && data.mip_levels <= 1 {
            if let Ok(tga) = TgaImage::from_bytes(&raw_bytes) {
                // TGA returns RGBA, convert to expected if needed, but our internal stores RGBA
                raw_bytes = tga.rgba_data;
                // Since TGA decodes the whole image, we just use it directly
            }
        }
    }

    for level in 0..data.mip_levels.max(1) {
        let level_w = (data.width >> level).max(1);
        let level_h = (data.height >> level).max(1);

        let mip_size = calculate_mip_size(level_w, level_h, data.format);
        
        if offset + mip_size > raw_bytes.len() {
            return Err(format!("Not enough raw data for mip level {}", level));
        }

        let mip_data = &raw_bytes[offset..offset + mip_size];
        
        let decoded_data = match data.format {
            TextureFormat::ARGB8888 | TextureFormat::XRGB8888 | TextureFormat::A8R8G8B8 => {
                mip_data.to_vec()
            },
            TextureFormat::DXT1 => {
                decode_dxt1(mip_data, level_w as usize, level_h as usize)
                    .map_err(|e| format!("DXT1 decode error: {}", e))?
            },
            TextureFormat::DXT3 => {
                decode_dxt3(mip_data, level_w as usize, level_h as usize)
                    .map_err(|e| format!("DXT3 decode error: {}", e))?
            },
            TextureFormat::DXT5 => {
                decode_dxt5(mip_data, level_w as usize, level_h as usize)
                    .map_err(|e| format!("DXT5 decode error: {}", e))?
            },
        };

        mipmaps.push(MipLevel {
            width: level_w,
            height: level_h,
            data: decoded_data,
        });

        offset += mip_size;
    }

    let handle_id = TEXTURE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    let handle = TextureHandle(handle_id);

    let internal_tex = InternalTexture {
        format: data.format,
        mipmaps,
    };

    let mut store = get_texture_store().write().unwrap();
    store.insert(handle, internal_tex);

    Ok(handle)
}

pub fn lock_texture_rect(handle: TextureHandle, level: u32) -> Option<&'static mut [u8]> {
    let mut store = get_texture_store().write().unwrap();
    if let Some(tex) = store.get_mut(&handle) {
        if let Some(mip) = tex.mipmaps.get_mut(level as usize) {
            // Unsafe required here because we are returning a mutable reference
            // with a 'static lifetime which is typically what FFI / C++ wrappers expect
            // for locking a texture. Alternatively, if signature meant strictly safe Rust,
            // returning a slice bound to the write lock is impossible without unsafe due to lock guard dropping.
            // But per standard Rust, returning Option<&mut [u8]> from a function that doesn't take &mut self is problematic.
            // Let's use raw pointer conversion.
            let ptr = mip.data.as_mut_ptr();
            let len = mip.data.len();
            return Some(unsafe { std::slice::from_raw_parts_mut(ptr, len) });
        }
    }
    None
}

pub fn release_texture(handle: TextureHandle) {
    let mut store = get_texture_store().write().unwrap();
    store.remove(&handle);
}
