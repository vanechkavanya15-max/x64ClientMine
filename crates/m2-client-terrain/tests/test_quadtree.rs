use m2_client_terrain::quadtree::Quadtree;

#[test]
fn test_integration_quadtree() {
    let instance = Quadtree::new();
    assert!(instance.is_ready());
}
