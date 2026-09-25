use m2_client_terrain::minimap_generator::{MinimapGenerator, PIXELS_COUNT, OUTPUT_RGB_SIZE};

#[test]
fn test_integration_minimap_generator() {
    let instance = MinimapGenerator::new();
    assert!(instance.is_ready());
}

#[test]
fn test_generate_tile_success() {
    let generator = MinimapGenerator::new();
    let height_data = vec![1000; PIXELS_COUNT];
    
    let result = generator.generate_tile(&height_data, None);
    assert!(result.is_ok());
    
    let output = result.unwrap();
    assert_eq!(output.len(), OUTPUT_RGB_SIZE);
}

#[test]
fn test_generate_tile_invalid_size() {
    let generator = MinimapGenerator::new();
    
    // Za mala tablica wysokosci
    let height_data = vec![1000; 100];
    let result = generator.generate_tile(&height_data, None);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Nieprawidlowy rozmiar mapy wysokosci (wymagane 256x256)");
    
    // Prawidlowa wysokosc, zla mapa kolorow
    let height_data_valid = vec![1000; PIXELS_COUNT];
    let color_map_invalid = vec![0; 100];
    let result2 = generator.generate_tile(&height_data_valid, Some(&color_map_invalid));
    assert!(result2.is_err());
    assert_eq!(result2.unwrap_err(), "Nieprawidlowy rozmiar mapy kolorow (wymagane 256x256x3)");
}

#[test]
fn test_generate_tile_with_colors() {
    let generator = MinimapGenerator::new();
    let height_data = vec![1000; PIXELS_COUNT];
    
    // Tworzenie mapy kolorow (jednolity kolor czerwony)
    let mut color_map = vec![0; OUTPUT_RGB_SIZE];
    for i in 0..PIXELS_COUNT {
        color_map[i * 3] = 255;     // R
        color_map[i * 3 + 1] = 0;   // G
        color_map[i * 3 + 2] = 0;   // B
    }
    
    let result = generator.generate_tile(&height_data, Some(&color_map));
    assert!(result.is_ok());
    
    let output = result.unwrap();
    assert_eq!(output.len(), OUTPUT_RGB_SIZE);
    
    // Sprawdzanie czy kolory sa poprawne po uwzglednieniu cieniowania.
    // Poniewaz roznica wysokosci to 0, mnoznik to 1.0, wiec powinien byc zachowany czerwony
    assert_eq!(output[0], 255);
    assert_eq!(output[1], 0);
    assert_eq!(output[2], 0);
}
