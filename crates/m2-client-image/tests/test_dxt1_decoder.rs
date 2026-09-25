use m2_client_image::dxt1_decoder::{Dxt1Decoder, Rgba, decode_rgb565};

#[test]
fn test_integration_dxt1_decoder() {
    let instance = Dxt1Decoder::new();
    assert!(instance.is_ready());
}

#[test]
fn test_decode_rgb565() {
    // Bialy: 11111 111111 11111 = 0xFFFF
    let white = decode_rgb565(0xFFFF);
    assert_eq!(white, Rgba::new(255, 255, 255, 255));

    // Czarny: 00000 000000 00000 = 0x0000
    let black = decode_rgb565(0x0000);
    assert_eq!(black, Rgba::new(0, 0, 0, 255));

    // Czerwony: 11111 000000 00000 = 0xF800
    let red = decode_rgb565(0xF800);
    assert_eq!(red, Rgba::new(255, 0, 0, 255));
}

#[test]
fn test_decode_block_opaque() {
    let decoder = Dxt1Decoder::new();
    
    // c0 > c1 w DXT1 (tryb nieprzezroczysty)
    // Bialy (0xFFFF) i Czarny (0x0000)
    // 0xFFFF w little endian: [0xFF, 0xFF]
    // 0x0000 w little endian: [0x00, 0x00]
    let block = [
        0xFF, 0xFF, // c0
        0x00, 0x00, // c1
        0x00, 0x55, 0xAA, 0xFF // indices: 00000000 (0x00), 01010101 (0x55), 10101010 (0xAA), 11111111 (0xFF)
        // 0x00: indeksy 0, 0, 0, 0 -> color0 (Bialy)
        // 0x55: indeksy 1, 1, 1, 1 -> color1 (Czarny)
        // 0xAA: indeksy 2, 2, 2, 2 -> color2 (Interpolacja 2/3 Bialy + 1/3 Czarny)
        // 0xFF: indeksy 3, 3, 3, 3 -> color3 (Interpolacja 1/3 Bialy + 2/3 Czarny)
    ];

    let result = decoder.decode_block(&block).unwrap();

    // Wiersz 0 (indeksy 0): Bialy
    for i in 0..4 {
        assert_eq!(result[i], Rgba::new(255, 255, 255, 255));
    }
    // Wiersz 1 (indeksy 1): Czarny
    for i in 4..8 {
        assert_eq!(result[i], Rgba::new(0, 0, 0, 255));
    }
    // Wiersz 2 (indeksy 2): ~66% Bialy -> 255 * 2/3 = 170
    for i in 8..12 {
        assert_eq!(result[i], Rgba::new(170, 170, 170, 255));
    }
    // Wiersz 3 (indeksy 3): ~33% Bialy -> 255 * 1/3 = 85
    for i in 12..16 {
        assert_eq!(result[i], Rgba::new(85, 85, 85, 255));
    }
}

#[test]
fn test_decode_block_transparent() {
    let decoder = Dxt1Decoder::new();
    
    // c0 <= c1 w DXT1 (tryb 1-bit alpha)
    // Czarny (0x0000) i Bialy (0xFFFF)
    let block = [
        0x00, 0x00, // c0
        0xFF, 0xFF, // c1
        0x00, 0x55, 0xAA, 0xFF // indices: 0x00, 0x55, 0xAA, 0xFF
        // 0x00: indeksy 0, 0, 0, 0 -> color0 (Czarny)
        // 0x55: indeksy 1, 1, 1, 1 -> color1 (Bialy)
        // 0xAA: indeksy 2, 2, 2, 2 -> color2 (Interpolacja 1/2 Czarny + 1/2 Bialy)
        // 0xFF: indeksy 3, 3, 3, 3 -> color3 (Przezroczysty Czarny)
    ];

    let result = decoder.decode_block(&block).unwrap();

    // Wiersz 0 (indeksy 0): Czarny
    for i in 0..4 {
        assert_eq!(result[i], Rgba::new(0, 0, 0, 255));
    }
    // Wiersz 1 (indeksy 1): Bialy
    for i in 4..8 {
        assert_eq!(result[i], Rgba::new(255, 255, 255, 255));
    }
    // Wiersz 2 (indeksy 2): ~50% Bialy -> 255 / 2 = 127
    for i in 8..12 {
        assert_eq!(result[i], Rgba::new(127, 127, 127, 255));
    }
    // Wiersz 3 (indeksy 3): Przezroczysty
    for i in 12..16 {
        assert_eq!(result[i], Rgba::new(0, 0, 0, 0));
    }
}

#[test]
fn test_decode_image_errors() {
    let decoder = Dxt1Decoder::new();
    
    // Nieprawidlowe wymiary
    assert!(decoder.decode_image(&[], 0, 0).is_err());
    assert!(decoder.decode_image(&[], 3, 4).is_err()); // szerokosc nie dzieli sie przez 4
    
    // Za malo danych
    assert!(decoder.decode_image(&[0; 7], 4, 4).is_err()); // oczekiwane 8 bajtow
}

#[test]
fn test_decode_image_success() {
    let decoder = Dxt1Decoder::new();
    
    // Pojedynczy blok (4x4 piksele)
    let data = [
        0xFF, 0xFF, 0x00, 0x00, 0x00, 0x55, 0xAA, 0xFF
    ];
    let result = decoder.decode_image(&data, 4, 4).unwrap();
    
    // Oczekiwana wielkosc bufora wyjsciowego: 4 * 4 * 4 (RGBA) = 64
    assert_eq!(result.len(), 64);
    
    // Pierwszy piksel (indeks 0, kolor0: bialy)
    assert_eq!(result[0], 255);
    assert_eq!(result[1], 255);
    assert_eq!(result[2], 255);
    assert_eq!(result[3], 255);
}
