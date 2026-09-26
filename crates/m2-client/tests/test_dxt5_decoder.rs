use m2_client::dxt5_decoder::decode_dxt5;

#[test]
fn test_invalid_dimensions() {
    let data = [0u8; 16];
    
    // Width/height 0
    assert!(decode_dxt5(&data, 0, 4).is_err());
    assert!(decode_dxt5(&data, 4, 0).is_err());

    // Not multiple of 4
    assert!(decode_dxt5(&data, 3, 4).is_err());
    assert!(decode_dxt5(&data, 4, 2).is_err());
    assert!(decode_dxt5(&data, 5, 5).is_err());

    // Buffer too small
    assert!(decode_dxt5(&data, 8, 4).is_err()); // Requires 32 bytes
}

#[test]
fn test_alpha_interpolation_8_step() {
    // alpha0 = 200, alpha1 = 60
    // a[0] = 200, a[1] = 60
    // a[2] = (6*200 + 1*60)/7 = 180
    // a[3] = (5*200 + 2*60)/7 = 160
    // a[4] = (4*200 + 3*60)/7 = 140
    // a[5] = (3*200 + 4*60)/7 = 120
    // a[6] = (2*200 + 5*60)/7 = 100
    // a[7] = (1*200 + 6*60)/7 = 80

    let mut data = [0u8; 16];
    data[0] = 200;
    data[1] = 60;
    
    // Let's set indices so that pixel 0 uses a[0], pixel 1 uses a[1] ... pixel 7 uses a[7]
    // 48 bits = 6 bytes
    // index 0: 000
    // index 1: 001
    // index 2: 010
    // index 3: 011
    // index 4: 100
    // index 5: 101
    // index 6: 110
    // index 7: 111
    
    // px0 (3 bits): 0 (0)
    // px1 (3 bits): 1 (1)
    // px2 (3 bits): 2 (2)
    // px3 (3 bits): 3 (3)
    // px4 (3 bits): 4 (4)
    // px5 (3 bits): 5 (5)
    // px6 (3 bits): 6 (6)
    // px7 (3 bits): 7 (7)
    // px8-15: 0
    
    // Indices in little endian, starting from LSB of byte 2:
    // byte 2: px0(0-2) | px1(3-5) | px2(6-7) = 000 | 001 << 3 | 010 << 6 = 0x08 | 0x80 = 0x88
    // wait, px2 is 010 (binary). Bottom 2 bits are 10. So byte 2 is 0x08 | (0x02 << 6) = 0x88
    // Let's just construct it as u64 and then convert to bytes
    
    let mut indices: u64 = 0;
    for i in 0..8 {
        indices |= (i as u64) << (i * 3);
    }
    // Remaining pixels use index 0.

    data[2] = (indices & 0xFF) as u8;
    data[3] = ((indices >> 8) & 0xFF) as u8;
    data[4] = ((indices >> 16) & 0xFF) as u8;
    data[5] = ((indices >> 24) & 0xFF) as u8;
    data[6] = ((indices >> 32) & 0xFF) as u8;
    data[7] = ((indices >> 40) & 0xFF) as u8;

    // Color block: color0 > color1 or color0 <= color1 doesn't matter much for this test, we just want to check alpha
    data[8] = 0; data[9] = 0;
    data[10] = 0; data[11] = 0;
    
    let res = decode_dxt5(&data, 4, 4).unwrap();
    
    assert_eq!(res[0*4 + 3], 200);
    assert_eq!(res[1*4 + 3], 60);
    assert_eq!(res[2*4 + 3], 180);
    assert_eq!(res[3*4 + 3], 160);
    assert_eq!(res[4*4 + 3], 140);
    assert_eq!(res[5*4 + 3], 120);
    assert_eq!(res[6*4 + 3], 100);
    assert_eq!(res[7*4 + 3], 80);
    
    // Pixel 8-15 use index 0
    assert_eq!(res[8*4 + 3], 200);
}

