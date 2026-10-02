use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Instant;

use crate::wgpu_engine_context::{GpuTexture, GLOBAL_WGPU_CONTEXT};
use crate::py_bridge_pack::get_mounted_packs;
use crate::vfs_pck_memory_mapped::read_file_from_archive;
use crate::sub_image_parser::SubImage;
use crate::ui_texture_mgr::UiTextureManager;
use crate::tga_decoder::TgaImage;

pub struct GpuTextureCache {
    textures: HashMap<String, (Arc<GpuTexture>, Instant)>,
    max_items: usize,
}

impl GpuTextureCache {
    pub fn new(max_items: usize) -> Self {
        Self {
            textures: HashMap::new(),
            max_items,
        }
    }

    pub fn get(&mut self, path: &str) -> Option<Arc<GpuTexture>> {
        let norm = path.to_lowercase().replace('\\', "/");
        if let Some((tex, last_acc)) = self.textures.get_mut(&norm) {
            *last_acc = Instant::now();
            Some(tex.clone())
        } else {
            None
        }
    }

    pub fn insert(&mut self, path: &str, tex: Arc<GpuTexture>) {
        let norm = path.to_lowercase().replace('\\', "/");
        
        if self.textures.len() >= self.max_items {
            if let Some(lru_key) = self.textures.iter().min_by_key(|(_, (_, time))| *time).map(|(k, _)| k.clone()) {
                self.textures.remove(&lru_key);
            }
        }
        
        self.textures.insert(norm, (tex, Instant::now()));
    }
}

pub static GLOBAL_VFS_TEXTURE_CACHE: OnceLock<Mutex<GpuTextureCache>> = OnceLock::new();

pub fn get_vfs_texture_cache() -> &'static Mutex<GpuTextureCache> {
    GLOBAL_VFS_TEXTURE_CACHE.get_or_init(|| Mutex::new(GpuTextureCache::new(512)))
}

pub fn read_vfs_bytes(path: &str) -> Option<Vec<u8>> {
    let norm = path.to_lowercase().replace('\\', "/");
    if let Ok(bytes) = std::fs::read(&norm) {
        return Some(bytes);
    }
    if let Ok(guard) = get_mounted_packs().lock() {
        for archive in guard.values() {
            if let Some(data) = read_file_from_archive(archive, &norm) {
                return Some(data);
            }
        }
    }
    None
}

pub fn load_gpu_texture_from_vfs(path: &str) -> Option<Arc<GpuTexture>> {
    let norm_path = path.to_lowercase().replace('\\', "/");
    
    {
        let cache_mutex = get_vfs_texture_cache();
        if let Ok(mut cache) = cache_mutex.lock() {
            if let Some(tex) = cache.get(&norm_path) {
                return Some(tex);
            }
        }
    }

    let raw_bytes = read_vfs_bytes(&norm_path)?;

    let (rgba, width, height) = if norm_path.ends_with(".sub") {
        let sub_text = String::from_utf8_lossy(&raw_bytes);
        let sub = SubImage::parse(&sub_text).ok()?;
        
        let mut base_img_path = sub.image.clone();
        if !base_img_path.contains("/") && !base_img_path.contains("\\") {
            if let Some(last_slash) = norm_path.rfind('/') {
                base_img_path = format!("{}/{}", &norm_path[..last_slash], base_img_path);
            }
        }
        
        let base_bytes = read_vfs_bytes(&base_img_path)?;
        let (base_rgba, base_w, base_h) = decode_image_bytes(&base_bytes, &base_img_path)?;
        
        crop_image(&base_rgba, base_w, base_h, sub.left, sub.top, sub.right, sub.bottom)?
    } else {
        decode_image_bytes(&raw_bytes, &norm_path)?
    };

    let gpu_tex = {
        let wgpu_guard = GLOBAL_WGPU_CONTEXT.get()?;
        let ctx_mutex = wgpu_guard.lock().ok()?;
        let ctx = ctx_mutex.as_ref()?;
        ctx.create_texture_from_rgba(&rgba, width, height, Some(&norm_path)).ok()?
    };
    
    let arc_tex = Arc::new(gpu_tex);
    
    {
        let cache_mutex = get_vfs_texture_cache();
        if let Ok(mut cache) = cache_mutex.lock() {
            cache.insert(&norm_path, arc_tex.clone());
        }
    }
    
    Some(arc_tex)
}

pub fn decode_image_bytes(bytes: &[u8], path: &str) -> Option<(Vec<u8>, u32, u32)> {
    let norm = path.to_lowercase().replace('\\', "/");
    if norm.ends_with(".dds") {
        UiTextureManager::decode_dds_to_rgba(bytes).ok()
    } else if norm.ends_with(".tga") {
        let tga = TgaImage::from_bytes(bytes).ok()?;
        Some((tga.rgba_data, tga.width as u32, tga.height as u32))
    } else {
        let img = image::load_from_memory(bytes).ok()?;
        let rgba_img = img.to_rgba8();
        Some((rgba_img.into_raw(), img.width(), img.height()))
    }
}

pub fn crop_image(rgba: &[u8], src_w: u32, src_h: u32, left: i32, top: i32, right: i32, bottom: i32) -> Option<(Vec<u8>, u32, u32)> {
    if left < 0 || top < 0 || right <= left || bottom <= top {
        return None;
    }
    
    let left = left as u32;
    let top = top as u32;
    let right = right as u32;
    let bottom = bottom as u32;
    
    if right > src_w || bottom > src_h {
        return None;
    }
    
    let dst_w = right - left;
    let dst_h = bottom - top;
    let mut dst_rgba = vec![0u8; (dst_w * dst_h * 4) as usize];
    
    for y in 0..dst_h {
        let src_y = top + y;
        let src_offset = ((src_y * src_w + left) * 4) as usize;
        let dst_offset = ((y * dst_w) * 4) as usize;
        let row_bytes = (dst_w * 4) as usize;
        
        dst_rgba[dst_offset..dst_offset + row_bytes].copy_from_slice(&rgba[src_offset..src_offset + row_bytes]);
    }
    
    Some((dst_rgba, dst_w, dst_h))
}
