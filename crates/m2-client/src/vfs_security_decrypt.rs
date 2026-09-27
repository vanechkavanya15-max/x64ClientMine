// vfs_security_decrypt.rs
use chacha20::XChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher};

pub type SecurityKey = [u32; 4];

pub const PACK_KEY_SIZE: usize = 32;
pub const PACK_NONCE_SIZE: usize = 24;

/// 256-bitowy klucz szyfrowania archiwow PCK (XChaCha20)
pub const PACK_KEY: [u8; PACK_KEY_SIZE] = [
    0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
    0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
    0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF,
    0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32, 0x10,
];

pub struct DecryptionEngine;

impl DecryptionEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn decrypt_pack_entry(&self, data: &mut [u8], nonce: &[u8; PACK_NONCE_SIZE]) {
        decrypt_xchacha20(data, nonce, &PACK_KEY);
    }

    pub fn encrypt_pack_entry(&self, data: &mut [u8], nonce: &[u8; PACK_NONCE_SIZE]) {
        encrypt_xchacha20(data, nonce, &PACK_KEY);
    }

    pub fn decompress_pack_data(&self, compressed: &[u8], expected_size: usize) -> Result<Vec<u8>, String> {
        decompress_zstd(compressed, expected_size)
    }

    pub fn decompress_lzo_data(&self, compressed: &[u8], expected_size: usize) -> Result<Vec<u8>, String> {
        decompress_lzo(compressed, expected_size)
    }

    pub fn decrypt_tea_data(&self, data: &mut [u8], key: &SecurityKey) {
        decrypt_tea(data, key);
    }

    pub fn encrypt_tea_data(&self, data: &mut [u8], key: &SecurityKey) {
        encrypt_tea(data, key);
    }
}

/// Szyfrowanie / deszyfrowanie strumieniowe XChaCha20 dla pakietow i plikow PCK
pub fn decrypt_xchacha20(data: &mut [u8], nonce: &[u8; PACK_NONCE_SIZE], key: &[u8; PACK_KEY_SIZE]) {
    let mut cipher = XChaCha20::new(key.into(), nonce.into());
    cipher.apply_keystream(data);
}

pub fn encrypt_xchacha20(data: &mut [u8], nonce: &[u8; PACK_NONCE_SIZE], key: &[u8; PACK_KEY_SIZE]) {
    let mut cipher = XChaCha20::new(key.into(), nonce.into());
    cipher.apply_keystream(data);
}

/// Dekompresja bufora z uzyciem ZSTD (zstd-rs)
pub fn decompress_zstd(compressed_data: &[u8], expected_size: usize) -> Result<Vec<u8>, String> {
    if compressed_data.is_empty() {
        return Ok(Vec::new());
    }
    zstd::bulk::decompress(compressed_data, expected_size)
        .map_err(|e| format!("ZSTD decompression failed: {}", e))
}

/// Kompresja bufora z uzyciem ZSTD (zstd-rs)
pub fn compress_zstd(data: &[u8], level: i32) -> Result<Vec<u8>, String> {
    if data.is_empty() {
        return Ok(Vec::new());
    }
    zstd::bulk::compress(data, level)
        .map_err(|e| format!("ZSTD compression failed: {}", e))
}

pub fn decrypt_tea(data: &mut [u8], key: &SecurityKey) {
    let chunks = data.chunks_exact_mut(8);
    for chunk in chunks {
        let mut y = u32::from_le_bytes(chunk[0..4].try_into().unwrap());
        let mut z = u32::from_le_bytes(chunk[4..8].try_into().unwrap());
        let mut sum: u32 = 0xC6EF3720;
        let delta: u32 = 0x9e3779b9;

        for _ in 0..32 {
            z = z.wrapping_sub(
                ((y << 4).wrapping_add(key[2]))
                    ^ (y.wrapping_add(sum))
                    ^ ((y >> 5).wrapping_add(key[3]))
            );
            y = y.wrapping_sub(
                ((z << 4).wrapping_add(key[0]))
                    ^ (z.wrapping_add(sum))
                    ^ ((z >> 5).wrapping_add(key[1]))
            );
            sum = sum.wrapping_sub(delta);
        }

        chunk[0..4].copy_from_slice(&y.to_le_bytes());
        chunk[4..8].copy_from_slice(&z.to_le_bytes());
    }
}

pub fn encrypt_tea(data: &mut [u8], key: &SecurityKey) {
    let chunks = data.chunks_exact_mut(8);
    for chunk in chunks {
        let mut y = u32::from_le_bytes(chunk[0..4].try_into().unwrap());
        let mut z = u32::from_le_bytes(chunk[4..8].try_into().unwrap());
        let mut sum: u32 = 0;
        let delta: u32 = 0x9e3779b9;

        for _ in 0..32 {
            sum = sum.wrapping_add(delta);
            y = y.wrapping_add(
                ((z << 4).wrapping_add(key[0]))
                    ^ (z.wrapping_add(sum))
                    ^ ((z >> 5).wrapping_add(key[1]))
            );
            z = z.wrapping_add(
                ((y << 4).wrapping_add(key[2]))
                    ^ (y.wrapping_add(sum))
                    ^ ((y >> 5).wrapping_add(key[3]))
            );
        }

        chunk[0..4].copy_from_slice(&y.to_le_bytes());
        chunk[4..8].copy_from_slice(&z.to_le_bytes());
    }
}

/// Bezpieczna implementacja dekompresji LZO1X w czystym Rust bez zaleznosci C
pub fn decompress_lzo(compressed_data: &[u8], expected_size: usize) -> Result<Vec<u8>, String> {
    if compressed_data.is_empty() {
        return Ok(Vec::new());
    }

    if compressed_data.len() >= expected_size && &compressed_data[..expected_size] == compressed_data {
        let mut out = Vec::with_capacity(expected_size);
        out.extend_from_slice(&compressed_data[..expected_size]);
        return Ok(out);
    }

    let mut out = Vec::with_capacity(expected_size);
    let mut in_pos = 0;

    while in_pos < compressed_data.len() {
        if out.len() > expected_size {
            return Err("Decompressed size exceeds expected size".to_string());
        }

        let b = compressed_data[in_pos];
        in_pos += 1;

        if b > 17 {
            let len = (b - 17) as usize;
            if in_pos + len > compressed_data.len() {
                return Err("Unexpected EOF in LZO stream".to_string());
            }
            if out.len() + len > expected_size {
                return Err("Output buffer overflow".to_string());
            }
            out.extend_from_slice(&compressed_data[in_pos..in_pos + len]);
            in_pos += len;
        } else if b < 16 {
            let len = (b + 3) as usize;
            if in_pos >= compressed_data.len() {
                return Err("Unexpected EOF in LZO stream".to_string());
            }
            let offset = ((compressed_data[in_pos] as usize) << 2) | ((b as usize) & 3);
            in_pos += 1;
            if offset == 0 || offset > out.len() {
                return Err("Invalid LZO offset".to_string());
            }
            if out.len() + len > expected_size {
                return Err("Output buffer overflow".to_string());
            }
            for _ in 0..len {
                let byte = out[out.len() - offset];
                out.push(byte);
            }
        } else {
            return Err("Unsupported LZO opcode".to_string());
        }

        if out.len() == expected_size {
            break;
        }
    }

    if out.len() != expected_size {
        return Err(format!("LZO decompression size mismatch: got {} bytes, expected {}", out.len(), expected_size));
    }

    Ok(out)
}
