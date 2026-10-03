#[path = "../src/wgpu_ground_items_renderer.rs"]
mod wgpu_ground_items_renderer;

use wgpu_ground_items_renderer::{WgpuGroundItemsRenderer, VNUM_MONEY};

#[test]
fn test_create_and_delete_item() {
    let mut renderer = WgpuGroundItemsRenderer::new();
    
    renderer.create_item(100, 5050, 10.0, 20.0, 30.0, false, 0);
    
    assert_eq!(renderer.ground_items.len(), 1);
    let item = renderer.ground_items.get(&100).unwrap();
    assert_eq!(item.virtual_number, 5050);
    assert_eq!(item.end_position.x, 10.0);
    assert_eq!(item.end_position.y, -20.0);
    assert_eq!(item.end_position.z, 30.0);
    assert!(item.anim_ended);
    
    renderer.delete_item(100);
    assert_eq!(renderer.ground_items.len(), 0);
}

#[test]
fn test_item_ownership() {
    let mut renderer = WgpuGroundItemsRenderer::new();
    
    renderer.create_item(200, 5050, 0.0, 0.0, 0.0, false, 0);
    
    assert_eq!(renderer.get_ownership(200), None);
    
    renderer.set_ownership(200, "Player1");
    assert_eq!(renderer.get_ownership(200).unwrap(), "Player1");
}

#[test]
fn test_drop_animation_math() {
    let mut renderer = WgpuGroundItemsRenderer::new();
    let current_time = 1000;
    
    renderer.create_item(300, 5050, 0.0, 0.0, 0.0, true, current_time);
    
    let item = renderer.ground_items.get(&300).unwrap();
    assert_eq!(item.start_time, 1000);
    assert_eq!(item.end_time, 1300);
    assert_eq!(item.anim_ended, false);
}

#[test]
fn test_get_close_item() {
    let mut renderer = WgpuGroundItemsRenderer::new();
    
    renderer.create_item(1, 5050, 10.0, -10.0, 0.0, false, 0);
    renderer.create_item(2, 5050, 20.0, -20.0, 0.0, false, 0); 
    
    // In create_item, end_position.y = -y. So if y = -10, end_position.y = 10.
    // In get_close_item, dy = pixel_y - (-end_position.y) = pixel_y - (-10) = pixel_y + 10.
    // Let's pass pixel_x = 10.0, pixel_y = -10.0.
    // dy = -10 - (-10) = 0. dx = 10 - 10 = 0. Distance = 0.
    let close = renderer.get_close_item(11.0, -11.0, 1000.0);
    assert_eq!(close, Some(1));
    
    // Point closer to 2
    let close2 = renderer.get_close_item(19.0, -19.0, 1000.0);
    assert_eq!(close2, Some(2));
    
    let close_none = renderer.get_close_item(100.0, 100.0, 10.0);
    assert_eq!(close_none, None);
}

#[test]
fn test_get_close_money() {
    let mut renderer = WgpuGroundItemsRenderer::new();
    
    renderer.create_item(1, 5050, 10.0, -10.0, 0.0, false, 0); // Not money
    renderer.create_item(2, VNUM_MONEY, 20.0, -20.0, 0.0, false, 0); // Money
    
    let close = renderer.get_close_money(10.0, -10.0, 1000.0);
    assert_eq!(close, Some(2)); // Should pick 2, even though 1 is closer, because 1 is not money
}

#[test]
fn test_update_animation_end() {
    let mut renderer = WgpuGroundItemsRenderer::new();
    let current_time = 1000;
    
    renderer.create_item(1, 5050, 0.0, 0.0, 0.0, true, current_time);
    
    // Animation shouldn't be ended at 1200
    renderer.update(1200);
    assert_eq!(renderer.ground_items.get(&1).unwrap().anim_ended, false);
    
    // Animation should be ended at 1300
    renderer.update(1300);
    assert_eq!(renderer.ground_items.get(&1).unwrap().anim_ended, true);
}

#[test]
fn test_clear_all() {
    let mut renderer = WgpuGroundItemsRenderer::new();
    
    renderer.create_item(1, 5050, 0.0, 0.0, 0.0, false, 0);
    renderer.create_item(2, 5050, 0.0, 0.0, 0.0, false, 0);
    
    assert_eq!(renderer.ground_items.len(), 2);
    
    renderer.delete_all_items();
    assert_eq!(renderer.ground_items.len(), 0);
}
