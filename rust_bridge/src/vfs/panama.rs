// Modul szyfrujacy Panama Cipher (Little-Endian)
// Implementacja portu z Crypto++ uzywana w VFS klienta Metin2

use std::panic::catch_unwind;

pub const VFS_PANAMA: u8 = 0;

#[repr(C)]
pub struct PanamaCipher {
    state: [u32; 32],
    buffer: [[u32; 8]; 32],
    bstart: usize,
}

impl PanamaCipher {
    pub fn new() -> Self {
        PanamaCipher {
            state: [0; 32], // Dopasowanie do layoutu pamieci
            buffer: [[0; 8]; 32],
            bstart: 0,
        }
    }

    pub fn reset(&mut self) {
        self.state.fill(0);
        for i in 0..32 {
            self.buffer[i].fill(0);
        }
        self.bstart = 0;
    }

    fn iterate(&mut self, count: usize, p: Option<&[u32]>, mut output: Option<&mut [u8]>, input: Option<&[u8]>, _is_pull: bool) {
        let mut count = count;
        
        let mut p_idx = 0;
        let mut out_idx = 0;
        let mut in_idx = 0;
        
        while count > 0 {
            count -= 1;
            
            if let Some(ref mut out) = output {
                for i in 0..8 {
                    let word = self.state[i + 9];
                    let bytes = word.to_le_bytes();
                    for j in 0..4 {
                        if out_idx < out.len() {
                            out[out_idx] = bytes[j];
                            if let Some(inp) = input {
                                if in_idx < inp.len() {
                                    out[out_idx] ^= inp[in_idx];
                                    in_idx += 1;
                                }
                            }
                            out_idx += 1;
                        }
                    }
                }
            }
            
            let b16 = (self.bstart + 16) % 32;
            let b4 = (self.bstart + 32 - 4) % 32;
            self.bstart = (self.bstart + 1) % 32;
            let b0 = self.bstart;
            let b25 = (self.bstart + 32 - 25) % 32;
            
            let mut c = [0u32; 17];
            
            if let Some(p_arr) = p {
                for i in 0..8 {
                    let t = self.buffer[b0][i];
                    self.buffer[b0][i] = p_arr[p_idx + i] ^ t;
                    self.buffer[b25][(i + 6) % 8] ^= t;
                }
                p_idx += 8;
            } else {
                for i in 0..8 {
                    let t = self.buffer[b0][i];
                    let a_idx = ((i + 1) * 13 + 16) % 17;
                    self.buffer[b0][i] = self.state[a_idx] ^ t;
                    self.buffer[b25][(i + 6) % 8] ^= t;
                }
            }
            
            for i in 0..17 {
                let a_i = self.state[(i * 13 + 16) % 17];
                let a_i_plus_1 = self.state[((i + 1) * 13 + 16) % 17];
                let a_i_plus_2 = self.state[((i + 2) * 13 + 16) % 17];
                
                let rot_amt = ((5 * i % 17) * ((5 * i % 17) + 1) / 2) % 32;
                c[(5 * i % 17) as usize] = (a_i ^ (a_i_plus_1 | !a_i_plus_2)).rotate_left(rot_amt as u32);
            }
            
            let mut new_state = [0u32; 17];
            new_state[16 % 17] = c[0] ^ c[1] ^ c[4] ^ 1;
            
            if let Some(p_arr) = p {
                for i in 0..8 {
                    new_state[((i + 1) * 13 + 16) % 17] = c[i + 1] ^ c[(i + 2) % 17] ^ c[(i + 5) % 17] ^ p_arr[p_idx - 8 + i];
                }
            } else {
                for i in 0..8 {
                    let b_4_i = self.buffer[b4][i];
                    new_state[((i + 1) * 13 + 16) % 17] = c[i + 1] ^ c[(i + 2) % 17] ^ c[(i + 5) % 17] ^ b_4_i;
                }
            }
            
            for i in 0..8 {
                let b_16_i = self.buffer[b16][i];
                new_state[((i + 9) * 13 + 16) % 17] = c[i + 9] ^ c[(i + 10) % 17] ^ c[(i + 13) % 17] ^ b_16_i;
            }
            
            for i in 0..17 {
                self.state[i] = new_state[i];
            }
        }
    }

    pub fn set_key_with_iv(&mut self, key: &[u8], iv: &[u8]) {
        self.reset();
        
        let mut key_words = [0u32; 8];
        for i in 0..8 {
            key_words[i] = u32::from_le_bytes([key[i*4], key[i*4+1], key[i*4+2], key[i*4+3]]);
        }
        
        self.iterate(1, Some(&key_words), None, None, false);
        
        let mut iv_words = [0u32; 8];
        for i in 0..8 {
            iv_words[i] = u32::from_le_bytes([iv[i*4], iv[i*4+1], iv[i*4+2], iv[i*4+3]]);
        }
        
        self.iterate(1, Some(&iv_words), None, None, false);
        self.iterate(32, None, None, None, true);
    }

    pub fn process_data(&mut self, inout: &mut [u8]) {
        let len = inout.len();
        if len == 0 {
            return;
        }
        
        let blocks = len / 32;
        let rem = len % 32;
        
        for i in 0..blocks {
            let offset = i * 32;
            let mut chunk = [0u8; 32];
            let mut in_chunk = [0u8; 32];
            for j in 0..32 {
                in_chunk[j] = inout[offset + j];
            }
            self.iterate(1, None, Some(&mut chunk), Some(&in_chunk), true);
            for j in 0..32 {
                inout[offset + j] = chunk[j];
            }
        }
        
        if rem > 0 {
            let offset = blocks * 32;
            let mut chunk = [0u8; 32];
            let mut in_chunk = [0u8; 32];
            for i in 0..rem {
                in_chunk[i] = inout[offset + i];
            }
            self.iterate(1, None, Some(&mut chunk), Some(&in_chunk), true);
            for i in 0..rem {
                inout[offset + i] = chunk[i];
            }
        }
    }
}

impl Default for PanamaCipher {
    fn default() -> Self {
        Self::new()
    }
}

impl PanamaCipher {
    pub fn init_key(&mut self, key: &[u8]) {
        self.set_key_with_iv(key, key);
    }
    
    pub fn decrypt(&mut self, data: &mut [u8]) {
        self.process_data(data);
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_vfs_panama_decrypt(data: *mut u8, len: u32, key: *const u8) -> bool {
    let result = catch_unwind(|| {
        if data.is_null() || key.is_null() || len == 0 {
            return false;
        }

        let data_slice = std::slice::from_raw_parts_mut(data, len as usize);
        let key_slice = std::slice::from_raw_parts(key, 32); 

        let mut cipher = PanamaCipher::new();
        cipher.init_key(key_slice);
        cipher.decrypt(data_slice);

        true
    });

    result.unwrap_or(false)
}
