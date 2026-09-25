//! Modul texture_processor dla skrzynki m2-client-image
//! Procesor tekstur: generowanie mipmap i usuwanie color key
//! Downsampling mipmap, obsluga przezroczystosci color key (0xFF00FF)

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct TextureProcessor {
    pub initialized: bool,
}

impl TextureProcessor {
    pub fn new() -> Self {
        Self { initialized: true }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Ustawia kanal alpha na 0 dla pikseli odpowiadajacych color key (0xFF00FF) dla bufora RGBA.
    pub fn apply_color_key_rgba(data: &mut [u8]) {
        for chunk in data.chunks_exact_mut(4) {
            // RGBA: 0=R, 1=G, 2=B, 3=A
            let r = chunk[0];
            let g = chunk[1];
            let b = chunk[2];
            if r == 255 && g == 0 && b == 255 {
                chunk[3] = 0;
            }
        }
    }

    /// Ustawia kanal alpha na 0 dla pikseli odpowiadajacych color key (0xFF00FF) dla bufora BGRA.
    pub fn apply_color_key_bgra(data: &mut [u8]) {
        for chunk in data.chunks_exact_mut(4) {
            // BGRA: 0=B, 1=G, 2=R, 3=A
            let b = chunk[0];
            let g = chunk[1];
            let r = chunk[2];
            if r == 255 && g == 0 && b == 255 {
                chunk[3] = 0;
            }
        }
    }

    /// Generuje kolejna warstwe mipmap (pomniejszona o polowe, box filter) dla obrazu 32-bitowego (RGBA/BGRA).
    /// Zwraca (nowa_szerokosc, nowa_wysokosc, nowe_dane).
    pub fn generate_mipmap_rgba(width: usize, height: usize, data: &[u8]) -> Option<(usize, usize, Vec<u8>)> {
        if width == 0 || height == 0 || data.len() < width * height * 4 {
            return None;
        }

        let new_width = std::cmp::max(1, width / 2);
        let new_height = std::cmp::max(1, height / 2);
        let mut new_data = vec![0; new_width * new_height * 4];

        for y in 0..new_height {
            for x in 0..new_width {
                let mut c0 = 0u32;
                let mut c1 = 0u32;
                let mut c2 = 0u32;
                let mut c3 = 0u32;
                let mut count = 0u32;

                let src_y = y * 2;
                let src_x = x * 2;

                let max_y = std::cmp::min(src_y + 2, height);
                let max_x = std::cmp::min(src_x + 2, width);

                for sy in src_y..max_y {
                    for sx in src_x..max_x {
                        let idx = (sy * width + sx) * 4;
                        if idx + 3 < data.len() {
                            c0 += data[idx] as u32;
                            c1 += data[idx + 1] as u32;
                            c2 += data[idx + 2] as u32;
                            c3 += data[idx + 3] as u32;
                            count += 1;
                        }
                    }
                }

                if count > 0 {
                    let dst_idx = (y * new_width + x) * 4;
                    new_data[dst_idx] = (c0 / count) as u8;
                    new_data[dst_idx + 1] = (c1 / count) as u8;
                    new_data[dst_idx + 2] = (c2 / count) as u8;
                    new_data[dst_idx + 3] = (c3 / count) as u8;
                }
            }
        }

        Some((new_width, new_height, new_data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_texture_processor() {
        let obj = TextureProcessor::new();
        assert!(obj.is_ready());
    }
}