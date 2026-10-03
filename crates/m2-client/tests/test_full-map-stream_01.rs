#[path = "../src/map_outdoor_area_streaming.rs"]
mod map_outdoor_area_streaming;

use map_outdoor_area_streaming::{AreaStreamingManager, SectorCoord};
use std::collections::HashSet;

#[test]
fn test_initial_update() {
    let mut manager = AreaStreamingManager::new();
    let (to_load, to_unload) = manager.update_center(0, 0);

    assert_eq!(to_unload.len(), 0);
    assert_eq!(to_load.len(), 9);

    let active_sectors = manager.get_active_sectors();
    assert_eq!(active_sectors.len(), 9);

    for dx in -1..=1 {
        for dy in -1..=1 {
            assert!(active_sectors.contains(&SectorCoord::new(dx, dy)));
        }
    }
}

#[test]
fn test_no_movement() {
    let mut manager = AreaStreamingManager::new();
    manager.update_center(1, 1);

    let (to_load, to_unload) = manager.update_center(1, 1);
    assert_eq!(to_load.len(), 0);
    assert_eq!(to_unload.len(), 0);

    assert_eq!(manager.get_active_sectors().len(), 9);
}

#[test]
fn test_small_movement() {
    let mut manager = AreaStreamingManager::new();
    manager.update_center(0, 0);

    // Move 1 step right
    let (to_load, to_unload) = manager.update_center(1, 0);

    // Leftmost column (-1, y) should be unloaded
    assert_eq!(to_unload.len(), 3);
    for dy in -1..=1 {
        assert!(to_unload.contains(&SectorCoord::new(-1, dy)));
    }

    // Rightmost column (2, y) should be loaded
    assert_eq!(to_load.len(), 3);
    for dy in -1..=1 {
        assert!(to_load.contains(&SectorCoord::new(2, dy)));
    }

    let active_sectors = manager.get_active_sectors();
    assert_eq!(active_sectors.len(), 9);
    for dx in 0..=2 {
        for dy in -1..=1 {
            assert!(active_sectors.contains(&SectorCoord::new(dx, dy)));
        }
    }
}

#[test]
fn test_large_movement() {
    let mut manager = AreaStreamingManager::new();
    manager.update_center(0, 0);

    // Move 10 steps up (teleport completely out of previous area)
    let (to_load, to_unload) = manager.update_center(0, 10);

    assert_eq!(to_unload.len(), 9);
    for dx in -1..=1 {
        for dy in -1..=1 {
            assert!(to_unload.contains(&SectorCoord::new(dx, dy)));
        }
    }

    assert_eq!(to_load.len(), 9);
    for dx in -1..=1 {
        for dy in 9..=11 {
            assert!(to_load.contains(&SectorCoord::new(dx, dy)));
        }
    }

    let active_sectors = manager.get_active_sectors();
    assert_eq!(active_sectors.len(), 9);
}

#[test]
fn test_clear() {
    let mut manager = AreaStreamingManager::new();
    manager.update_center(0, 0);

    let unloaded = manager.clear();
    assert_eq!(unloaded.len(), 9);
    for dx in -1..=1 {
        for dy in -1..=1 {
            assert!(unloaded.contains(&SectorCoord::new(dx, dy)));
        }
    }

    assert_eq!(manager.get_active_sectors().len(), 0);
    assert_eq!(manager.get_center(), None);
}
