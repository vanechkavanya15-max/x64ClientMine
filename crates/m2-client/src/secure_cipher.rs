// secure_cipher.rs - Pelna implementacja SecureCipher (X25519 + HMAC + XChaCha20-Poly1305) zgodna 1:1 z C++
// Komentarze w jezyku polskim bez polskich znakow diakrytycznych.

pub const PK_SIZE: usize = 32;
pub const SK_SIZE: usize = 32;
pub const KEY_SIZE: usize = 32;
pub const NONCE_SIZE: usize = 24;
pub const TAG_SIZE: usize = 16;
pub const CHALLENGE_SIZE: usize = 32;
pub const SESSION_TOKEN_SIZE: usize = 32;

extern "C" {
    fn sodium_init() -> i32;
    fn crypto_kx_keypair(pk: *mut u8, sk: *mut u8) -> i32;
    fn crypto_kx_client_session_keys(
        rx: *mut u8,
        tx: *mut u8,
        client_pk: *const u8,
        client_sk: *const u8,
        server_pk: *const u8,
    ) -> i32;
    fn crypto_kx_server_session_keys(
        rx: *mut u8,
        tx: *mut u8,
        server_pk: *const u8,
        server_sk: *const u8,
        client_pk: *const u8,
    ) -> i32;
    fn crypto_auth(out: *mut u8, m: *const u8, mlen: u64, k: *const u8) -> i32;
    fn crypto_auth_verify(h: *const u8, m: *const u8, mlen: u64, k: *const u8) -> i32;
    fn crypto_aead_xchacha20poly1305_ietf_decrypt(
        m: *mut u8,
        mlen: *mut u64,
        nsec: *mut u8,
        c: *const u8,
        clen: u64,
        ad: *const u8,
        adlen: u64,
        npub: *const u8,
        k: *const u8,
    ) -> i32;
    fn crypto_stream_xchacha20_xor_ic(
        c: *mut u8,
        m: *const u8,
        mlen: u64,
        n: *const u8,
        ic: u64,
        k: *const u8,
    ) -> i32;
}

#[cfg(target_env = "gnu")]
#[no_mangle]
pub static mut __security_cookie: usize = 0x2B992DDFA232;

#[cfg(all(target_env = "gnu", target_arch = "x86_64"))]
core::arch::global_asm!(
    ".globl __chkstk",
    "__chkstk:",
    "jmp ___chkstk_ms",
    "",
    ".globl __security_check_cookie",
    "__security_check_cookie:",
    "ret",
    "",
    ".globl __GSHandlerCheck",
    "__GSHandlerCheck:",
    "ret"
);

#[derive(Debug, Clone)]
pub struct SecureCipher {
    pub initialized: bool,
    pub activated: bool,
    pub pk: [u8; PK_SIZE],
    pub sk: [u8; SK_SIZE],
    pub tx_key: [u8; KEY_SIZE],
    pub rx_key: [u8; KEY_SIZE],
    pub tx_stream_nonce: [u8; NONCE_SIZE],
    pub rx_stream_nonce: [u8; NONCE_SIZE],
    pub tx_nonce: u64,
    pub rx_nonce: u64,
    pub session_token: [u8; SESSION_TOKEN_SIZE],
}

impl Default for SecureCipher {
    fn default() -> Self {
        Self::new()
    }
}

impl SecureCipher {
    pub fn new() -> Self {
        Self {
            initialized: false,
            activated: false,
            pk: [0u8; PK_SIZE],
            sk: [0u8; SK_SIZE],
            tx_key: [0u8; KEY_SIZE],
            rx_key: [0u8; KEY_SIZE],
            tx_stream_nonce: [0u8; NONCE_SIZE],
            rx_stream_nonce: [0u8; NONCE_SIZE],
            tx_nonce: 0,
            rx_nonce: 0,
            session_token: [0u8; SESSION_TOKEN_SIZE],
        }
    }

    pub fn initialize(&mut self) -> bool {
        unsafe {
            if sodium_init() < 0 {
                return false;
            }
            if crypto_kx_keypair(self.pk.as_mut_ptr(), self.sk.as_mut_ptr()) != 0 {
                return false;
            }
        }
        self.tx_nonce = 0;
        self.rx_nonce = 0;
        self.initialized = true;
        self.activated = false;
        true
    }

    pub fn compute_client_keys(&mut self, server_pk: &[u8; 32]) -> bool {
        if !self.initialized {
            return false;
        }
        unsafe {
            if crypto_kx_client_session_keys(
                self.rx_key.as_mut_ptr(),
                self.tx_key.as_mut_ptr(),
                self.pk.as_ptr(),
                self.sk.as_ptr(),
                server_pk.as_ptr(),
            ) != 0 {
                return false;
            }
        }
        self.tx_stream_nonce = [0u8; NONCE_SIZE];
        self.tx_stream_nonce[0] = 0x02; // client -> server
        self.rx_stream_nonce = [0u8; NONCE_SIZE];
        self.rx_stream_nonce[0] = 0x01; // server -> client
        true
    }

