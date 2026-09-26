use m2_client::terrain_height_patch::{
    HeightPatchGrid, RawHeightMap,
};

#[test]
fn test_height_patch_grid_build() {
    let width = 128;
    let height = 128;
    let mut heights = vec![0.0; width * height];

    // Set specific heights for patch (1, 1) -> x in 16..32, y in 16..32
    for y in 16..32 {
        for x in 16..32 {
            heights[y * width + x] = 50.0;
        }
    }
    // Set a min and max value for patch (1, 1)
    heights[16 * width + 16] = 10.0;
    heights[31 * width + 31] = 100.0;

    let height_map = RawHeightMap::new(width, height, heights);
    let grid = HeightPatchGrid::build(&height_map);

    assert_eq!(grid.patches.len(), 64);

    // Verify patch (0, 0)
    let patch_0_0 = grid
        .patches
        .iter()
        .find(|p| p.patch_x == 0 && p.patch_y == 0)
        .unwrap();
    assert_eq!(patch_0_0.min_z, 0.0);
    assert_eq!(patch_0_0.max_z, 0.0);

    // Verify patch (1, 1)
    let patch_1_1 = grid
        .patches
        .iter()
        .find(|p| p.patch_x == 1 && p.patch_y == 1)
        .unwrap();
    assert_eq!(patch_1_1.min_z, 10.0);
    assert_eq!(patch_1_1.max_z, 100.0);
}

#[test]
fn test_ray_intersects_patch_hit() {
    let width = 128;
    let height = 128;
    let heights = vec![50.0; width * height];
    let height_map = RawHeightMap::new(width, height, heights);
    let grid = HeightPatchGrid::build(&height_map);

    let patch_x = 2;
    let patch_y = 2;

    // Bounds for patch (2, 2):
    // min_x = 2 * 16 * 200 = 6400
    // min_y = 2 * 16 * 200 = 6400
    // max_x = 6400 + 3200 = 9600
    // max_y = 6400 + 3200 = 9600
    // min_z = 50, max_z = 50

    // Ray starting directly above the center of the patch, pointing straight down
    let ray_start = (8000.0, 8000.0, 1000.0);
    let ray_dir = (0.0, 0.0, -1.0);

    let hit = grid.ray_intersects_patch(patch_x, patch_y, ray_start, ray_dir);
    assert!(hit, "Ray should intersect the patch");
}

#[test]
fn test_ray_intersects_patch_miss() {
    let width = 128;
    let height = 128;
    let heights = vec![50.0; width * height];
    let height_map = RawHeightMap::new(width, height, heights);
    let grid = HeightPatchGrid::build(&height_map);

    let patch_x = 2;
    let patch_y = 2;

    // Ray starting outside the bounds in X and Y, pointing straight down
    let ray_start = (2000.0, 2000.0, 1000.0);
    let ray_dir = (0.0, 0.0, -1.0);

    let hit = grid.ray_intersects_patch(patch_x, patch_y, ray_start, ray_dir);
    assert!(!hit, "Ray should NOT intersect the patch");
}

#[test]
fn test_ray_intersects_patch_diagonal_miss() {
    let width = 128;
    let height = 128;
    let heights = vec![50.0; width * height];
    let height_map = RawHeightMap::new(width, height, heights);
    let grid = HeightPatchGrid::build(&height_map);

    let patch_x = 2;
    let patch_y = 2;

    // Ray pointing away from the patch
    let ray_start = (6300.0, 6300.0, 1000.0);
    let ray_dir = (-1.0, -1.0, -1.0);

    let hit = grid.ray_intersects_patch(patch_x, patch_y, ray_start, ray_dir);
    assert!(!hit, "Ray pointing away should NOT intersect the patch");
}
