use m2_client::wgpu_terrain_array_splatter::{TerrainArraySplatter, calculate_heightmap_normals};

#[test]
fn test_terrain_array_splatter_weight() {
    let mut splatter = TerrainArraySplatter::new(256, 256, 16);
    
    // Set some weights
    splatter.set_weight(10, 20, 5, 128);
    splatter.set_weight(10, 21, 5, 255);
    splatter.set_weight(11, 20, 5, 64);
    splatter.set_weight(11, 21, 5, 192);

    assert_eq!(splatter.get_weight(10, 20, 5), 128);
    assert_eq!(splatter.get_weight(10, 21, 5), 255);
    
    // Test bilinear interpolation at center (10.5, 20.5)
    // w00 = 128/255 = 0.50196
    // w10 = 64/255 = 0.25098
    // w01 = 255/255 = 1.0
    // w11 = 192/255 = 0.75294
    // Top = (0.50196 + 0.25098) / 2 = 0.37647
    // Bottom = (1.0 + 0.75294) / 2 = 0.87647
    // Final = (0.37647 + 0.87647) / 2 = 0.62647
    
    let w = splatter.get_bilinear_alpha_weight(10.5, 20.5, 5);
    assert!((w - 0.62647).abs() < 0.001);
}

#[test]
fn test_calculate_heightmap_normals() {
    let width = 3;
    let height = 3;
    let height_scale = 1.0;
    let cell_scale = 100.0;
    
    let heightmap = vec![
        100, 100, 100,
        100, 200, 100,
        100, 100, 100,
    ];
    
    let normals = calculate_heightmap_normals(&heightmap, width, height, height_scale, cell_scale);
    
    assert_eq!(normals.len(), 9);
    
    // Center pixel (1,1) should point straight up since left/right and top/bottom are symmetric (100 - 100 = 0)
    let center_normal = normals[1 * width + 1];
    assert_eq!(center_normal, [0, 0, 127]); // Normal should be fully pointing in Z direction
    
    // Pixel (0,1) Left is 100 (clamp), Right is 200. nx = -(100 - 200) = 100.
    // ny is 0. nz is 200. Length = sqrt(100^2 + 200^2) = 223.6. 
    // nx = 100 / 223.6 * 127 = 56.8 -> 57
    // nz = 200 / 223.6 * 127 = 113.6 -> 114
    let left_normal = normals[1 * width + 0];
    assert_eq!(left_normal, [57, 0, 114]);
}

#[test]
#[should_panic]
fn test_too_many_layers() {
    let _ = TerrainArraySplatter::new(256, 256, 17);
}
