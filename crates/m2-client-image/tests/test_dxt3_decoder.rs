use m2_client_image::dxt3_decoder::{decode_dxt3, decode_dxt3_block};

#[test]
fn test_decode_dxt3_block() {
    let mut block = [0u8; 16];
    
    // Alpha channel: First 8 bytes. Let's set some specific values.
    // Pixel 0 (byte 0, low nibble) = 0x5
    // Pixel 1 (byte 0, high nibble) = 0xA
    block[0] = 0xA5; 
    
    // RGB Palette: 8-15 bytes
    // Color 0: White (0xFFFF)
    block[8] = 0xFF;
    block[9] = 0xFF;
    // Color 1: Black (0x0000)
    block[10] = 0x00;
    block[11] = 0x00;
    
    // Indices: all 0, except pixel 1 is index 1
    block[12] = 0x04; // 0000 0100 (binary)
    
    let mut out = [0u8; 64];
    decode_dxt3_block(&block, &mut out);
    
    // Check pixel 0 (White, Alpha 0x55)
    assert_eq!(out[0], 255); // R
    assert_eq!(out[1], 255); // G
    assert_eq!(out[2], 255); // B
    assert_eq!(out[3], 0x55); // A
    
    // Check pixel 1 (Black, Alpha 0xAA)
    assert_eq!(out[4], 0); // R
    assert_eq!(out[5], 0); // G
    assert_eq!(out[6], 0); // B
    assert_eq!(out[7], 0xAA); // A
}

#[test]
fn test_decode_dxt3_invalid_size() {
    let data = [0u8; 10]; // Too small for even 1 block
    let res = decode_dxt3(&data, 4, 4);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Dane wejsciowe sa zbyt krotkie dla podanych wymiarow");
}

#[test]
fn test_decode_dxt3_zero_dimensions() {
    let data = [0u8; 16];
    let res = decode_dxt3(&data, 0, 0);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Nieprawidlowe wymiary obrazu");
}

#[test]
fn test_decode_dxt3_non_multiple_of_four() {
    // 5x5 image requires 2x2 blocks = 4 blocks = 64 bytes
    let data = [0u8; 64]; 
    let res = decode_dxt3(&data, 5, 5);
    
    assert!(res.is_ok());
    let out = res.unwrap();
    assert_eq!(out.len(), 5 * 5 * 4);
}
