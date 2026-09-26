#[derive(Debug)]
pub struct BmpImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u32>, // RGBA 32-bit (0xAABBGGRR in little endian, or typically stored as RGBA u32)
}

impl BmpImage {
    pub fn from_bytes(data: &[u8]) -> Result<BmpImage, &'static str> {
        if data.len() < 14 + 40 {
            return Err("Plik jest za krotki aby zawierac naglowek BMP");
        }

        // BITMAPFILEHEADER (14 bytes)
        let bf_type = u16::from_le_bytes([data[0], data[1]]);
        if bf_type != 0x4D42 {
            return Err("Nieprawidlowy magic number (oczekiwano 'BM')");
        }

        let _bf_size = u32::from_le_bytes([data[2], data[3], data[4], data[5]]);
        let bf_off_bits = u32::from_le_bytes([data[10], data[11], data[12], data[13]]) as usize;

        // BITMAPINFOHEADER (min 40 bytes)
        let bi_size = u32::from_le_bytes([data[14], data[15], data[16], data[17]]);
        if bi_size < 40 {
            return Err("Nieobslugiwany lub nieprawidlowy rozmiar naglowka informacyjnego");
        }

        let bi_width = i32::from_le_bytes([data[18], data[19], data[20], data[21]]);
        let bi_height = i32::from_le_bytes([data[22], data[23], data[24], data[25]]);
        let bi_bit_count = u16::from_le_bytes([data[28], data[29]]);
        let bi_compression = u32::from_le_bytes([data[30], data[31], data[32], data[33]]);

        if bi_compression != 0 {
            return Err("Obslugiwana jest tylko nieskompresowana kompresja BI_RGB (0)");
        }

        if bi_bit_count != 24 && bi_bit_count != 32 {
            return Err("Obslugiwane sa tylko formaty 24-bit BGR i 32-bit BGRA");
        }

        let width = bi_width.unsigned_abs();
        let height = bi_height.unsigned_abs();

        let row_stride = ((((width as usize) * (bi_bit_count as usize) + 31) / 32) * 4) as usize;

        let total_pixels = (width as usize).checked_mul(height as usize).ok_or("Przekroczono maksymalny rozmiar obrazu")?;

        if data.len() < bf_off_bits + (height as usize) * row_stride {
            return Err("Dane pikseli sa uciete");
        }

        let mut pixels = Vec::with_capacity(total_pixels);
        pixels.resize(total_pixels, 0u32);

        let pixel_data = &data[bf_off_bits..];
        let bytes_per_pixel = (bi_bit_count / 8) as usize;
        let bottom_up = bi_height > 0;

        for y in 0..height {
            let row_idx = if bottom_up { height - 1 - y } else { y } as usize;
            let row_start = y as usize * row_stride;

            for x in 0..width as usize {
                let pixel_start = row_start + x * bytes_per_pixel;
                
                let b = pixel_data[pixel_start];
                let g = pixel_data[pixel_start + 1];
                let r = pixel_data[pixel_start + 2];
                let a = if bytes_per_pixel == 4 {
                    pixel_data[pixel_start + 3]
                } else {
                    255
                };

                // Store as RGBA: R, G, B, A in bytes or 0xAABBGGRR in little endian u32.
                // Let's store as native u32 representing RGBA for easy use.
                // Actually, standard memory layout for RGBA is R, G, B, A sequentially.
                // So little endian u32 is (A << 24) | (B << 16) | (G << 8) | R
                let color: u32 = ((a as u32) << 24) | ((b as u32) << 16) | ((g as u32) << 8) | (r as u32);
                
                pixels[row_idx * (width as usize) + x] = color;
            }
        }

        Ok(BmpImage {
            width,
            height,
            data: pixels,
        })
    }
}
