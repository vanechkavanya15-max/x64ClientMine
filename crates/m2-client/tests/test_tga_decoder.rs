#[path = "../src/tga_decoder.rs"]
mod tga_decoder;

use tga_decoder::TgaImage;

fn create_header(
    id_len: u8,
    colormap_type: u8,
    image_type: u8,
    width: u16,
    height: u16,
    bpp: u8,
    descriptor: u8,
) -> Vec<u8> {
    let mut header = vec![0; 18];
    header[0] = id_len;
    header[1] = colormap_type;
    header[2] = image_type;
    
    // colormap spec: 3..8 (all 0 for now)
    
    // x_origin, y_origin
    header[8] = 0; header[9] = 0;
    header[10] = 0; header[11] = 0;
    
    let w_bytes = width.to_le_bytes();
    header[12] = w_bytes[0];
    header[13] = w_bytes[1];
    
    let h_bytes = height.to_le_bytes();
    header[14] = h_bytes[0];
    header[15] = h_bytes[1];
    
    header[16] = bpp;
    header[17] = descriptor;
    
    header
}

#[test]
fn test_invalid_header_size() {
    let data = vec![0; 10];
    let result = TgaImage::from_bytes(&data);
    assert_eq!(result.err(), Some("TGA header is too small"));
}

#[test]
fn test_invalid_dimensions() {
    let header = create_header(0, 0, 2, 0, 10, 24, 0);
    let result = TgaImage::from_bytes(&header);
    assert_eq!(result.err(), Some("Invalid image dimensions"));
    
    let header = create_header(0, 0, 2, 10, 0, 24, 0);
    let result = TgaImage::from_bytes(&header);
    assert_eq!(result.err(), Some("Invalid image dimensions"));
}

#[test]
fn test_unsupported_formats() {
    // Wrong image type
    let header = create_header(0, 0, 3, 10, 10, 24, 0);
    let result = TgaImage::from_bytes(&header);
    assert_eq!(result.err(), Some("Unsupported TGA image type (only 2 and 10 are supported)"));
    
    // Wrong bpp
    let header = create_header(0, 0, 2, 10, 10, 16, 0);
    let result = TgaImage::from_bytes(&header);
    assert_eq!(result.err(), Some("Unsupported bpp (only 24 and 32 are supported)"));
}

#[test]
fn test_uncompressed_24bit() {
    let mut data = create_header(0, 0, 2, 2, 2, 24, 0x20); // Top-to-bottom
    
    // Pixel 1: Blue (B=255, G=0, R=0) -> output RGBA: 0, 0, 255, 255
    data.extend_from_slice(&[255, 0, 0]);
    // Pixel 2: Green (B=0, G=255, R=0) -> output RGBA: 0, 255, 0, 255
    data.extend_from_slice(&[0, 255, 0]);
    // Pixel 3: Red (B=0, G=0, R=255) -> output RGBA: 255, 0, 0, 255
    data.extend_from_slice(&[0, 0, 255]);
    // Pixel 4: White (B=255, G=255, R=255) -> output RGBA: 255, 255, 255, 255
    data.extend_from_slice(&[255, 255, 255]);
    
    let img = TgaImage::from_bytes(&data).unwrap();
    assert_eq!(img.width, 2);
    assert_eq!(img.height, 2);
    assert_eq!(img.rgba_data.len(), 16);
    
    assert_eq!(&img.rgba_data[0..4], &[0, 0, 255, 255]);
    assert_eq!(&img.rgba_data[4..8], &[0, 255, 0, 255]);
    assert_eq!(&img.rgba_data[8..12], &[255, 0, 0, 255]);
    assert_eq!(&img.rgba_data[12..16], &[255, 255, 255, 255]);
}

#[test]
fn test_uncompressed_32bit() {
    let mut data = create_header(0, 0, 2, 2, 1, 32, 0x20); // Top-to-bottom
    
    // Pixel 1: B=10, G=20, R=30, A=40 -> output RGBA: 30, 20, 10, 40
    data.extend_from_slice(&[10, 20, 30, 40]);
    // Pixel 2: B=50, G=60, R=70, A=80 -> output RGBA: 70, 60, 50, 80
    data.extend_from_slice(&[50, 60, 70, 80]);
    
    let img = TgaImage::from_bytes(&data).unwrap();
    assert_eq!(img.width, 2);
    assert_eq!(img.height, 1);
    
    assert_eq!(&img.rgba_data[0..4], &[30, 20, 10, 40]);
    assert_eq!(&img.rgba_data[4..8], &[70, 60, 50, 80]);
}

#[test]
fn test_rle_compressed_24bit() {
    let mut data = create_header(0, 0, 10, 4, 1, 24, 0x20); // Top-to-bottom
    
    // RLE packet: repeat next pixel 3 times. Packet header = 0x80 | 0x02 = 0x82
    data.push(0x82);
    data.extend_from_slice(&[100, 150, 200]); // B, G, R
    
    // Raw packet: 1 pixel. Packet header = 0x00
    data.push(0x00);
    data.extend_from_slice(&[10, 20, 30]); // B, G, R
    
    let img = TgaImage::from_bytes(&data).unwrap();
    assert_eq!(img.width, 4);
    assert_eq!(img.height, 1);
    
    // 3 times R=200, G=150, B=100, A=255
    for i in 0..3 {
        let idx = i * 4;
        assert_eq!(&img.rgba_data[idx..idx+4], &[200, 150, 100, 255]);
    }
    // 1 time R=30, G=20, B=10, A=255
    assert_eq!(&img.rgba_data[12..16], &[30, 20, 10, 255]);
}

