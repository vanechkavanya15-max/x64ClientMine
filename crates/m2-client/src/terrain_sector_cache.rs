use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

#[derive(Clone, Debug, PartialEq)]
pub struct RawHeightMap {
    pub data: Vec<u16>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AttrMap {
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WaterMap {
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SectorData {
    pub raw_height_map: RawHeightMap,
    pub attr_map: AttrMap,
    pub water_map: WaterMap,
}

pub struct TerrainSectorCache {
    capacity: usize,
    map: HashMap<(u32, u32), (SectorData, u32)>,
    pq: BinaryHeap<Reverse<(u32, (u32, u32))>>,
}

impl TerrainSectorCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: if capacity == 0 { 16 } else { capacity },
            map: HashMap::new(),
            pq: BinaryHeap::new(),
        }
    }

    pub fn get_sector(&mut self, col: u32, row: u32, cur_time_ms: u32) -> Option<&SectorData> {
        if let Some((_, last_time)) = self.map.get_mut(&(col, row)) {
            *last_time = cur_time_ms;
            self.pq.push(Reverse((cur_time_ms, (col, row))));
        }
        self.map.get(&(col, row)).map(|(sector, _)| sector)
    }

    pub fn insert_sector(&mut self, col: u32, row: u32, sector: SectorData, cur_time_ms: u32) {
        if !self.map.contains_key(&(col, row)) {
            while self.map.len() >= self.capacity {
                self.evict_lru();
            }
        }
        self.map.insert((col, row), (sector, cur_time_ms));
        self.pq.push(Reverse((cur_time_ms, (col, row))));
    }

    fn evict_lru(&mut self) {
        while let Some(Reverse((time, (c, r)))) = self.pq.pop() {
            if let Some((_, actual_time)) = self.map.get(&(c, r)) {
                if *actual_time == time {
                    self.map.remove(&(c, r));
                    break;
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.pq.clear();
    }
}

impl Default for TerrainSectorCache {
    fn default() -> Self {
        Self::new(16)
    }
}
