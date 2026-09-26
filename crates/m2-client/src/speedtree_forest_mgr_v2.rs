use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

pub const SECTOR_SIZE_CM: f32 = 25600.0;

static FOREST_MANAGER: OnceLock<RwLock<SpeedTreeForestManager>> = OnceLock::new();

pub fn get_forest_manager() -> &'static RwLock<SpeedTreeForestManager> {
    FOREST_MANAGER.get_or_init(|| RwLock::new(SpeedTreeForestManager::new()))
}

#[derive(Debug, Clone, PartialEq)]
pub struct SptDefinition {
    pub magic: String,
    pub version: u32,
    pub raw_data: Vec<u8>,
}

impl SptDefinition {
    /// Parsuje definicje pliku .spt.
    /// Zgodnie z formatem oczekiwana jest poprawna sygnatura pliku oraz wersja.
    pub fn parse(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < 8 {
            return Err("File too short for SPT definition");
        }
        
        let magic_bytes = &data[0..4];
        let magic = String::from_utf8_lossy(magic_bytes).into_owned();
        
        if magic != "SRT " && magic != "SPT " {
            return Err("Invalid magic signature in SPT definition");
        }
        
        let version = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        
        Ok(Self {
            magic,
            version,
            raw_data: data[8..].to_vec(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TreeInstance {
    pub tree_id: u32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub scale: f32,
    pub rotation: f32,
}

pub struct SpeedTreeForestManager {
    pub definitions: HashMap<u32, SptDefinition>,
    pub sectors: HashMap<(u16, u16), Vec<TreeInstance>>,
}

impl SpeedTreeForestManager {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            sectors: HashMap::new(),
        }
    }

    pub fn register_definition(&mut self, tree_id: u32, spt: SptDefinition) {
        self.definitions.insert(tree_id, spt);
    }

    pub fn add_instance(&mut self, instance: TreeInstance) {
        let sec_x = (instance.pos_x.max(0.0) / SECTOR_SIZE_CM).floor() as u16;
        let sec_y = (instance.pos_y.max(0.0) / SECTOR_SIZE_CM).floor() as u16;
        
        self.sectors
            .entry((sec_x, sec_y))
            .or_insert_with(Vec::new)
            .push(instance);
    }
    
    pub fn clear(&mut self) {
        self.definitions.clear();
        self.sectors.clear();
    }
}

impl Default for SpeedTreeForestManager {
    fn default() -> Self {
        Self::new()
    }
}

#[no_mangle]
pub extern "C" fn m2_forest_get_tree_count(sec_x: u16, sec_y: u16) -> u32 {
    if let Ok(manager) = get_forest_manager().read() {
        if let Some(instances) = manager.sectors.get(&(sec_x, sec_y)) {
            return instances.len() as u32;
        }
    }
    0
}
