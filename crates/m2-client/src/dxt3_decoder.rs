pub fn decode_dxt3(data: &[u8], width: usize, height: usize) -> Result<Vec<u8>, &'static str> {
    if width == 0 || height == 0 {
        return Err("Width and height must be greater than 0");
    }
    if width % 4 != 0 || height % 4 != 0 {
        return Err("Width and height must be multiples of 4");
    }

    let blocks_x = width / 4;
    let blocks_y = height / 4;
    let expected_len = blocks_x * blocks_y * 16;

    if data.len() != expected_len {
        return Err("Invalid data size for DXT3");
    }

    let mut output = vec![0u8; width * height * 4];

    for by in 0..blocks_y {
        for bx in 0..blocks_x {
            let block_idx = by * blocks_x + bx;
            let block_offset = block_idx * 16;
            let block_data = &data[block_offset..block_offset + 16];

            // Decode alpha
            let mut alphas = [0u8; 16];
            for i in 0..8 {
                let byte = block_data[i];
                let a0 = byte & 0x0F;
                let a1 = (byte & 0xF0) >> 4;
                alphas[i * 2] = (a0 << 4) | a0;
                alphas[i * 2 + 1] = (a1 << 4) | a1;
            }

            // Decode colors
            let c0 = u16::from_le_bytes([block_data[8], block_data[9]]);
            let c1 = u16::from_le_bytes([block_data[10], block_data[11]]);

            let r0 = ((c0 >> 11) & 0x1F) as u32;
            let g0 = ((c0 >> 5) & 0x3F) as u32;
            let b0 = (c0 & 0x1F) as u32;

            let r1 = ((c1 >> 11) & 0x1F) as u32;
            let g1 = ((c1 >> 5) & 0x3F) as u32;
            let b1 = (c1 & 0x1F) as u32;

            // Expand to 8-bit
            let r0 = (r0 << 3) | (r0 >> 2);
            let g0 = (g0 << 2) | (g0 >> 4);
            let b0 = (b0 << 3) | (b0 >> 2);

            let r1 = (r1 << 3) | (r1 >> 2);
            let g1 = (g1 << 2) | (g1 >> 4);
            let b1 = (b1 << 3) | (b1 >> 2);

            let color0 = [r0 as u8, g0 as u8, b0 as u8];
            let color1 = [r1 as u8, g1 as u8, b1 as u8];

            let color2 = [
                ((2 * r0 + r1) / 3) as u8,
                ((2 * g0 + g1) / 3) as u8,
                ((2 * b0 + b1) / 3) as u8,
            ];

            let color3 = [
                ((r0 + 2 * r1) / 3) as u8,
                ((g0 + 2 * g1) / 3) as u8,
                ((b0 + 2 * b1) / 3) as u8,
            ];

            let colors = [color0, color1, color2, color3];

            // Decode indices and output
            let indices = u32::from_le_bytes([
                block_data[12],
                block_data[13],
                block_data[14],
                block_data[15],
            ]);

            for py in 0..4 {
                for px in 0..4 {
                    let pixel_idx = py * 4 + px;
                    let color_idx = (indices >> (pixel_idx * 2)) & 0x3;
                    let color = colors[color_idx as usize];
                    let alpha = alphas[pixel_idx as usize];

                    let out_x = bx * 4 + px;
                    let out_y = by * 4 + py;
                    let out_idx = (out_y * width + out_x) * 4;

                    output[out_idx] = color[0];
                    output[out_idx + 1] = color[1];
                    output[out_idx + 2] = color[2];
                    output[out_idx + 3] = alpha;
                }
            }
        }
    }

    Ok(output)
}
