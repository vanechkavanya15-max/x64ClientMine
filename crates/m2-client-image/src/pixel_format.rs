//! Modul pixel_format dla skrzynki m2-client-image
//! Formaty pikseli i konwersje przestrzeni barwnych
//! PixelFormat (RGBA8, BGRA8, RGB565, ARGB1555, DXT1, DXT3, DXT5) oraz konwersje

/// Typy obslugiwanych formatow pikseli
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    RGBA8,
    BGRA8,
    RGB565,
    ARGB1555,
    DXT1,
    DXT3,
    DXT5,
}

impl PixelFormat {
    /// Zwraca liczbe bajtow na piksel, jesli format jest nieskompresowany
    pub fn bytes_per_pixel(&self) -> Option<usize> {
        match self {
            PixelFormat::RGBA8 | PixelFormat::BGRA8 => Some(4),
            PixelFormat::RGB565 | PixelFormat::ARGB1555 => Some(2),
            PixelFormat::DXT1 | PixelFormat::DXT3 | PixelFormat::DXT5 => None,
        }
    }

    /// Zwraca true, jesli format to jeden z formatow skompresowanych DXT
    pub fn is_compressed(&self) -> bool {
        match self {
            PixelFormat::DXT1 | PixelFormat::DXT3 | PixelFormat::DXT5 => true,
            _ => false,
        }
    }

    /// Zwraca rozmiar bloku DXT (4x4 piksele) w bajtach, jesli format jest skompresowany
    pub fn block_size_bytes(&self) -> Option<usize> {
        match self {
            PixelFormat::DXT1 => Some(8),
            PixelFormat::DXT3 | PixelFormat::DXT5 => Some(16),
            _ => None,
        }
    }
}

/// Konwersja pojedynczego piksela RGBA8 do BGRA8
#[inline]
pub fn rgba8_to_bgra8(r: u8, g: u8, b: u8, a: u8) -> (u8, u8, u8, u8) {
    (b, g, r, a)
}

/// Konwersja pojedynczego piksela RGBA8 do RGB565 (bez kanalu alpha)
#[inline]
pub fn rgba8_to_rgb565(r: u8, g: u8, b: u8) -> u16 {
    let r5 = (r as u16) >> 3;
    let g6 = (g as u16) >> 2;
    let b5 = (b as u16) >> 3;
    (r5 << 11) | (g6 << 5) | b5
}

/// Konwersja pojedynczego piksela RGBA8 do ARGB1555 (1 bit alpha, 5 bitow RGB)
#[inline]
pub fn rgba8_to_argb1555(r: u8, g: u8, b: u8, a: u8) -> u16 {
    let a1 = if a >= 128 { 1 } else { 0 };
    let r5 = (r as u16) >> 3;
    let g5 = (g as u16) >> 3;
    let b5 = (b as u16) >> 3;
    (a1 << 15) | (r5 << 10) | (g5 << 5) | b5
}

/// Konwersja pojedynczego piksela RGB565 do RGBA8
#[inline]
pub fn rgb565_to_rgba8(pixel: u16) -> (u8, u8, u8, u8) {
    let r5 = (pixel >> 11) & 0x1F;
    let g6 = (pixel >> 5) & 0x3F;
    let b5 = pixel & 0x1F;
    
    let r = ((r5 * 255) + 15) / 31;
    let g = ((g6 * 255) + 31) / 63;
    let b = ((b5 * 255) + 15) / 31;
    
    (r as u8, g as u8, b as u8, 255)
}

/// Konwersja pojedynczego piksela ARGB1555 do RGBA8
#[inline]
pub fn argb1555_to_rgba8(pixel: u16) -> (u8, u8, u8, u8) {
    let a1 = (pixel >> 15) & 0x1;
    let r5 = (pixel >> 10) & 0x1F;
    let g5 = (pixel >> 5) & 0x1F;
    let b5 = pixel & 0x1F;
    
    let a = if a1 == 1 { 255 } else { 0 };
    let r = ((r5 * 255) + 15) / 31;
    let g = ((g5 * 255) + 15) / 31;
    let b = ((b5 * 255) + 15) / 31;
    
    (r as u8, g as u8, b as u8, a)
}

