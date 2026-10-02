use m2_client::secure_cipher::SecureCipher;
use m2_client::secure_cipher_handshake::*;

extern "C" {
    fn crypto_kx_server_session_keys(
        rx: *mut u8,
        tx: *mut u8,
        server_pk: *const u8,
        server_sk: *const u8,
        client_pk: *const u8,
    ) -> i32;
    fn crypto_auth_verify(h: *const u8, m: *const u8, mlen: u64, k: *const u8) -> i32;
}

#[test]
fn test_successful_handshake() {
    let mut client_cipher = SecureCipher::new();
    let mut server_cipher = SecureCipher::new();
    
    server_cipher.initialize();
    client_cipher.initialize();
    
    let challenge = [0xAA; 32];
    
    let challenge_packet = TPacketGCKeyChallenge {
        header: 0x000B,
        length: std::mem::size_of::<TPacketGCKeyChallenge>() as u16,
        server_pk: server_cipher.pk,
        challenge,
        server_time: 12345,
    };
    
    let response_result = handle_key_challenge(&mut client_cipher, &challenge_packet);
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    
    let header = response.header;
    assert_eq!(header, 0x000A);
    assert_eq!(response.client_pk, client_cipher.pk);
    
    // Simulate server processing response to check if keys match
    unsafe {
        // Mocking server key calculation
        crypto_kx_server_session_keys(
            server_cipher.rx_key.as_mut_ptr(),
            server_cipher.tx_key.as_mut_ptr(),
            server_cipher.pk.as_ptr(),
            server_cipher.sk.as_ptr(),
            client_cipher.pk.as_ptr(),
        );
    }
    
    // verify auth
    let mut auth_verify = 0;
    unsafe {
        auth_verify = crypto_auth_verify(
            response.challenge_response.as_ptr(),
            challenge.as_ptr(),
            32,
            server_cipher.rx_key.as_ptr(),
        );
    }
    
    assert_eq!(auth_verify, 0, "Challenge response verification failed");
}

#[test]
fn test_handle_key_challenge_uninitialized() {
    let mut cipher = SecureCipher::new();
    assert!(!cipher.initialized);
    
    let packet = TPacketGCKeyChallenge {
        header: 0x000B,
        length: std::mem::size_of::<TPacketGCKeyChallenge>() as u16,
        server_pk: [1u8; 32],
        challenge: [2u8; 32],
        server_time: 1000,
    };
    
    let res = handle_key_challenge(&mut cipher, &packet);
    assert!(res.is_ok());
    assert!(cipher.initialized);
}

#[test]
fn test_handle_key_complete_uninitialized() {
    let mut cipher = SecureCipher::new();
    
    let packet = TPacketGCKeyComplete {
        header: 0x000C,
        length: std::mem::size_of::<TPacketGCKeyComplete>() as u16,
        encrypted_token: [0u8; 48],
        nonce: [0u8; 24],
    };
    
    let res = handle_key_complete(&mut cipher, &packet);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Failed to decrypt session token");
}

#[test]
fn test_handle_key_complete_invalid_token() {
    let mut cipher = SecureCipher::new();
    cipher.initialize();
    
    let packet = TPacketGCKeyComplete {
        header: 0x000C,
        length: std::mem::size_of::<TPacketGCKeyComplete>() as u16,
        encrypted_token: [0xFF; 48],
        nonce: [0xEE; 24],
    };
    
    let res = handle_key_complete(&mut cipher, &packet);
    assert!(res.is_err()); // Decryption should fail due to invalid token/keys
}

#[test]
fn test_struct_sizes() {
    assert_eq!(std::mem::size_of::<TPacketGCKeyChallenge>(), 72);
    assert_eq!(std::mem::size_of::<TPacketCGKeyResponse>(), 68);
    assert_eq!(std::mem::size_of::<TPacketGCKeyComplete>(), 76);
}
