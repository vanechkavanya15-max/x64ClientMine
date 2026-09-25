//! Modul attr_map dla skrzynki m2-client-terrain
//! Parser mapy atrybutow terenu .atr (kolizje, woda, bezpieczne strefy)
//! Weryfikacja flag BLOCK, WATER, WARP, SAFE_ZONE dla kazdego pola siatki

pub const ATTR_BLOCK: u8 = 1 << 0;
pub const ATTR_WATER: u8 = 1 << 1;
pub const ATTR_WARP: u8 = 1 << 2;
pub const ATTR_SAFE_ZONE: u8 = 1 << 3;

/// Glowna struktura implementacyjna mapy atrybutow
#[derive(Debug, Clone, Default)]
pub struct AttrMap {
    pub width: u16,
    pub height: u16,
    pub data: Vec<u8>,
}

impl AttrMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_ready(&self) -> bool {
        !self.data.is_empty()
    }

    /// Bezpieczne parsowanie danych bez panic
    pub fn parse(buffer: &[u8]) -> Result<Self, &'static str> {
        if buffer.len() < 4 {
            return Err("Bufor jest zbyt krotki by zawierac naglowek mapy atrybutow");
        }

        let width = u16::from_le_bytes([buffer[0], buffer[1]]);
        let height = u16::from_le_bytes([buffer[2], buffer[3]]);

        let expected_size = 4 + (width as usize * height as usize);
        if buffer.len() < expected_size {
            return Err("Bufor jest mniejszy niz zadeklarowany rozmiar mapy");
        }

        let data = buffer[4..expected_size].to_vec();

        Ok(Self {
            width,
            height,
            data,
        })
    }

    /// Pobieranie atrybutu bez panic
    fn get_attr(&self, x: u16, y: u16) -> Option<u8> {
        if x < self.width && y < self.height {
            let index = (y as usize * self.width as usize) + x as usize;
            Some(self.data[index])
        } else {
            None
        }
    }

    pub fn is_block(&self, x: u16, y: u16) -> bool {
        self.get_attr(x, y).map_or(false, |attr| attr & ATTR_BLOCK != 0)
    }

    pub fn is_water(&self, x: u16, y: u16) -> bool {
        self.get_attr(x, y).map_or(false, |attr| attr & ATTR_WATER != 0)
    }

    pub fn is_warp(&self, x: u16, y: u16) -> bool {
        self.get_attr(x, y).map_or(false, |attr| attr & ATTR_WARP != 0)
    }

    pub fn is_safe_zone(&self, x: u16, y: u16) -> bool {
        self.get_attr(x, y).map_or(false, |attr| attr & ATTR_SAFE_ZONE != 0)
    }
}
