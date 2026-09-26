use m2_client::color_key_mask::{apply_color_key, apply_color_key_tolerance};

#[test]
fn test_exact_match_magenta_color_key() {
    // Magenta: RGB(255, 0, 255)
    let mut rgba_buf = [
        255, 0, 255, 255, // Pixel 1: Magenta, Alpha 255 (should become Alpha 0)
        100, 100, 100, 255, // Pixel 2: Gray, Alpha 255 (should remain 255)
        255, 0, 255, 128, // Pixel 3: Magenta, Alpha 128 (should become Alpha 0)
        0, 0, 0, 0,       // Pixel 4: Black, Alpha 0 (should become Alpha 255)
    ];

    apply_color_key(&mut rgba_buf, 255, 0, 255);

    // Assert Pixel 1
    assert_eq!(rgba_buf[0..4], [255, 0, 255, 0]);
    // Assert Pixel 2
    assert_eq!(rgba_buf[4..8], [100, 100, 100, 255]);
    // Assert Pixel 3
    assert_eq!(rgba_buf[8..12], [255, 0, 255, 0]);
    // Assert Pixel 4
    assert_eq!(rgba_buf[12..16], [0, 0, 0, 255]);
}

#[test]
fn test_color_key_with_tolerance() {
    // We are looking for roughly (100, 150, 200) with tolerance 10
    let mut rgba_buf = [
        100, 150, 200, 255, // Exact match, Alpha 255 -> 0
        110, 140, 210, 255, // Edge match (+10, -10, +10), Alpha 255 -> 0
        111, 150, 200, 255, // Outside match (+11 on R), Alpha 255 -> 255
        90, 160, 190, 255,  // Edge match (-10, +10, -10), Alpha 255 -> 0
        89, 150, 200, 255,  // Outside match (-11 on R), Alpha 255 -> 255
    ];

    apply_color_key_tolerance(&mut rgba_buf, 100, 150, 200, 10);

    // Pixel 1: exact
    assert_eq!(rgba_buf[0..4], [100, 150, 200, 0]);
    // Pixel 2: edge match
    assert_eq!(rgba_buf[4..8], [110, 140, 210, 0]);
    // Pixel 3: outside
    assert_eq!(rgba_buf[8..12], [111, 150, 200, 255]);
    // Pixel 4: edge match
    assert_eq!(rgba_buf[12..16], [90, 160, 190, 0]);
    // Pixel 5: outside
    assert_eq!(rgba_buf[16..20], [89, 150, 200, 255]);
}

#[test]
fn test_safe_handling_incomplete_pixels() {
    let mut rgba_buf = [
        255, 0, 255, 255, // Pixel 1: Magenta
        255, 0, 255,      // Incomplete Pixel 2 (only 3 bytes)
    ];

    apply_color_key(&mut rgba_buf, 255, 0, 255);

    // The first pixel should be processed.
    assert_eq!(rgba_buf[0..4], [255, 0, 255, 0]);

    // The incomplete pixel should remain untouched.
    assert_eq!(rgba_buf[4..7], [255, 0, 255]);
}

#[test]
fn test_alpha_always_set_to_255_when_not_matching() {
    let mut rgba_buf = [
        10, 20, 30, 0, // Not matching, alpha 0
        10, 20, 30, 50, // Not matching, alpha 50
    ];

    apply_color_key(&mut rgba_buf, 255, 0, 255);

    // Should set alpha to 255 for non-matching pixels
    assert_eq!(rgba_buf[0..4], [10, 20, 30, 255]);
    assert_eq!(rgba_buf[4..8], [10, 20, 30, 255]);
}

#[test]
fn test_empty_buffer() {
    let mut rgba_buf: [u8; 0] = [];
    apply_color_key(&mut rgba_buf, 255, 0, 255);
    // Should not panic, buffer remains empty.
    assert_eq!(rgba_buf.len(), 0);
}
