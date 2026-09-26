use m2_client::dxt1_decoder::decode_dxt1;

#[test]
fn test_invalid_size() {
    let data = vec![0; 8];
    assert!(decode_dxt1(&data, 3, 4).is_err());
    assert!(decode_dxt1(&data, 4, 3).is_err());
    assert!(decode_dxt1(&data, 0, 4).is_err());
    assert!(decode_dxt1(&data, 4, 0).is_err());
}

#[test]
fn test_invalid_buffer_length() {
    let data = vec![0; 7];
    assert!(decode_dxt1(&data, 4, 4).is_err());

    let data2 = vec![0; 9];
    assert!(decode_dxt1(&data2, 4, 4).is_err());
}

fn pack_rgb565(r: u8, g: u8, b: u8) -> u16 {
    let r5 = (r >> 3) as u16;
    let g6 = (g >> 2) as u16;
    let b5 = (b >> 3) as u16;
    (r5 << 11) | (g6 << 5) | b5
}

#[test]
fn test_decode_opaque_block() {
    // Test bloku z c0 > c1 (brak przezroczystosci)
    // Tworzymy c0 i c1 tak, aby c0 > c1.
    // Np. c0 = bialy, c1 = czarny.
    let c0 = pack_rgb565(255, 255, 255);
    let c1 = pack_rgb565(0, 0, 0);

    assert!(c0 > c1);

    let mut block = vec![0u8; 8];
    block[0..2].copy_from_slice(&c0.to_le_bytes());
    block[2..4].copy_from_slice(&c1.to_le_bytes());

    // Ustawiamy indeksy:
    // Indeksy sa na pikselach 0-15.
    // Chcemy: pixel 0 -> indeks 0, pixel 1 -> indeks 1, pixel 2 -> indeks 2, pixel 3 -> indeks 3
    // Reszta pikseli -> indeks 0.
    let mut indices: u32 = 0;
    indices |= 0b00 << 0; // pixel 0, indeks 0
    indices |= 0b01 << 2; // pixel 1, indeks 1
    indices |= 0b10 << 4; // pixel 2, indeks 2
    indices |= 0b11 << 6; // pixel 3, indeks 3

    block[4..8].copy_from_slice(&indices.to_le_bytes());

    let decoded = decode_dxt1(&block, 4, 4).expect("Failed to decode");

    assert_eq!(decoded.len(), 4 * 4 * 4);

    // pixel 0 (indeks 0 -> color0)
    assert_eq!(&decoded[0..4], &[255, 255, 255, 255]);

    // pixel 1 (indeks 1 -> color1)
    assert_eq!(&decoded[4..8], &[0, 0, 0, 255]);

    // pixel 2 (indeks 2 -> (2*color0 + color1)/3) = (2*255)/3 = 510/3 = 170
    assert_eq!(&decoded[8..12], &[170, 170, 170, 255]);

    // pixel 3 (indeks 3 -> (color0 + 2*color1)/3) = 255/3 = 85
    assert_eq!(&decoded[12..16], &[85, 85, 85, 255]);
}

#[test]
fn test_decode_transparent_block() {
    // Test bloku z c0 <= c1 (1-bit alpha)
    // Tworzymy c0 i c1 tak, aby c0 <= c1.
    // Np. c0 = czarny, c1 = bialy.
    let c0 = pack_rgb565(0, 0, 0);
    let c1 = pack_rgb565(255, 255, 255);

    assert!(c0 <= c1);

    let mut block = vec![0u8; 8];
    block[0..2].copy_from_slice(&c0.to_le_bytes());
    block[2..4].copy_from_slice(&c1.to_le_bytes());

    // Indeksy: pixel 0->0, pixel 1->1, pixel 2->2, pixel 3->3
    let mut indices: u32 = 0;
    indices |= 0b00 << 0; // pixel 0, indeks 0
    indices |= 0b01 << 2; // pixel 1, indeks 1
    indices |= 0b10 << 4; // pixel 2, indeks 2
    indices |= 0b11 << 6; // pixel 3, indeks 3

    block[4..8].copy_from_slice(&indices.to_le_bytes());

    let decoded = decode_dxt1(&block, 4, 4).expect("Failed to decode");

    // pixel 0 (indeks 0 -> color0)
    assert_eq!(&decoded[0..4], &[0, 0, 0, 255]);

    // pixel 1 (indeks 1 -> color1)
    assert_eq!(&decoded[4..8], &[255, 255, 255, 255]);

    // pixel 2 (indeks 2 -> (color0 + color1)/2) = 255/2 = 127
    assert_eq!(&decoded[8..12], &[127, 127, 127, 255]);

    // pixel 3 (indeks 3 -> przezroczysty czarny)
    assert_eq!(&decoded[12..16], &[0, 0, 0, 0]);
}

