use std::collections::HashMap;

pub const TILE_SIZE: usize = 256;
pub const TILE_MAGIC: u16 = 2634;

pub const FLAG_BLOCKED: u8 = 1 << 0;
pub const FLAG_WATER: u8 = 1 << 1;
pub const FLAG_SAFE_ZONE: u8 = 1 << 2;
#[allow(dead_code)]
pub const FLAG_CANT_BUILD: u8 = 1 << 3;

#[derive(Debug, Clone)]
pub struct TerrainAttrTile {
    #[allow(dead_code)]
    pub tx: i32,
    #[allow(dead_code)]
    pub ty: i32,
    pub grid: Vec<u8>,
}

pub struct TerrainAttrTileLoader {
    tiles: HashMap<(i32, i32), TerrainAttrTile>,
}

impl TerrainAttrTileLoader {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    pub fn load_tile(&mut self, tx: i32, ty: i32, data: &[u8]) -> Result<(), &'static str> {
        let expected_size = 6 + (TILE_SIZE * TILE_SIZE);
        if data.len() < 6 {
            return Err("Invalid .atr file size");
        }
        
        let magic = u16::from_le_bytes([data[0], data[1]]);
        let width = u16::from_le_bytes([data[2], data[3]]);
        let height = u16::from_le_bytes([data[4], data[5]]);

        if magic != TILE_MAGIC {
            return Err("Invalid magic number");
        }
        if width as usize != TILE_SIZE || height as usize != TILE_SIZE {
            return Err("Invalid dimensions");
        }
        if data.len() != expected_size {
            return Err("Invalid .atr file size");
        }

        let mut grid = vec![0u8; TILE_SIZE * TILE_SIZE];
        grid.copy_from_slice(&data[6..expected_size]);

        self.tiles.insert((tx, ty), TerrainAttrTile {
            tx,
            ty,
            grid,
        });

        Ok(())
    }

    pub fn remove_tile(&mut self, tx: i32, ty: i32) {
        self.tiles.remove(&(tx, ty));
    }

    pub fn clear(&mut self) {
        self.tiles.clear();
    }

    pub fn get_attribute(&self, global_x: usize, global_y: usize) -> u8 {
        let tx = (global_x / TILE_SIZE) as i32;
        let ty = (global_y / TILE_SIZE) as i32;

        let local_x = global_x % TILE_SIZE;
        let local_y = global_y % TILE_SIZE;

        if let Some(tile) = self.tiles.get(&(tx, ty)) {
            tile.grid[local_y * TILE_SIZE + local_x]
        } else {
            FLAG_BLOCKED
        }
    }

    pub fn is_walkable(&self, global_x: usize, global_y: usize) -> bool {
        let attr = self.get_attribute(global_x, global_y);
        (attr & (FLAG_BLOCKED | FLAG_WATER)) == 0
    }
}

impl Default for TerrainAttrTileLoader {
    fn default() -> Self {
        Self::new()
    }
}
