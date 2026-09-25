//! Modul dxt1_decoder dla skrzynki m2-client-image
//! Dekoder kompresji blokowej DXT1/BC1 (nieprzezroczyste/1-bit alpha)
//! Dekompresja blokow 4x4 DXT1, interpolacja kolorow RGB565, 1-bitowa maska alpha

/// Reprezentacja koloru RGBA, 8 bitow na kanal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

/// Dekoduje kolor zapisany w 16-bitowym formacie RGB565 do struktury Rgba (z alfa=255)
pub fn decode_rgb565(color: u16) -> Rgba {
    let r_5 = (color >> 11) & 0x1F;
    let g_6 = (color >> 5) & 0x3F;
    let b_5 = color & 0x1F;

    let r = ((r_5 as u32 * 255 + 15) / 31) as u8;
    let g = ((g_6 as u32 * 255 + 31) / 63) as u8;
    let b = ((b_5 as u32 * 255 + 15) / 31) as u8;

    Rgba::new(r, g, b, 255)
}

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct Dxt1Decoder {
    pub initialized: bool,
}

impl Dxt1Decoder {
    pub fn new() -> Self {
        Self { initialized: true }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Dekompresuje jeden 8-bajtowy blok DXT1 (4x4 piksele).
    /// Wynikiem jest tablica 16 struktur `Rgba` w kolejnosci wierszowej (od lewej do prawej, z gory na dol).
    pub fn decode_block(&self, block: &[u8]) -> Result<[Rgba; 16], &'static str> {
        if block.len() < 8 {
            return Err("Blok DXT1 musi miec 8 bajtow");
        }

        let c0_bits = u16::from_le_bytes([block[0], block[1]]);
        let c1_bits = u16::from_le_bytes([block[2], block[3]]);

        let color0 = decode_rgb565(c0_bits);
        let color1 = decode_rgb565(c1_bits);

        let mut colors = [Rgba::new(0, 0, 0, 0); 4];
        colors[0] = color0;
        colors[1] = color1;

        if c0_bits > c1_bits {
            // Brak przezroczystosci (nieprzezroczysty, interpolacja 1/3 i 2/3)
            colors[2] = Rgba::new(
                ((2 * color0.r as u32 + color1.r as u32) / 3) as u8,
                ((2 * color0.g as u32 + color1.g as u32) / 3) as u8,
                ((2 * color0.b as u32 + color1.b as u32) / 3) as u8,
                255,
            );
            colors[3] = Rgba::new(
                ((color0.r as u32 + 2 * color1.r as u32) / 3) as u8,
                ((color0.g as u32 + 2 * color1.g as u32) / 3) as u8,
                ((color0.b as u32 + 2 * color1.b as u32) / 3) as u8,
                255,
            );
        } else {
            // 1-bit alpha (interpolacja 1/2 i calkowicie przezroczysty)
            colors[2] = Rgba::new(
                ((color0.r as u32 + color1.r as u32) / 2) as u8,
                ((color0.g as u32 + color1.g as u32) / 2) as u8,
                ((color0.b as u32 + color1.b as u32) / 2) as u8,
                255,
            );
            colors[3] = Rgba::new(0, 0, 0, 0);
        }

        let mut indices = [0u8; 16];
        let mut idx_offset = 0;
        for i in 4..8 {
            let byte = block[i];
            indices[idx_offset] = byte & 0x03;
            indices[idx_offset + 1] = (byte >> 2) & 0x03;
            indices[idx_offset + 2] = (byte >> 4) & 0x03;
            indices[idx_offset + 3] = (byte >> 6) & 0x03;
            idx_offset += 4;
        }

        let mut result = [Rgba::new(0, 0, 0, 0); 16];
        for (i, &idx) in indices.iter().enumerate() {
            result[i] = colors[idx as usize];
        }

        Ok(result)
    }

    /// Dekompresuje caly obrazek z formatu DXT1 na jednowymiarowy bufor RGBA.
    /// Wymiary podane jako szerokosc (width) i wysokosc (height) w pikselach musza byc wielokrotnoscia 4.
    pub fn decode_image(&self, data: &[u8], width: usize, height: usize) -> Result<Vec<u8>, &'static str> {
        if width == 0 || height == 0 || width % 4 != 0 || height % 4 != 0 {
            return Err("Szerokosc i wysokosc musza byc wielokrotnosciami 4 i byc wieksze od 0");
        }

        let blocks_x = width / 4;
        let blocks_y = height / 4;
        let expected_len = blocks_x * blocks_y * 8;

        if data.len() < expected_len {
            return Err("Zbyt malo danych dla podanych wymiarow");
        }

        let mut output = vec![0u8; width * height * 4];

        for by in 0..blocks_y {
            for bx in 0..blocks_x {
                let block_offset = (by * blocks_x + bx) * 8;
                let block_data = &data[block_offset..block_offset + 8];
                let decoded_block = self.decode_block(block_data)?;

                for py in 0..4 {
                    for px in 0..4 {
                        let global_x = bx * 4 + px;
                        let global_y = by * 4 + py;
                        let out_offset = (global_y * width + global_x) * 4;
                        let color = decoded_block[py * 4 + px];
                        output[out_offset] = color.r;
                        output[out_offset + 1] = color.g;
                        output[out_offset + 2] = color.b;
                        output[out_offset + 3] = color.a;
                    }
                }
            }
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_dxt1_decoder() {
        let obj = Dxt1Decoder::new();
        assert!(obj.is_ready());
    }
}
