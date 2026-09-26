#![allow(non_snake_case)]

const ATTRMAP_XSIZE: u16 = 256;
const ATTRMAP_YSIZE: u16 = 256;
const ATTRMAP_MAGIC: u16 = 2634;

const ATTRIBUTE_BLOCK: u8 = 1 << 0;
const ATTRIBUTE_WATER: u8 = 1 << 1;
const ATTRIBUTE_BANPK: u8 = 1 << 2;

#[repr(C, packed)]
struct SAttrMapHeader {
    m_wMagic: u16,
    m_wWidth: u16,
    m_wHeight: u16,
}

pub struct AttrMap {
    attributes: Vec<u8>,
}

impl AttrMap {
    pub fn from_bytes(data: &[u8]) -> Result<AttrMap, &'static str> {
        let header_size = std::mem::size_of::<SAttrMapHeader>();
        
        if data.len() < header_size {
            return Err("FILE SIZE ERROR");
        }
        
        let mut magic_bytes = [0u8; 2];
        magic_bytes.copy_from_slice(&data[0..2]);
        let m_wMagic = u16::from_le_bytes(magic_bytes);
        
        let mut width_bytes = [0u8; 2];
        width_bytes.copy_from_slice(&data[2..4]);
        let m_wWidth = u16::from_le_bytes(width_bytes);
        
        let mut height_bytes = [0u8; 2];
        height_bytes.copy_from_slice(&data[4..6]);
        let m_wHeight = u16::from_le_bytes(height_bytes);
        
        if m_wMagic != ATTRMAP_MAGIC {
            return Err("MAGIC NUMBER ERROR");
        }
        
        if m_wWidth != ATTRMAP_XSIZE {
            return Err("WIDTH ERROR");
        }
        
        if m_wHeight != ATTRMAP_YSIZE {
            return Err("HEIGHT ERROR");
        }
        
        let expected_size = header_size + (ATTRMAP_XSIZE as usize * ATTRMAP_YSIZE as usize);
        
        if data.len() != expected_size {
            return Err("FILE DATA SIZE ERROR");
        }
        
        let mut attributes = vec![0u8; expected_size - header_size];
        attributes.copy_from_slice(&data[header_size..expected_size]);
        
        Ok(AttrMap {
            attributes,
        })
    }

    pub fn get_attr(&self, x: u16, y: u16) -> u8 {
        if x >= ATTRMAP_XSIZE || y >= ATTRMAP_YSIZE {
            return 0;
        }
        self.attributes[y as usize * ATTRMAP_XSIZE as usize + x as usize]
    }

    pub fn is_blocked(&self, x: u16, y: u16) -> bool {
        (self.get_attr(x, y) & ATTRIBUTE_BLOCK) != 0
    }

    pub fn is_water(&self, x: u16, y: u16) -> bool {
        (self.get_attr(x, y) & ATTRIBUTE_WATER) != 0
    }

    pub fn is_safe_zone(&self, x: u16, y: u16) -> bool {
        (self.get_attr(x, y) & ATTRIBUTE_BANPK) != 0
    }
}
