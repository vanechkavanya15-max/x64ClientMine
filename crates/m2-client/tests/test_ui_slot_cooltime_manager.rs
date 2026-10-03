#[path = "../src/ui_slot_cooltime_manager.rs"]
mod ui_slot_cooltime_manager;

use ui_slot_cooltime_manager::*;

#[test]
fn test_set_slot_cool_time() {
    let mut manager = SlotCooltimeManager::new();
    let current_time = 100.0;
    manager.set_slot_cool_time(1, 10.0, 2.0, current_time);

    let slot = manager.get_slot(1).unwrap();
    assert_eq!(slot.cool_time, 10.0);
    assert_eq!(slot.start_cool_time, 98.0); // current_time - elapsed_time
}

#[test]
fn test_store_and_restore_slot_cool_time() {
    let mut manager = SlotCooltimeManager::new();
    let key = 1;
    let slot_index = 5;
    let current_time = 200.0;

    manager.store_slot_cool_time(key, slot_index, 15.0, 5.0, current_time);
    
    // Validate store
    let store = manager.cooldown_store.get(&key).unwrap();
    let stored = store.get(&slot_index).unwrap();
    assert_eq!(stored.cool_time, 15.0);
    assert_eq!(stored.elapsed_time, 195.0);
    assert_eq!(stored.active, false);

    // Now restore
    manager.restore_slot_cool_time(key);
    let slot = manager.get_slot(slot_index).unwrap();
    assert_eq!(slot.cool_time, 15.0);
    assert_eq!(slot.start_cool_time, 195.0);
}

#[test]
fn test_transfer_slot_cool_time() {
    let mut manager = SlotCooltimeManager::new();
    let current_time = 300.0;
    let index1 = 10;
    let index2 = 20;

    // First store to active skill (key = SKILL_TYPE_ACTIVE)
    manager.store_slot_cool_time(SKILL_TYPE_ACTIVE, index1, 20.0, 10.0, current_time);
    manager.activate_slot(index1); // make it active for index1

    // Transfer from index1 to index2
    manager.transfer_slot_cool_time(index1, index2, 1);

    // index1 should be removed from store
    let active_store = manager.cooldown_store.get(&SKILL_TYPE_ACTIVE).unwrap();
    assert!(active_store.get(&index1).is_none());
    assert!(active_store.get(&index2).is_some());

    // index1 slot should be reset
    let slot1 = manager.get_slot(index1).unwrap();
    assert_eq!(slot1.cool_time, 0.0);
    assert_eq!(slot1.start_cool_time, 0.0);

    // index2 slot should be updated
    let slot2 = manager.get_slot(index2).unwrap();
    assert_eq!(slot2.cool_time, 20.0);
    assert_eq!(slot2.start_cool_time, 290.0);
    assert_eq!(slot2.active, true);
}

#[test]
fn test_render_cool_time_box_math_empty() {
    // fTime >= 1.0f returns empty
    let vertices = render_cool_time_box_math(100.0, 100.0, 16.0, 1.0);
    assert!(vertices.is_empty());
    
    let vertices = render_cool_time_box_math(100.0, 100.0, 16.0, 1.5);
    assert!(vertices.is_empty());
}

#[test]
fn test_render_cool_time_box_math_full() {
    // fTime = 0.0 -> full circle (square box fan)
    let vertices = render_cool_time_box_math(50.0, 50.0, 10.0, 0.0);
    
    // Center, Top Center, and 8 surrounding points = 10 vertices
    assert_eq!(vertices.len(), 10);
    assert_eq!(vertices[0].x, 50.0);
    assert_eq!(vertices[0].y, 50.0);
    
    // Top center
    assert_eq!(vertices[1].x, 50.0);
    assert_eq!(vertices[1].y, 40.0);

    // Box position 0: -1, -1 -> x = 40, y = 40
    assert_eq!(vertices[2].x, 40.0);
    assert_eq!(vertices[2].y, 40.0);
}

#[test]
fn test_render_cool_time_box_math_partial() {
    // 0.5 time -> 4.0 tri_count (half of the 8 triangles)
    let vertices = render_cool_time_box_math(0.0, 0.0, 10.0, 0.5);
    
    // Center, top center + 4 full edges = 6 vertices
    assert_eq!(vertices.len(), 6);
    
    // Let's test a very specific partial percentage 
    // fTime = 0.875 -> 8 - 7 = 1 triangle
    let vertices2 = render_cool_time_box_math(0.0, 0.0, 10.0, 0.875);
    // Center, top center, and 1 surrounding point
    assert_eq!(vertices2.len(), 3);
}

#[test]
fn test_render_cool_time_box_math_fractional() {
    // fTime = 0.9 -> 8 - 7.2 = 0.8 -> tri_count = 0, last_percentage = 0.8
    let vertices = render_cool_time_box_math(0.0, 0.0, 10.0, 0.9);
    // Center, top center + 1 interpolated point = 3 vertices
    assert_eq!(vertices.len(), 3);
    
    // The interpolated point should be 80% between the 7th and 0th position
    // last pos = box_pos[7] = (0.0, -1.0)
    // pos = box_pos[0] = (-1.0, -1.0)
    // inter_x = ( -1.0 - 0.0 ) * 0.8 + 0.0 = -0.8
    // inter_y = ( -1.0 - -1.0 ) * 0.8 + -1.0 = -1.0
    
    let last_v = vertices.last().unwrap();
    // Approximately matching floats
    assert!((last_v.x - (-8.0)).abs() < 0.001);
    assert!((last_v.y - (-10.0)).abs() < 0.001);
}
