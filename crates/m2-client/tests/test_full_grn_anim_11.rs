#[path = "../src/granny_bitknit_decompressor.rs"]
mod granny_bitknit_decompressor;

use granny_bitknit_decompressor::*;

#[test]
fn test_bitknit_literal_init() {
    let lit = BitknitLiteral::new();
    assert_eq!(lit.freq[0], 1);
    assert_eq!(lit.a[300], 0x8000);
}

#[test]
fn test_bitknit_distance_init() {
    let dist_lsb = BitknitDistanceLsb::new();
    assert_eq!(dist_lsb.freq[0], 1);
    assert_eq!(dist_lsb.a[40], 0x8000);

    let dist_bits = BitknitDistanceBits::new();
    assert_eq!(dist_bits.freq[0], 1);
    assert_eq!(dist_bits.a[21], 0x8000);
}

#[test]
fn test_bitknit_state_init() {
    let state = BitknitState::new();
    assert_eq!(state.last_match_dist, 1);
    assert_eq!(state.recent_dist[0], 1);
    assert_eq!(state.recent_dist[7], 1);
}

#[test]
fn test_decode_buffer_too_small() {
    let src = [0u8; 3];
    let mut dst = [0u8; 100];
    assert_eq!(bitknit_decode(&src, &mut dst), None);
}

#[test]
fn test_decode_invalid_header_v() {
    // If v < 0x10000, decode should return Some(0) according to logic.
    let src = [0x00, 0x00, 0x00, 0x00];
    let mut dst = [0u8; 100];
    assert_eq!(bitknit_decode(&src, &mut dst), Some(0));
}

#[test]
fn test_decode_truncation() {
    // A header that claims bits requires reading more, but buffer is truncated
    let v: u32 = 0x10000;
    let src = v.to_le_bytes();
    let mut dst = [0u8; 100];
    // Since src length is exactly 4, it should return None because it needs to read more bytes for `a`
    assert_eq!(bitknit_decode(&src, &mut dst), None);
}
