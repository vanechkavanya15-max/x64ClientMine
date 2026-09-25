use m2_client_image::image_ffi::{m2_image_decode_dxt, m2_image_decode_tga, m2_image_free};
use std::ptr;

#[test]
fn test_m2_image_decode_dxt_valid() {
    let input = vec![1, 2, 3, 4];
    let img_ptr = unsafe { m2_image_decode_dxt(input.as_ptr(), input.len(), 2, 2) };
    assert!(!img_ptr.is_null());
    
    let img = unsafe { &*img_ptr };
    assert_eq!(img.width, 2);
    assert_eq!(img.height, 2);
    assert_eq!(img.data_len, 16);
    assert!(!img.data.is_null());
    
    unsafe { m2_image_free(img_ptr) };
}

#[test]
fn test_m2_image_decode_dxt_invalid() {
    let input = vec![1, 2, 3, 4];
    let img_ptr = unsafe { m2_image_decode_dxt(ptr::null(), input.len(), 2, 2) };
    assert!(img_ptr.is_null());
    
    let img_ptr = unsafe { m2_image_decode_dxt(input.as_ptr(), 0, 2, 2) };
    assert!(img_ptr.is_null());
    
    let img_ptr = unsafe { m2_image_decode_dxt(input.as_ptr(), input.len(), 0, 2) };
    assert!(img_ptr.is_null());
}

#[test]
fn test_m2_image_decode_tga_valid() {
    let input = vec![1, 2, 3, 4];
    let img_ptr = unsafe { m2_image_decode_tga(input.as_ptr(), input.len()) };
    assert!(!img_ptr.is_null());
    
    let img = unsafe { &*img_ptr };
    assert_eq!(img.width, 1);
    assert_eq!(img.height, 1);
    assert_eq!(img.data_len, 4);
    assert!(!img.data.is_null());
    
    unsafe { m2_image_free(img_ptr) };
}

#[test]
fn test_m2_image_decode_tga_invalid() {
    let input = vec![1, 2, 3, 4];
    let img_ptr = unsafe { m2_image_decode_tga(ptr::null(), input.len()) };
    assert!(img_ptr.is_null());
    
    let img_ptr = unsafe { m2_image_decode_tga(input.as_ptr(), 0) };
    assert!(img_ptr.is_null());
}

#[test]
fn test_m2_image_free_null() {
    // Upewnij sie, ze zwolnienie null pointera nie powoduje paniki
    unsafe { m2_image_free(ptr::null_mut()) };
}
