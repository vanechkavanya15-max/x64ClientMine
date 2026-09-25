//! Modul tga_decoder dla skrzynki m2-client-image
//! Dekoder formatu Targa TGA (ikony, elementy interfejsu)
//! Parser TGA nieskompresowanego oraz z kompresja RLE, obsluga 24 i 32-bit

#[derive(Debug, PartialEq, Clone)]
pub enum TgaError {
    InvalidSize,
    UnsupportedFormat,
    InvalidData,
}

#[derive(Debug, Clone, Default)]
pub struct TgaDecoder {
    pub width: u32,
    pub height: u32,
    pub rgba_data: Vec<u8>, // Zdekodowane dane pikseli w formacie RGBA (32 bity na piksel)
}

impl TgaDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Dekoduje dane TGA z bufora do struktury TgaDecoder
    pub fn decode(data: &[u8]) -> Result<Self, TgaError> {
        if data.len() < 18 {
            return Err(TgaError::InvalidSize);
        }

        let id_length = data[0] as usize;
        let color_map_type = data[1];
        let image_type = data[2];

        // Sprawdzamy czy format jest obslugiwany (brak palety kolorow)
        if color_map_type != 0 {
            return Err(TgaError::UnsupportedFormat);
        }

        // Obslugujemy tylko nieskompresowane (2) i RLE (10) true-color
        if image_type != 2 && image_type != 10 {
            return Err(TgaError::UnsupportedFormat);
        }

        let width = u16::from_le_bytes([data[12], data[13]]) as u32;
        let height = u16::from_le_bytes([data[14], data[15]]) as u32;
        let pixel_depth = data[16];
        let image_descriptor = data[17];

        if width == 0 || height == 0 {
            return Err(TgaError::InvalidSize);
        }

        if pixel_depth != 24 && pixel_depth != 32 {
            return Err(TgaError::UnsupportedFormat);
        }

        let mut offset = 18 + id_length;
        let bytes_per_pixel = (pixel_depth / 8) as usize;
        let total_pixels = (width * height) as usize;
        
        let mut pixels = vec![0u8; total_pixels * 4];
        let mut pixel_idx = 0;

        if image_type == 2 {
            // Nieskompresowane dane
            let required_data = offset + total_pixels * bytes_per_pixel;
            if data.len() < required_data {
                return Err(TgaError::InvalidData);
            }

            for _ in 0..total_pixels {
                let chunk = &data[offset..offset + bytes_per_pixel];
                let (b, g, r, a) = if pixel_depth == 24 {
                    (chunk[0], chunk[1], chunk[2], 255)
                } else {
                    (chunk[0], chunk[1], chunk[2], chunk[3])
                };

                let p = pixel_idx * 4;
                pixels[p] = r;
                pixels[p + 1] = g;
                pixels[p + 2] = b;
                pixels[p + 3] = a;

                pixel_idx += 1;
                offset += bytes_per_pixel;
            }
        } else if image_type == 10 {
            // RLE (Run-Length Encoding)
            while pixel_idx < total_pixels {
                if offset >= data.len() {
                    return Err(TgaError::InvalidData);
                }
                
                let packet_header = data[offset];
                offset += 1;
                
                let run_length = (packet_header & 0x7F) as usize + 1;
                if pixel_idx + run_length > total_pixels {
                    return Err(TgaError::InvalidData);
                }

                if packet_header & 0x80 != 0 {
                    // Pakiet RLE: jeden piksel powtorzony run_length razy
                    if offset + bytes_per_pixel > data.len() {
                        return Err(TgaError::InvalidData);
                    }
                    
                    let chunk = &data[offset..offset + bytes_per_pixel];
                    offset += bytes_per_pixel;
                    
                    let (b, g, r, a) = if pixel_depth == 24 {
                        (chunk[0], chunk[1], chunk[2], 255)
                    } else {
                        (chunk[0], chunk[1], chunk[2], chunk[3])
                    };

                    for _ in 0..run_length {
                        let p = pixel_idx * 4;
                        pixels[p] = r;
                        pixels[p + 1] = g;
                        pixels[p + 2] = b;
                        pixels[p + 3] = a;
                        pixel_idx += 1;
                    }
                } else {
                    // Pakiet Raw: run_length unikalnych pikseli
                    if offset + run_length * bytes_per_pixel > data.len() {
                        return Err(TgaError::InvalidData);
                    }

                    for _ in 0..run_length {
                        let chunk = &data[offset..offset + bytes_per_pixel];
                        offset += bytes_per_pixel;
                        
                        let (b, g, r, a) = if pixel_depth == 24 {
                            (chunk[0], chunk[1], chunk[2], 255)
                        } else {
                            (chunk[0], chunk[1], chunk[2], chunk[3])
                        };

                        let p = pixel_idx * 4;
                        pixels[p] = r;
                        pixels[p + 1] = g;
                        pixels[p + 2] = b;
                        pixels[p + 3] = a;
                        pixel_idx += 1;
                    }
                }
            }
        }

        // TGA domyslnie przechowuje obraz od dolu do gory.
        // Bit 5 w image_descriptor oznacza orientacje (0 - od dolu, 1 - od gory).
        // Odwracamy w pionie jesli nie jest to orientacja "od gory".
        let top_down = (image_descriptor & (1 << 5)) != 0;
        if !top_down {
            Self::flip_vertically(&mut pixels, width as usize, height as usize);
        }

        // Bit 4 oznacza odwracanie w poziomie.
        let right_to_left = (image_descriptor & (1 << 4)) != 0;
        if right_to_left {
            Self::flip_horizontally(&mut pixels, width as usize, height as usize);
        }

        Ok(Self {
            width,
            height,
            rgba_data: pixels,
        })
    }

    fn flip_vertically(pixels: &mut [u8], width: usize, height: usize) {
        let row_bytes = width * 4;
        let mut temp_row = vec![0u8; row_bytes];
        for y in 0..(height / 2) {
            let top_offset = y * row_bytes;
            let bottom_offset = (height - 1 - y) * row_bytes;
            
            temp_row.copy_from_slice(&pixels[top_offset..top_offset + row_bytes]);
            let bottom_slice = pixels[bottom_offset..bottom_offset + row_bytes].to_vec();
            pixels[top_offset..top_offset + row_bytes].copy_from_slice(&bottom_slice);
            pixels[bottom_offset..bottom_offset + row_bytes].copy_from_slice(&temp_row);
        }
    }

    fn flip_horizontally(pixels: &mut [u8], width: usize, height: usize) {
        for y in 0..height {
            let row_offset = y * width * 4;
            for x in 0..(width / 2) {
                let left_idx = row_offset + x * 4;
                let right_idx = row_offset + (width - 1 - x) * 4;
                
                for i in 0..4 {
                    pixels.swap(left_idx + i, right_idx + i);
                }
            }
        }
    }
}
