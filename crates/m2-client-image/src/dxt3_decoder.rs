//! Modul dxt3_decoder dla skrzynki m2-client-image
//! Dekoder kompresji blokowej DXT3/BC2 (jawny 4-bitowy kanal alpha)
//! Dekompresja blokow 4x4 DXT3, bezposredni kanal alpha 4 bity na piksel

/// Dekoduje pojedynczy blok 4x4 DXT3.
/// `block` musi miec dokladnie 16 bajtow.
/// `out` musi miec miejsce na 16 pikseli RGBA (64 bajty).
pub fn decode_dxt3_block(block: &[u8], out: &mut [u8]) {
    if block.len() < 16 || out.len() < 64 {
        return;
    }

    // Dekodowanie palety kolorow (podobnie jak DXT1, ale bez koloru przezroczystego w palecie)
    let c0 = u16::from_le_bytes([block[8], block[9]]);
    let c1 = u16::from_le_bytes([block[10], block[11]]);

    let r0 = ((c0 >> 11) & 0x1F) as u8;
    let g0 = ((c0 >> 5) & 0x3F) as u8;
    let b0 = (c0 & 0x1F) as u8;

    let r1 = ((c1 >> 11) & 0x1F) as u8;
    let g1 = ((c1 >> 5) & 0x3F) as u8;
    let b1 = (c1 & 0x1F) as u8;

    let mut colors = [[0u8; 3]; 4];

    // Kolor 0
    colors[0][0] = (r0 << 3) | (r0 >> 2);
    colors[0][1] = (g0 << 2) | (g0 >> 4);
    colors[0][2] = (b0 << 3) | (b0 >> 2);

    // Kolor 1
    colors[1][0] = (r1 << 3) | (r1 >> 2);
    colors[1][1] = (g1 << 2) | (g1 >> 4);
    colors[1][2] = (b1 << 3) | (b1 >> 2);

    // DXT3 zawsze uzywa 4 kolorow (interpolacja 1/3 i 2/3),
    // bo przezroczystosc jest w osobnym kanale alpha.
    colors[2][0] = ((2 * colors[0][0] as u16 + colors[1][0] as u16) / 3) as u8;
    colors[2][1] = ((2 * colors[0][1] as u16 + colors[1][1] as u16) / 3) as u8;
    colors[2][2] = ((2 * colors[0][2] as u16 + colors[1][2] as u16) / 3) as u8;

    colors[3][0] = ((colors[0][0] as u16 + 2 * colors[1][0] as u16) / 3) as u8;
    colors[3][1] = ((colors[0][1] as u16 + 2 * colors[1][1] as u16) / 3) as u8;
    colors[3][2] = ((colors[0][2] as u16 + 2 * colors[1][2] as u16) / 3) as u8;

    let color_indices = u32::from_le_bytes([block[12], block[13], block[14], block[15]]);

    // Odczytywanie kanalu alpha
    let mut alphas = [0u8; 16];
    for i in 0..8 {
        let alpha_byte = block[i];
        let a0 = alpha_byte & 0x0F;
        let a1 = (alpha_byte >> 4) & 0x0F;
        
        // Skalowanie 4-bit -> 8-bit poprzez powielenie bitow
        alphas[i * 2] = (a0 << 4) | a0;
        alphas[i * 2 + 1] = (a1 << 4) | a1;
    }

    for i in 0..16 {
        let index = (color_indices >> (i * 2)) & 0x03;
        let color = &colors[index as usize];
        
        let out_idx = i * 4;
        out[out_idx] = color[0];
        out[out_idx + 1] = color[1];
        out[out_idx + 2] = color[2];
        out[out_idx + 3] = alphas[i];
    }
}

/// Dekoduje caly obraz DXT3 o podanych wymiarach
pub fn decode_dxt3(data: &[u8], width: usize, height: usize) -> Result<Vec<u8>, &'static str> {
    if width == 0 || height == 0 {
        return Err("Nieprawidlowe wymiary obrazu");
    }

    let blocks_x = (width + 3) / 4;
    let blocks_y = (height + 3) / 4;
    let expected_len = blocks_x * blocks_y * 16;

    if data.len() < expected_len {
        return Err("Dane wejsciowe sa zbyt krotkie dla podanych wymiarow");
    }

    let mut out = vec![0u8; width * height * 4];
    let mut block_out = [0u8; 64];

    for by in 0..blocks_y {
        for bx in 0..blocks_x {
            let block_idx = by * blocks_x + bx;
            let data_offset = block_idx * 16;
            
            decode_dxt3_block(&data[data_offset..data_offset + 16], &mut block_out);
            
            for y in 0..4 {
                for x in 0..4 {
                    let px = bx * 4 + x;
                    let py = by * 4 + y;
                    
                    if px < width && py < height {
                        let out_idx = (py * width + px) * 4;
                        let block_out_idx = (y * 4 + x) * 4;
                        
                        out[out_idx] = block_out[block_out_idx];
                        out[out_idx + 1] = block_out[block_out_idx + 1];
                        out[out_idx + 2] = block_out[block_out_idx + 2];
                        out[out_idx + 3] = block_out[block_out_idx + 3];
                    }
                }
            }
        }
    }

    Ok(out)
}
