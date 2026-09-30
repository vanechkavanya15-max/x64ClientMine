// ui_font_renderer.rs - Native Metin2 Font Atlas and Text Quad Generator for Pure Rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::wgpu_engine_context::{GpuTexture, Quad2D, GLOBAL_WGPU_CONTEXT};

#[derive(Clone, Debug)]
pub struct FontGlyph {
    pub uv: [f32; 4], // [u0, v0, u1, v1]
    pub width: f32,
    pub height: f32,
    pub advance: f32,
}

pub struct FontAtlas {
    pub texture: Option<Arc<GpuTexture>>,
    pub glyphs: HashMap<char, FontGlyph>,
    pub line_height: f32,
}

impl Default for FontAtlas {
    fn default() -> Self {
        Self::new()
    }
}

impl FontAtlas {
    pub fn new() -> Self {
        Self {
            texture: None,
            glyphs: HashMap::new(),
            line_height: 16.0,
        }
    }

    /// Inicjalizuje atlas czcionki na GPU
    pub fn init_atlas(&mut self) -> Result<(), String> {
        if self.texture.is_some() {
            return Ok(());
        }

        let atlas_w = 512u32;
        let atlas_h = 256u32;
        let mut rgba = vec![0u8; (atlas_w * atlas_h * 4) as usize];

        // Wygeneruj glify (32..127 ASCII oraz typowe znaki)
        let cell_w = 12u32;
        let cell_h = 16u32;
        let cols = atlas_w / cell_w;

        #[cfg(windows)]
        {
            self.rasterize_gdi_glyphs(&mut rgba, atlas_w, atlas_h, cell_w, cell_h, cols);
        }

        #[cfg(not(windows))]
        {
            self.rasterize_fallback_glyphs(&mut rgba, atlas_w, atlas_h, cell_w, cell_h, cols);
        }

        // Stworz teksture GPU w WGPU
        let wgpu_guard = GLOBAL_WGPU_CONTEXT.get()
            .ok_or_else(|| "WGPU context not available".to_string())?;
        let ctx_mutex = wgpu_guard.lock()
            .map_err(|_| "Failed to lock WGPU mutex".to_string())?;
        let ctx = ctx_mutex.as_ref()
            .ok_or_else(|| "WGPU context is None".to_string())?;

        let gpu_tex = ctx.create_texture_from_rgba(&rgba, atlas_w, atlas_h, Some("UI Font Atlas"))?;
        self.texture = Some(Arc::new(gpu_tex));
        Ok(())
    }

