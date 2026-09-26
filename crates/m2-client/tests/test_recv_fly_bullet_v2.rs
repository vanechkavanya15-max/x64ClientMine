use m2_client::recv_fly_bullet_v2::*;
use m2_client::character_manager_core::get_character_manager;
use std::mem::size_of;

#[test]
fn test_parse_recv_fly_valid() {
    let packet = TPacketGCFly {
        header: 0,
        length: size_of::<TPacketGCFly>() as u16,
        bType: FLY_TYPE_ARROW,
        dwStartVID: 100,
        dwEndVID: 200,
    };
    
    let mut data = Vec::new();
    data.extend_from_slice(&packet.header.to_le_bytes());
    data.extend_from_slice(&packet.length.to_le_bytes());
    data.push(packet.bType);
    data.extend_from_slice(&packet.dwStartVID.to_le_bytes());
    data.extend_from_slice(&packet.dwEndVID.to_le_bytes());
    
    let result = parse_recv_fly(&data);
    assert!(result.is_ok());
    let (b_type, start_vid, end_vid, trajectory) = result.unwrap();
    assert_eq!(b_type, FLY_TYPE_ARROW);
    assert_eq!(start_vid, 100);
    assert_eq!(end_vid, 200);
}

#[test]
fn test_parse_recv_fly_short_buffer() {
    let data = vec![0u8; size_of::<TPacketGCFly>() - 1];
    let result = parse_recv_fly(&data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Buffer too short");
}

#[test]
fn test_ballistic_calculate() {
    let trajectory = BallisticTrajectory::calculate(
        0.0, 0.0, 0.0,
        1000.0, 0.0, 0.0,
        1000.0,
        980.0
    );
    
    assert_eq!(trajectory.start_x, 0.0);
    assert_eq!(trajectory.end_x, 1000.0);
    assert_eq!(trajectory.distance, 1000.0);
    assert_eq!(trajectory.travel_time, 1.0);
    assert_eq!(trajectory.max_height, (980.0 * 1.0 * 1.0) / 8.0);
}

#[test]
fn test_ballistic_position_at_start() {
    let trajectory = BallisticTrajectory::calculate(
        0.0, 0.0, 0.0,
        1000.0, 0.0, 0.0,
        1000.0,
        980.0
    );
    
    let pos = trajectory.get_position_at(0.0, 980.0);
    assert_eq!(pos, (0.0, 0.0, 0.0));
}

#[test]
fn test_ballistic_position_at_end() {
    let trajectory = BallisticTrajectory::calculate(
        0.0, 0.0, 0.0,
        1000.0, 0.0, 0.0,
        1000.0,
        980.0
    );
    
    let pos = trajectory.get_position_at(1.0, 980.0);
    assert_eq!(pos, (1000.0, 0.0, 0.0));
}

#[test]
fn test_ballistic_position_at_middle() {
    let trajectory = BallisticTrajectory::calculate(
        0.0, 0.0, 0.0,
        1000.0, 0.0, 0.0,
        1000.0,
        980.0
    );
    
    let pos = trajectory.get_position_at(0.5, 980.0);
    assert_eq!(pos.0, 500.0);
    assert_eq!(pos.1, 0.0);
    
    // Z grawitacja: z = v0_z * t - 0.5 * g * t^2
    // v0_z = g * travel_time / 2 = 980 * 1 / 2 = 490
    // z at 0.5: 490 * 0.5 - 0.5 * 980 * 0.25 = 245 - 122.5 = 122.5
    assert_eq!(pos.2, 122.5);
    assert_eq!(pos.2, trajectory.max_height);
}

#[test]
fn test_ballistic_with_existing_actors() {
    // Setup character manager with two actors
    {
        let mut mgr = get_character_manager().write().unwrap();
        mgr.clear();
        mgr.create_instance(500, 0, 100.0, 200.0, 0.0);
        mgr.create_instance(600, 0, 500.0, 500.0, 0.0);
    }
    
    let packet = TPacketGCFly {
        header: 0,
        length: size_of::<TPacketGCFly>() as u16,
        bType: FLY_TYPE_MAGIC,
        dwStartVID: 500,
        dwEndVID: 600,
    };
    
    let mut data = Vec::new();
    data.extend_from_slice(&packet.header.to_le_bytes());
    data.extend_from_slice(&packet.length.to_le_bytes());
    data.push(packet.bType);
    data.extend_from_slice(&packet.dwStartVID.to_le_bytes());
    data.extend_from_slice(&packet.dwEndVID.to_le_bytes());
    
    let result = parse_recv_fly(&data);
    assert!(result.is_ok());
    let (_, _, _, trajectory_opt) = result.unwrap();
    assert!(trajectory_opt.is_some());
    
    let trajectory = trajectory_opt.unwrap();
    assert_eq!(trajectory.start_x, 100.0);
    assert_eq!(trajectory.start_y, 200.0);
    assert_eq!(trajectory.end_x, 500.0);
    assert_eq!(trajectory.end_y, 500.0);
    
    let distance = ((500.0 - 100.0_f32).powi(2) + (500.0 - 200.0_f32).powi(2)).sqrt();
    assert_eq!(trajectory.distance, distance);
}

#[test]
fn test_ffi_m2_net_recv_fly() {
    let packet = TPacketGCFly {
        header: 0,
        length: size_of::<TPacketGCFly>() as u16,
        bType: FLY_TYPE_ARROW,
        dwStartVID: 100,
        dwEndVID: 200,
    };
    
    let mut data = Vec::new();
    data.extend_from_slice(&packet.header.to_le_bytes());
    data.extend_from_slice(&packet.length.to_le_bytes());
    data.push(packet.bType);
    data.extend_from_slice(&packet.dwStartVID.to_le_bytes());
    data.extend_from_slice(&packet.dwEndVID.to_le_bytes());
    
    let result = m2_net_recv_fly(data.as_ptr(), data.len());
    assert!(result);
    
    // Test null ptr
    let result_null = m2_net_recv_fly(std::ptr::null(), data.len());
    assert!(!result_null);
    
    // Test zero len
    let result_zero = m2_net_recv_fly(data.as_ptr(), 0);
    assert!(!result_zero);
}
