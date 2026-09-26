use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

pub const HEIGHT_MAP_X: usize = 131;
pub const HEIGHT_MAP_Y: usize = 131;
pub const HEIGHT_MAP_SIZE: usize = HEIGHT_MAP_X * HEIGHT_MAP_Y;
pub const RAW_HEIGHT_BYTES: usize = HEIGHT_MAP_SIZE * 2;

#[derive(Debug, Clone, PartialEq)]
pub struct SectorChunk {
    pub heights: Vec<f32>,
    pub version: u64,
}

pub struct AreaChunkCache {
    capacity: usize,
    chunks: HashMap<(u16, u16), SectorChunk>,
    // Access order from oldest (index 0) to newest (index n-1)
    access_order: Vec<(u16, u16)>,
    global_version: u64,
}

impl AreaChunkCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: if capacity == 0 { 16 } else { capacity },
            chunks: HashMap::new(),
            access_order: Vec::new(),
            global_version: 0,
        }
    }

    pub fn load_sector(&mut self, sec_x: u16, sec_y: u16, raw_data: &[u8]) -> bool {
        if raw_data.len() < RAW_HEIGHT_BYTES {
            return false;
        }

        let mut heights = Vec::with_capacity(HEIGHT_MAP_SIZE);
        for chunk in raw_data[..RAW_HEIGHT_BYTES].chunks_exact(2) {
            let val = u16::from_le_bytes([chunk[0], chunk[1]]);
            heights.push(val as f32);
        }

        self.global_version = self.global_version.wrapping_add(1);

        let chunk = SectorChunk {
            heights,
            version: self.global_version,
        };

        // Update LRU tracking
        if let Some(pos) = self.access_order.iter().position(|&k| k == (sec_x, sec_y)) {
            self.access_order.remove(pos);
        } else if self.chunks.len() >= self.capacity {
            if let Some(oldest) = self.access_order.first().copied() {
                self.chunks.remove(&oldest);
                self.access_order.remove(0);
            }
        }

        self.chunks.insert((sec_x, sec_y), chunk);
        self.access_order.push((sec_x, sec_y));

        true
    }

    pub fn get_sector(&mut self, sec_x: u16, sec_y: u16) -> Option<SectorChunk> {
        if let Some(pos) = self.access_order.iter().position(|&k| k == (sec_x, sec_y)) {
            let key = self.access_order.remove(pos);
            self.access_order.push(key);
            self.chunks.get(&(sec_x, sec_y)).cloned()
        } else {
            None
        }
    }

    pub fn get_capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.chunks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chunks.is_empty()
    }

    pub fn clear(&mut self) {
        self.chunks.clear();
        self.access_order.clear();
        self.global_version = 0;
    }
}

pub static AREA_CACHE: LazyLock<RwLock<AreaChunkCache>> = LazyLock::new(|| {
    RwLock::new(AreaChunkCache::new(64))
});

#[no_mangle]
pub extern "C" fn m2_area_load_sector(sec_x: u16, sec_y: u16, raw_data: *const u8) -> bool {
    if raw_data.is_null() {
        return false;
    }

    let data_slice = unsafe { std::slice::from_raw_parts(raw_data, RAW_HEIGHT_BYTES) };
    if let Ok(mut cache) = AREA_CACHE.write() {
        cache.load_sector(sec_x, sec_y, data_slice)
    } else {
        false
    }
}
