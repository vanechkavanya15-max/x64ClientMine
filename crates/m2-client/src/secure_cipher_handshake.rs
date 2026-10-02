use crate::secure_cipher::SecureCipher;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCKeyChallenge {
    pub header: u16,
    pub length: u16,
    pub server_pk: [u8; 32],
    pub challenge: [u8; 32],
    pub server_time: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGKeyResponse {
    pub header: u16,
    pub length: u16,
    pub client_pk: [u8; 32],
    pub challenge_response: [u8; 32],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCKeyComplete {
    pub header: u16,
    pub length: u16,
    pub encrypted_token: [u8; 48], // 32 + 16 tag
    pub nonce: [u8; 24],
}

pub fn handle_key_challenge(
    cipher: &mut SecureCipher,
    packet: &TPacketGCKeyChallenge,
) -> Result<TPacketCGKeyResponse, String> {
    if !cipher.initialized {
        if !cipher.initialize() {
            return Err("Failed to initialize SecureCipher".to_string());
        }
    }

    if !cipher.compute_client_keys(&packet.server_pk) {
        return Err("Failed to compute client session keys".to_string());
    }

    let challenge_response = cipher.compute_challenge_response(&packet.challenge);

    Ok(TPacketCGKeyResponse {
        header: 0x000A, // CG::KEY_RESPONSE
        length: std::mem::size_of::<TPacketCGKeyResponse>() as u16,
        client_pk: cipher.pk,
        challenge_response,
    })
}

pub fn handle_key_complete(
    cipher: &mut SecureCipher,
    packet: &TPacketGCKeyComplete,
) -> Result<(), String> {
    if !cipher.decrypt_token(&packet.encrypted_token, &packet.nonce) {
        return Err("Failed to decrypt session token".to_string());
    }

    cipher.set_activated(true);
    Ok(())
}
