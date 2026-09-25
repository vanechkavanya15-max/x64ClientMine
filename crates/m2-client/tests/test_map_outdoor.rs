use m2_client::map_outdoor::{MapOutdoor, MapPart, TerrainRenderSort, TerrainCoordinate};
 
 #[test]
fn test_map_outdoor_initialization() {
    let mut instance = MapOutdoor::new();
     assert!(instance.is_ready());
    assert_eq!(instance.get_terrain_count(), (0, 0));
    assert_eq!(instance.get_base_xy(), (0, 0));
    assert_eq!(instance.is_enable_portal(), false);
    assert_eq!(instance.get_opaque_water_depth(), 400.0);
    assert_eq!(instance.get_terrain_render_sort(), TerrainRenderSort::DistanceSort);
    assert_eq!(instance.visible_part_flags, 0xffffffff);
    assert_eq!(instance.splat_limit, 50000);
    assert_eq!(instance.cur_coordinate, TerrainCoordinate { terrain_coord_x: -1, terrain_coord_y: -1 });

    instance.clear();
    assert!(!instance.is_ready());
    assert_eq!(instance.get_terrain_count(), (0, 0));
}

#[test]
fn test_map_outdoor_getters_setters() {
    let mut instance = MapOutdoor::new();

    // Terrain Count
    assert!(!instance.set_terrain_count(0, 0));
    assert!(!instance.set_terrain_count(256, 10));
    assert!(instance.set_terrain_count(10, 20));
    assert_eq!(instance.get_terrain_count(), (10, 20));

    // Base XY
    instance.set_base_xy(100, 200);
    assert_eq!(instance.get_base_xy(), (100, 200));

    // Portal & Terrain flags
    instance.enable_portal(true);
    assert!(instance.is_enable_portal());

    instance.enable_terrain_only_for_height(true);
    assert!(instance.enable_terrain_only_for_height);

    // Opaque Water Depth
    instance.set_opaque_water_depth(500.0);
    assert_eq!(instance.get_opaque_water_depth(), 500.0);

    // Terrain Render Sort
    instance.set_terrain_render_sort(TerrainRenderSort::TextureSort);
    assert_eq!(instance.get_terrain_render_sort(), TerrainRenderSort::TextureSort);

    // Splat limit
    instance.set_splat_limit(100000);
    assert_eq!(instance.splat_limit, 100000);
}

#[test]
fn test_map_outdoor_set_visible_part() {
    let mut instance = MapOutdoor::new();
    
    // Default is all 1s (0xffffffff)
    assert_eq!(instance.visible_part_flags, 0xffffffff);

    // Turn off Sky (bit 5)
    instance.set_visible_part(MapPart::Sky, false);
    assert_eq!(instance.visible_part_flags & (1 << (MapPart::Sky as u32)), 0);

    // Turn off Terrain (bit 0)
    instance.set_visible_part(MapPart::Terrain, false);
    assert_eq!(instance.visible_part_flags & (1 << (MapPart::Terrain as u32)), 0);

    // Turn on Sky again
    instance.set_visible_part(MapPart::Sky, true);
    assert_ne!(instance.visible_part_flags & (1 << (MapPart::Sky as u32)), 0);
}

#[test]
fn test_map_outdoor_height_mock() {
    let instance = MapOutdoor::new();
    assert_eq!(instance.get_terrain_height(10.0, 20.0), 0.0);
    assert_eq!(instance.get_height(10.0, 20.0), 0.0);
    assert_eq!(instance.get_cache_height(10.0, 20.0), 0.0);
}

#[test]
fn test_map_outdoor_convert_to_map_coords() {
    let instance = MapOutdoor::new();
    let (cx, cy, subx, suby, tx, ty) = instance.convert_to_map_coords(1000.0, 2000.0);
    
    // Validate output is deterministic and matches expected mock math
    assert_eq!(cx, 10);
    assert_eq!(cy, 20);
    assert_eq!(subx, 0);
    assert_eq!(suby, 0);
    assert_eq!(tx, 0);
    assert_eq!(ty, 0);
 }
