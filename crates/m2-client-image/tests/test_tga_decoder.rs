use m2_client_image::tga_decoder::{TgaDecoder, TgaError};

fn create_tga_header(
    image_type: u8,
    width: u16,
    height: u16,
    pixel_depth: u8,
    image_descriptor: u8,
) -> Vec<u8> {
    let mut header = vec![0u8; 18];
    header[2] = image_type;
    header[12..14].copy_from_slice(&width.to_le_bytes());
    header[14..16].copy_from_slice(&height.to_le_bytes());
    header[16] = pixel_depth;
    header[17] = image_descriptor;
    header
}

#[test]
fn test_decode_uncompressed_24bit_top_down() {
    let mut data = create_tga_header(2, 2, 2, 24, 0x20); // Top-down
    // Pixels (BGR format in TGA)
    // Row 0: Red (0, 0, 255), Green (0, 255, 0)
    // Row 1: Blue (255, 0, 0), White (255, 255, 255)
    data.extend_from_slice(&[
        0, 0, 255,  // Red
        0, 255, 0,  // Green
        255, 0, 0,  // Blue
        255, 255, 255 // White
    ]);

    let decoder = TgaDecoder::decode(&data).unwrap();
    assert_eq!(decoder.width, 2);
    assert_eq!(decoder.height, 2);

    // Expected RGBA output
    let expected = vec![
        255, 0, 0, 255,     // Red
        0, 255, 0, 255,     // Green
        0, 0, 255, 255,     // Blue
        255, 255, 255, 255, // White
    ];
    assert_eq!(decoder.rgba_data, expected);
}

#[test]
fn test_decode_uncompressed_32bit_bottom_up() {
    let mut data = create_tga_header(2, 2, 2, 32, 0x00); // Bottom-up
    // Pixels (BGRA format in TGA) stored bottom-up
    // Physical row 0 (Image Bottom Row 1): Blue (255, 0, 0, 255), White (255, 255, 255, 255)
    // Physical row 1 (Image Top Row 0): Red (0, 0, 255, 255), Green (0, 255, 0, 255)
    data.extend_from_slice(&[
        255, 0, 0, 255,     // Blue
        255, 255, 255, 255, // White
        0, 0, 255, 255,     // Red
        0, 255, 0, 255,     // Green
    ]);

    let decoder = TgaDecoder::decode(&data).unwrap();
    
    let expected = vec![
        255, 0, 0, 255,     // Red (Top-left)
        0, 255, 0, 255,     // Green (Top-right)
        0, 0, 255, 255,     // Blue (Bottom-left)
        255, 255, 255, 255, // White (Bottom-right)
    ];
    assert_eq!(decoder.rgba_data, expected);
}

#[test]
fn test_decode_rle_24bit_top_down() {
    let mut data = create_tga_header(10, 4, 1, 24, 0x20); // Top-down
    // 2 Red pixels (RLE packet)
    data.push(0x80 | 1); // RLE flag + count = 2
    data.extend_from_slice(&[0, 0, 255]); // Red (BGR)
    // 2 Raw pixels (Blue, White)
    data.push(1); // Raw count = 2
    data.extend_from_slice(&[255, 0, 0]); // Blue (BGR)
    data.extend_from_slice(&[255, 255, 255]); // White (BGR)

    let decoder = TgaDecoder::decode(&data).unwrap();
    assert_eq!(decoder.width, 4);
    assert_eq!(decoder.height, 1);

    let expected = vec![
        255, 0, 0, 255,     // Red
        255, 0, 0, 255,     // Red
        0, 0, 255, 255,     // Blue
        255, 255, 255, 255, // White
    ];
    assert_eq!(decoder.rgba_data, expected);
}

#[test]
fn test_decode_invalid_size() {
    let data = vec![0u8; 10]; // Za krotkie na naglowek
    assert_eq!(TgaDecoder::decode(&data).unwrap_err(), TgaError::InvalidSize);

    let data2 = create_tga_header(2, 0, 10, 24, 0); // Szerokosc 0
    assert_eq!(TgaDecoder::decode(&data2).unwrap_err(), TgaError::InvalidSize);
}

#[test]
fn test_decode_unsupported_format() {
    let data = create_tga_header(1, 2, 2, 24, 0); // Typ 1 (z paleta kolorow)
    assert_eq!(TgaDecoder::decode(&data).unwrap_err(), TgaError::UnsupportedFormat);

    let data2 = create_tga_header(2, 2, 2, 16, 0); // 16-bit
    assert_eq!(TgaDecoder::decode(&data2).unwrap_err(), TgaError::UnsupportedFormat);
}

#[test]
fn test_decode_truncated_data() {
    let mut data = create_tga_header(2, 2, 2, 24, 0x20);
    // Brakuje danych pikseli (oczekiwane 4 * 3 = 12, podajemy 10)
    data.extend_from_slice(&[0u8; 10]);
    assert_eq!(TgaDecoder::decode(&data).unwrap_err(), TgaError::InvalidData);
}
