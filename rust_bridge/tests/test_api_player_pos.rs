use m2_client_bridge::api::player_pos::m2_player_get_position;
use std::ptr;

#[test]
fn test_m2_player_get_position_valid_pointers() {
    let mut x: f32 = 99.0;
    let mut y: f32 = 99.0;
    let mut rot: f32 = 99.0;

    let result = m2_player_get_position(&mut x, &mut y, &mut rot);
    
    assert!(result);
    assert_eq!(x, 0.0);
    assert_eq!(y, 0.0);
    assert_eq!(rot, 0.0);
}

#[test]
fn test_m2_player_get_position_null_pointers() {
    let mut x: f32 = 99.0;
    let mut y: f32 = 99.0;
    
    // out_rot jest null
    let result = m2_player_get_position(&mut x, &mut y, ptr::null_mut());
    assert!(!result);

    // out_y jest null
    let result2 = m2_player_get_position(&mut x, ptr::null_mut(), ptr::null_mut());
    assert!(!result2);
    
    // wszystkie sa null
    let result3 = m2_player_get_position(ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
    assert!(!result3);

    // Wartosci nie powinny byc zmienione
    assert_eq!(x, 99.0);
    assert_eq!(y, 99.0);
}
