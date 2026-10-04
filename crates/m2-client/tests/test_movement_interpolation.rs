use m2_client::recv_movement::{CharacterMoveState, CharacterMoveData, TPacketGCMove};

#[test]
fn test_packet_size() {
    assert_eq!(std::mem::size_of::<TPacketGCMove>(), 27);
}

#[test]
fn test_movement_interpolation_steps() {
    let mut state = CharacterMoveState::new(1, 100, 200);

    let move_data = CharacterMoveData {
        vid: 1,
        func: 0,
        arg: 0,
        rot: 18, // 18 * 5.0 = 90.0 degrees
        x: 500,
        y: 600,
        time: 1000,
        duration: 2000, // 2 seconds
    };

    let start_time = 5000;
    state.set_target(&move_data, start_time);

    assert_eq!(state.rot, 90.0);
    assert_eq!(state.src_x, 100);
    assert_eq!(state.src_y, 200);
    assert_eq!(state.dst_x, 500);
    assert_eq!(state.dst_y, 600);

    // 0% of time
    state.update_position(start_time);
    assert_eq!(state.current_x, 100);
    assert_eq!(state.current_y, 200);

    // 50% of time
    state.update_position(start_time + 1000);
    assert_eq!(state.current_x, 300); // 100 + (500 - 100) * 0.5
    assert_eq!(state.current_y, 400); // 200 + (600 - 200) * 0.5

    // 100% of time
    state.update_position(start_time + 2000);
    assert_eq!(state.current_x, 500);
    assert_eq!(state.current_y, 600);
    
    // 120% of time (should cap at destination)
    state.update_position(start_time + 2400);
    assert_eq!(state.current_x, 500);
    assert_eq!(state.current_y, 600);
}
