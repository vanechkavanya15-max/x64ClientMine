use m2_client::recv_movement::{CharacterMoveData, SyncPositionData, MovementManager};

#[test]
fn test_character_move_updates_target() {
    let mut manager = MovementManager::new();
    let vid = 1001;
    manager.add_actor(vid, 1000, 2000);

    let move_data = CharacterMoveData {
        vid,
        func: 1,
        arg: 0,
        rot: 10,
        x: 1500,
        y: 2500,
        time: 12345,
        duration: 500,
    };

    let result = manager.handle_move_packet(&move_data);
    assert!(result.is_ok(), "Should successfully handle move packet for existing actor");

    let actor_state = manager.actors.get(&vid).expect("Actor should exist");
    
    // Check calculations and assignments
    assert_eq!(actor_state.dst_x, 1500, "Destination X should be updated");
    assert_eq!(actor_state.dst_y, 2500, "Destination Y should be updated");
    // bRot * 5.0f mapping from C++
    assert_eq!(actor_state.rot, 50.0, "Rotation should be rot * 5.0");
    assert_eq!(actor_state.func, 1, "Function type should be updated");
    assert_eq!(actor_state.start_time, 12345, "Start time should be updated");
    assert_eq!(actor_state.duration, 500, "Duration should be updated");
    
    // Ensure current position wasn't instantly snapped by move packet
    assert_eq!(actor_state.current_x, 1000, "Current X should remain unchanged until interpolated");
    assert_eq!(actor_state.current_y, 2000, "Current Y should remain unchanged until interpolated");
}

#[test]
fn test_sync_position_snaps_instantly() {
    let mut manager = MovementManager::new();
    let vid = 1002;
    manager.add_actor(vid, 5000, 5000);

    // najpierw nadajmy jakis cel ruchu
    manager.handle_move_packet(&CharacterMoveData {
        vid,
        func: 2,
        arg: 1,
        rot: 0,
        x: 6000,
        y: 6000,
        time: 100,
        duration: 2000,
    }).unwrap();

    // nastepnie wymuszamy twarda synchronizacje
    let sync_data = SyncPositionData {
        vid,
        x: 5500,
        y: 5200,
    };

    let result = manager.handle_sync_position(&sync_data);
    assert!(result.is_ok(), "Should successfully handle sync position for existing actor");

    let actor_state = manager.actors.get(&vid).expect("Actor should exist");
    assert_eq!(actor_state.current_x, 5500, "Current X should be instantly snapped");
    assert_eq!(actor_state.current_y, 5200, "Current Y should be instantly snapped");
    assert_eq!(actor_state.dst_x, 5500, "Destination X should also be reset to prevent rubberbanding");
    assert_eq!(actor_state.dst_y, 5200, "Destination Y should also be reset to prevent rubberbanding");
}

#[test]
fn test_handle_packets_for_missing_vid() {
    let mut manager = MovementManager::new();
    
    let move_result = manager.handle_move_packet(&CharacterMoveData {
        vid: 9999, // nieistniejacy
        func: 0, arg: 0, rot: 0, x: 0, y: 0, time: 0, duration: 0,
    });
    
    assert!(move_result.is_err(), "Move packet should fail for non-existent VID");
    assert_eq!(move_result.unwrap_err(), "Actor not found for move packet");

    let sync_result = manager.handle_sync_position(&SyncPositionData {
        vid: 9999, // nieistniejacy
        x: 10, y: 10,
    });
    
    assert!(sync_result.is_err(), "Sync packet should fail for non-existent VID");
    assert_eq!(sync_result.unwrap_err(), "Actor not found for sync position");
}

#[test]
fn test_multiple_actors() {
    let mut manager = MovementManager::new();
    manager.set_main_actor_vid(1);
    manager.add_actor(1, 0, 0);   // main player
    manager.add_actor(2, 50, 50); // other player
    
    // Move other player
    manager.handle_move_packet(&CharacterMoveData {
        vid: 2, func: 1, arg: 0, rot: 18, x: 100, y: 100, time: 1000, duration: 500
    }).unwrap();

    let main_state = manager.actors.get(&1).unwrap();
    assert_eq!(main_state.dst_x, 0, "Main player should not be affected");
    
    let other_state = manager.actors.get(&2).unwrap();
    assert_eq!(other_state.dst_x, 100, "Other player should move");
    assert_eq!(other_state.rot, 90.0, "Other player rotation 18 * 5 = 90.0");
}
