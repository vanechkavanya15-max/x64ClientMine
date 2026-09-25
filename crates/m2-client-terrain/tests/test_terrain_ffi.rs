use m2_client_terrain::terrain_ffi::TerrainFfi;

#[test]
fn test_integration_terrain_ffi() {
    let instance = TerrainFfi::new();
    assert!(instance.is_ready());
}