    #[cfg(windows)]
    fn rasterize_gdi_glyphs(
        &mut self,
        rgba: &mut [u8],
        atlas_w: u32,
        atlas_h: u32,
        cell_w: u32,
        cell_h: u32,
        cols: u32,
    ) {
        use std::ffi::c_void;

        type HDC = *mut c_void;
        type HFONT = *mut c_void;
        type HBITMAP = *mut c_void;

        #[repr(C)]
        struct BITMAPINFOHEADER {
            biSize: u32,
            biWidth: i32,
            biHeight: i32,
            biPlanes: u16,
            biBitCount: u16,
            biCompression: u32,
            biSizeImage: u32,
            biXPelsPerMeter: i32,
            biYPelsPerMeter: i32,
            biClrUsed: u32,
            biClrImportant: u32,
        }

        #[repr(C)]
        struct RECT {
            left: i32,
            top: i32,
            right: i32,
            bottom: i32,
        }

        extern "system" {
            fn CreateCompatibleDC(hdc: HDC) -> HDC;
            fn DeleteDC(hdc: HDC) -> i32;
            fn CreateFontW(
                nHeight: i32, nWidth: i32, nEscapement: i32, nOrientation: i32,
                fnWeight: i32, fdwItalic: u32, fdwUnderline: u32, fdwStrikeOut: u32,
                fdwCharSet: u32, fdwOutputPrecision: u32, fdwClipPrecision: u32,
                fdwQuality: u32, fdwPitchAndFamily: u32, lpszFace: *const u16,
            ) -> HFONT;
            fn SelectObject(hdc: HDC, hgdiobj: *mut c_void) -> *mut c_void;
            fn DeleteObject(ho: *mut c_void) -> i32;
            fn CreateDIBSection(
                hdc: HDC, pbmi: *const BITMAPINFOHEADER, usage: u32,
                ppvBits: *mut *mut c_void, hSection: *mut c_void, offset: u32,
            ) -> HBITMAP;
            fn SetBkMode(hdc: HDC, mode: i32) -> i32;
            fn SetTextColor(hdc: HDC, color: u32) -> u32;
            fn DrawTextW(hdc: HDC, lpchText: *const u16, cchText: i32, lprc: *mut RECT, format: u32) -> i32;
            fn GetCharWidth32W(hdc: HDC, iFirstChar: u32, iLastChar: u32, lpBuffer: *mut i32) -> i32;
        }

        unsafe {
            let hdc = CreateCompatibleDC(std::ptr::null_mut());
            if hdc.is_null() {
                self.rasterize_fallback_glyphs(rgba, atlas_w, atlas_h, cell_w, cell_h, cols);
                return;
            }

            let bmi = BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: atlas_w as i32,
                biHeight: -(atlas_h as i32), // top-down
                biPlanes: 1,
                biBitCount: 32,
                biCompression: 0,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            };

            let mut bits: *mut c_void = std::ptr::null_mut();
            let hbm = CreateDIBSection(hdc, &bmi, 0, &mut bits, std::ptr::null_mut(), 0);
            if hbm.is_null() || bits.is_null() {
                DeleteDC(hdc);
                self.rasterize_fallback_glyphs(rgba, atlas_w, atlas_h, cell_w, cell_h, cols);
                return;
            }

            let old_bm = SelectObject(hdc, hbm);

            let font_name: Vec<u16> = "Tahoma\0".encode_utf16().collect();
            let hfont = CreateFontW(
                -13, 0, 0, 0, 400, 0, 0, 0, 1, 0, 0, 5, 0, font_name.as_ptr(),
            );
            let old_font = if !hfont.is_null() { SelectObject(hdc, hfont) } else { std::ptr::null_mut() };

            SetBkMode(hdc, 1); // TRANSPARENT
            SetTextColor(hdc, 0x00FFFFFF); // White text

            let dib_slice = std::slice::from_raw_parts_mut(bits as *mut u8, (atlas_w * atlas_h * 4) as usize);

            // Renderuj znaki od 32 (spacja) do 126 (tylda)
            for code in 32u8..=126u8 {
                let c = code as char;
                let idx = (code - 32) as u32;
                let col = idx % cols;
                let row = idx / cols;

                let gx = col * cell_w;
                let gy = row * cell_h;

                let mut char_w: i32 = 0;
                GetCharWidth32W(hdc, code as u32, code as u32, &mut char_w);
                let actual_w = if char_w > 0 { char_w.min(cell_w as i32) } else { 7 };

                let mut rc = RECT {
                    left: gx as i32,
                    top: gy as i32,
                    right: (gx + cell_w) as i32,
                    bottom: (gy + cell_h) as i32,
                };

                let ch_utf16: [u16; 1] = [code as u16];
                DrawTextW(hdc, ch_utf16.as_ptr(), 1, &mut rc, 0x00000000 | 0x00000004); // DT_LEFT | DT_VCENTER

                let u0 = gx as f32 / atlas_w as f32;
                let v0 = gy as f32 / atlas_h as f32;
                let u1 = (gx as f32 + actual_w as f32) / atlas_w as f32;
                let v1 = (gy + cell_h) as f32 / atlas_h as f32;

                self.glyphs.insert(c, FontGlyph {
                    uv: [u0, v0, u1, v1],
                    width: actual_w as f32,
                    height: cell_h as f32,
                    advance: actual_w as f32,
                });
            }

            // Kopiuj piksele z DIB (BGRA) do atlasu (RGBA)
            for i in 0..(atlas_w * atlas_h) as usize {
                let b = dib_slice[i * 4];
                let g = dib_slice[i * 4 + 1];
                let r = dib_slice[i * 4 + 2];
                let lum = ((r as u32 + g as u32 + b as u32) / 3) as u8;

                rgba[i * 4] = 255;
                rgba[i * 4 + 1] = 255;
                rgba[i * 4 + 2] = 255;
                rgba[i * 4 + 3] = lum; // Alpha mask
            }

            if !old_font.is_null() { SelectObject(hdc, old_font); }
            if !hfont.is_null() { DeleteObject(hfont); }
            SelectObject(hdc, old_bm);
            DeleteObject(hbm);
            DeleteDC(hdc);
        }
    }

    fn rasterize_fallback_glyphs(
        &mut self,
        rgba: &mut [u8],
        atlas_w: u32,
        _atlas_h: u32,
        cell_w: u32,
        cell_h: u32,
        cols: u32,
    ) {
        // Podstawowy generator bitmapowy dla znakow ASCII
        for code in 32u8..=126u8 {
            let c = code as char;
            let idx = (code - 32) as u32;
            let col = idx % cols;
            let row = idx / cols;
            let gx = col * cell_w;
            let gy = row * cell_h;

            let u0 = gx as f32 / atlas_w as f32;
            let v0 = gy as f32 / _atlas_h as f32;
            let u1 = (gx + cell_w) as f32 / atlas_w as f32;
            let v1 = (gy + cell_h) as f32 / _atlas_h as f32;

            // Rysuj prosty kontur znaku
            for py in 2..cell_h - 2 {
                for px in 2..cell_w - 2 {
                    let pixel_idx = ((gy + py) * atlas_w + (gx + px)) as usize;
                    rgba[pixel_idx * 4] = 255;
                    rgba[pixel_idx * 4 + 1] = 255;
                    rgba[pixel_idx * 4 + 2] = 255;
                    rgba[pixel_idx * 4 + 3] = 220;
                }
            }

            let adv = if c == ' ' { 5.0 } else { 8.0 };
            self.glyphs.insert(c, FontGlyph {
                uv: [u0, v0, u1, v1],
                width: cell_w as f32,
                height: cell_h as f32,
                advance: adv,
            });
        }
    }

    /// Oblicza szerokosc tekstu w pikselach
    pub fn measure_text_width(&self, text: &str) -> f32 {
        let mut width = 0.0f32;
        for c in text.chars() {
            if let Some(glyph) = self.glyphs.get(&c) {
                width += glyph.advance;
            } else {
                width += 7.0;
            }
        }
        width
    }

    /// Generuje liste quadów 2D dla podanego tekstu
    pub fn build_text_quads(
        &self,
        text: &str,
        mut x: f32,
        y: f32,
        color: [f32; 4],
        scale: f32,
    ) -> Vec<Quad2D> {
        let mut quads = Vec::with_capacity(text.len());
        let default_glyph = self.glyphs.get(&'?');

        for c in text.chars() {
            if c == '\n' {
                continue;
            }
            let glyph = self.glyphs.get(&c).or(default_glyph);
            if let Some(g) = glyph {
                if c != ' ' {
                    let mut q = Quad2D::with_uv(
                        x,
                        y,
                        g.width * scale,
                        g.height * scale,
                        g.uv[0],
                        g.uv[1],
                        g.uv[2],
                        g.uv[3],
                    );
                    q.color = color;
                    quads.push(q);
                }
                x += g.advance * scale;
            } else {
                x += 7.0 * scale;
            }
        }
        quads
    }
}

static GLOBAL_FONT_ATLAS: OnceLock<Mutex<FontAtlas>> = OnceLock::new();

pub fn get_global_font_atlas() -> std::sync::MutexGuard<'static, FontAtlas> {
    GLOBAL_FONT_ATLAS
        .get_or_init(|| Mutex::new(FontAtlas::new()))
        .lock()
        .unwrap()
}
