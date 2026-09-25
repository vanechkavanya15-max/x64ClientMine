//! Modul szyfrowania Tiny Encryption Algorithm (TEA) zgodny w 100% z EterBase/tea.cpp.
//!
//! Uzywany w komunikacji pakietowej Metin2 oraz w starszych formatach archiwow VFS.

const TEA_ROUND: u32 = 32;
const DELTA: u32 = 0x9E3779B9;

/// Szyfruje pojedynczy 8-bajtowy blok (dwa 32-bitowe slowa sy, sz) kluczem 128-bitowym
pub fn tea_code(sz: u32, sy: u32, key: &[u32; 4]) -> (u32, u32) {
    let mut y = sy;
    let mut z = sz;
    let mut sum: u32 = 0;

    for _ in 0..TEA_ROUND {
        let part1 = ((z << 4) ^ (z >> 5)).wrapping_add(z);
        let part2 = sum.wrapping_add(key[(sum & 3) as usize]);
        y = y.wrapping_add(part1 ^ part2);

        sum = sum.wrapping_add(DELTA);

        let part3 = ((y << 4) ^ (y >> 5)).wrapping_add(y);
        let part4 = sum.wrapping_add(key[((sum >> 11) & 3) as usize]);
        z = z.wrapping_add(part3 ^ part4);
    }

    (y, z)
}

/// Odszyfrowuje pojedynczy 8-bajtowy blok
pub fn tea_decode(sz: u32, sy: u32, key: &[u32; 4]) -> (u32, u32) {
    let mut y = sy;
    let mut z = sz;
    let mut sum: u32 = DELTA.wrapping_mul(TEA_ROUND);

    for _ in 0..TEA_ROUND {
        let part3 = ((y << 4) ^ (y >> 5)).wrapping_add(y);
        let part4 = sum.wrapping_add(key[((sum >> 11) & 3) as usize]);
        z = z.wrapping_sub(part3 ^ part4);

        sum = sum.wrapping_sub(DELTA);

        let part1 = ((z << 4) ^ (z >> 5)).wrapping_add(z);
        let part2 = sum.wrapping_add(key[(sum & 3) as usize]);
        y = y.wrapping_sub(part1 ^ part2);
    }

    (y, z)
}

/// Szyfruje bufor danych in-place lub do bufora wyjsciowego.
/// Dlugosc bufora wyjsciowego musi byc zaokraglona w gore do wielokrotnosci 8 bajtow.
pub fn tea_encrypt(dest: &mut [u8], src: &[u8], key: &[u32; 4]) -> usize {
    let src_len = src.len();
    let resize = if src_len % 8 != 0 {
        src_len + 8 - (src_len % 8)
    } else {
        src_len
    };

    assert!(dest.len() >= resize, "Bufor docelowy za maly na zaszyfrowany blok!");

    // Skopiuj zrodlo i wyzeruj ewentualny padding
    let mut temp = vec![0u8; resize];
    temp[..src_len].copy_from_slice(src);

    for i in 0..(resize / 8) {
        let offset = i * 8;
        let y_in = u32::from_ne_bytes(temp[offset..offset + 4].try_into().unwrap());
        let z_in = u32::from_ne_bytes(temp[offset + 4..offset + 8].try_into().unwrap());

        let (y_out, z_out) = tea_code(z_in, y_in, key);

        dest[offset..offset + 4].copy_from_slice(&y_out.to_ne_bytes());
        dest[offset + 4..offset + 8].copy_from_slice(&z_out.to_ne_bytes());
    }

    resize
}

/// Deszyfruje bufor danych
pub fn tea_decrypt(dest: &mut [u8], src: &[u8], key: &[u32; 4]) -> usize {
    let src_len = src.len();
    let resize = if src_len % 8 != 0 {
        src_len + 8 - (src_len % 8)
    } else {
        src_len
    };

    assert!(dest.len() >= resize, "Bufor docelowy za maly na odszyfrowany blok!");

    for i in 0..(resize / 8) {
        let offset = i * 8;
        let y_in = u32::from_ne_bytes(src[offset..offset + 4].try_into().unwrap());
        let z_in = u32::from_ne_bytes(src[offset + 4..offset + 8].try_into().unwrap());

        let (y_out, z_out) = tea_decode(z_in, y_in, key);

        dest[offset..offset + 4].copy_from_slice(&y_out.to_ne_bytes());
        dest[offset + 4..offset + 8].copy_from_slice(&z_out.to_ne_bytes());
    }

    resize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tea_block_roundtrip() {
        let key = [0x12345678, 0x9ABCDEF0, 0xFEDCBA98, 0x76543210];
        let sy = 0x11223344;
        let sz = 0x55667788;

        let (y_enc, z_enc) = tea_code(sz, sy, &key);
        let (y_dec, z_dec) = tea_decode(z_enc, y_enc, &key);

        assert_eq!(y_dec, sy, "Odszyfrowane Y musi byc rowne poczatkowemu");
        assert_eq!(z_dec, sz, "Odszyfrowane Z musi byc rowne poczatkowemu");
    }

    #[test]
    fn test_tea_buffer_roundtrip() {
        let key = [0xCAFEBABE, 0xDEADBEEF, 0x01020304, 0x05060708];
        let original_data = b"TajemnaWiadomoscDoSerweraGry123";
        let mut encrypted = vec![0u8; 40];
        let enc_len = tea_encrypt(&mut encrypted, original_data, &key);

        let mut decrypted = vec![0u8; 40];
        tea_decrypt(&mut decrypted, &encrypted[..enc_len], &key);

        assert_eq!(&decrypted[..original_data.len()], original_data);
    }
}
