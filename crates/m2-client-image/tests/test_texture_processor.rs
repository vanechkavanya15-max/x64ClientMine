use m2_client_image::texture_processor::TextureProcessor;

#[test]
fn test_integration_texture_processor() {
    let instance = TextureProcessor::new();
    assert!(instance.is_ready());
}

#[test]
fn test_apply_color_key_rgba() {
    // RGBA: 255, 0, 255 (color key), 255 (alpha)
    let mut data = vec![
        255, 0, 255, 255, // matches color key
        255, 255, 255, 255, // white, should not match
        255, 0, 0, 255, // red, should not match
        255, 0, 255, 128, // matches color key, alpha should become 0
    ];

    TextureProcessor::apply_color_key_rgba(&mut data);

    assert_eq!(data[0..4], [255, 0, 255, 0]);
    assert_eq!(data[4..8], [255, 255, 255, 255]);
    assert_eq!(data[8..12], [255, 0, 0, 255]);
    assert_eq!(data[12..16], [255, 0, 255, 0]);
}

#[test]
fn test_apply_color_key_bgra() {
    // BGRA: B, G, R, A (color key R=255, G=0, B=255)
    let mut data = vec![
        255, 0, 255, 255, // matches color key
        255, 255, 255, 255, // white, should not match
        0, 0, 255, 255, // red (in BGRA: B=0, G=0, R=255), should not match
        255, 0, 255, 128, // matches color key
    ];

    TextureProcessor::apply_color_key_bgra(&mut data);

    assert_eq!(data[0..4], [255, 0, 255, 0]);
    assert_eq!(data[4..8], [255, 255, 255, 255]);
    assert_eq!(data[8..12], [0, 0, 255, 255]);
    assert_eq!(data[12..16], [255, 0, 255, 0]);
}

#[test]
fn test_generate_mipmap_rgba_2x2() {
    let width = 2;
    let height = 2;
    let data = vec![
        10, 20, 30, 40,
        50, 60, 70, 80,
        90, 100, 110, 120,
        130, 140, 150, 160,
    ];

    let result = TextureProcessor::generate_mipmap_rgba(width, height, &data);
    assert!(result.is_some());
    let (new_w, new_h, new_data) = result.unwrap();
    
    assert_eq!(new_w, 1);
    assert_eq!(new_h, 1);
    assert_eq!(new_data.len(), 4);

    let expected_r = (10 + 50 + 90 + 130) / 4;
    let expected_g = (20 + 60 + 100 + 140) / 4;
    let expected_b = (30 + 70 + 110 + 150) / 4;
    let expected_a = (40 + 80 + 120 + 160) / 4;

    assert_eq!(new_data[0], expected_r as u8);
    assert_eq!(new_data[1], expected_g as u8);
    assert_eq!(new_data[2], expected_b as u8);
    assert_eq!(new_data[3], expected_a as u8);
}

#[test]
fn test_generate_mipmap_rgba_odd_size() {
    let width = 3;
    let height = 3;
    // 3x3 = 9 pixels = 36 bytes
    let mut data = vec![0; 36];
    
    // Fill first 2x2 with specific values
    // Row 0
    data[0..4].copy_from_slice(&[10, 10, 10, 10]);
    data[4..8].copy_from_slice(&[30, 30, 30, 30]);
    // Row 1
    data[12..16].copy_from_slice(&[50, 50, 50, 50]);
    data[16..20].copy_from_slice(&[70, 70, 70, 70]);

    let result = TextureProcessor::generate_mipmap_rgba(width, height, &data);
    assert!(result.is_some());
    let (new_w, new_h, new_data) = result.unwrap();
    
    assert_eq!(new_w, 1);
    assert_eq!(new_h, 1);
    
    let expected = (10 + 30 + 50 + 70) / 4;
    assert_eq!(new_data[0], expected);
    assert_eq!(new_data[1], expected);
    assert_eq!(new_data[2], expected);
    assert_eq!(new_data[3], expected);
}

#[test]
fn test_generate_mipmap_rgba_1x1() {
    let width = 1;
    let height = 1;
    let data = vec![100, 150, 200, 250];

    let result = TextureProcessor::generate_mipmap_rgba(width, height, &data);
    assert!(result.is_some());
    let (new_w, new_h, new_data) = result.unwrap();
    
    assert_eq!(new_w, 1);
    assert_eq!(new_h, 1);
    assert_eq!(new_data, vec![100, 150, 200, 250]);
}

#[test]
fn test_generate_mipmap_invalid() {
    let width = 2;
    let height = 2;
    let data = vec![0; 10]; // Too small for 2x2x4 (16 bytes)

    let result = TextureProcessor::generate_mipmap_rgba(width, height, &data);
    assert!(result.is_none());

    let result2 = TextureProcessor::generate_mipmap_rgba(0, 0, &vec![]);
    assert!(result2.is_none());
}
