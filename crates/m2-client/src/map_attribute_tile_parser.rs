#[derive(Debug, PartialEq)]
pub enum AttrMapError {
    InvalidFileSize,
    InvalidMagicNumber(u16),
    InvalidWidth(u16),
    InvalidHeight(u16),
    DataSizeMismatch { expected: usize, actual: usize },
}

pub const TERRAIN_SIZE: usize = 128;
pub const ATTRMAP_XSIZE: usize = TERRAIN_SIZE * 2;
pub const ATTRMAP_YSIZE: usize = TERRAIN_SIZE * 2;
pub const ATTRMAP_MAGIC: u16 = 2634;

pub const ATTRIBUTE_BLOCK: u8 = 1 << 0;
pub const ATTRIBUTE_WATER: u8 = 1 << 1;
pub const ATTRIBUTE_BANPK: u8 = 1 << 2;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct AttrMapHeader {
    pub magic: u16,
    pub width: u16,
    pub height: u16,
}

pub struct MapAttributeTile {
    pub attributes: [u8; ATTRMAP_XSIZE * ATTRMAP_YSIZE],
}

impl MapAttributeTile {
    pub fn new() -> Self {
        Self {
            attributes: [0; ATTRMAP_XSIZE * ATTRMAP_YSIZE],
        }
    }

    pub fn load_from_buffer(buffer: &[u8]) -> Result<Self, AttrMapError> {
        let header_size = std::mem::size_of::<AttrMapHeader>();

        if buffer.len() < header_size {
            return Err(AttrMapError::InvalidFileSize);
        }

        let mut header_bytes = [0u8; 6];
        header_bytes.copy_from_slice(&buffer[0..6]);
        
        let magic = u16::from_ne_bytes([header_bytes[0], header_bytes[1]]);
        let width = u16::from_ne_bytes([header_bytes[2], header_bytes[3]]);
        let height = u16::from_ne_bytes([header_bytes[4], header_bytes[5]]);

        if magic != ATTRMAP_MAGIC {
            return Err(AttrMapError::InvalidMagicNumber(magic));
        }

        if width as usize != ATTRMAP_XSIZE {
            return Err(AttrMapError::InvalidWidth(width));
        }

        if height as usize != ATTRMAP_YSIZE {
            return Err(AttrMapError::InvalidHeight(height));
        }

        let rest_size = buffer.len() - header_size;
        let expected_size = ATTRMAP_XSIZE * ATTRMAP_YSIZE;

        if rest_size != expected_size {
            return Err(AttrMapError::DataSizeMismatch {
                expected: expected_size,
                actual: rest_size,
            });
        }

        let mut map = Self::new();
        map.attributes.copy_from_slice(&buffer[header_size..]);

        Ok(map)
    }

    pub fn get_attribute(&self, x: usize, y: usize) -> u8 {
        if x >= ATTRMAP_XSIZE || y >= ATTRMAP_YSIZE {
            return 0;
        }
        self.attributes[y * ATTRMAP_XSIZE + x]
    }

    pub fn is_block(&self, x: usize, y: usize) -> bool {
        (self.get_attribute(x, y) & ATTRIBUTE_BLOCK) != 0
    }

    pub fn is_water(&self, x: usize, y: usize) -> bool {
        (self.get_attribute(x, y) & ATTRIBUTE_WATER) != 0
    }

    pub fn is_banpk(&self, x: usize, y: usize) -> bool {
        (self.get_attribute(x, y) & ATTRIBUTE_BANPK) != 0
    }
}
