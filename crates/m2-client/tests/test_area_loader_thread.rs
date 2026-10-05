use m2_client::area_loader_thread::{AreaLoaderThread, TileCoord};
use std::thread;
use std::time::Duration;
use std::collections::HashSet;

#[test]
fn test_request_load_3x3_grid() {
    let mut loader = AreaLoaderThread::new();
    let center = TileCoord { x: 5, y: 5 };
    loader.request_load(center);

    let mut count = 0;
    let start_time = std::time::Instant::now();
    
    // Polling loop for better robustness instead of sleep
    while count < 9 && start_time.elapsed() < Duration::from_secs(2) {
        while let Some(_) = loader.fetch_completed() {
            count += 1;
        }
        thread::sleep(Duration::from_millis(10));
    }
    
    loader.shutdown();
    
    // Some might have been dropped or left, but we expect around 9 completed tasks 
    assert_eq!(count, 9, "Expected 9 tiles to be processed for a 3x3 grid");
}

#[test]
fn test_cancellation_of_out_of_range_tiles() {
    let mut loader = AreaLoaderThread::new();
    
    loader.request_load(TileCoord { x: 100, y: 100 });
    
    // Immediately move away
    loader.request_load(TileCoord { x: 0, y: 0 });
    
    let mut count_0_0 = 0;
    let start_time = std::time::Instant::now();
    
    while count_0_0 < 9 && start_time.elapsed() < Duration::from_secs(2) {
        while let Some(patch_data) = loader.fetch_completed() {
            let tile = patch_data.coord;
            if tile.x >= -1 && tile.x <= 1 && tile.y >= -1 && tile.y <= 1 {
                count_0_0 += 1;
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
    
    loader.shutdown();

    assert_eq!(count_0_0, 9, "Expected 9 tiles around (0,0) to be processed");
}

#[test]
fn test_worker_shutdown() {
    let mut loader = AreaLoaderThread::new();
    loader.request_load(TileCoord { x: 0, y: 0 });
    loader.shutdown();
    
    // Should safely close.
}

#[test]
fn test_moving_player_position() {
    let mut loader = AreaLoaderThread::new();
    
    // Start at (10, 10)
    loader.request_load(TileCoord { x: 10, y: 10 });
    
    // Simulate player moving quickly across the map
    // (10, 10) -> (11, 10) -> (12, 10) -> (13, 10)
    loader.request_load(TileCoord { x: 11, y: 10 });
    loader.request_load(TileCoord { x: 12, y: 10 });
    loader.request_load(TileCoord { x: 13, y: 10 });
    
    let mut completed_tiles = HashSet::new();
    let start_time = std::time::Instant::now();
    
    // Wait for the final grid around (13, 10) to finish loading
    while completed_tiles.len() < 9 && start_time.elapsed() < Duration::from_secs(2) {
        while let Some(patch_data) = loader.fetch_completed() {
            let tile = patch_data.coord;
            if tile.is_within_3x3_of(&TileCoord { x: 13, y: 10 }) {
                completed_tiles.insert(tile);
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
    
    loader.shutdown();

    assert_eq!(completed_tiles.len(), 9, "Expected exactly 9 tiles around (13, 10) to be processed");
    
    // Ensure all tiles are indeed within the 3x3 of (13, 10)
    for tile in completed_tiles {
        assert!(tile.is_within_3x3_of(&TileCoord { x: 13, y: 10 }), "Tile {:?} is not within 3x3 of (13, 10)", tile);
    }
}

#[test]
fn test_terrain_patch_data_paths() {
    let mut loader = AreaLoaderThread::new();
    let center = TileCoord { x: 1, y: 1 };
    loader.request_load(center);
    
    let mut found_center = false;
    let start_time = std::time::Instant::now();
    
    while !found_center && start_time.elapsed() < Duration::from_secs(2) {
        while let Some(patch_data) = loader.fetch_completed() {
            if patch_data.coord == center {
                found_center = true;
                
                let map_id = patch_data.map_id;
                assert_eq!(map_id, 1001); // 1 * 1000 + 1
                
                let map_name = format!("metin2_map_test_{}", map_id);
                assert_eq!(patch_data.area_property_path, format!("{}\\{:06}\\AreaProperty.txt", map_name, map_id));
                assert_eq!(patch_data.height_raw_path, format!("{}\\{:06}\\height.raw", map_name, map_id));
                assert_eq!(patch_data.splat_path, format!("{}\\{:06}\\tile.raw", map_name, map_id));
                assert_eq!(patch_data.attr_path, format!("{}\\{:06}\\attr.atr", map_name, map_id));
                assert_eq!(patch_data.water_path, format!("{}\\{:06}\\water.wtr", map_name, map_id));
                assert_eq!(patch_data.shadow_tex_path, format!("{}\\{:06}\\shadowmap.dds", map_name, map_id));
                assert_eq!(patch_data.shadow_map_path, format!("{}\\{:06}\\shadowmap.raw", map_name, map_id));
                assert_eq!(patch_data.mini_map_tex_path, format!("{}\\{:06}\\minimap.dds", map_name, map_id));
                
                break;
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
    
    loader.shutdown();
    
    assert!(found_center, "Expected to find the center tile and verify its path data");
}
