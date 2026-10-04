use std::collections::HashMap;
use std::sync::{Mutex, LazyLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GuildBuildingType {
    Castle,
    Altar,
    BlacksmithWeapon,
    BlacksmithArmor,
    BlacksmithAccessory,
    Furnace(u32), // Furnace with specific ore vnum
}

#[derive(Debug, Clone)]
pub struct GuildBuilding {
    pub building_type: GuildBuildingType,
    pub x: u32,
    pub y: u32,
    pub guild_id: u32,
}

pub struct GuildLandManager {
    buildings: HashMap<u32, Vec<GuildBuilding>>, // guild_id -> buildings
}

impl GuildLandManager {
    pub fn new() -> Self {
        Self {
            buildings: HashMap::new(),
        }
    }

    pub fn register_building(&mut self, guild_id: u32, building: GuildBuilding) {
        let list = self.buildings.entry(guild_id).or_insert_with(Vec::new);
        list.push(building);
    }

    pub fn get_buildings(&self, guild_id: u32) -> Option<&Vec<GuildBuilding>> {
        self.buildings.get(&guild_id)
    }
    
    pub fn clear_buildings(&mut self, guild_id: u32) {
        self.buildings.remove(&guild_id);
    }

    pub fn get_blacksmith_refine_bonus(&self, guild_id: u32, building_type: GuildBuildingType) -> u8 {
        if let Some(buildings) = self.buildings.get(&guild_id) {
            for b in buildings {
                if b.building_type == building_type {
                    return 10;
                }
            }
        }
        0
    }
}

pub static GUILD_LAND_MANAGER: LazyLock<Mutex<GuildLandManager>> = LazyLock::new(|| {
    Mutex::new(GuildLandManager::new())
});

#[no_mangle]
pub extern "C" fn m2_guild_land_register_building(
    guild_id: u32,
    building_type: u32,
    param: u32,
    x: u32,
    y: u32,
) -> bool {
    let b_type = match building_type {
        1 => GuildBuildingType::Castle,
        2 => GuildBuildingType::Altar,
        3 => GuildBuildingType::BlacksmithWeapon,
        4 => GuildBuildingType::BlacksmithArmor,
        5 => GuildBuildingType::BlacksmithAccessory,
        6 => GuildBuildingType::Furnace(param),
        _ => return false,
    };

    let mut mgr = GUILD_LAND_MANAGER.lock().unwrap();
    mgr.register_building(guild_id, GuildBuilding {
        building_type: b_type,
        x,
        y,
        guild_id,
    });
    true
}

#[no_mangle]
pub extern "C" fn m2_guild_land_get_refine_bonus(
    guild_id: u32,
    building_type: u32,
) -> u8 {
    let b_type = match building_type {
        3 => GuildBuildingType::BlacksmithWeapon,
        4 => GuildBuildingType::BlacksmithArmor,
        5 => GuildBuildingType::BlacksmithAccessory,
        _ => return 0,
    };
    
    let mgr = GUILD_LAND_MANAGER.lock().unwrap();
    mgr.get_blacksmith_refine_bonus(guild_id, b_type)
}
