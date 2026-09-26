use m2_client::send_target_fly_v2::{m2_send_target, m2_send_fly_target};
use std::ptr;

#[test]
fn test_m2_send_target_success() {
    let mut out_buf = [0u8; 8];
    let vid: u32 = 12345;
    
    let size = m2_send_target(vid, out_buf.as_mut_ptr());
    
    assert_eq!(size, 8);
    
    // header: 0x0A01
    assert_eq!(out_buf[0], 0x01);
    assert_eq!(out_buf[1], 0x0A);
    // length: 8
    assert_eq!(out_buf[2], 0x08);
    assert_eq!(out_buf[3], 0x00);
    // vid: 12345 (0x3039)
    assert_eq!(out_buf[4], 0x39);
    assert_eq!(out_buf[5], 0x30);
    assert_eq!(out_buf[6], 0x00);
    assert_eq!(out_buf[7], 0x00);
}

#[test]
fn test_m2_send_target_null_buffer() {
    let vid: u32 = 12345;
    
    let size = m2_send_target(vid, ptr::null_mut());
    
    assert_eq!(size, 0);
}

#[test]
fn test_m2_send_target_vid_max() {
    let mut out_buf = [0u8; 8];
    let vid: u32 = u32::MAX;
    
    let size = m2_send_target(vid, out_buf.as_mut_ptr());
    
    assert_eq!(size, 8);
    
    // header: 0x0A01
    assert_eq!(out_buf[0], 0x01);
    assert_eq!(out_buf[1], 0x0A);
    // length: 8
    assert_eq!(out_buf[2], 0x08);
    assert_eq!(out_buf[3], 0x00);
    // vid: 0xFFFFFFFF
    assert_eq!(out_buf[4], 0xFF);
    assert_eq!(out_buf[5], 0xFF);
    assert_eq!(out_buf[6], 0xFF);
    assert_eq!(out_buf[7], 0xFF);
}

#[test]
fn test_m2_send_fly_target_success() {
    let mut out_buf = [0u8; 16];
    let vid: u32 = 98765; // 0x0181CD
    let x: i32 = 100000;  // 0x0186A0
    let y: i32 = -50000;  // 0xFFFF3CB0
    
    let size = m2_send_fly_target(vid, x, y, out_buf.as_mut_ptr());
    
    assert_eq!(size, 16);
    
    // header: 0x0404
    assert_eq!(out_buf[0], 0x04);
    assert_eq!(out_buf[1], 0x04);
    // length: 16
    assert_eq!(out_buf[2], 0x10);
    assert_eq!(out_buf[3], 0x00);
    // vid: 98765 (0x0181CD)
    assert_eq!(out_buf[4], 0xCD);
    assert_eq!(out_buf[5], 0x81);
    assert_eq!(out_buf[6], 0x01);
    assert_eq!(out_buf[7], 0x00);
    // x: 100000 (0x0186A0)
    assert_eq!(out_buf[8], 0xA0);
    assert_eq!(out_buf[9], 0x86);
    assert_eq!(out_buf[10], 0x01);
    assert_eq!(out_buf[11], 0x00);
    // y: -50000 (0xFFFF3CB0)
    assert_eq!(out_buf[12], 0xB0);
    assert_eq!(out_buf[13], 0x3C);
    assert_eq!(out_buf[14], 0xFF);
    assert_eq!(out_buf[15], 0xFF);
}

#[test]
fn test_m2_send_fly_target_null_buffer() {
    let vid: u32 = 98765;
    let x: i32 = 100000;
    let y: i32 = -50000;
    
    let size = m2_send_fly_target(vid, x, y, ptr::null_mut());
    
    assert_eq!(size, 0);
}

#[test]
fn test_m2_send_fly_target_zero_coords() {
    let mut out_buf = [0u8; 16];
    let vid: u32 = 1;
    let x: i32 = 0;
    let y: i32 = 0;
    
    let size = m2_send_fly_target(vid, x, y, out_buf.as_mut_ptr());
    
    assert_eq!(size, 16);
    
    // header: 0x0404
    assert_eq!(out_buf[0], 0x04);
    assert_eq!(out_buf[1], 0x04);
    // length: 16
    assert_eq!(out_buf[2], 0x10);
    assert_eq!(out_buf[3], 0x00);
    // vid: 1
    assert_eq!(out_buf[4], 0x01);
    assert_eq!(out_buf[5], 0x00);
    assert_eq!(out_buf[6], 0x00);
    assert_eq!(out_buf[7], 0x00);
    // x: 0
    assert_eq!(out_buf[8], 0x00);
    assert_eq!(out_buf[9], 0x00);
    assert_eq!(out_buf[10], 0x00);
    assert_eq!(out_buf[11], 0x00);
    // y: 0
    assert_eq!(out_buf[12], 0x00);
    assert_eq!(out_buf[13], 0x00);
    assert_eq!(out_buf[14], 0x00);
    assert_eq!(out_buf[15], 0x00);
}

#[test]
fn test_m2_send_fly_target_large_coords() {
    let mut out_buf = [0u8; 16];
    let vid: u32 = u32::MAX;
    let x: i32 = i32::MAX;
    let y: i32 = i32::MIN;
    
    let size = m2_send_fly_target(vid, x, y, out_buf.as_mut_ptr());
    
    assert_eq!(size, 16);
    
    // header: 0x0404
    assert_eq!(out_buf[0], 0x04);
    assert_eq!(out_buf[1], 0x04);
    // length: 16
    assert_eq!(out_buf[2], 0x10);
    assert_eq!(out_buf[3], 0x00);
    // vid: u32::MAX
    assert_eq!(out_buf[4], 0xFF);
    assert_eq!(out_buf[5], 0xFF);
    assert_eq!(out_buf[6], 0xFF);
    assert_eq!(out_buf[7], 0xFF);
    // x: i32::MAX (0x7FFFFFFF)
    assert_eq!(out_buf[8], 0xFF);
    assert_eq!(out_buf[9], 0xFF);
    assert_eq!(out_buf[10], 0xFF);
    assert_eq!(out_buf[11], 0x7F);
    // y: i32::MIN (0x80000000)
    assert_eq!(out_buf[12], 0x00);
    assert_eq!(out_buf[13], 0x00);
    assert_eq!(out_buf[14], 0x00);
    assert_eq!(out_buf[15], 0x80);
}