/// Konwertuje caly bufor z RGBA8 do BGRA8
pub fn convert_buffer_rgba8_to_bgra8(src: &[u8], dst: &mut [u8]) -> Result<(), &'static str> {
    if src.len() != dst.len() {
        return Err("Rozmiary buforow wejsciowego i wyjsciowego musza byc rowne");
    }
    if src.len() % 4 != 0 {
        return Err("Rozmiar bufora musi byc wielokrotnoscia 4 (RGBA)");
    }
    
    for i in (0..src.len()).step_by(4) {
        let (b, g, r, a) = rgba8_to_bgra8(src[i], src[i+1], src[i+2], src[i+3]);
        dst[i] = b;
        dst[i+1] = g;
        dst[i+2] = r;
        dst[i+3] = a;
    }
    Ok(())
}

/// Konwertuje caly bufor z RGBA8 do RGB565
pub fn convert_buffer_rgba8_to_rgb565(src: &[u8], dst: &mut [u8]) -> Result<(), &'static str> {
    if src.len() / 2 != dst.len() {
        return Err("Rozmiar bufora wyjsciowego musi byc polowa rozmiaru bufora wejsciowego");
    }
    if src.len() % 4 != 0 {
        return Err("Rozmiar bufora wejsciowego musi byc wielokrotnoscia 4 (RGBA)");
    }
    
    for i in (0..src.len()).step_by(4) {
        let pixel16 = rgba8_to_rgb565(src[i], src[i+1], src[i+2]);
        let out_idx = i / 2;
        let bytes = pixel16.to_le_bytes();
        dst[out_idx] = bytes[0];
        dst[out_idx+1] = bytes[1];
    }
    Ok(())
}

/// Konwertuje caly bufor z RGBA8 do ARGB1555
pub fn convert_buffer_rgba8_to_argb1555(src: &[u8], dst: &mut [u8]) -> Result<(), &'static str> {
    if src.len() / 2 != dst.len() {
        return Err("Rozmiar bufora wyjsciowego musi byc polowa rozmiaru bufora wejsciowego");
    }
    if src.len() % 4 != 0 {
        return Err("Rozmiar bufora wejsciowego musi byc wielokrotnoscia 4 (RGBA)");
    }
    
    for i in (0..src.len()).step_by(4) {
        let pixel16 = rgba8_to_argb1555(src[i], src[i+1], src[i+2], src[i+3]);
        let out_idx = i / 2;
        let bytes = pixel16.to_le_bytes();
        dst[out_idx] = bytes[0];
        dst[out_idx+1] = bytes[1];
    }
    Ok(())
}

/// Konwertuje caly bufor z RGB565 do RGBA8
pub fn convert_buffer_rgb565_to_rgba8(src: &[u8], dst: &mut [u8]) -> Result<(), &'static str> {
    if src.len() * 2 != dst.len() {
        return Err("Rozmiar bufora wyjsciowego musi byc dwukrotnoscia rozmiaru bufora wejsciowego");
    }
    if src.len() % 2 != 0 {
        return Err("Rozmiar bufora wejsciowego musi byc wielokrotnoscia 2");
    }
    
    for i in (0..src.len()).step_by(2) {
        let pixel16 = u16::from_le_bytes([src[i], src[i+1]]);
        let (r, g, b, a) = rgb565_to_rgba8(pixel16);
        let out_idx = i * 2;
        dst[out_idx] = r;
        dst[out_idx+1] = g;
        dst[out_idx+2] = b;
        dst[out_idx+3] = a;
    }
    Ok(())
}

/// Konwertuje caly bufor z ARGB1555 do RGBA8
pub fn convert_buffer_argb1555_to_rgba8(src: &[u8], dst: &mut [u8]) -> Result<(), &'static str> {
    if src.len() * 2 != dst.len() {
        return Err("Rozmiar bufora wyjsciowego musi byc dwukrotnoscia rozmiaru bufora wejsciowego");
    }
    if src.len() % 2 != 0 {
        return Err("Rozmiar bufora wejsciowego musi byc wielokrotnoscia 2");
    }
    
    for i in (0..src.len()).step_by(2) {
        let pixel16 = u16::from_le_bytes([src[i], src[i+1]]);
        let (r, g, b, a) = argb1555_to_rgba8(pixel16);
        let out_idx = i * 2;
        dst[out_idx] = r;
        dst[out_idx+1] = g;
        dst[out_idx+2] = b;
        dst[out_idx+3] = a;
    }
    Ok(())
}
