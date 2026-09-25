use m2_client_terrain::tile_splatting::{SplatWeights, TileSplatting};

#[test]
fn test_default_values() {
    let instance = TileSplatting::new();
    assert!(!instance.is_ready());
    assert!(instance.get_weights(0, 0).is_none());
}

#[test]
fn test_initialization() {
    let mut instance = TileSplatting::new();
    instance.init(2, 2);
    assert!(instance.is_ready());

    // Sprawdzenie czy wszystkie poczatkowe wagi sa zerowe
    for y in 0..2 {
        for x in 0..2 {
            let weights = instance.get_weights(x, y).unwrap();
            assert_eq!(weights.layers, [0, 0, 0, 0]);
        }
    }
}

#[test]
fn test_set_and_get_weights() {
    let mut instance = TileSplatting::new();
    instance.init(4, 4);

    let new_weights = SplatWeights { layers: [255, 128, 64, 32] };
    assert!(instance.set_weights(1, 2, new_weights));

    let retrieved = instance.get_weights(1, 2).unwrap();
    assert_eq!(retrieved.layers, [255, 128, 64, 32]);

    // Przekroczenie granic
    assert!(!instance.set_weights(4, 4, new_weights));
    assert!(instance.get_weights(4, 4).is_none());
}

#[test]
fn test_load_from_buffer_success() {
    let mut instance = TileSplatting::new();
    // Mapa 2x2 = 4 wierzcholki * 4 warstwy = 16 bajtow
    let buffer: Vec<u8> = vec![
        1, 2, 3, 4,       // (0, 0)
        5, 6, 7, 8,       // (1, 0)
        9, 10, 11, 12,    // (0, 1)
        13, 14, 15, 16,   // (1, 1)
    ];

    let result = instance.load_from_buffer(2, 2, &buffer);
    assert!(result.is_ok());
    assert!(instance.is_ready());

    let w1 = instance.get_weights(0, 0).unwrap();
    assert_eq!(w1.layers, [1, 2, 3, 4]);

    let w4 = instance.get_weights(1, 1).unwrap();
    assert_eq!(w4.layers, [13, 14, 15, 16]);
}

#[test]
fn test_load_from_buffer_too_small() {
    let mut instance = TileSplatting::new();
    let buffer: Vec<u8> = vec![0; 15]; // Potrzeba 16 bajtow dla 2x2

    let result = instance.load_from_buffer(2, 2, &buffer);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Bufor jest za maly, aby zaladowac mape wag splattingu");
}

#[test]
fn test_export_to_buffer() {
    let mut instance = TileSplatting::new();
    instance.init(2, 1);
    
    instance.set_weights(0, 0, SplatWeights { layers: [10, 20, 30, 40] });
    instance.set_weights(1, 0, SplatWeights { layers: [50, 60, 70, 80] });

    let exported = instance.export_to_buffer();
    assert_eq!(exported.len(), 8);
    assert_eq!(exported, vec![10, 20, 30, 40, 50, 60, 70, 80]);
}
