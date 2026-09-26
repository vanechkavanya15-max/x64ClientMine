use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::ffi::CStr;
use std::os::raw::c_char;

// Funkcja pomocnicza do dekodowania DXT1 (RGB565)
fn rgb565_to_rgba(val: u16) -> (u8, u8, u8, u8) {
    let r = (((val >> 11) & 0x1F) as u32 * 255 / 31) as u8;
    let g = (((val >> 5) & 0x3F) as u32 * 255 / 63) as u8;
    let b = ((val & 0x1F) as u32 * 255 / 31) as u8;
    (r, g, b, 255)
}

fn lerp_rgb(c0: (u8, u8, u8, u8), c1: (u8, u8, u8, u8), n: u32, d: u32) -> (u8, u8, u8, u8) {
    let r = ((c0.0 as u32 * (d - n) + c1.0 as u32 * n) / d) as u8;
    let g = ((c0.1 as u32 * (d - n) + c1.1 as u32 * n) / d) as u8;
    let b = ((c0.2 as u32 * (d - n) + c1.2 as u32 * n) / d) as u8;
    (r, g, b, 255)
}

fn decode_dxt1_block(data: &[u8], out: &mut [u8], stride: usize) {
    if data.len() < 8 { return; }
    
    let c0_val = u16::from_le_bytes([data[0], data[1]]);
    let c1_val = u16::from_le_bytes([data[2], data[3]]);
    
    let c0 = rgb565_to_rgba(c0_val);
    let c1 = rgb565_to_rgba(c1_val);
    
    let mut colors = [c0, c1, (0, 0, 0, 0), (0, 0, 0, 0)];
    
    if c0_val > c1_val {
        colors[2] = lerp_rgb(c0, c1, 1, 3);
        colors[3] = lerp_rgb(c0, c1, 2, 3);
    } else {
        colors[2] = lerp_rgb(c0, c1, 1, 2);
        colors[3] = (0, 0, 0, 0); // Przezroczysty
    }
    
    let mut code = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
    
    for y in 0..4 {
        for x in 0..4 {
            let idx = (code & 0x03) as usize;
            code >>= 2;
            
            let p_idx = y * stride + x * 4;
            if p_idx + 3 < out.len() {
                out[p_idx] = colors[idx].0;
                out[p_idx + 1] = colors[idx].1;
                out[p_idx + 2] = colors[idx].2;
                out[p_idx + 3] = colors[idx].3;
            }
        }
    }
}

fn decode_dxt3_block(data: &[u8], out: &mut [u8], stride: usize) {
    if data.len() < 16 { return; }
    
    // Alpha
    let alpha_data = &data[0..8];
    
    // Color
    let c0_val = u16::from_le_bytes([data[8], data[9]]);
    let c1_val = u16::from_le_bytes([data[10], data[11]]);
    
    let c0 = rgb565_to_rgba(c0_val);
    let c1 = rgb565_to_rgba(c1_val);
    
    let mut colors = [c0, c1, (0, 0, 0, 0), (0, 0, 0, 0)];
    colors[2] = lerp_rgb(c0, c1, 1, 3);
    colors[3] = lerp_rgb(c0, c1, 2, 3);
    
    let mut code = u32::from_le_bytes([data[12], data[13], data[14], data[15]]);
    
    for y in 0..4 {
        for x in 0..4 {
            let idx = (code & 0x03) as usize;
            code >>= 2;
            
            let alpha_val = alpha_data[y * 2 + (x / 2)];
            let alpha = if x % 2 == 0 {
                (alpha_val & 0x0F) * 17
            } else {
                ((alpha_val >> 4) & 0x0F) * 17
            };
            
            let p_idx = y * stride + x * 4;
            if p_idx + 3 < out.len() {
                out[p_idx] = colors[idx].0;
                out[p_idx + 1] = colors[idx].1;
                out[p_idx + 2] = colors[idx].2;
                out[p_idx + 3] = alpha;
            }
        }
    }
}

