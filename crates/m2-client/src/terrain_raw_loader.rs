pub const HEIGHTMAP_RAW_XSIZE: usize = 131;
pub const HEIGHTMAP_RAW_YSIZE: usize = 131;
pub const HEIGHTMAP_RAW_SIZE: usize = HEIGHTMAP_RAW_XSIZE * HEIGHTMAP_RAW_YSIZE;
pub const HEIGHTMAP_RAW_BYTES: usize = HEIGHTMAP_RAW_SIZE * 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawHeightMap {
    heights: [u16; HEIGHTMAP_RAW_SIZE],
}

impl RawHeightMap {
    /// Parses a raw heightmap from a byte slice.
    /// The byte slice must be exactly 34322 bytes long (131x131 little-endian WORDs).
    pub fn from_bytes(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() != HEIGHTMAP_RAW_BYTES {
            return Err("Invalid raw heightmap data size");
        }

        let mut heights = [0u16; HEIGHTMAP_RAW_SIZE];

        for (i, chunk) in data.chunks_exact(2).enumerate() {
            heights[i] = u16::from_le_bytes([chunk[0], chunk[1]]);
        }

        Ok(Self { heights })
    }

    /// Gets the raw height sample at the given (x, y) coordinates.
    /// Returns None if out of bounds.
    pub fn get_height_raw(&self, x: usize, y: usize) -> Option<u16> {
        if x < HEIGHTMAP_RAW_XSIZE && y < HEIGHTMAP_RAW_YSIZE {
            Some(self.heights[y * HEIGHTMAP_RAW_XSIZE + x])
        } else {
            None
        }
    }

    /// Finds and returns the minimum and maximum height values in the map.
    pub fn min_max_height(&self) -> (u16, u16) {
        let mut min = u16::MAX;
        let mut max = u16::MIN;

        for &height in &self.heights {
            if height < min {
                min = height;
            }
            if height > max {
                max = height;
            }
        }

        (min, max)
    }
}
