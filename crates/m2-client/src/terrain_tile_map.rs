pub const TILEMAP_RAW_XSIZE: usize = 258;
pub const TILEMAP_RAW_YSIZE: usize = 258;
pub const TILEMAP_RAW_SIZE: usize = TILEMAP_RAW_XSIZE * TILEMAP_RAW_YSIZE;

pub const PATCH_XSIZE: usize = 32;
pub const PATCH_YSIZE: usize = 32;

#[derive(Debug)]
pub struct TileMap {
    data: Box<[u8; TILEMAP_RAW_SIZE]>,
}

impl TileMap {
    pub fn from_bytes(data: &[u8]) -> Result<TileMap, &'static str> {
        if data.len() != TILEMAP_RAW_SIZE {
            return Err("Invalid tile map data size");
        }

        let mut map_data = Box::new([0u8; TILEMAP_RAW_SIZE]);
        map_data.copy_from_slice(data);

        Ok(TileMap { data: map_data })
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<u8> {
        if x >= TILEMAP_RAW_XSIZE || y >= TILEMAP_RAW_YSIZE {
            return None;
        }
        let index = y * TILEMAP_RAW_XSIZE + x;
        Some(self.data[index])
    }

    pub fn count_unique_textures(&self) -> usize {
        let mut used = [false; 256];
        let mut count = 0;
        
        for &tile in self.data.iter() {
            if !used[tile as usize] {
                used[tile as usize] = true;
                count += 1;
            }
        }
        
        count
    }

    pub fn get_patch_texture_counts(&self, patch_x: usize, patch_y: usize) -> [u32; 256] {
        let mut counts = [0u32; 256];
        
        // C++ uses patches of 32x32 tiles. 
        // 1 tile boundary margin, so patch grid starts at (1, 1).
        let start_x = 1 + patch_x * PATCH_XSIZE;
        let start_y = 1 + patch_y * PATCH_YSIZE;
        
        for y in 0..PATCH_YSIZE {
            for x in 0..PATCH_XSIZE {
                if let Some(tile) = self.get_tile(start_x + x, start_y + y) {
                    counts[tile as usize] += 1;
                }
            }
        }
        
        counts
    }
}
