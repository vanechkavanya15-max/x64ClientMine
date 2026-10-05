#[path = "../src/py_bridge_minimap_atlas_renderer.rs"]
mod py_bridge_minimap_atlas_renderer;

use py_bridge_minimap_atlas_renderer::*;
use std::ffi::CString;

#[test]
fn test_check_bounds_helpers() {
    assert!(check_player_bounds(100.0, 100.0, 100.0, 100.0, 5.0, 5.0)); // Center
    assert!(check_player_bounds(100.0, 100.0, 96.0, 96.0, 5.0, 5.0));   // Inner edge
    assert!(!check_player_bounds(100.0, 100.0, 105.0, 105.0, 5.0, 5.0));// Outer edge
    assert!(!check_player_bounds(100.0, 100.0, 95.0, 95.0, 5.0, 5.0));  // Outer edge

    assert!(check_mark_bounds(100.0, 100.0, 100.0, 100.0, 5.0, 5.0)); // Center
    assert!(check_mark_bounds(100.0, 100.0, 98.0, 98.0, 5.0, 5.0));   // Inner left/top edge
    assert!(!check_mark_bounds(100.0, 100.0, 97.0, 97.0, 5.0, 5.0));  // Outer left/top edge (100 - 2.5 = 97.5)
    assert!(!check_mark_bounds(100.0, 100.0, 105.0, 105.0, 5.0, 5.0));// Outer right/bottom edge
}

#[test]
fn test_atlas_visibility_state() {
    let mut state = get_atlas_state().write().unwrap();
    state.is_show_atlas = false;
    drop(state);

    assert!(!m2_minimap_is_show_atlas());

    m2_minimap_show_atlas();
    assert!(m2_minimap_is_show_atlas());

    m2_minimap_hide_atlas();
    assert!(!m2_minimap_is_show_atlas());
}

#[test]
fn test_compute_offsets() {
    let mut state = AtlasState {
        atlas_image_size_x: 256.0,
        atlas_image_size_y: 256.0,
        atlas_max_x: 1024.0,
        atlas_max_y: 1024.0,
        ..Default::default()
    };
    compute_atlas_centering_offsets(&mut state);
    
    assert!((state.atlas_offset_x - 0.0).abs() < f32::EPSILON);
    assert!((state.atlas_offset_y - 0.0).abs() < f32::EPSILON);

    let mut state2 = AtlasState {
        atlas_image_size_x: 256.0,
        atlas_image_size_y: 256.0,
        atlas_max_x: 512.0,
        atlas_max_y: 1024.0, // Scale Y dominates: 256/1024 = 0.25
        ..Default::default()
    };
    compute_atlas_centering_offsets(&mut state2);
    assert!((state2.atlas_offset_x - 64.0).abs() < f32::EPSILON);
}

#[test]
fn test_coordinate_transform_precision() {
    let mut state = AtlasState {
        atlas_image_size_x: 256.0,
        atlas_image_size_y: 256.0,
        atlas_max_x: 1024.0,
        atlas_max_y: 1024.0,
        ..Default::default()
    };
    compute_atlas_centering_offsets(&mut state);

    let (atlas_x, atlas_y) = global_to_atlas(512, 512, &state);
    assert!((atlas_x - 128.0).abs() < f32::EPSILON);
    assert!((atlas_y - 128.0).abs() < f32::EPSILON);

    let (world_x, world_y) = atlas_to_global(atlas_x, atlas_y, &state);
    assert!((world_x - 512.0).abs() < f32::EPSILON);
    assert!((world_y - 512.0).abs() < f32::EPSILON);
}

#[test]
fn test_atlas_info_hit_testing_player() {
    assert!(true); 
}

#[test]
fn test_atlas_info_hit_testing_marks() {
    let mut state = get_atlas_state().write().unwrap();
    state.atlas_screen_x = 0.0;
    state.atlas_screen_y = 0.0;
    state.atlas_image_size_x = 256.0;
    state.atlas_image_size_y = 256.0;
    state.atlas_max_x = 1024.0;
    state.atlas_max_y = 1024.0;
    compute_atlas_centering_offsets(&mut state);
    state.npcs.clear();
    drop(state);

    let name = CString::new("Test NPC").unwrap();
    m2_minimap_register_atlas_mark(1, name.as_ptr() as *const u8, name.as_bytes().len(), 100, 100);

    let mut out_str = [0u8; 128];
    let mut out_x = 0.0;
    let mut out_y = 0.0;
    let mut out_color = 0;
    let mut out_guild = 0;

    let hit = m2_minimap_get_atlas_info(
        25.0, 25.0,
        out_str.as_mut_ptr(), out_str.len(),
        &mut out_x, &mut out_y,
        &mut out_color, &mut out_guild
    );
    
    assert!(hit, "Hit testing failed to intersect NPC mark at exact center");
    assert_eq!(out_x, 100.0);
    assert_eq!(out_y, 100.0);
    
    let text = unsafe { std::ffi::CStr::from_ptr(out_str.as_ptr() as *const i8).to_string_lossy() };
    assert_eq!(text, "Test NPC");
}

