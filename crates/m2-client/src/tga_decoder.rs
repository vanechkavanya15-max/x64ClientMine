pub struct TgaImage {
    pub width: u16,
    pub height: u16,
    pub rgba_data: Vec<u8>,
}

impl TgaImage {
    pub fn from_bytes(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < 18 {
            return Err("TGA header is too small");
        }

        let id_len = data[0];
        // data[1] is colormap_type
        let image_type = data[2];
        // bytes 3..8 are colormap_spec

        let _x_origin = u16::from_le_bytes([data[8], data[9]]);
        let _y_origin = u16::from_le_bytes([data[10], data[11]]);
        let width = u16::from_le_bytes([data[12], data[13]]);
        let height = u16::from_le_bytes([data[14], data[15]]);
        let bpp = data[16];
        let descriptor = data[17];

        if width == 0 || height == 0 {
            return Err("Invalid image dimensions");
        }

        if image_type != 2 && image_type != 10 {
            return Err("Unsupported TGA image type (only 2 and 10 are supported)");
        }

        if bpp != 24 && bpp != 32 {
            return Err("Unsupported bpp (only 24 and 32 are supported)");
        }

        let pixel_bytes = (bpp / 8) as usize;
        let mut offset = 18 + id_len as usize;

        // Ignore color map if present (not supported for true color, but skip it anyway)
        let colormap_type = data[1];
        if colormap_type == 1 {
            let colormap_length = u16::from_le_bytes([data[5], data[6]]) as usize;
            let colormap_entry_size = data[7] as usize;
            offset += colormap_length * ((colormap_entry_size + 7) / 8);
        }

        let num_pixels = (width as usize) * (height as usize);
        let mut rgba_data = vec![0u8; num_pixels * 4];

        if image_type == 2 {
            // Uncompressed
            let required_bytes = num_pixels * pixel_bytes;
            if offset + required_bytes > data.len() {
                return Err("Unexpected EOF while reading uncompressed pixels");
            }

            let mut out_idx = 0;
            for _ in 0..num_pixels {
                let b = data[offset];
                let g = data[offset + 1];
                let r = data[offset + 2];
                let a = if bpp == 32 { data[offset + 3] } else { 255 };
                
                rgba_data[out_idx] = r;
                rgba_data[out_idx + 1] = g;
                rgba_data[out_idx + 2] = b;
                rgba_data[out_idx + 3] = a;

                offset += pixel_bytes;
                out_idx += 4;
            }
        } else if image_type == 10 {
            // RLE Compressed
            let mut out_idx = 0;
            let mut pixels_read = 0;

            while pixels_read < num_pixels {
                if offset >= data.len() {
                    return Err("Unexpected EOF while reading RLE pixels");
                }
                
                let packet_header = data[offset];
                offset += 1;

                let is_rle = (packet_header & 0x80) != 0;
                let count = ((packet_header & 0x7F) as usize) + 1;

                if pixels_read + count > num_pixels {
                    return Err("RLE packet exceeds image boundaries");
                }

                if is_rle {
                    if offset + pixel_bytes > data.len() {
                        return Err("Unexpected EOF while reading RLE pixel");
                    }
                    
                    let b = data[offset];
                    let g = data[offset + 1];
                    let r = data[offset + 2];
                    let a = if bpp == 32 { data[offset + 3] } else { 255 };
                    offset += pixel_bytes;

                    for _ in 0..count {
                        rgba_data[out_idx] = r;
                        rgba_data[out_idx + 1] = g;
                        rgba_data[out_idx + 2] = b;
                        rgba_data[out_idx + 3] = a;
                        out_idx += 4;
                    }
                    pixels_read += count;
                } else {
                    if offset + count * pixel_bytes > data.len() {
                        return Err("Unexpected EOF while reading RAW pixels");
                    }

                    for _ in 0..count {
                        let b = data[offset];
                        let g = data[offset + 1];
                        let r = data[offset + 2];
                        let a = if bpp == 32 { data[offset + 3] } else { 255 };
                        offset += pixel_bytes;

                        rgba_data[out_idx] = r;
                        rgba_data[out_idx + 1] = g;
                        rgba_data[out_idx + 2] = b;
                        rgba_data[out_idx + 3] = a;
                        out_idx += 4;
                    }
                    pixels_read += count;
                }
            }
        }

        // Descriptor bit 5 specifies whether origin is top-left (1) or bottom-left (0).
        // OpenGL typically expects bottom-left, but many engines expect top-left.
        // The spec requires flipping rows if origin is bottom-left (0) so the output data
        // represents a top-to-bottom layout in memory.
        let top_to_bottom = (descriptor & (1 << 5)) != 0;
        
        if !top_to_bottom {
            let row_bytes = (width as usize) * 4;
            let half_height = (height as usize) / 2;
            let mut temp_row = vec![0u8; row_bytes];

            for y in 0..half_height {
                let top_idx = y * row_bytes;
                let bottom_idx = ((height as usize) - 1 - y) * row_bytes;

                temp_row.copy_from_slice(&rgba_data[top_idx..top_idx + row_bytes]);
                rgba_data.copy_within(bottom_idx..bottom_idx + row_bytes, top_idx);
                rgba_data[bottom_idx..bottom_idx + row_bytes].copy_from_slice(&temp_row);
            }
        }

        Ok(Self {
            width,
            height,
            rgba_data,
        })
    }
}
