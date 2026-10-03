use m2_client::full_map_stream_11::map_warp_phase_coordinator::{MapWarpPhaseCoordinator, TPacketGCWarp, TileBuffer, WarpPhaseError};

#[test]
fn test_handle_warp_packet_initializes_coordinator() {
    let mut coordinator = MapWarpPhaseCoordinator::new();
    
    // Add dummy data to ensure reset works
    coordinator.tile_buffers.push(TileBuffer { x_index: 0, y_index: 0, data: vec![1, 2, 3] });
    coordinator.is_ready = true;

    let packet = TPacketGCWarp {
        header: 0x0306,
        length: 18,
        lX: 1000,
        lY: 2000,
        lAddr: 127_0_0_1,
        wPort: 13000,
    };

    let result = coordinator.handle_warp_packet(&packet);
    assert!(result.is_ok());

    assert_eq!(coordinator.tile_buffers.len(), 0);
    assert_eq!(coordinator.is_ready, false);
    assert_eq!(coordinator.is_loading, true);
    assert_eq!(coordinator.map_base_x, 1000);
    assert_eq!(coordinator.map_base_y, 2000);
    assert_eq!(coordinator.server_addr, 127_0_0_1);
    assert_eq!(coordinator.server_port, 13000);
}

#[test]
fn test_add_tile_buffer_success() {
    let mut coordinator = MapWarpPhaseCoordinator::new();
    coordinator.is_loading = true; // Must be loading to add tiles

    let tile = TileBuffer { x_index: 1, y_index: 2, data: vec![10, 20, 30] };
    let result = coordinator.add_tile_buffer(tile);
    
    assert!(result.is_ok());
    assert_eq!(coordinator.tile_buffers.len(), 1);
    assert_eq!(coordinator.tile_buffers[0].x_index, 1);
    assert_eq!(coordinator.tile_buffers[0].y_index, 2);
}

#[test]
fn test_add_tile_buffer_fails_not_loading() {
    let mut coordinator = MapWarpPhaseCoordinator::new();
    coordinator.is_loading = false; 

    let tile = TileBuffer { x_index: 1, y_index: 2, data: vec![10, 20, 30] };
    let result = coordinator.add_tile_buffer(tile);
    
    assert_eq!(result, Err(WarpPhaseError::NotLoading));
}

#[test]
fn test_add_tile_buffer_fails_empty_data() {
    let mut coordinator = MapWarpPhaseCoordinator::new();
    coordinator.is_loading = true;

    let tile = TileBuffer { x_index: 1, y_index: 2, data: vec![] };
    let result = coordinator.add_tile_buffer(tile);
    
    assert_eq!(result, Err(WarpPhaseError::InvalidTileData));
}

#[test]
fn test_finalize_map_load_success() {
    let mut coordinator = MapWarpPhaseCoordinator::new();
    coordinator.is_loading = true;

    let result = coordinator.finalize_map_load();
    
    assert!(result.is_ok());
    assert_eq!(coordinator.is_loading, false);
    assert_eq!(coordinator.is_ready, true);
}

#[test]
fn test_finalize_map_load_fails_not_loading() {
    let mut coordinator = MapWarpPhaseCoordinator::new();
    coordinator.is_loading = false;

    let result = coordinator.finalize_map_load();
    
    assert_eq!(result, Err(WarpPhaseError::NotLoading));
}

#[test]
fn test_finalize_map_load_fails_already_ready() {
    let mut coordinator = MapWarpPhaseCoordinator::new();
    coordinator.is_loading = true;
    coordinator.is_ready = true;

    let result = coordinator.finalize_map_load();
    
    assert_eq!(result, Err(WarpPhaseError::AlreadyReady));
}
