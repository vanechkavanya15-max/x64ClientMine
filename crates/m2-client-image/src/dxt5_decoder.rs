//! Modul dxt5_decoder dla skrzynki m2-client-image
//! Dekoder kompresji blokowej DXT5/BC3 (8-stopniowy kanal alpha)

/// Glowna struktura dekodera DXT5.
#[derive(Debug, Clone, Default)]
pub struct Dxt5Decoder;

impl Dxt5Decoder {
    /// Tworzy nowa instancje dekodera.
    pub fn new() -> Self {
        Self
    }

    /// Dekoduje pojedynczy blok 4x4 pikseli (16 bajtow) formatu DXT5.
    /// Zwraca 16 pikseli w formacie RGBA (4 bajty na piksel).
    pub fn decode_block(&self, block: &[u8]) -> Result<[u8; 64], &'static str> {
        if block.len() < 16 {
            return Err("Blok DXT5 musi miec co najmniej 16 bajtow");
        }

        let mut output = [0u8; 64];

        // 1. Dekodowanie bloku alpha (pierwsze 8 bajtow)
        let a0 = block[0];
        let a1 = block[1];

        let mut alpha_palette = [0u8; 8];
        alpha_palette[0] = a0;
        alpha_palette[1] = a1;

        if a0 > a1 {
            // 8-stopniowa interpolacja
            alpha_palette[2] = ((6 * a0 as u16 + 1 * a1 as u16) / 7) as u8;
            alpha_palette[3] = ((5 * a0 as u16 + 2 * a1 as u16) / 7) as u8;
            alpha_palette[4] = ((4 * a0 as u16 + 3 * a1 as u16) / 7) as u8;
            alpha_palette[5] = ((3 * a0 as u16 + 4 * a1 as u16) / 7) as u8;
            alpha_palette[6] = ((2 * a0 as u16 + 5 * a1 as u16) / 7) as u8;
            alpha_palette[7] = ((1 * a0 as u16 + 6 * a1 as u16) / 7) as u8;
        } else {
            // 6-stopniowa interpolacja + 0 i 255
            alpha_palette[2] = ((4 * a0 as u16 + 1 * a1 as u16) / 5) as u8;
            alpha_palette[3] = ((3 * a0 as u16 + 2 * a1 as u16) / 5) as u8;
            alpha_palette[4] = ((2 * a0 as u16 + 3 * a1 as u16) / 5) as u8;
            alpha_palette[5] = ((1 * a0 as u16 + 4 * a1 as u16) / 5) as u8;
            alpha_palette[6] = 0;
            alpha_palette[7] = 255;
        }

        let alpha_indices: u64 = (block[2] as u64) |
                                 ((block[3] as u64) << 8) |
                                 ((block[4] as u64) << 16) |
                                 ((block[5] as u64) << 24) |
                                 ((block[6] as u64) << 32) |
                                 ((block[7] as u64) << 40);

        // 2. Dekodowanie bloku kolorow (kolejne 8 bajtow)
        let color_block = &block[8..16];
        let c0 = u16::from_le_bytes([color_block[0], color_block[1]]);
        let c1 = u16::from_le_bytes([color_block[2], color_block[3]]);

        let mut color_palette = [[0u8; 3]; 4];

        // Wyciaganie kolorow RGB565
        let extract_color = |c: u16| -> [u8; 3] {
            let r = ((c >> 11) & 0x1F) as u8;
            let g = ((c >> 5) & 0x3F) as u8;
            let b = (c & 0x1F) as u8;
            [
                (r << 3) | (r >> 2),
                (g << 2) | (g >> 4),
                (b << 3) | (b >> 2),
            ]
        };

        color_palette[0] = extract_color(c0);
        color_palette[1] = extract_color(c1);

        // W DXT5 kolory sa zawsze w trybie 4-kolorowym, ale czesto dekodery 
        // zachowuja konwencje sprawdzania c0 > c1 dla bezpieczenstwa.
        // My uzyjemy 4-kolorowego trybu jako domyslnego dla BC3.
        color_palette[2] = [
            ((2 * color_palette[0][0] as u16 + color_palette[1][0] as u16) / 3) as u8,
            ((2 * color_palette[0][1] as u16 + color_palette[1][1] as u16) / 3) as u8,
            ((2 * color_palette[0][2] as u16 + color_palette[1][2] as u16) / 3) as u8,
        ];
        color_palette[3] = [
            ((color_palette[0][0] as u16 + 2 * color_palette[1][0] as u16) / 3) as u8,
            ((color_palette[0][1] as u16 + 2 * color_palette[1][1] as u16) / 3) as u8,
            ((color_palette[0][2] as u16 + 2 * color_palette[1][2] as u16) / 3) as u8,
        ];

        let color_indices = u32::from_le_bytes([
            color_block[4], color_block[5], color_block[6], color_block[7]
        ]);

        // 3. Budowanie 16 pikseli
        for i in 0..16 {
            let alpha_index = ((alpha_indices >> (i * 3)) & 0x07) as usize;
            let a = alpha_palette[alpha_index];

            let color_index = ((color_indices >> (i * 2)) & 0x03) as usize;
            let color = color_palette[color_index];

            output[i * 4] = color[0];     // R
            output[i * 4 + 1] = color[1]; // G
            output[i * 4 + 2] = color[2]; // B
            output[i * 4 + 3] = a;        // A
        }

        Ok(output)
    }

    /// Dekoduje caly obraz z formatu DXT5 do formatu RGBA,
    /// uwzgledniajac wymiary i przycinajac do podanej szerokosci/wysokosci.
    pub fn decode_image(
        &self,
        data: &[u8],
        width: usize,
        height: usize,
    ) -> Result<Vec<u8>, &'static str> {
        let blocks_x = (width + 3) / 4;
        let blocks_y = (height + 3) / 4;
        let expected_size = blocks_x.checked_mul(blocks_y).and_then(|m| m.checked_mul(16)).unwrap_or(usize::MAX);

        if data.len() < expected_size {
            return Err("Bufor danych jest zbyt maly dla podanych wymiarow");
        }

        let mut image_data = vec![0u8; width * height * 4];

        for by in 0..blocks_y {
            for bx in 0..blocks_x {
                let block_offset = (by * blocks_x + bx) * 16;
                let block = &data[block_offset..block_offset + 16];
                
                let decoded_block = self.decode_block(block)?;

                for py in 0..4 {
                    let pixel_y = by * 4 + py;
                    if pixel_y >= height {
                        break;
                    }

                    for px in 0..4 {
                        let pixel_x = bx * 4 + px;
                        if pixel_x >= width {
                            break;
                        }

                        let block_pixel_idx = py * 4 + px;
                        let out_idx = (pixel_y * width + pixel_x) * 4;

                        image_data[out_idx] = decoded_block[block_pixel_idx * 4];
                        image_data[out_idx + 1] = decoded_block[block_pixel_idx * 4 + 1];
                        image_data[out_idx + 2] = decoded_block[block_pixel_idx * 4 + 2];
                        image_data[out_idx + 3] = decoded_block[block_pixel_idx * 4 + 3];
                    }
                }
            }
        }

        Ok(image_data)
    }
}
