pub fn decode_dxt5(data: &[u8], width: usize, height: usize) -> Result<Vec<u8>, &'static str> {
    if width == 0 || height == 0 {
        return Err("Width and height must be greater than zero");
    }
    if width % 4 != 0 || height % 4 != 0 {
        return Err("Width and height must be multiples of 4");
    }

    let block_count_x = width / 4;
    let block_count_y = height / 4;
    let total_blocks = block_count_x * block_count_y;

    if data.len() < total_blocks * 16 {
        return Err("Data buffer is too small for given dimensions");
    }

    let mut output = vec![0u8; width * height * 4];

    for by in 0..block_count_y {
        for bx in 0..block_count_x {
            let block_offset = (by * block_count_x + bx) * 16;
            let block = &data[block_offset..block_offset + 16];

            // 1. Alpha block
            let alpha0 = block[0];
            let alpha1 = block[1];
            let mut a = [0u8; 8];
            a[0] = alpha0;
            a[1] = alpha1;

            if alpha0 > alpha1 {
                a[2] = ((6 * a[0] as u16 + 1 * a[1] as u16) / 7) as u8;
                a[3] = ((5 * a[0] as u16 + 2 * a[1] as u16) / 7) as u8;
                a[4] = ((4 * a[0] as u16 + 3 * a[1] as u16) / 7) as u8;
                a[5] = ((3 * a[0] as u16 + 4 * a[1] as u16) / 7) as u8;
                a[6] = ((2 * a[0] as u16 + 5 * a[1] as u16) / 7) as u8;
                a[7] = ((1 * a[0] as u16 + 6 * a[1] as u16) / 7) as u8;
            } else {
                a[2] = ((4 * a[0] as u16 + 1 * a[1] as u16) / 5) as u8;
                a[3] = ((3 * a[0] as u16 + 2 * a[1] as u16) / 5) as u8;
                a[4] = ((2 * a[0] as u16 + 3 * a[1] as u16) / 5) as u8;
                a[5] = ((1 * a[0] as u16 + 4 * a[1] as u16) / 5) as u8;
                a[6] = 0;
                a[7] = 255;
            }

            let alpha_indices: u64 = (block[2] as u64)
                | ((block[3] as u64) << 8)
                | ((block[4] as u64) << 16)
                | ((block[5] as u64) << 24)
                | ((block[6] as u64) << 32)
                | ((block[7] as u64) << 40);

            // 2. Color block
            let color0 = u16::from_le_bytes([block[8], block[9]]);
            let color1 = u16::from_le_bytes([block[10], block[11]]);
            
            let r0 = ((color0 >> 11) & 0x1F) as u8;
            let g0 = ((color0 >> 5) & 0x3F) as u8;
            let b0 = (color0 & 0x1F) as u8;
            let r0 = (r0 << 3) | (r0 >> 2);
            let g0 = (g0 << 2) | (g0 >> 4);
            let b0 = (b0 << 3) | (b0 >> 2);

            let r1 = ((color1 >> 11) & 0x1F) as u8;
            let g1 = ((color1 >> 5) & 0x3F) as u8;
            let b1 = (color1 & 0x1F) as u8;
            let r1 = (r1 << 3) | (r1 >> 2);
            let g1 = (g1 << 2) | (g1 >> 4);
            let b1 = (b1 << 3) | (b1 >> 2);

            let mut c_r = [0u8; 4];
            let mut c_g = [0u8; 4];
            let mut c_b = [0u8; 4];
            c_r[0] = r0; c_g[0] = g0; c_b[0] = b0;
            c_r[1] = r1; c_g[1] = g1; c_b[1] = b1;

            if color0 > color1 {
                c_r[2] = ((2 * r0 as u16 + r1 as u16) / 3) as u8;
                c_g[2] = ((2 * g0 as u16 + g1 as u16) / 3) as u8;
                c_b[2] = ((2 * b0 as u16 + b1 as u16) / 3) as u8;
                c_r[3] = ((r0 as u16 + 2 * r1 as u16) / 3) as u8;
                c_g[3] = ((g0 as u16 + 2 * g1 as u16) / 3) as u8;
                c_b[3] = ((b0 as u16 + 2 * b1 as u16) / 3) as u8;
            } else {
                c_r[2] = ((r0 as u16 + r1 as u16) / 2) as u8;
                c_g[2] = ((g0 as u16 + g1 as u16) / 2) as u8;
                c_b[2] = ((b0 as u16 + b1 as u16) / 2) as u8;
                c_r[3] = 0;
                c_g[3] = 0;
                c_b[3] = 0;
            }

            let color_indices = u32::from_le_bytes([block[12], block[13], block[14], block[15]]);

            // 3. Write pixels
            for i in 0..16 {
                let px = i % 4;
                let py = i / 4;
                
                let alpha_index = ((alpha_indices >> (i * 3)) & 0x07) as usize;
                let color_index = ((color_indices >> (i * 2)) & 0x03) as usize;

                let target_x = bx * 4 + px;
                let target_y = by * 4 + py;
                let out_idx = (target_y * width + target_x) * 4;

                output[out_idx] = c_r[color_index];
                output[out_idx + 1] = c_g[color_index];
                output[out_idx + 2] = c_b[color_index];
                output[out_idx + 3] = a[alpha_index];
            }
        }
    }

    Ok(output)
}
