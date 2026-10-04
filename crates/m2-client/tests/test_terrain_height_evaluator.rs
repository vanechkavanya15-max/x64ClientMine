use m2_client::terrain_height_evaluator::*;

#[test]
fn test_get_height() {
    let mut raw_height_map = vec![1000; HEIGHTMAP_RAW_XSIZE * HEIGHTMAP_RAW_YSIZE];
    raw_height_map[1 * HEIGHTMAP_RAW_XSIZE + 1] = 2000; // x=0, y=0 cell

    let attr_map = vec![0; ATTRMAP_XSIZE * ATTRMAP_YSIZE];
    let evaluator = TerrainHeightEvaluator::new(raw_height_map, attr_map, 1.0, 0, 0);

    let h = evaluator.get_height(0.0, 0.0);
    assert_eq!(h, 2000.0);
}

#[test]
fn test_is_attr_on() {
    let raw_height_map = vec![0; HEIGHTMAP_RAW_XSIZE * HEIGHTMAP_RAW_YSIZE];
    let mut attr_map = vec![0; ATTRMAP_XSIZE * ATTRMAP_YSIZE];
    
    // Set an attribute at 0, 0
    attr_map[0] = ATTRIBUTE_WATER;

    let evaluator = TerrainHeightEvaluator::new(raw_height_map, attr_map, 1.0, 0, 0);

    assert!(evaluator.is_attr_on(0.0, 0.0, ATTRIBUTE_WATER));
    assert!(!evaluator.is_attr_on(0.0, 0.0, ATTRIBUTE_BLOCK));
}

#[test]
fn test_raycast() {
    let mut raw_height_map = vec![0; HEIGHTMAP_RAW_XSIZE * HEIGHTMAP_RAW_YSIZE];
    for y in 0..HEIGHTMAP_RAW_YSIZE {
        for x in 0..HEIGHTMAP_RAW_XSIZE {
            raw_height_map[y * HEIGHTMAP_RAW_XSIZE + x] = 10;
        }
    }

    let attr_map = vec![0; ATTRMAP_XSIZE * ATTRMAP_YSIZE];
    let evaluator = TerrainHeightEvaluator::new(raw_height_map, attr_map, 1.0, 0, 0);

    // Line of sight above terrain (z=20.0 to z=20.0, terrain is at 10.0)
    assert!(evaluator.raycast(0.0, 0.0, 20.0, 200.0, 200.0, 20.0));

    // Line of sight going through terrain (z=20.0 to z=0.0, hits terrain at 10.0)
    assert!(!evaluator.raycast(0.0, 0.0, 20.0, 200.0, 200.0, 0.0));
}
