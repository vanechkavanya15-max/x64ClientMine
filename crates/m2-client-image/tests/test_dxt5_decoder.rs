use m2_client_image::dxt5_decoder::Dxt5Decoder;

#[test]
fn test_dxt5_decoder_creation() {
    let _decoder = Dxt5Decoder::new();
    // Testowanie poprawnego tworzenia dekodera (np. Dxt5Decoder::new() bez awarii)
}

#[test]
fn test_decode_block_invalid_size() {
    let decoder = Dxt5Decoder::new();
    let block = [0u8; 15]; // Za maly blok
    let result = decoder.decode_block(&block);
    assert!(result.is_err(), "Powinien zwrocic blad dla bloku mniejszego niz 16 bajtow");
}

#[test]
fn test_decode_image_invalid_size() {
    let decoder = Dxt5Decoder::new();
    let data = vec![0u8; 15]; // Za maly dla obrazu 4x4
    let result = decoder.decode_image(&data, 4, 4);
    assert!(result.is_err(), "Powinien zwrocic blad dla danych zbyt malych wzgledem wymiarow");
}

#[test]
fn test_decode_block_alpha_a0_gt_a1() {
    let decoder = Dxt5Decoder::new();
    // Pierwsze 8 bajtow = blok alpha
    // a0 = 255, a1 = 0
    // Indeksy alpha = 0 (zeby wszystkie piksele uzywaly pierwszego alpha czyli a0)
    // Nastepne 8 bajtow = blok kolorow
    // Kolory 0xFFFF i 0x0000, color indices = 0
    let block: [u8; 16] = [
        255, 0, 0, 0, 0, 0, 0, 0, // Alpha
        0xFF, 0xFF, 0x00, 0x00, 0, 0, 0, 0 // Kolory
    ];
    
    let result = decoder.decode_block(&block);
    assert!(result.is_ok());
    let output = result.unwrap();
    
    // Sprawdzamy pierwszy piksel (wszystkie z uwagi na index 0 beda identyczne)
    assert_eq!(output[3], 255); // Alpha a0 (poniewaz alpha_index = 0)
}

#[test]
fn test_decode_block_alpha_a0_le_a1() {
    let decoder = Dxt5Decoder::new();
    // Pierwsze 8 bajtow = blok alpha
    // a0 = 0, a1 = 255
    // Indeksy alpha uzyja np palety indeksu 7 (co da 255) z 6-stopniowej
    let block: [u8; 16] = [
        0, 255, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // Alpha (all index 7, c/d = 111b)
        0xFF, 0xFF, 0x00, 0x00, 0, 0, 0, 0 // Kolory
    ];
    
    let result = decoder.decode_block(&block);
    assert!(result.is_ok());
    let output = result.unwrap();
    
    // Z uwagi na indeks 7, w trybie a0 <= a1 (0 <= 255), wartosc palety alpha nr 7 to 255
    // Zatem ostatni uzyty indeks (tutaj z ukladania bitow, index=7)
    assert_eq!(output[3], 255);
}

#[test]
fn test_decode_image_clipping() {
    let decoder = Dxt5Decoder::new();
    // Potrzebujemy 1 bloku (16 bajtow)
    let data = [0u8; 16]; 
    
    // Pobieramy obraz np 3x3 piksele z 4x4 bloku
    let width = 3;
    let height = 3;
    
    let result = decoder.decode_image(&data, width, height);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    
    assert_eq!(decoded.len(), width * height * 4);
}