fn decode_dxt5_block(data: &[u8], out: &mut [u8], stride: usize) {
    if data.len() < 16 { return; }
    
    // Alpha
    let a0 = data[0];
    let a1 = data[1];
    let mut alphas = [0u8; 8];
    alphas[0] = a0;
    alphas[1] = a1;
    
    if a0 > a1 {
        for i in 1..=6 {
            alphas[i + 1] = (((7 - i) as u32 * a0 as u32 + i as u32 * a1 as u32) / 7) as u8;
        }
    } else {
        for i in 1..=4 {
            alphas[i + 1] = (((5 - i) as u32 * a0 as u32 + i as u32 * a1 as u32) / 5) as u8;
        }
        alphas[6] = 0;
        alphas[7] = 255;
    }
    
    let alpha_bits = u64::from_le_bytes([
        data[2], data[3], data[4], data[5],
        data[6], data[7], 0, 0
    ]);
    
    // Color
    let c0_val = u16::from_le_bytes([data[8], data[9]]);
    let c1_val = u16::from_le_bytes([data[10], data[11]]);
    
    let c0 = rgb565_to_rgba(c0_val);
    let c1 = rgb565_to_rgba(c1_val);
    
    let mut colors = [c0, c1, (0, 0, 0, 0), (0, 0, 0, 0)];
    colors[2] = lerp_rgb(c0, c1, 1, 3);
    colors[3] = lerp_rgb(c0, c1, 2, 3);
    
    let mut code = u32::from_le_bytes([data[12], data[13], data[14], data[15]]);
    let mut alpha_code = alpha_bits;
    
    for y in 0..4 {
        for x in 0..4 {
            let idx = (code & 0x03) as usize;
            code >>= 2;
            
            let alpha_idx = (alpha_code & 0x07) as usize;
            alpha_code >>= 3;
            
            let p_idx = y * stride + x * 4;
            if p_idx + 3 < out.len() {
                out[p_idx] = colors[idx].0;
                out[p_idx + 1] = colors[idx].1;
                out[p_idx + 2] = colors[idx].2;
                out[p_idx + 3] = alphas[alpha_idx];
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn m2_image_decode_dxt(
    dxt_type: u32,
    data_ptr: *const u8,
    len: usize,
    width: u32,
    height: u32,
    out_buf: *mut u8,
) -> bool {
    safe_ffi_call(false, || {
        if data_ptr.is_null() || out_buf.is_null() || width == 0 || height == 0 || len == 0 {
            return false;
        }

        let out_len = (width as usize) * (height as usize) * 4;
        let in_data = unsafe { std::slice::from_raw_parts(data_ptr, len) };
        let out_data = unsafe { std::slice::from_raw_parts_mut(out_buf, out_len) };

        let block_size = if dxt_type == 1 { 8 } else { 16 };
        
        // Ensure we have enough data
        let blocks_x = (width as usize + 3) / 4;
        let blocks_y = (height as usize + 3) / 4;
        let required_len = blocks_x * blocks_y * block_size;
        
        if len < required_len {
            return false;
        }

        let stride = (width as usize) * 4;

        for by in 0..blocks_y {
            for bx in 0..blocks_x {
                let block_idx = by * blocks_x + bx;
                let offset = block_idx * block_size;
                let block = &in_data[offset..offset + block_size];
                
                // Czasami bloki leza na krańcach, musimy to przewidzieć
                let out_offset = (by * 4) * stride + (bx * 4) * 4;
                if out_offset < out_data.len() {
                    let _max_stride = if out_offset + stride * 4 <= out_data.len() {
                        stride * 4
                    } else {
                        out_data.len() - out_offset
                    };
                    
                    let mut temp_block = vec![0u8; 4 * 4 * 4];
                    let temp_stride = 16;
                    
                    match dxt_type {
                        1 => decode_dxt1_block(block, &mut temp_block, temp_stride),
                        3 => decode_dxt3_block(block, &mut temp_block, temp_stride),
                        5 => decode_dxt5_block(block, &mut temp_block, temp_stride),
                        _ => return false,
                    }
                    
                    // Kopiowanie z bufora tymczasowego do głownego bufora, z uwzględnieniem granic
                    for y in 0..4 {
                        if by * 4 + y >= height as usize { break; }
                        for x in 0..4 {
                            if bx * 4 + x >= width as usize { break; }
                            
                            let src_idx = y * temp_stride + x * 4;
                            let dst_idx = out_offset + y * stride + x * 4;
                            
                            if dst_idx + 3 < out_data.len() {
                                out_data[dst_idx] = temp_block[src_idx];
                                out_data[dst_idx + 1] = temp_block[src_idx + 1];
                                out_data[dst_idx + 2] = temp_block[src_idx + 2];
                                out_data[dst_idx + 3] = temp_block[src_idx + 3];
                            }
                        }
                    }
                }
            }
        }

        true
    })
}

#[no_mangle]
pub extern "C" fn m2_image_parse_sub(
    sub_text_ptr: *const c_char,
    out_left: *mut i32,
    out_top: *mut i32,
    out_right: *mut i32,
    out_bottom: *mut i32,
) -> bool {
    safe_ffi_call(false, || {
        if sub_text_ptr.is_null() || out_left.is_null() || out_top.is_null() || out_right.is_null() || out_bottom.is_null() {
            return false;
        }

        let c_str = unsafe { CStr::from_ptr(sub_text_ptr) };
        let text = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };

        let mut left: Option<i32> = None;
        let mut top: Option<i32> = None;
        let mut right: Option<i32> = None;
        let mut bottom: Option<i32> = None;

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }

            let key = parts[0].to_lowercase();
            let value = parts[1].parse::<i32>().unwrap_or(0);

            match key.as_str() {
                "left" => left = Some(value),
                "top" => top = Some(value),
                "right" => right = Some(value),
                "bottom" => bottom = Some(value),
                _ => {}
            }
        }

        if left.is_some() && top.is_some() && right.is_some() && bottom.is_some() {
            unsafe {
                *out_left = left.unwrap();
                *out_top = top.unwrap();
                *out_right = right.unwrap();
                *out_bottom = bottom.unwrap();
            }
            return true;
        }

        false
    })
}

#[no_mangle]
pub extern "C" fn m2_image_apply_colorkey(
    rgba_buf: *mut u8,
    len: usize,
    r: u8,
    g: u8,
    b: u8,
) -> bool {
    safe_ffi_call(false, || {
        if rgba_buf.is_null() || len == 0 || len % 4 != 0 {
            return false;
        }

        let data = unsafe { std::slice::from_raw_parts_mut(rgba_buf, len) };

        for i in (0..len).step_by(4) {
            if data[i] == r && data[i + 1] == g && data[i + 2] == b {
                data[i + 3] = 0; // Transparent
            } else {
                data[i + 3] = 255; // Opaque (zgodnie z implementacja dla colorkey alpha)
            }
        }

        true
    })
}

#[no_mangle]
pub extern "C" fn m2_image_decode_tga(
    data_ptr: *const u8,
    len: usize,
    out_width: *mut u32,
    out_height: *mut u32,
    out_buf: *mut u8,
    max_buf_len: usize,
) -> bool {
    safe_ffi_call(false, || {
        if data_ptr.is_null() || out_width.is_null() || out_height.is_null() || out_buf.is_null() || len == 0 {
            return false;
        }

        let in_data = unsafe { std::slice::from_raw_parts(data_ptr, len) };
        let img = match crate::tga_decoder::TgaImage::from_bytes(in_data) {
            Ok(im) => im,
            Err(_) => return false,
        };

        let w = img.width as u32;
        let h = img.height as u32;
        let required_bytes = (w as usize) * (h as usize) * 4;

        if max_buf_len < required_bytes || img.rgba_data.len() < required_bytes {
            return false;
        }

        unsafe {
            *out_width = w;
            *out_height = h;
            std::ptr::copy_nonoverlapping(img.rgba_data.as_ptr(), out_buf, required_bytes);
        }

        true
    })
}
