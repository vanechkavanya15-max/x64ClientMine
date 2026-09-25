use m2_client_terrain::raw_heightmap::RawHeightmap;

#[test]
fn test_integration_raw_heightmap() {
    let instance = RawHeightmap::new();
    assert!(instance.is_ready());
}
