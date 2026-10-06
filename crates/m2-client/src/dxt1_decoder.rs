pub fn decode_dxt1(data: &[u8], width: usize, height: usize) -> Result<Vec<u8>, &'static str> {
    if width == 0 || height == 0 || width % 4 != 0 || height % 4 != 0 {
        return Err("Width and height must be non-zero and multiples of 4");
    }

    let block_w = width / 4;
    let block_h = height / 4;
    let expected_len = block_w * block_h * 8;

    if data.len() < expected_len {
        return Err("Invalid buffer length for given dimensions");
    }

    let mut rgba_buffer = vec![0u8; width * height * 4];

    for by in 0..block_h {
        for bx in 0..block_w {
            let offset = (by * block_w + bx) * 8;
            let block_data = &data[offset..offset + 8];

            let c0 = u16::from_le_bytes([block_data[0], block_data[1]]);
            let c1 = u16::from_le_bytes([block_data[2], block_data[3]]);
            let indices = u32::from_le_bytes([block_data[4], block_data[5], block_data[6], block_data[7]]);

            let color0 = unpack_rgb565(c0);
            let color1 = unpack_rgb565(c1);
            let mut colors = [[0u8; 4]; 4];

            colors[0] = color0;
            colors[1] = color1;

            if c0 > c1 {
                // Tryb nieprzezroczysty
                colors[2] = [
                    ((2 * color0[0] as u16 + color1[0] as u16) / 3) as u8,
                    ((2 * color0[1] as u16 + color1[1] as u16) / 3) as u8,
                    ((2 * color0[2] as u16 + color1[2] as u16) / 3) as u8,
                    255,
                ];
                colors[3] = [
                    ((color0[0] as u16 + 2 * color1[0] as u16) / 3) as u8,
                    ((color0[1] as u16 + 2 * color1[1] as u16) / 3) as u8,
                    ((color0[2] as u16 + 2 * color1[2] as u16) / 3) as u8,
                    255,
                ];
            } else {
                // Tryb przezroczysty 1-bit alpha
                colors[2] = [
                    ((color0[0] as u16 + color1[0] as u16) / 2) as u8,
                    ((color0[1] as u16 + color1[1] as u16) / 2) as u8,
                    ((color0[2] as u16 + color1[2] as u16) / 2) as u8,
                    255,
                ];
                colors[3] = [0, 0, 0, 0];
            }

            for py in 0..4 {
                for px in 0..4 {
                    // Indeksy sa liczone bitowo (od najmlodszych bitow dla pierwszego piksela w bloku)
                    let pixel_index = (py * 4) + px; // od 0 do 15
                    let index = (indices >> (pixel_index * 2)) & 0b11;
                    
                    let global_x = bx * 4 + px;
                    let global_y = by * 4 + py;

                    let target_offset = (global_y * width + global_x) * 4;
                    let color = colors[index as usize];
                    
                    rgba_buffer[target_offset] = color[0];
                    rgba_buffer[target_offset + 1] = color[1];
                    rgba_buffer[target_offset + 2] = color[2];
                    rgba_buffer[target_offset + 3] = color[3];
                }
            }
        }
    }

    Ok(rgba_buffer)
}

fn unpack_rgb565(c: u16) -> [u8; 4] {
    let r5 = (c >> 11) & 0x1F;
    let g6 = (c >> 5) & 0x3F;
    let b5 = c & 0x1F;

    let r = ((r5 << 3) | (r5 >> 2)) as u8;
    let g = ((g6 << 2) | (g6 >> 4)) as u8;
    let b = ((b5 << 3) | (b5 >> 2)) as u8;

    [r, g, b, 255]
}
