// ui_texture_mgr.rs - Unified UI Texture and SubImage Resource Manager for Metin2 Pure Rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::wgpu_engine_context::{GpuTexture, GLOBAL_WGPU_CONTEXT};
use crate::py_bridge_pack::get_mounted_packs;
use crate::vfs_pck_memory_mapped::read_file_from_archive;
use crate::sub_image_parser::SubImage;

#[derive(Clone)]
pub struct UiImageResource {
    pub texture: Arc<GpuTexture>,
    pub uv: [f32; 4], // [u0, v0, u1, v1]
    pub width: u32,
    pub height: u32,
}

pub struct UiTextureManager {
    textures: HashMap<String, Arc<GpuTexture>>,
    sub_images: HashMap<String, UiImageResource>,
}

impl UiTextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            sub_images: HashMap::new(),
        }
    }

    /// Pobiera surowe bajty pliku z zamontowanych archiwow VFS
    pub fn read_vfs_bytes(path: &str) -> Option<Vec<u8>> {
        // Sprawdz najpierw na dysku
        if let Ok(bytes) = std::fs::read(path) {
            return Some(bytes);
        }

        let packs_guard = get_mounted_packs().lock().ok()?;
        for archive in packs_guard.values() {
            if let Some(data) = read_file_from_archive(archive, path) {
                return Some(data);
            }
        }
        None
    }

    /// Dekoduje plik DDS (DXT1, DXT3, DXT5) do formatu RGBA8
    pub fn decode_dds_to_rgba(data: &[u8]) -> Result<(Vec<u8>, u32, u32), String> {
        if data.len() < 128 {
            return Err("DDS data too small (<128 bytes)".to_string());
        }
        let magic = u32::from_le_bytes(data[0..4].try_into().unwrap());
        if magic != 0x20534444 { // "DDS "
            return Err("Invalid DDS magic header".to_string());
        }

        let height = u32::from_le_bytes(data[12..16].try_into().unwrap());
        let width = u32::from_le_bytes(data[16..20].try_into().unwrap());
        let fourcc = &data[84..88];

        let pixel_data = &data[128..];

        if fourcc == b"DXT1" {
            let rgba = crate::dxt1_decoder::decode_dxt1(pixel_data, width as usize, height as usize)
                .map_err(|e| format!("DXT1 decode error: {}", e))?;
            Ok((rgba, width, height))
        } else if fourcc == b"DXT3" {
            let rgba = crate::dxt3_decoder::decode_dxt3(pixel_data, width as usize, height as usize)
                .map_err(|e| format!("DXT3 decode error: {}", e))?;
            Ok((rgba, width, height))
        } else if fourcc == b"DXT5" {
            let rgba = crate::dxt5_decoder::decode_dxt5(pixel_data, width as usize, height as usize)
                .map_err(|e| format!("DXT5 decode error: {}", e))?;
            Ok((rgba, width, height))
        } else {
            // Nieskompresowany fallback (np. jesli rozmiar pasuje do RGBA8)
            if pixel_data.len() >= (width * height * 4) as usize {
                Ok((pixel_data[0..(width * height * 4) as usize].to_vec(), width, height))
            } else {
                Err(format!("Unsupported DDS fourcc: {:?}", fourcc))
            }
        }
    }

    /// Laduje i tworzy teksture GPU z pliku graficznego (DDS, TGA, JPG, PNG)
    pub fn load_or_get_texture(&mut self, file_path: &str) -> Option<Arc<GpuTexture>> {
        let norm_path = file_path.to_lowercase().replace('\\', "/");
        if let Some(tex) = self.textures.get(&norm_path) {
            return Some(tex.clone());
        }

        let raw_bytes = Self::read_vfs_bytes(&norm_path)?;

        let (rgba, width, height) = if norm_path.ends_with(".dds") {
            Self::decode_dds_to_rgba(&raw_bytes).ok()?
        } else if norm_path.ends_with(".tga") {
            let tga = crate::tga_decoder::TgaImage::from_bytes(&raw_bytes).ok()?;
            (tga.rgba_data, tga.width as u32, tga.height as u32)
        } else {
            let img = image::load_from_memory(&raw_bytes).ok()?;
            let rgba_img = img.to_rgba8();
            (rgba_img.into_raw(), img.width(), img.height())
        };

        // Tworzenie GpuTexture w kontekscie WGPU
        let wgpu_guard = GLOBAL_WGPU_CONTEXT.get()?;
        let ctx_mutex = wgpu_guard.lock().ok()?;
        let ctx = ctx_mutex.as_ref()?;

        match ctx.create_texture_from_rgba(&rgba, width, height, Some(&norm_path)) {
            Ok(gpu_tex) => {
                let arc_tex = Arc::new(gpu_tex);
                self.textures.insert(norm_path, arc_tex.clone());
                Some(arc_tex)
            }
            Err(e) => {
                eprintln!("[-] Blad tworzenia tekstury GPU dla {}: {}", norm_path, e);
                None
            }
        }
    }

    /// Pobiera zasob graficzny (obsluguje bezposrednie tekstury oraz wycinki .sub)
    pub fn get_image_resource(&mut self, path: &str) -> Option<UiImageResource> {
        let norm_path = path.to_lowercase().replace('\\', "/");

        if let Some(res) = self.sub_images.get(&norm_path) {
            return Some(res.clone());
        }

        if norm_path.ends_with(".sub") {
            let sub_bytes = Self::read_vfs_bytes(&norm_path)?;
            let sub_text = String::from_utf8_lossy(&sub_bytes);
            let sub = SubImage::parse(&sub_text).ok()?;

            let clean_img = sub.image.trim().trim_matches('"').trim();

            // Poszukiwanie pliku bazowego obrazu wskazanego przez sub.image
            let mut base_candidates = Vec::new();

            // 1. W tym samym katalogu co plik .sub
            if let Some(idx) = norm_path.rfind('/') {
                let dir = &norm_path[..idx + 1];
                base_candidates.push(format!("{}{}", dir, clean_img));
            }
            // 2. W katalogu UI bazowym: d:/ymir work/ui/
            base_candidates.push(format!("d:/ymir work/ui/{}", clean_img));
            base_candidates.push(format!("d:/ymir work/ui/intro/login/{}", clean_img));
            base_candidates.push(clean_img.to_string());

            let mut texture_opt = None;
            for cand in base_candidates {
                if let Some(tex) = self.load_or_get_texture(&cand) {
                    texture_opt = Some(tex);
                    break;
                }
            }

            let texture = texture_opt?;
            let (u0, v0, u1, v1) = sub.get_uv(texture.width, texture.height);
            let width = (sub.right - sub.left).max(1) as u32;
            let height = (sub.bottom - sub.top).max(1) as u32;

            let res = UiImageResource {
                texture,
                uv: [u0, v0, u1, v1],
                width,
                height,
            };

            self.sub_images.insert(norm_path, res.clone());
            Some(res)
        } else {
            // Zwykly plik graficzny (TGA, DDS, JPG, PNG)
            let texture = self.load_or_get_texture(&norm_path)?;
            let width = texture.width;
            let height = texture.height;

            let res = UiImageResource {
                texture,
                uv: [0.0, 0.0, 1.0, 1.0],
                width,
                height,
            };

            self.sub_images.insert(norm_path, res.clone());
            Some(res)
        }
    }
}

static UI_TEXTURE_MANAGER: OnceLock<Mutex<UiTextureManager>> = OnceLock::new();

pub fn get_ui_texture_manager() -> std::sync::MutexGuard<'static, UiTextureManager> {
    UI_TEXTURE_MANAGER
        .get_or_init(|| Mutex::new(UiTextureManager::new()))
        .lock()
        .unwrap()
}
