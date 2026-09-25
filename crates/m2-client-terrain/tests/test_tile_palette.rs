use m2_client_terrain::tile_palette::TilePalette;

#[test]
fn test_integration_tile_palette() {
    let instance = TilePalette::new();
    assert!(instance.is_ready());
}
