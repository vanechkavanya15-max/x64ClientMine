use m2_client::bmp_decoder::BmpImage;

#[test]
fn test_bmp_decoder_invalid_magic() {
    let mut data = vec![0u8; 100];
    data[0] = b'M';
    data[1] = b'Z'; // Invalid magic, expected 'BM'

    let result = BmpImage::from_bytes(&data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Nieprawidlowy magic number (oczekiwano 'BM')");
}

#[test]
fn test_bmp_decoder_invalid_size() {
    let data = vec![0u8; 10]; // Too small for header
    let result = BmpImage::from_bytes(&data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Plik jest za krotki aby zawierac naglowek BMP");
}

#[test]
fn test_bmp_decoder_unsupported_compression() {
    let mut data = vec![0u8; 100];
    // Magic 'BM'
    data[0] = b'B';
    data[1] = b'M';
    // Offset
    data[10] = 54;
    // DIB header size
    data[14] = 40;
    // Width and height
    data[18] = 2; // width
    data[22] = 2; // height
    // Bit count
    data[28] = 24;
    // Compression (BI_RLE8 = 1, expected 0 for BI_RGB)
    data[30] = 1;

    let result = BmpImage::from_bytes(&data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Obslugiwana jest tylko nieskompresowana kompresja BI_RGB (0)");
}

#[test]
fn test_bmp_decoder_unsupported_bpp() {
    let mut data = vec![0u8; 100];
    // Magic 'BM'
    data[0] = b'B';
    data[1] = b'M';
    // Offset
    data[10] = 54;
    // DIB header size
    data[14] = 40;
    // Width and height
    data[18] = 2; // width
    data[22] = 2; // height
    // Bit count
    data[28] = 16; // 16-bit not supported
    // Compression
    data[30] = 0;

    let result = BmpImage::from_bytes(&data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Obslugiwane sa tylko formaty 24-bit BGR i 32-bit BGRA");
}

fn create_valid_bmp(width: i32, height: i32, bpp: u16, pixels: &[u8]) -> Vec<u8> {
    let mut data = vec![0u8; 54];
    // Magic 'BM'
    data[0] = b'B';
    data[1] = b'M';
    // Offset to pixel data
    data[10] = 54;
    // DIB header size
    data[14] = 40;
    
    // Width
    let w_bytes = width.to_le_bytes();
    data[18..22].copy_from_slice(&w_bytes);
    
    // Height
    let h_bytes = height.to_le_bytes();
    data[22..26].copy_from_slice(&h_bytes);
    
    // Planes (must be 1)
    data[26] = 1;
    
    // Bit count
    let b_bytes = bpp.to_le_bytes();
    data[28..30].copy_from_slice(&b_bytes);
    
    // Compression (0 = BI_RGB)
    data[30] = 0;
    
    // Append pixel data
    data.extend_from_slice(pixels);
    
    data
}

#[test]
fn test_bmp_decoder_24bit_with_padding() {
    // 2x2 image, 24-bit BGR
    // Row stride = ((2 * 24 + 31) / 32) * 4 = (79 / 32) * 4 = 2 * 4 = 8 bytes per row
    // Pixels (B, G, R):
    // Row 0: (10, 20, 30), (40, 50, 60), padding (0, 0)
    // Row 1: (70, 80, 90), (100, 110, 120), padding (0, 0)
    
    let mut pixels = vec![0u8; 16];
    // Row 0
    pixels[0] = 10; pixels[1] = 20; pixels[2] = 30; // Pixel (0, 0)
    pixels[3] = 40; pixels[4] = 50; pixels[5] = 60; // Pixel (1, 0)
    // padding at 6, 7 is 0
    
    // Row 1
    pixels[8] = 70; pixels[9] = 80; pixels[10] = 90; // Pixel (0, 1)
    pixels[11] = 100; pixels[12] = 110; pixels[13] = 120; // Pixel (1, 1)
    
    let data = create_valid_bmp(2, 2, 24, &pixels); // height > 0 means bottom-up
    
    let result = BmpImage::from_bytes(&data).expect("Poprawne zdekodowanie BMP");
    assert_eq!(result.width, 2);
    assert_eq!(result.height, 2);
    assert_eq!(result.data.len(), 4);
    
    // Bottom-up: Row 1 in BMP is Row 0 in our output (top row)
    // Top row (y=0) -> BMP Row 1 -> Pixel (0, 1), Pixel (1, 1)
    let p00 = result.data[0];
    assert_eq!(p00, (255 << 24) | (70 << 16) | (80 << 8) | 90); // a << 24 | b << 16 | g << 8 | r
    
    let p10 = result.data[1];
    assert_eq!(p10, (255 << 24) | (100 << 16) | (110 << 8) | 120);
    
    // Bottom row (y=1) -> BMP Row 0 -> Pixel (0, 0), Pixel (1, 0)
    let p01 = result.data[2];
    assert_eq!(p01, (255 << 24) | (10 << 16) | (20 << 8) | 30);
    
    let p11 = result.data[3];
    assert_eq!(p11, (255 << 24) | (40 << 16) | (50 << 8) | 60);
}

#[test]
fn test_bmp_decoder_32bit_top_down() {
    // 1x2 image, 32-bit BGRA
    // Row stride = ((1 * 32 + 31) / 32) * 4 = (63 / 32) * 4 = 1 * 4 = 4 bytes per row
    // Pixels (B, G, R, A):
    // Row 0: (10, 20, 30, 40)
    // Row 1: (50, 60, 70, 80)
    
    let mut pixels = vec![0u8; 8];
    // Row 0
    pixels[0] = 10; pixels[1] = 20; pixels[2] = 30; pixels[3] = 40;
    // Row 1
    pixels[4] = 50; pixels[5] = 60; pixels[6] = 70; pixels[7] = 80;
    
    // Negative height means top-down
    let data = create_valid_bmp(1, -2, 32, &pixels);
    
    let result = BmpImage::from_bytes(&data).expect("Poprawne zdekodowanie BMP");
    assert_eq!(result.width, 1);
    assert_eq!(result.height, 2);
    assert_eq!(result.data.len(), 2);
    
    // Top-down: Row 0 in BMP is Row 0 in our output
    let p00 = result.data[0];
    assert_eq!(p00, (40 << 24) | (10 << 16) | (20 << 8) | 30); // a << 24 | b << 16 | g << 8 | r
    
    let p01 = result.data[1];
    assert_eq!(p01, (80 << 24) | (50 << 16) | (60 << 8) | 70);
}

#[test]
fn test_bmp_decoder_truncated_pixels() {
    let pixels = vec![0u8; 4]; // Only 4 bytes, but 2x2 24-bit needs at least 8 bytes per row * 2 rows = 16 bytes
    let data = create_valid_bmp(2, 2, 24, &pixels);
    
    let result = BmpImage::from_bytes(&data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Dane pikseli sa uciete");
}
