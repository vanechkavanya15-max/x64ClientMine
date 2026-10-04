use m2_client::atlas_radar_blip_manager::{
    AtlasRadarBlipManager, BlipType, RadarBlip,
};

#[test]
fn test_radar_coordinate_transformations() {
    let mut manager = AtlasRadarBlipManager::new(5000);
    manager.add_blip(RadarBlip {
        id: 1,
        blip_type: BlipType::Player,
        x_cm: 1000.0,
        y_cm: 1000.0,
    });

    // Test with 0 degree rotation
    let blips_0_deg = manager.get_radar_blips(
        0.0, 0.0,      // center
        2000.0,        // radar radius cm
        100.0,         // radar screen radius
        0.0,           // camera rotation
        50.0, 50.0,    // radar center on screen
    );

    assert_eq!(blips_0_deg.len(), 1);
    let b = &blips_0_deg[0];
    
    // Scale is 100/2000 = 0.05
    // scaled_dx = 1000 * 0.05 = 50
    // scaled_dy = 1000 * 0.05 = 50
    // Center is 50,50 so result is 100,100
    assert!((b.screen_x - 100.0).abs() < 1e-4);
    assert!((b.screen_y - 100.0).abs() < 1e-4);

    // Test with 90 degree rotation
    let blips_90_deg = manager.get_radar_blips(
        0.0, 0.0,
        2000.0,
        100.0,
        90.0,
        50.0, 50.0,
    );
    
    assert_eq!(blips_90_deg.len(), 1);
    let b2 = &blips_90_deg[0];
    // dx=50, dy=50
    // rot_x = 50 * cos(90) - 50 * sin(90) = -50
    // rot_y = 50 * sin(90) + 50 * cos(90) = 50
    // + center(50,50) => (0, 100)
    assert!((b2.screen_x - 0.0).abs() < 1e-4);
    assert!((b2.screen_y - 100.0).abs() < 1e-4);
}

#[test]
fn test_atlas_coordinate_transformations() {
    let mut manager = AtlasRadarBlipManager::new(5000);
    manager.add_blip(RadarBlip {
        id: 1,
        blip_type: BlipType::Monster,
        x_cm: 2000.0,
        y_cm: 4000.0,
    });

    let blips = manager.get_atlas_blips(
        1000.0, 2000.0,    // atlas start cm
        10000.0, 10000.0,  // atlas width/height cm
        10.0, 20.0,        // atlas screen start
        500.0, 500.0,      // atlas screen size
    );

    assert_eq!(blips.len(), 1);
    let b = &blips[0];

    // scale_x = 500 / 10000 = 0.05
    // scale_y = 500 / 10000 = 0.05
    // dx = 2000 - 1000 = 1000
    // dy = 4000 - 2000 = 2000
    // screen_x = 10 + 1000 * 0.05 = 60
    // screen_y = 20 + 2000 * 0.05 = 120
    
    assert!((b.screen_x - 60.0).abs() < 1e-4);
    assert!((b.screen_y - 120.0).abs() < 1e-4);
}

#[test]
fn test_radar_radius_filtering() {
    let mut manager = AtlasRadarBlipManager::new(5000);
    manager.add_blip(RadarBlip {
        id: 1,
        blip_type: BlipType::Npc,
        x_cm: 100.0,
        y_cm: 0.0,
    });
    manager.add_blip(RadarBlip {
        id: 2,
        blip_type: BlipType::Npc,
        x_cm: 3000.0,
        y_cm: 0.0,
    });

    let radar_blips = manager.get_radar_blips(
        0.0, 0.0,
        1500.0, // Only covers id 1
        100.0,
        0.0,
        0.0, 0.0,
    );

    assert_eq!(radar_blips.len(), 1);
    assert_eq!(radar_blips[0].id, 1);

    // But atlas sees both if they are within bounds
    let atlas_blips = manager.get_atlas_blips(
        -5000.0, -5000.0,
        10000.0, 10000.0,
        0.0, 0.0,
        100.0, 100.0,
    );
    assert_eq!(atlas_blips.len(), 2);
}

#[test]
fn test_blip_colors() {
    assert_eq!(BlipType::Player.get_color_argb(), 0xFF00FF00);
    assert_eq!(BlipType::Party.get_color_argb(), 0xFF0000FF);
    assert_eq!(BlipType::Monster.get_color_argb(), 0xFFFF0000);
    assert_eq!(BlipType::Npc.get_color_argb(), 0xFFFFFF00);
    assert_eq!(BlipType::MetinStone.get_color_argb(), 0xFFFF00FF);
    assert_eq!(BlipType::Warp.get_color_argb(), 0xFF00FFFF);
}

#[test]
fn test_signal_points_fade() {
    let mut manager = AtlasRadarBlipManager::new(5000); // 5000ms fade duration
    
    // Add point at current time = 1000
    manager.add_signal_point(100.0, 200.0, 1000);
    
    // Test that it exists at time 3000
    manager.update_signal_points(3000);
    let signals = manager.get_radar_signal_points(
        0.0, 0.0, 1000.0, 100.0, 0.0, 0.0, 0.0
    );
    assert_eq!(signals.len(), 1);
    
    // Test that it expires at time 6001 (1000 + 5000 = 6000 expire time)
    manager.update_signal_points(6001);
    let signals = manager.get_radar_signal_points(
        0.0, 0.0, 1000.0, 100.0, 0.0, 0.0, 0.0
    );
    assert_eq!(signals.len(), 0);
}
