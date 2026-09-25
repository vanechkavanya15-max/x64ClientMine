use m2_client_terrain::sector_loader::SectorLoader;

#[test]
fn test_integration_sector_loader() {
    let instance = SectorLoader::new();
    assert!(instance.is_ready());
}
