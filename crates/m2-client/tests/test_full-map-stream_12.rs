use m2_client::full_map_stream_12::*;
use std::collections::HashSet;

#[test]
fn test_default_wind_sound() {
    let mut trigger = MapAmbienceSoundTrigger::new();
    
    // Test base coordinates (0,0) in an empty environment
    trigger.update_player_position(0, 0, 0, 0);
    
    let active = trigger.get_active_sounds();
    assert_eq!(active.len(), 1);
    assert!(active.contains(&AmbienceSound::Wind));
}

#[test]
fn test_river_sound() {
    let mut trigger = MapAmbienceSoundTrigger::new();
    
    // Register sector (0,0) with water
    trigger.add_sector_environment(0, 0, SectorEnvironment {
        has_water: true,
        has_trees: false,
    });
    
    // Move player to sector (0,0)
    trigger.update_player_position(100, 100, 0, 0);
    
    let active = trigger.get_active_sounds();
    assert_eq!(active.len(), 2);
    assert!(active.contains(&AmbienceSound::Wind));
    assert!(active.contains(&AmbienceSound::River));
}

#[test]
fn test_birds_sound() {
    let mut trigger = MapAmbienceSoundTrigger::new();
    
    // Register sector (1,0) with trees
    // sector size is 25600, so global_x > 25600 puts us in col 1
    trigger.add_sector_environment(1, 0, SectorEnvironment {
        has_water: false,
        has_trees: true,
    });
    
    // Move player to sector (1,0)
    trigger.update_player_position(26000, 100, 0, 0);
    
    let active = trigger.get_active_sounds();
    assert_eq!(active.len(), 2);
    assert!(active.contains(&AmbienceSound::Wind));
    assert!(active.contains(&AmbienceSound::Birds));
}

#[test]
fn test_all_sounds() {
    let mut trigger = MapAmbienceSoundTrigger::new();
    
    // Register sector (0,1) with water and trees
    // sector size is 25600, so global_y > 25600 puts us in row 1
    trigger.add_sector_environment(0, 1, SectorEnvironment {
        has_water: true,
        has_trees: true,
    });
    
    // Move player to sector (0,1)
    trigger.update_player_position(100, 26000, 0, 0);
    
    let active = trigger.get_active_sounds();
    assert_eq!(active.len(), 3);
    assert!(active.contains(&AmbienceSound::Wind));
    assert!(active.contains(&AmbienceSound::River));
    assert!(active.contains(&AmbienceSound::Birds));
}

#[test]
fn test_out_of_bounds_sector() {
    let mut trigger = MapAmbienceSoundTrigger::new();
    
    // Register sector (0,0) with water and trees
    trigger.add_sector_environment(0, 0, SectorEnvironment {
        has_water: true,
        has_trees: true,
    });
    
    // Move player to sector (1,1) which is not registered
    trigger.update_player_position(26000, 26000, 0, 0);
    
    let active = trigger.get_active_sounds();
    // Default should fallback to just wind if the sector has no explicitly registered environment
    assert_eq!(active.len(), 1);
    assert!(active.contains(&AmbienceSound::Wind));
}
