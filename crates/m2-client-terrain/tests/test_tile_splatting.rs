use m2_client_terrain::tile_splatting::TileSplatting;

#[test]
fn test_integration_tile_splatting() {
    let instance = TileSplatting::new();
    assert!(instance.is_ready());
}