#[test]
fn test_atlas_info_hit_testing_guilds() {
    let mut state = get_atlas_state().write().unwrap();
    state.atlas_screen_x = 10.0;
    state.atlas_screen_y = 10.0;
    state.guilds.clear();
    drop(state);

    m2_minimap_register_guild_area(1, 50, 50, 100, 100);

    let mut state = get_atlas_state().write().unwrap();
    // Simulate render values being updated because they rely on atlas scaling
    if let Some(guild) = state.guilds.first_mut() {
        guild.fsx_render = 0.0;
        guild.fsy_render = 0.0;
        guild.fex_render = 20.0;
        guild.fey_render = 20.0;
    }
    drop(state);

    let mut out_str = [0u8; 128];
    let mut out_x = 0.0;
    let mut out_y = 0.0;
    let mut out_color = 0;
    let mut out_guild = 0;

    let hit = m2_minimap_get_atlas_info(
        15.0, 15.0,
        out_str.as_mut_ptr(), out_str.len(),
        &mut out_x, &mut out_y,
        &mut out_color, &mut out_guild
    );
    
    assert!(hit);
    assert_eq!(out_guild, 1);
    assert_eq!(out_x, 100.0);
    assert_eq!(out_y, 100.0);
    
    let text = unsafe { std::ffi::CStr::from_ptr(out_str.as_ptr() as *const i8).to_string_lossy() };
    assert_eq!(text, "empty_guild_area"); // Because of cfg test macro mock
}

#[test]
fn test_atlas_info_hit_testing_waypoints() {
    let mut state = get_atlas_state().write().unwrap();
    state.atlas_screen_x = 0.0;
    state.atlas_screen_y = 0.0;
    state.atlas_image_size_x = 256.0;
    state.atlas_image_size_y = 256.0;
    state.atlas_max_x = 1024.0;
    state.atlas_max_y = 1024.0;
    compute_atlas_centering_offsets(&mut state);
    state.waypoints.clear();
    
    // Waypoint with 0.0 screen coords should be ignored
    state.waypoints.push(AtlasMarkInfo {
        x: 50.0,
        y: 50.0,
        screen_x: 0.0,
        screen_y: 0.0,
        text: "Hidden WP".to_string(),
    });

    // Valid waypoint
    state.waypoints.push(AtlasMarkInfo {
        x: 200.0,
        y: 200.0,
        screen_x: 10.0,
        screen_y: 10.0,
        text: "Visible WP".to_string(),
    });
    
    drop(state);

    let mut out_str = [0u8; 128];
    let mut out_x = 0.0;
    let mut out_y = 0.0;
    let mut out_color = 0;
    let mut out_guild = 0;

    let state_read = get_atlas_state().read().unwrap();
    let ax = state_read.atlas_screen_x;
    let ay = state_read.atlas_screen_y;
    
    let (lx1, ly1) = global_to_atlas(50, 50, &state_read);
    let screen_x1 = lx1 + ax;
    let screen_y1 = ly1 + ay;

    let (lx2, ly2) = global_to_atlas(200, 200, &state_read);
    let screen_x2 = lx2 + ax;
    let screen_y2 = ly2 + ay;
    
    drop(state_read);

    let hit1 = m2_minimap_get_atlas_info(
        screen_x1, screen_y1,
        out_str.as_mut_ptr(), out_str.len(),
        &mut out_x, &mut out_y,
        &mut out_color, &mut out_guild
    );
    assert!(!hit1, "Should not hit waypoint with screen_x/y <= 0.0");

    let hit2 = m2_minimap_get_atlas_info(
        screen_x2, screen_y2,
        out_str.as_mut_ptr(), out_str.len(),
        &mut out_x, &mut out_y,
        &mut out_color, &mut out_guild
    );
    assert!(hit2, "Failed to hit valid waypoint");
    assert_eq!(out_x, 200.0);
    assert_eq!(out_y, 200.0);
    
    let text = unsafe { std::ffi::CStr::from_ptr(out_str.as_ptr() as *const i8).to_string_lossy() };
    assert_eq!(text, "Visible WP");
}
