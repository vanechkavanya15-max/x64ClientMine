use m2_client::ui_minimap::{MarkType, UiMinimap};

#[test]
fn test_integration_ui_minimap() {
    let instance = UiMinimap::new();
    assert!(instance.is_ready());
}

#[test]
fn test_minimap_scale_clamp() {
    let mut minimap = UiMinimap::new();
    minimap.set_scale(1.0);
    minimap.scale_up();
    assert_eq!(minimap.scale, 2.0);
    minimap.scale_up();
    assert_eq!(minimap.scale, 4.0);
    minimap.scale_up(); // should clamp at 4.0
    assert_eq!(minimap.scale, 4.0);

    minimap.scale_down();
    assert_eq!(minimap.scale, 2.0);
    minimap.scale_down();
    assert_eq!(minimap.scale, 1.0);
    minimap.scale_down();
    assert_eq!(minimap.scale, 0.5);
    minimap.scale_down(); // should clamp at 0.5
    assert_eq!(minimap.scale, 0.5);

    minimap.set_scale(5.0);
    assert_eq!(minimap.scale, 4.0);

    minimap.set_scale(0.1);
    assert_eq!(minimap.scale, 0.5);
}

#[test]
fn test_minimap_observers() {
    let mut minimap = UiMinimap::new();
    minimap.add_observer(1, 10.0, 20.0);
    
    let obs = minimap.map_vid_observer.get(&1).unwrap();
    assert_eq!(obs.cur_x, 10.0);
    assert_eq!(obs.cur_y, 20.0);

    minimap.move_observer(1, 30.0, 40.0);
    let obs_moved = minimap.map_vid_observer.get(&1).unwrap();
    assert_eq!(obs_moved.dst_x, 30.0);
    assert_eq!(obs_moved.dst_y, 40.0);
    assert_eq!(obs_moved.src_x, 10.0); // original cur_x
    
    minimap.remove_observer(1);
    assert!(minimap.map_vid_observer.get(&1).is_none());
}

#[test]
fn test_waypoints_and_signal_points() {
    let mut minimap = UiMinimap::new();
    minimap.add_signal_point(100.0, 200.0);
    
    assert_eq!(minimap.signal_point_vector.len(), 1);
    assert_eq!(minimap.signal_point_vector[0].pos, [100.0, 200.0]);
    assert_eq!(minimap.atlas_waypoint_info_vector.len(), 1);

    minimap.clear_all_signal_point();
    assert_eq!(minimap.signal_point_vector.len(), 0);
    assert_eq!(minimap.atlas_waypoint_info_vector.len(), 0);
}

#[test]
fn test_guild_area() {
    let mut minimap = UiMinimap::new();
    minimap.register_guild_area(1, 100, 50, 50, 20, 20);
    
    assert_eq!(minimap.get_guild_area_id(60, 60), 100);
    assert_eq!(minimap.get_guild_area_id(40, 40), 0);
    
    minimap.clear_guild_area();
    assert_eq!(minimap.get_guild_area_id(60, 60), 0);
}

#[test]
fn test_targets() {
    let mut minimap = UiMinimap::new();
    minimap.create_target(1, "Target 1".to_string());
    assert_eq!(minimap.atlas_waypoint_info_vector.len(), 1);
    assert_eq!(minimap.atlas_waypoint_info_vector[0].mark_type, MarkType::Target);

    minimap.update_target(1, 150, 250);
    assert_eq!(minimap.atlas_waypoint_info_vector[0].x, 150.0);
    assert_eq!(minimap.atlas_waypoint_info_vector[0].y, 250.0);

    minimap.delete_target(1);
    assert_eq!(minimap.atlas_waypoint_info_vector.len(), 0);
}

#[test]
fn test_minimap_size_and_center() {
    let mut minimap = UiMinimap::new();
    minimap.set_minimap_size(128.0, 128.0);
    assert_eq!(minimap.width, 128.0);
    assert_eq!(minimap.height, 128.0);
    
    minimap.set_center_position(64.0, 64.0);
    assert_eq!(minimap.center_x, 64.0);
    assert_eq!(minimap.center_y, 64.0);
}

#[test]
fn test_add_entity_mark_logic() {
    let mut minimap = UiMinimap::new();
    minimap.set_minimap_size(128.0, 128.0);
    minimap.set_scale(1.0);
    minimap.set_center_position(0.0, 0.0);

    // Assert radius updated by set_scale
    assert_eq!(minimap.minimap_radius, 64.0);

    // Entity within radius
    minimap.add_entity_mark(100.0, 100.0, MarkType::Monster, 1);
    assert_eq!(minimap.monster_position_vector.len(), 1);
    
    // Entity exactly at center
    minimap.add_entity_mark(0.0, 0.0, MarkType::Npc, 2);
    assert_eq!(minimap.npc_position_vector.len(), 1);

    // Entity way out of bounds
    minimap.add_entity_mark(10000.0, 10000.0, MarkType::Party, 3);
    assert_eq!(minimap.party_pc_position_vector.len(), 0);
}

#[test]
fn test_update_observer_interpolation() {
    let mut minimap = UiMinimap::new();
    minimap.width = 128.0;
    minimap.height = 128.0;
    minimap.scale = 1.0;
    minimap.set_center_position(0.0, 0.0);
    minimap.minimap_radius = 64.0;
    
    minimap.add_observer(1, 0.0, 0.0);
    minimap.move_observer(1, 100.0, 100.0); // 1000ms duration
    
    // Halfway through
    minimap.update(0.0, 0.0, 500); 
    
    let obs = minimap.map_vid_observer.get(&1).unwrap();
    assert_eq!(obs.cur_x, 50.0);
    assert_eq!(obs.cur_y, 50.0);
    
    // Check that it clamped to 1.0
    minimap.update(0.0, 0.0, 1500);
    let obs_end = minimap.map_vid_observer.get(&1).unwrap();
    assert_eq!(obs_end.cur_x, 100.0);
    assert_eq!(obs_end.cur_y, 100.0);
}

#[test]
fn test_update_atlas_guild_area() {
    let mut minimap = UiMinimap::new();
    minimap.atlas_max_x = 1000.0;
    minimap.atlas_max_y = 1000.0;
    minimap.atlas_image_size_x = 100.0;
    minimap.atlas_image_size_y = 100.0;
    
    minimap.register_guild_area(1, 1, 200, 200, 100, 100);
    minimap.update_atlas(0.0, 0.0, 0.0);
    
    let info = &minimap.guild_area_info_vector[0];
    assert!((info.sx_render - 20.0).abs() < f32::EPSILON);
    assert!((info.sy_render - 20.0).abs() < f32::EPSILON);
    
    // Allow small float precision differences
    assert!((info.ex_render - 30.0).abs() < 1e-5);
    assert!((info.ey_render - 30.0).abs() < 1e-5);
}
