use m2_client_bridge::vfs::panama::{PanamaCipher, m2_vfs_panama_decrypt};

#[test]
fn test_panama_cipher_determinism() {
    let mut cipher1 = PanamaCipher::new();
    let mut cipher2 = PanamaCipher::new();
    
    let key = [0xAA; 32];
    let iv = [0x55; 32];
    
    cipher1.set_key_with_iv(&key, &iv);
    cipher2.set_key_with_iv(&key, &iv);
    
    let mut data1 = [0u8; 64];
    let mut data2 = [0u8; 64];
    
    for i in 0..64 {
        data1[i] = i as u8;
        data2[i] = i as u8;
    }
    
    cipher1.process_data(&mut data1);
    cipher2.process_data(&mut data2);
    
    assert_eq!(data1, data2, "Szyfrowanie Panama powinno byc deterministyczne");
}

#[test]
fn test_panama_cipher_reversibility() {
    let mut cipher_enc = PanamaCipher::new();
    let mut cipher_dec = PanamaCipher::new();
    
    let key = [0x12; 32];
    let iv = [0x34; 32];
    
    cipher_enc.set_key_with_iv(&key, &iv);
    cipher_dec.set_key_with_iv(&key, &iv);
    
    let original_data: [u8; 100] = {
        let mut data = [0u8; 100];
        for i in 0..100 {
            data[i] = (i * 3) as u8;
        }
        data
    };
    
    let mut data = original_data.clone();
    
    // Encrypt
    cipher_enc.process_data(&mut data);
    
    // Decrypt (same operation as encrypt for stream ciphers like Panama)
    cipher_dec.process_data(&mut data);
    
    assert_eq!(data, original_data, "Deszyfrowanie powinno przywrocic oryginalne dane");
}

#[test]
fn test_m2_vfs_panama_decrypt_ffi() {
    let mut data = [0u8; 64];
    for i in 0..64 {
        data[i] = i as u8;
    }
    
    let original = data.clone();
    let key = [0xAA; 32];
    
    let mut cipher_enc = PanamaCipher::new();
    cipher_enc.set_key_with_iv(&key, &key);
    cipher_enc.process_data(&mut data);
    
    assert_ne!(data, original);
    
    unsafe {
        let result = m2_vfs_panama_decrypt(data.as_mut_ptr(), data.len() as u32, key.as_ptr());
        assert!(result);
    }
    
    assert_eq!(data, original);
}
