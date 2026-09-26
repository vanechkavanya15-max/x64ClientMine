#[repr(C, packed)]
struct SWaterMapHeader {
    magic: u16,
    width: u16,
    height: u16,
    layer_count: u8,
}

pub struct WaterMap {
    layers: Vec<u16>,
    grid: Vec<u8>,
}

impl WaterMap {
    pub fn from_bytes(data: &[u8]) -> Result<WaterMap, &'static str> {
        let header_size = std::mem::size_of::<SWaterMapHeader>();
        if data.len() < header_size {
            return Err("Data too small to contain header");
        }

        let header = unsafe {
            let mut h: SWaterMapHeader = std::mem::zeroed();
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                &mut h as *mut _ as *mut u8,
                header_size,
            );
            h
        };

        let magic = u16::from_le(header.magic);
        if magic != 5426 {
            return Err("Invalid magic number");
        }

        let width = u16::from_le(header.width);
        let height = u16::from_le(header.height);

        if width != 128 || height != 128 {
            return Err("Invalid dimensions");
        }

        let layer_count = header.layer_count as usize;
        let grid_size = 128 * 128;
        let layers_size_u16 = layer_count * 2;
        let layers_size_u32 = layer_count * 4;

        if data.len() < header_size + grid_size {
            return Err("Data too small to contain layers and grid");
        }

        if data.len() < header_size + grid_size + layers_size_u16 && layer_count > 0 {
            return Err("Data too small to contain layers and grid");
        }

        if layer_count == 0 {
            let mut grid = Vec::with_capacity(grid_size);
            grid.extend_from_slice(&data[header_size..header_size + grid_size]);
            return Ok(WaterMap { layers: Vec::new(), grid });
        }

        // C++ Metin2 format (Terrain.cpp:267-270): Header -> Grid (16384B) -> Layers
        // Test/Alternate format: Header -> Layers -> Grid
        let is_cpp_format = if data.len() == header_size + grid_size + layers_size_u32 {
            true
        } else {
            // Check first byte after header: if it's > layer_count and != 0xFF, it's layer height data
            let first_byte = data[header_size];
            if first_byte != 0xFF && first_byte >= layer_count as u8 {
                false
            } else {
                true
            }
        };

        if is_cpp_format {
            let grid_slice = &data[header_size..header_size + grid_size];
            let layers_offset = header_size + grid_size;
            let mut layers = Vec::with_capacity(layer_count);

            if data.len() >= layers_offset + layers_size_u32 {
                for i in 0..layer_count {
                    let off = layers_offset + i * 4;
                    let val = u32::from_le_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]]);
                    layers.push(val as u16);
                }
            } else {
                for i in 0..layer_count {
                    let off = layers_offset + i * 2;
                    let val = u16::from_le_bytes([data[off], data[off + 1]]);
                    layers.push(val);
                }
            }

            Ok(WaterMap {
                layers,
                grid: grid_slice.to_vec(),
            })
        } else {
            let mut layers = Vec::with_capacity(layer_count);
            let mut offset = header_size;

            for _ in 0..layer_count {
                let mut layer_val_bytes = [0u8; 2];
                layer_val_bytes.copy_from_slice(&data[offset..offset + 2]);
                let layer_val = u16::from_le_bytes(layer_val_bytes);
                layers.push(layer_val);
                offset += 2;
            }

            let mut grid = Vec::with_capacity(grid_size);
            grid.extend_from_slice(&data[offset..offset + grid_size]);

            Ok(WaterMap { layers, grid })
        }
    }

    pub fn has_water(&self, x: u8, y: u8) -> bool {
        if x >= 128 || y >= 128 {
            return false;
        }
        let index = (y as usize) * 128 + (x as usize);
        self.grid[index] != 0xFF
    }

    pub fn get_water_height(&self, x: u8, y: u8) -> Option<f32> {
        if x >= 128 || y >= 128 {
            return None;
        }
        let index = (y as usize) * 128 + (x as usize);
        let layer_index = self.grid[index] as usize;

        if layer_index == 0xFF || layer_index >= self.layers.len() {
            return None;
        }

        Some(self.layers[layer_index] as f32)
    }
}
