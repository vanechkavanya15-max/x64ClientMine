use m2_client_terrain::raw_heightmap::RawHeightmap;

#[test]
fn test_integration_raw_heightmap() {
    let instance = RawHeightmap::new();
    assert!(instance.is_ready());
}

#[test]
fn test_from_bytes_invalid_size() {
    let bytes: [u8; 3] = [0, 0, 0];
    let result = RawHeightmap::from_bytes(2, 2, &bytes);
    assert!(result.is_err());
}

#[test]
fn test_from_bytes_valid_and_get_raw_height() {
    // 2x2 heightmap, 16-bit little-endian
    // [0, 0] = 10 -> [10, 0]
    // [1, 0] = 20 -> [20, 0]
    // [0, 1] = 30 -> [30, 0]
    // [1, 1] = 40 -> [40, 0]
    let bytes: [u8; 8] = [10, 0, 20, 0, 30, 0, 40, 0];
    let heightmap = RawHeightmap::from_bytes(2, 2, &bytes).expect("Powinno sie wczytac");
    
    assert!(heightmap.is_ready());
    assert_eq!(heightmap.width, 2);
    assert_eq!(heightmap.height, 2);
    
    assert_eq!(heightmap.get_raw_height(0, 0), 10);
    assert_eq!(heightmap.get_raw_height(1, 0), 20);
    assert_eq!(heightmap.get_raw_height(0, 1), 30);
    assert_eq!(heightmap.get_raw_height(1, 1), 40);
    
    // Poza zakresem
    assert_eq!(heightmap.get_raw_height(2, 0), 0);
    assert_eq!(heightmap.get_raw_height(0, 2), 0);
}

#[test]
fn test_get_height_interpolation() {
    let bytes: [u8; 8] = [10, 0, 20, 0, 30, 0, 40, 0];
    let heightmap = RawHeightmap::from_bytes(2, 2, &bytes).unwrap();
    
    // Rogi (bez interpolacji)
    assert_eq!(heightmap.get_height(0.0, 0.0), 10.0);
    assert_eq!(heightmap.get_height(1.0, 0.0), 20.0);
    assert_eq!(heightmap.get_height(0.0, 1.0), 30.0);
    assert_eq!(heightmap.get_height(1.0, 1.0), 40.0);
    
    // Srodek, interpolacja miedzy 10, 20, 30, 40 -> (10+20+30+40)/4 = 25
    assert_eq!(heightmap.get_height(0.5, 0.5), 25.0);
    
    // Interpolacja w osi X miedzy 0,0 a 1,0 (10 a 20)
    assert_eq!(heightmap.get_height(0.5, 0.0), 15.0);
    
    // Poza zakresem - powinno zrobic clamp do krawedzi
    assert_eq!(heightmap.get_height(-1.0, -1.0), 10.0); // Clamp do (0,0)
    assert_eq!(heightmap.get_height(2.0, 2.0), 40.0); // Clamp do (1,1)
}

#[test]
fn test_get_normal() {
    // Prosta plaska powierzchnia
    // [0,0]=10, [1,0]=10, [2,0]=10
    // [0,1]=10, [1,1]=10, [2,1]=10
    // [0,2]=10, [1,2]=10, [2,2]=10
    let mut bytes = vec![0u8; 18]; // 3x3 * 2 bytes = 18
    for chunk in bytes.chunks_exact_mut(2) {
        chunk[0] = 10;
        chunk[1] = 0;
    }
    
    let heightmap = RawHeightmap::from_bytes(3, 3, &bytes).unwrap();
    let normal = heightmap.get_normal(1.0, 1.0);
    
    // Poniewaz h jest wszedzie takie samo:
    // dx = 0, dy = 0, nz > 0
    // wiec normalny powinien byc skierowany pionowo w gore (0, 0, 1)
    assert_eq!(normal[0], 0.0);
    assert_eq!(normal[1], 0.0);
    assert_eq!(normal[2], 1.0);
}