    pub fn compute_challenge_response(&self, challenge: &[u8; 32]) -> [u8; 32] {
        let mut resp = [0u8; 32];
        unsafe {
            crypto_auth(resp.as_mut_ptr(), challenge.as_ptr(), 32, self.tx_key.as_ptr());
        }
        resp
    }

    pub fn decrypt_token(&mut self, ciphertext: &[u8], nonce: &[u8; 24]) -> bool {
        if !self.initialized {
            return false;
        }
        let mut decrypted_len: u64 = 0;
        let mut plaintext = [0u8; SESSION_TOKEN_SIZE];
        unsafe {
            let res = crypto_aead_xchacha20poly1305_ietf_decrypt(
                plaintext.as_mut_ptr(),
                &mut decrypted_len,
                std::ptr::null_mut(),
                ciphertext.as_ptr(),
                ciphertext.len() as u64,
                std::ptr::null(),
                0,
                nonce.as_ptr(),
                self.rx_key.as_ptr(),
            );
            if res != 0 {
                return false;
            }
        }
        self.session_token = plaintext;
        true
    }

    pub fn set_activated(&mut self, val: bool) {
        self.activated = val;
    }

    pub fn is_activated(&self) -> bool {
        self.activated
    }

    pub fn apply_stream_cipher(
        buf: &mut [u8],
        key: &[u8; 32],
        byte_counter: &mut u64,
        stream_nonce: &[u8; 24],
    ) {
        let offset = (*byte_counter % 64) as usize;
        let mut len = buf.len();
        let mut p = 0;

        if offset != 0 && len > 0 {
            let mut ks = [0u8; 64];
            unsafe {
                crypto_stream_xchacha20_xor_ic(
                    ks.as_mut_ptr(),
                    ks.as_ptr(),
                    64,
                    stream_nonce.as_ptr(),
                    *byte_counter / 64,
                    key.as_ptr(),
                );
            }
            let use_len = len.min(64 - offset);
            for i in 0..use_len {
                buf[p + i] ^= ks[offset + i];
            }
            p += use_len;
            len -= use_len;
            *byte_counter += use_len as u64;
        }

        if len > 0 {
            unsafe {
                crypto_stream_xchacha20_xor_ic(
                    buf[p..].as_mut_ptr(),
                    buf[p..].as_ptr(),
                    len as u64,
                    stream_nonce.as_ptr(),
                    *byte_counter / 64,
                    key.as_ptr(),
                );
            }
            *byte_counter += len as u64;
        }
    }

    pub fn encrypt_in_place(&mut self, buf: &mut [u8]) {
        if !self.activated || buf.is_empty() {
            return;
        }
        Self::apply_stream_cipher(buf, &self.tx_key, &mut self.tx_nonce, &self.tx_stream_nonce);
    }

    pub fn decrypt_in_place(&mut self, buf: &mut [u8]) {
        if !self.activated || buf.is_empty() {
            return;
        }
        Self::apply_stream_cipher(buf, &self.rx_key, &mut self.rx_nonce, &self.rx_stream_nonce);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_cipher_roundtrip() {
        let mut client = SecureCipher::new();
        assert!(client.initialize());

        let mut server = SecureCipher::new();
        assert!(server.initialize());

        // Client computes keys using server's public key
        assert!(client.compute_client_keys(&server.pk));

        // Server computes keys using client's public key
        unsafe {
            assert_eq!(
                crypto_kx_server_session_keys(
                    server.rx_key.as_mut_ptr(),
                    server.tx_key.as_mut_ptr(),
                    server.pk.as_ptr(),
                    server.sk.as_ptr(),
                    client.pk.as_ptr(),
                ),
                0
            );
        }
        server.tx_stream_nonce = [0u8; NONCE_SIZE];
        server.tx_stream_nonce[0] = 0x01; // server -> client
        server.rx_stream_nonce = [0u8; NONCE_SIZE];
        server.rx_stream_nonce[0] = 0x02; // client -> server

        // Key verification: client.tx_key must match server.rx_key
        assert_eq!(client.tx_key, server.rx_key);
        // client.rx_key must match server.tx_key
        assert_eq!(client.rx_key, server.tx_key);

        // Challenge-response verification
        let challenge = [42u8; 32];
        let resp = client.compute_challenge_response(&challenge);
        unsafe {
            assert_eq!(
                crypto_auth_verify(resp.as_ptr(), challenge.as_ptr(), 32, server.rx_key.as_ptr()),
                0
            );
        }

        // Stream encryption verification
        client.set_activated(true);
        server.set_activated(true);

        let mut msg = b"Metin2 Pure Rust to Server Packet Test".to_vec();
        let original = msg.clone();

        client.encrypt_in_place(&mut msg);
        assert_ne!(msg, original);

        SecureCipher::apply_stream_cipher(&mut msg, &server.rx_key, &mut server.rx_nonce, &server.rx_stream_nonce);
        assert_eq!(msg, original);
    }

    #[test]
    fn test_live_auth_handshake_and_login() {
        use std::io::{Read, Write};
        let target_addr = "127.0.0.1:11000";
        let mut stream = match std::net::TcpStream::connect_timeout(&target_addr.parse().unwrap(), std::time::Duration::from_secs(2)) {
            Ok(s) => s,
            Err(_) => return, // Jesli serwer jest wylaczony w srodowisku CI, pomin
        };
        let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(4)));
        let _ = stream.set_write_timeout(Some(std::time::Duration::from_secs(4)));

