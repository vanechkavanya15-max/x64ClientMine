#[path = "../src/dxt3_decoder.rs"]
mod dxt3_decoder;

use dxt3_decoder::decode_dxt3;

#[test]
fn test_input_size_verification() {
    // Width and height not > 0
    assert_eq!(
        decode_dxt3(&[], 0, 4),
        Err("Width and height must be greater than 0")
    );
    assert_eq!(
        decode_dxt3(&[], 4, 0),
        Err("Width and height must be greater than 0")
    );

    // Width and height not multiples of 4
    assert_eq!(
        decode_dxt3(&[], 3, 4),
        Err("Width and height must be multiples of 4")
    );
    assert_eq!(
        decode_dxt3(&[], 4, 2),
        Err("Width and height must be multiples of 4")
    );

    // Insufficient data size
    // 4x4 requires 1 block = 16 bytes
    let data = vec![0u8; 15];
    assert_eq!(
        decode_dxt3(&data, 4, 4),
        Err("Invalid data size for DXT3")
    );

    // Oversized data size
    let data = vec![0u8; 17];
    assert_eq!(
        decode_dxt3(&data, 4, 4),
        Err("Invalid data size for DXT3")
    );
}

#[test]
fn test_unpacking_alpha() {
    // 1 block: 4x4
    let mut data = vec![0u8; 16];
    
    // First 8 bytes are alpha
    // 0x10 -> a0=0, a1=1 => (0<<4|0=0, 1<<4|1=17)
    // 0x2F -> a0=15, a1=2 => (15<<4|15=255, 2<<4|2=34)
    data[0] = 0x10;
    data[1] = 0x2F;
    
    let result = decode_dxt3(&data, 4, 4).unwrap();
    
    // Pixel 0 (a0 from byte 0) -> alpha is 0
    assert_eq!(result[3], 0);
    // Pixel 1 (a1 from byte 0) -> alpha is 17
    assert_eq!(result[7], 17);
    
    // Pixel 2 (a0 from byte 1) -> alpha is 255
    assert_eq!(result[11], 255);
    // Pixel 3 (a1 from byte 1) -> alpha is 34
    assert_eq!(result[15], 34);
}

#[test]
fn test_full_4x4_block_decoding() {
    // 4x4 block
    let mut data = vec![0u8; 16];
    
    // Alpha: all F (255)
    for i in 0..8 {
        data[i] = 0xFF;
    }
    
    // c0 = 0xFFFF (White: R=31, G=63, B=31 -> R=255, G=255, B=255)
    data[8] = 0xFF;
    data[9] = 0xFF;
    
    // c1 = 0x0000 (Black: R=0, G=0, B=0)
    data[10] = 0x00;
    data[11] = 0x00;
    
    // Color indices:
    // colors[0] = White
    // colors[1] = Black
    // colors[2] = (2*White + Black) / 3 -> (170, 170, 170)
    // colors[3] = (White + 2*Black) / 3 -> (85, 85, 85)
    
    // Set indices for first row (4 pixels):
    // px0=0 (White), px1=1 (Black), px2=2 (Gray), px3=3 (Dark Gray)
    // byte = 00 | 01 | 10 | 11 (in binary, reversed for little endian)
    // 11 10 01 00 = 0xE4
    data[12] = 0xE4; // first row
    data[13] = 0xE4; // second row
    data[14] = 0xE4; // third row
    data[15] = 0xE4; // fourth row
    
    let result = decode_dxt3(&data, 4, 4).unwrap();
    
    // Checking first row:
    // px 0: color 0 -> White
    assert_eq!(&result[0..4], &[255, 255, 255, 255]);
    // px 1: color 1 -> Black
    assert_eq!(&result[4..8], &[0, 0, 0, 255]);
    // px 2: color 2 -> Gray (255*2/3 = 170)
    assert_eq!(&result[8..12], &[170, 170, 170, 255]);
    // px 3: color 3 -> Dark Gray (255/3 = 85)
    assert_eq!(&result[12..16], &[85, 85, 85, 255]);
}

#[test]
fn test_8x8_full_image() {
    let mut data = vec![0u8; 16 * 4]; // 4 blocks for 8x8
    
    // Block 0 (top-left)
    for i in 0..8 { data[i] = 0x11; } // Alpha = 17
    data[8] = 0xFF; data[9] = 0xFF; // c0 = white
    data[10] = 0xFF; data[11] = 0xFF; // c1 = white
    for i in 12..16 { data[i] = 0x00; } // indices = 0
    
    // Block 1 (top-right)
    for i in 0..8 { data[16 + i] = 0x22; } // Alpha = 34
    data[16 + 8] = 0x00; data[16 + 9] = 0x00; // c0 = black
    data[16 + 10] = 0x00; data[16 + 11] = 0x00; // c1 = black
    for i in 12..16 { data[16 + i] = 0x55; } // indices = 1 (black)
    
    // Block 2 (bottom-left)
    for i in 0..8 { data[32 + i] = 0x33; } // Alpha = 51
    data[32 + 8] = 0xFF; data[32 + 9] = 0xFF; // c0 = white
    data[32 + 10] = 0x00; data[32 + 11] = 0x00; // c1 = black
    for i in 12..16 { data[32 + i] = 0xAA; } // indices = 2 (gray 170)
    
    // Block 3 (bottom-right)
    for i in 0..8 { data[48 + i] = 0x44; } // Alpha = 68
    data[48 + 8] = 0xFF; data[48 + 9] = 0xFF; // c0 = white
    data[48 + 10] = 0x00; data[48 + 11] = 0x00; // c1 = black
    for i in 12..16 { data[48 + i] = 0xFF; } // indices = 3 (dark gray 85)
    
    let result = decode_dxt3(&data, 8, 8).unwrap();
    
    // Check pixel at (0, 0) -> Block 0
    // out_idx = (0 * 8 + 0) * 4 = 0
    assert_eq!(&result[0..4], &[255, 255, 255, 17]);
    
    // Check pixel at (4, 0) -> Block 1
    // out_idx = (0 * 8 + 4) * 4 = 16
    assert_eq!(&result[16..20], &[0, 0, 0, 34]);
    
    // Check pixel at (0, 4) -> Block 2
    // out_idx = (4 * 8 + 0) * 4 = 128
    assert_eq!(&result[128..132], &[170, 170, 170, 51]);
    
    // Check pixel at (4, 4) -> Block 3
    // out_idx = (4 * 8 + 4) * 4 = 144
    assert_eq!(&result[144..148], &[85, 85, 85, 68]);
}
