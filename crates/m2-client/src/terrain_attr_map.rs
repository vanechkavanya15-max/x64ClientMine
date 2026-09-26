const ATTR_BLOCK: u8 = 1 << 0;
const ATTR_WATER: u8 = 1 << 1;
#[allow(dead_code)]
const ATTR_WARP: u8 = 1 << 2;
const ATTR_SAFE_ZONE: u8 = 1 << 3;

pub struct TerrainAttrMap {
    width: usize,
    height: usize,
    attr_grid: Vec<u8>,
}

impl TerrainAttrMap {
    pub fn new(width: usize, height: usize, attr_grid: Vec<u8>) -> Result<Self, &'static str> {
        if attr_grid.len() != width * height {
            return Err("Invalid grid size");
        }
        Ok(Self {
            width,
            height,
            attr_grid,
        })
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        let attr = self.attr_grid[y * self.width + x];
        (attr & (ATTR_BLOCK | ATTR_WATER)) == 0
    }

    pub fn is_safe_zone(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        let attr = self.attr_grid[y * self.width + x];
        (attr & ATTR_SAFE_ZONE) != 0
    }
}