        let mut buf = [0u8; 1024];
        let mut total_read = 0;
        while total_read < 77 {
            let n = match stream.read(&mut buf[total_read..]) {
                Ok(n) if n > 0 => n,
                _ => return, // Timeout lub zerwane polaczenie w srodowisku testowym
            };
            total_read += n;
        }

        // GC::PHASE: 0x0008, len: 5
        let phase_hdr = u16::from_le_bytes(buf[0..2].try_into().unwrap());
        assert_eq!(phase_hdr, 0x0008);

        // GC::KEY_CHALLENGE: 0x000B, len: 72
        let challenge_hdr = u16::from_le_bytes(buf[5..7].try_into().unwrap());
        assert_eq!(challenge_hdr, 0x000B);

        let mut server_pk = [0u8; 32];
        server_pk.copy_from_slice(&buf[9..41]);
        let mut challenge = [0u8; 32];
        challenge.copy_from_slice(&buf[41..73]);

        let mut cipher = SecureCipher::new();
        assert!(cipher.initialize());
        assert!(cipher.compute_client_keys(&server_pk));
        let challenge_resp = cipher.compute_challenge_response(&challenge);

        // Send CG::KEY_RESPONSE (68 bytes)
        let mut key_resp = [0u8; 68];
        key_resp[0..2].copy_from_slice(&0x000Au16.to_le_bytes()); // CG::KEY_RESPONSE
        key_resp[2..4].copy_from_slice(&68u16.to_le_bytes());
        key_resp[4..36].copy_from_slice(&cipher.pk);
        key_resp[36..68].copy_from_slice(&challenge_resp);
        stream.write_all(&key_resp).expect("Blad zapisu KEY_RESPONSE");

        // Receive GC::KEY_COMPLETE (76 bytes)
        let mut complete_buf = [0u8; 76];
        let mut complete_read = 0;
        while complete_read < 76 {
            let n = stream.read(&mut complete_buf[complete_read..]).expect("Blad odczytu KEY_COMPLETE");
            assert!(n > 0);
            complete_read += n;
        }
        let complete_hdr = u16::from_le_bytes(complete_buf[0..2].try_into().unwrap());
        assert_eq!(complete_hdr, 0x000C);

        let mut token_enc = [0u8; 48];
        token_enc.copy_from_slice(&complete_buf[4..52]);
        let mut nonce = [0u8; 24];
        nonce.copy_from_slice(&complete_buf[52..76]);
        assert!(cipher.decrypt_token(&token_enc, &nonce));

        // Aktywacja szyfru strumieniowego
        cipher.set_activated(true);

        // Odbior nowej fazy serwera (GC::PHASE)
        let mut phase_buf = [0u8; 5];
        let n_phase = stream.read(&mut phase_buf).expect("Blad odczytu nowej fazy");
        assert!(n_phase > 0);
        cipher.decrypt_in_place(&mut phase_buf[..n_phase]);
        let new_phase_hdr = u16::from_le_bytes(phase_buf[0..2].try_into().unwrap());
        assert_eq!(new_phase_hdr, 0x0008);
        assert_eq!(phase_buf[4], 10); // PHASE_AUTH = 10

        // Wyslanie zaszyfrowanego pakietu CG_LOGIN3 dla konta 'admin'
        let mut login_pkt = [0u8; 52];
        login_pkt[0..2].copy_from_slice(&0x0102u16.to_le_bytes()); // CG::LOGIN3
        login_pkt[2..4].copy_from_slice(&52u16.to_le_bytes());
        let id_b = b"admin";
        login_pkt[4..4 + id_b.len()].copy_from_slice(id_b);
        let pwd_b = b"admin";
        login_pkt[35..35 + pwd_b.len()].copy_from_slice(pwd_b);

        cipher.encrypt_in_place(&mut login_pkt);
        stream.write_all(&login_pkt).expect("Blad wysylania CG_LOGIN3");

        // Odbior odpowiedzi serwera na logowanie
        let mut resp_buf = [0u8; 1024];
        let n_resp = stream.read(&mut resp_buf).expect("Blad odczytu odpowiedzi logowania");
        assert!(n_resp > 0);
        cipher.decrypt_in_place(&mut resp_buf[..n_resp]);

        let resp_hdr = u16::from_le_bytes(resp_buf[0..2].try_into().unwrap());
        println!("Test live login response opcode: 0x{:04X}", resp_hdr);
        // Powinno zwrocic sukces autoryzacji: 0x0108 (AUTH_SUCCESS) lub 0x0104 (LOGIN_SUCCESS3)
        assert!(resp_hdr == 0x0108 || resp_hdr == 0x0104);
    }
}