#[test]
fn test_alpha_interpolation_6_step() {
    // alpha0 = 50, alpha1 = 200
    // a[0] = 50, a[1] = 200
    // a[2] = (4*50 + 1*200)/5 = 80
    // a[3] = (3*50 + 2*200)/5 = 110
    // a[4] = (2*50 + 3*200)/5 = 140
    // a[5] = (1*50 + 4*200)/5 = 170
    // a[6] = 0
    // a[7] = 255

    let mut data = [0u8; 16];
    data[0] = 50;
    data[1] = 200;
    
    let mut indices: u64 = 0;
    for i in 0..8 {
        indices |= (i as u64) << (i * 3);
    }
    
    data[2] = (indices & 0xFF) as u8;
    data[3] = ((indices >> 8) & 0xFF) as u8;
    data[4] = ((indices >> 16) & 0xFF) as u8;
    data[5] = ((indices >> 24) & 0xFF) as u8;
    data[6] = ((indices >> 32) & 0xFF) as u8;
    data[7] = ((indices >> 40) & 0xFF) as u8;

    let res = decode_dxt5(&data, 4, 4).unwrap();
    
    assert_eq!(res[0*4 + 3], 50);
    assert_eq!(res[1*4 + 3], 200);
    assert_eq!(res[2*4 + 3], 80);
    assert_eq!(res[3*4 + 3], 110);
    assert_eq!(res[4*4 + 3], 140);
    assert_eq!(res[5*4 + 3], 170);
    assert_eq!(res[6*4 + 3], 0);
    assert_eq!(res[7*4 + 3], 255);
}

#[test]
fn test_full_color_and_alpha() {
    let mut data = [0u8; 16];
    
    // Alpha
    data[0] = 200; data[1] = 60; // 8-step
    data[2] = 0; data[3] = 0; data[4] = 0; data[5] = 0; data[6] = 0; data[7] = 0; // all index 0 -> a[0] = 200
    
    // Color
    // RGB565: 
    // R: 5 bits, G: 6 bits, B: 5 bits
    // color0 = red
    // R = 31, G = 0, B = 0 -> 0xF800
    // color1 = blue
    // R = 0, G = 0, B = 31 -> 0x001F
    let color0: u16 = 0xF800; // > 0x001F, so 4-color interpolation
    let color1: u16 = 0x001F;
    
    data[8] = (color0 & 0xFF) as u8;
    data[9] = ((color0 >> 8) & 0xFF) as u8;
    data[10] = (color1 & 0xFF) as u8;
    data[11] = ((color1 >> 8) & 0xFF) as u8;
    
    // color indices: all 0
    data[12] = 0; data[13] = 0; data[14] = 0; data[15] = 0;
    
    let res = decode_dxt5(&data, 4, 4).unwrap();
    
    // r0 = (31 << 3) | (31 >> 2) = 248 | 7 = 255
    // g0 = 0
    // b0 = 0
    // So pixel 0 should be RGBA(255, 0, 0, 200)
    for i in 0..16 {
        assert_eq!(res[i*4], 255);
        assert_eq!(res[i*4+1], 0);
        assert_eq!(res[i*4+2], 0);
        assert_eq!(res[i*4+3], 200);
    }

    // Test with color1 > color0 (3-color + transparent)
    let color0: u16 = 0x001F; // blue
    let color1: u16 = 0xF800; // red
    data[8] = (color0 & 0xFF) as u8;
    data[9] = ((color0 >> 8) & 0xFF) as u8;
    data[10] = (color1 & 0xFF) as u8;
    data[11] = ((color1 >> 8) & 0xFF) as u8;

    // Set pixel 0 to index 3 (transparent color)
    // Index 3 is binary 11.
    // 16 pixels, 2 bits per pixel.
    // Pixel 0 is bits 0-1.
    data[12] = 0x03; // pixel 0 = index 3, pixel 1..3 = index 0
    data[13] = 0; data[14] = 0; data[15] = 0;

    let res = decode_dxt5(&data, 4, 4).unwrap();
    
    // Note that the alpha from DXT5 alpha block is still applied!
    // The "transparent" color index in 3-color mode only sets RGB to 0 in standard DXT1,
    // but in DXT5, alpha comes from the separate block.
    // So RGB should be 0, Alpha should be 200.
    assert_eq!(res[0*4], 0);
    assert_eq!(res[0*4+1], 0);
    assert_eq!(res[0*4+2], 0);
    assert_eq!(res[0*4+3], 200);

    // Pixel 1 uses index 0 (blue)
    // b0 = (31 << 3) | (31 >> 2) = 255
    assert_eq!(res[1*4], 0);
    assert_eq!(res[1*4+1], 0);
    assert_eq!(res[1*4+2], 255);
    assert_eq!(res[1*4+3], 200);
}