#[test]
fn test_rle_compressed_32bit() {
    let mut data = create_header(0, 0, 10, 3, 1, 32, 0x20); // Top-to-bottom
    
    // Raw packet: 2 pixels. Packet header = 0x01
    data.push(0x01);
    data.extend_from_slice(&[10, 20, 30, 40]); // B, G, R, A
    data.extend_from_slice(&[50, 60, 70, 80]);
    
    // RLE packet: repeat next pixel 1 time. Packet header = 0x80
    data.push(0x80);
    data.extend_from_slice(&[90, 100, 110, 120]);
    
    let img = TgaImage::from_bytes(&data).unwrap();
    assert_eq!(img.width, 3);
    assert_eq!(img.height, 1);
    
    assert_eq!(&img.rgba_data[0..4], &[30, 20, 10, 40]);
    assert_eq!(&img.rgba_data[4..8], &[70, 60, 50, 80]);
    assert_eq!(&img.rgba_data[8..12], &[110, 100, 90, 120]);
}

#[test]
fn test_orientation_flip() {
    // Descriptor bit 5 = 0 means bottom-to-top.
    let mut data = create_header(0, 0, 2, 2, 2, 24, 0x00);
    
    // Original data:
    // Row 0 (bottom in TGA): Red, Green
    data.extend_from_slice(&[0, 0, 255]); // Red
    data.extend_from_slice(&[0, 255, 0]); // Green
    
    // Row 1 (top in TGA): Blue, White
    data.extend_from_slice(&[255, 0, 0]); // Blue
    data.extend_from_slice(&[255, 255, 255]); // White
    
    let img = TgaImage::from_bytes(&data).unwrap();
    
    // In RGBA layout, we want top row first, so Row 1 then Row 0.
    // Expected output:
    // Row 0: Blue, White
    assert_eq!(&img.rgba_data[0..4], &[0, 0, 255, 255]); // Blue
    assert_eq!(&img.rgba_data[4..8], &[255, 255, 255, 255]); // White
    // Row 1: Red, Green
    assert_eq!(&img.rgba_data[8..12], &[255, 0, 0, 255]); // Red
    assert_eq!(&img.rgba_data[12..16], &[0, 255, 0, 255]); // Green
}

#[test]
fn test_id_field_and_colormap_skip() {
    let mut header = create_header(5, 1, 2, 1, 1, 24, 0x20); // Top-to-bottom
    // colormap start/length:
    header[5] = 2; // length = 2 entries
    header[6] = 0;
    header[7] = 24; // 24-bit per entry = 3 bytes -> total 6 bytes colormap
    
    // ID field (5 bytes)
    header.extend_from_slice(&[1, 2, 3, 4, 5]);
    
    // Colormap (6 bytes)
    header.extend_from_slice(&[9, 9, 9, 9, 9, 9]);
    
    // Pixel (3 bytes) B=255, G=0, R=0
    header.extend_from_slice(&[255, 0, 0]);
    
    let img = TgaImage::from_bytes(&header).unwrap();
    assert_eq!(img.width, 1);
    assert_eq!(img.height, 1);
    assert_eq!(&img.rgba_data, &[0, 0, 255, 255]);
}

#[test]
fn test_unexpected_eof() {
    let data = create_header(0, 0, 2, 2, 2, 24, 0x20);
    // Missing pixels entirely
    let result = TgaImage::from_bytes(&data);
    assert_eq!(result.err(), Some("Unexpected EOF while reading uncompressed pixels"));
    
    let mut rle_data = create_header(0, 0, 10, 4, 1, 24, 0x20);
    // Missing packet completely
    let result = TgaImage::from_bytes(&rle_data);
    assert_eq!(result.err(), Some("Unexpected EOF while reading RLE pixels"));
    
    // RLE packet header but no pixel data
    rle_data.push(0x82);
    let result = TgaImage::from_bytes(&rle_data);
    assert_eq!(result.err(), Some("Unexpected EOF while reading RLE pixel"));
    
    let mut rle_data_raw = create_header(0, 0, 10, 4, 1, 24, 0x20);
    // Raw packet header but not enough pixels
    rle_data_raw.push(0x02); // 3 pixels expected
    rle_data_raw.extend_from_slice(&[0, 0, 0, 0, 0, 0]); // only 2 pixels provided
    let result = TgaImage::from_bytes(&rle_data_raw);
    assert_eq!(result.err(), Some("Unexpected EOF while reading RAW pixels"));
}

#[test]
fn test_rle_exceeds_bounds() {
    let mut rle_data = create_header(0, 0, 10, 2, 1, 24, 0x20);
    // Ask for 5 pixels, but image is only 2 pixels wide
    rle_data.push(0x84); 
    rle_data.extend_from_slice(&[255, 0, 0]);
    let result = TgaImage::from_bytes(&rle_data);
    assert_eq!(result.err(), Some("RLE packet exceeds image boundaries"));
}