#[test]
fn test_decode_multi_block_8x8() {
    // Tworzymy 4 bloki. Obraz 8x8.
    // Blok 0 (lewy gorny) - c0>c1 (c0 = bialy)
    // Blok 1 (prawy gorny) - c0<=c1 (c0 = czerwony, c1 = cyjan)
    // Blok 2 (lewy dolny) - c0>c1 (c0 = zielony)
    // Blok 3 (prawy dolny) - c0>c1 (c0 = niebieski)
    let mut data = vec![0u8; 32]; // 4 * 8 bajtow

    // Blok 0
    let c0 = pack_rgb565(255, 255, 255);
    let c1 = pack_rgb565(0, 0, 0);
    data[0..2].copy_from_slice(&c0.to_le_bytes());
    data[2..4].copy_from_slice(&c1.to_le_bytes());
    data[4..8].copy_from_slice(&0u32.to_le_bytes()); // same zera = indeks 0 = bialy

    // Blok 1
    let c0 = pack_rgb565(0, 0, 0);
    let c1 = pack_rgb565(255, 255, 255);
    assert!(c0 <= c1);
    data[8..10].copy_from_slice(&c0.to_le_bytes());
    data[10..12].copy_from_slice(&c1.to_le_bytes());
    // Dla bloku 1 wszystkie piksele niech beda index 3 (0b11 dla kazdego = 0xFFFFFFFF)
    data[12..16].copy_from_slice(&0xFFFFFFFFu32.to_le_bytes()); // przezroczyste

    // Blok 2
    let c0 = pack_rgb565(0, 255, 0);
    let c1 = pack_rgb565(0, 0, 0);
    data[16..18].copy_from_slice(&c0.to_le_bytes());
    data[18..20].copy_from_slice(&c1.to_le_bytes());
    data[20..24].copy_from_slice(&0u32.to_le_bytes()); // same zera = indeks 0 = zielony

    // Blok 3
    let c0 = pack_rgb565(0, 0, 255);
    let c1 = pack_rgb565(0, 0, 0);
    data[24..26].copy_from_slice(&c0.to_le_bytes());
    data[26..28].copy_from_slice(&c1.to_le_bytes());
    data[28..32].copy_from_slice(&0u32.to_le_bytes()); // same zera = indeks 0 = niebieski

    let decoded = decode_dxt1(&data, 8, 8).expect("Failed to decode 8x8");

    // Sprawdzmy piksel z pierwszego bloku (x=0, y=0)
    let p_0_0 = &decoded[0..4];
    assert_eq!(p_0_0, &[255, 255, 255, 255]);

    // Sprawdzmy piksel z drugiego bloku (x=4, y=0). Szerokosc = 8, wiec index to (0*8 + 4)*4 = 16
    let p_4_0 = &decoded[16..20];
    assert_eq!(p_4_0, &[0, 0, 0, 0]);

    // Sprawdzmy piksel z trzeciego bloku (x=0, y=4). Index to (4*8 + 0)*4 = 128
    let p_0_4 = &decoded[128..132];
    assert_eq!(p_0_4, &[0, 255, 0, 255]);

    // Sprawdzmy piksel z czwartego bloku (x=4, y=4). Index to (4*8 + 4)*4 = 144
    let p_4_4 = &decoded[144..148];
    assert_eq!(p_4_4, &[0, 0, 255, 255]);
}
