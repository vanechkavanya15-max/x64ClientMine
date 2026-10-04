use m2_client::game_guild_land_and_altar::*;

#[test]
fn test_register_building() {
    let mut mgr = GuildLandManager::new();
    mgr.register_building(1, GuildBuilding {
        building_type: GuildBuildingType::Castle,
        x: 100,
        y: 200,
        guild_id: 1,
    });
    
    let buildings = mgr.get_buildings(1).unwrap();
    assert_eq!(buildings.len(), 1);
    assert_eq!(buildings[0].building_type, GuildBuildingType::Castle);
}

#[test]
fn test_get_blacksmith_refine_bonus() {
    let mut mgr = GuildLandManager::new();
    mgr.register_building(2, GuildBuilding {
        building_type: GuildBuildingType::BlacksmithWeapon,
        x: 100,
        y: 200,
        guild_id: 2,
    });
    
    assert_eq!(mgr.get_blacksmith_refine_bonus(2, GuildBuildingType::BlacksmithWeapon), 10);
    assert_eq!(mgr.get_blacksmith_refine_bonus(2, GuildBuildingType::BlacksmithArmor), 0);
}

#[test]
fn test_furnace_registration() {
    let mut mgr = GuildLandManager::new();
    mgr.register_building(3, GuildBuilding {
        building_type: GuildBuildingType::Furnace(50601), // Diamond
        x: 100,
        y: 200,
        guild_id: 3,
    });
    
    let buildings = mgr.get_buildings(3).unwrap();
    assert_eq!(buildings.len(), 1);
    assert_eq!(buildings[0].building_type, GuildBuildingType::Furnace(50601));
}

#[test]
fn test_clear_buildings() {
    let mut mgr = GuildLandManager::new();
    mgr.register_building(4, GuildBuilding {
        building_type: GuildBuildingType::Altar,
        x: 100,
        y: 200,
        guild_id: 4,
    });
    
    mgr.clear_buildings(4);
    assert!(mgr.get_buildings(4).is_none());
}
