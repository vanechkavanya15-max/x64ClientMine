use m2_client::image_ffi::{m2_image_decode_dxt, m2_image_parse_sub, m2_image_apply_colorkey};
use std::ffi::CString;
use std::ptr;

#[test]
fn test_m2_image_decode_dxt_null_pointers() {
    let mut out_buf = vec![0u8; 16 * 4];
    let data = vec![0u8; 8];

    // Null data
    assert_eq!(
        m2_image_decode_dxt(1, ptr::null(), 8, 4, 4, out_buf.as_mut_ptr()),
        false
    );

    // Null out_buf
    assert_eq!(
        m2_image_decode_dxt(1, data.as_ptr(), 8, 4, 4, ptr::null_mut()),
        false
    );

    // Invalid dimensions
    assert_eq!(
        m2_image_decode_dxt(1, data.as_ptr(), 8, 0, 4, out_buf.as_mut_ptr()),
        false
    );
}

#[test]
fn test_m2_image_decode_dxt1() {
    // 4x4 DXT1 block, one color is red, one is green
    // color 0: RGB565 red (0xF800)
    // color 1: RGB565 green (0x07E0)
    // indices: all 0 (color 0, red)
    let dxt1_data: [u8; 8] = [
        0x00, 0xF8, // c0
        0xE0, 0x07, // c1
        0x00, 0x00, 0x00, 0x00, // code
    ];

    let mut out_buf = vec![0u8; 4 * 4 * 4];

    assert_eq!(
        m2_image_decode_dxt(
            1,
            dxt1_data.as_ptr(),
            dxt1_data.len(),
            4,
            4,
            out_buf.as_mut_ptr()
        ),
        true
    );

    // Ensure out_buf[0..4] is red (255, 0, 0, 255) based on RGB565 to RGB math
    let r = (((0xF800 >> 11) & 0x1F) as u32 * 255 / 31) as u8;
    assert_eq!(out_buf[0], r);
    assert_eq!(out_buf[1], 0);
    assert_eq!(out_buf[2], 0);
    assert_eq!(out_buf[3], 255);
}

#[test]
fn test_m2_image_parse_sub_null_pointers() {
    let mut left = 0;
    let mut top = 0;
    let mut right = 0;
    let mut bottom = 0;

    assert_eq!(
        m2_image_parse_sub(
            ptr::null(),
            &mut left,
            &mut top,
            &mut right,
            &mut bottom
        ),
        false
    );
}

#[test]
fn test_m2_image_parse_sub_valid() {
    let sub_script = "
        title \"Test Image\"
        version 1.0
        left 10
        top 20
        right 30
        bottom 40
    ";
    let c_str = CString::new(sub_script).unwrap();

    let mut left = 0;
    let mut top = 0;
    let mut right = 0;
    let mut bottom = 0;

    assert_eq!(
        m2_image_parse_sub(
            c_str.as_ptr(),
            &mut left,
            &mut top,
            &mut right,
            &mut bottom
        ),
        true
    );

    assert_eq!(left, 10);
    assert_eq!(top, 20);
    assert_eq!(right, 30);
    assert_eq!(bottom, 40);
}

#[test]
fn test_m2_image_parse_sub_invalid() {
    let sub_script = "
        title \"Test Image\"
        left 10
        top 20
        right 30
        # bottom is missing
    ";
    let c_str = CString::new(sub_script).unwrap();

    let mut left = 0;
    let mut top = 0;
    let mut right = 0;
    let mut bottom = 0;

    assert_eq!(
        m2_image_parse_sub(
            c_str.as_ptr(),
            &mut left,
            &mut top,
            &mut right,
            &mut bottom
        ),
        false
    );
}

#[test]
fn test_m2_image_apply_colorkey() {
    // 2 pixels (8 bytes)
    // Pixel 1: 255, 0, 255 (magenta)
    // Pixel 2: 0, 255, 0 (green)
    let mut rgba_buf: [u8; 8] = [
        255, 0, 255, 255, // Pixel 1
        0, 255, 0, 255,   // Pixel 2
    ];

    // Colorkey is Magenta
    assert_eq!(
        m2_image_apply_colorkey(rgba_buf.as_mut_ptr(), rgba_buf.len(), 255, 0, 255),
        true
    );

    // Pixel 1 should become transparent (alpha = 0)
    assert_eq!(rgba_buf[0..4], [255, 0, 255, 0]);
    // Pixel 2 should stay opaque (alpha = 255)
    assert_eq!(rgba_buf[4..8], [0, 255, 0, 255]);
}

#[test]
fn test_m2_image_apply_colorkey_null() {
    assert_eq!(
        m2_image_apply_colorkey(ptr::null_mut(), 8, 255, 0, 255),
        false
    );
}
