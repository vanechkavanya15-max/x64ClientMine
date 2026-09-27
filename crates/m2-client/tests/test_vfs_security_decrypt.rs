use m2_client::vfs_security_decrypt::{
    compress_zstd, decompress_lzo, decompress_zstd, decrypt_tea, decrypt_xchacha20,
    encrypt_tea, encrypt_xchacha20, DecryptionEngine, SecurityKey, PACK_KEY, PACK_NONCE_SIZE,
};

#[test]
fn test_decryption_engine_new() {
    let engine = DecryptionEngine::new();
    let nonce = [0x42u8; PACK_NONCE_SIZE];
    let mut data = b"Hello World XChaCha20".to_vec();
    let original = data.clone();

    engine.encrypt_pack_entry(&mut data, &nonce);
    assert_ne!(data, original);

    engine.decrypt_pack_entry(&mut data, &nonce);
    assert_eq!(data, original);
}

#[test]
fn test_symmetric_tea_encryption_decryption() {
    let key: SecurityKey = [0x11223344, 0x55667788, 0x99AABBCC, 0xDDEEFF00];
    let original_data = b"Metin2PackTest12".to_vec(); // Length 16, exactly 2 blocks

    let mut data = original_data.clone();
    encrypt_tea(&mut data, &key);

    assert_ne!(data, original_data);

    decrypt_tea(&mut data, &key);
    assert_eq!(data, original_data);
}

#[test]
fn test_symmetric_tea_large_data() {
    let key: SecurityKey = [0x01020304, 0x05060708, 0x090A0B0C, 0x0D0E0F00];
    let original_data = vec![0xAB; 64];

    let mut data = original_data.clone();
    encrypt_tea(&mut data, &key);
    assert_ne!(data, original_data);

    decrypt_tea(&mut data, &key);
    assert_eq!(data, original_data);
}

#[test]
fn test_xchacha20_encryption_decryption() {
    let nonce = [0x37u8; PACK_NONCE_SIZE];
    let original_data = b"Metin2 XChaCha20 encrypted payload with 24-byte nonce and 32-byte key".to_vec();

    let mut data = original_data.clone();
    encrypt_xchacha20(&mut data, &nonce, &PACK_KEY);
    assert_ne!(data, original_data);

    decrypt_xchacha20(&mut data, &nonce, &PACK_KEY);
    assert_eq!(data, original_data);
}

#[test]
fn test_zstd_compression_decompression() {
    let original_data = b"This is some sample text data that will be compressed using ZSTD algorithm. Metin2 Client VFS uses it extensively.".to_vec();

    let compressed = compress_zstd(&original_data, 3).expect("ZSTD compression should succeed");
    assert!(!compressed.is_empty());

    let decompressed = decompress_zstd(&compressed, original_data.len()).expect("ZSTD decompression should succeed");
    assert_eq!(decompressed, original_data);
}

#[test]
fn test_zstd_too_short_expected_size() {
    let original_data = b"Short text to compress with ZSTD".to_vec();
    let compressed = compress_zstd(&original_data, 1).unwrap();

    let result = decompress_zstd(&compressed, original_data.len() - 10);
    assert!(result.is_err(), "Expected error for too short expected size, but got {:?}", result);
}

#[test]
fn test_zstd_invalid_data() {
    let invalid_data = vec![0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x11, 0x22];
    let result = decompress_zstd(&invalid_data, 100);
    assert!(result.is_err());
}

#[test]
fn test_lzo_invalid_data() {
    let invalid_lzo = vec![0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x11, 0x22];
    let result = decompress_lzo(&invalid_lzo, 100);
    assert!(result.is_err());
}

#[test]
fn test_lzo_raw_uncompressed_passthrough() {
    let raw = b"ExactMatchRawData".to_vec();
    let result = decompress_lzo(&raw, raw.len()).unwrap();
    assert_eq!(result, raw);
}

#[test]
fn test_decryption_engine_lzo_and_tea() {
    let engine = DecryptionEngine::new();

    // Test decompress_pack_data (ZSTD)
    let original_data = b"Sample payload for DecryptionEngine ZSTD pack data testing.".to_vec();
    let compressed = compress_zstd(&original_data, 1).unwrap();
    let decompressed = engine.decompress_pack_data(&compressed, original_data.len()).unwrap();
    assert_eq!(decompressed, original_data);

    // Test decompress_lzo_data
    let raw = b"PassthroughRawData12345".to_vec();
    let decomp_lzo = engine.decompress_lzo_data(&raw, raw.len()).unwrap();
    assert_eq!(decomp_lzo, raw);

    // Test TEA encryption and decryption via engine
    let key: SecurityKey = [0x01020304, 0x05060708, 0x090A0B0C, 0x0D0E0F10];
    let mut tea_data = b"BlockDataForTEA!".to_vec(); // 16 bytes
    let orig_tea = tea_data.clone();

    engine.encrypt_tea_data(&mut tea_data, &key);
    assert_ne!(tea_data, orig_tea);

    engine.decrypt_tea_data(&mut tea_data, &key);
    assert_eq!(tea_data, orig_tea);
}
