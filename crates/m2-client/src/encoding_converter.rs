// crates/m2-client/src/encoding_converter.rs
//! Konwerter kodowania tekstu dla klienta Metin2.
//! Zapewnia plynna transkodacje ze starego kodowania Windows-1250 / CP1250
//! (uzywanego w oryginalnych plikach gry, questach i proto) do nowoczesnego standardu UTF-8.

/// Konwertuje surowe bajty (potencjalnie Windows-1250 lub UTF-8) na poprawny napis UTF-8.
/// Jesli bufor jest poprawnym UTF-8, zwraca go bez zmian.
/// W przeciwnym razie dekoduje znaki Windows-1250 (w tym polskie znaki diakrytyczne).
pub fn decode_metin2_text(bytes: &[u8]) -> String {
    // 1. Sprawdz czy to juz jest prawidlowy UTF-8
    if let Ok(s) = std::str::from_utf8(bytes) {
        return s.to_string();
    }

    // 2. Dekodowanie Windows-1250 do UTF-8
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        if b < 0x80 {
            out.push(b as char);
        } else {
            let ch = match b {
                0x85 => '…',
                0x8A => 'Š',
                0x8C => 'Ś',
                0x8D => 'Ť',
                0x8E => 'Ž',
                0x8F => 'Ź',
                0x9A => 'š',
                0x9C => 'ś',
                0x9D => 'ť',
                0x9E => 'ž',
                0x9F => 'ź',
                0xA1 => 'ˇ',
                0xA2 => '˘',
                0xA3 => 'Ł',
                0xA4 => '¤',
                0xA5 => 'Ą',
                0xA6 => '¦',
                0xA7 => '§',
                0xA8 => '¨',
                0xA9 => '©',
                0xAA => 'Ş',
                0xAB => '«',
                0xAC => '¬',
                0xAD => '-',
                0xAE => '®',
                0xAF => 'Ż',
                0xB0 => '°',
                0xB1 => '±',
                0xB2 => '˛',
                0xB3 => 'ł',
                0xB4 => '´',
                0xB5 => 'µ',
                0xB6 => '¶',
                0xB7 => '·',
                0xB8 => '¸',
                0xB9 => 'ą',
                0xBA => 'ş',
                0xBB => '»',
                0xBC => 'Ľ',
                0xBD => '˝',
                0xBE => 'ľ',
                0xBF => 'ż',
                0xC0 => 'Ŕ',
                0xC1 => 'Á',
                0xC2 => 'Â',
                0xC3 => 'Ă',
                0xC4 => 'Ä',
                0xC5 => 'Ĺ',
                0xC6 => 'Ć',
                0xC7 => 'Ç',
                0xC8 => 'Č',
                0xC9 => 'É',
                0xCA => 'Ę',
                0xCB => 'Ë',
                0xCC => 'Ě',
                0xCD => 'Í',
                0xCE => 'Î',
                0xCF => 'Ď',
                0xD0 => 'Đ',
                0xD1 => 'Ń',
                0xD2 => 'Ň',
                0xD3 => 'Ó',
                0xD4 => 'Ô',
                0xD5 => 'Ő',
                0xD6 => 'Ö',
                0xD7 => '×',
                0xD8 => 'Ř',
                0xD9 => 'Ů',
                0xDA => 'Ú',
                0xDB => 'Ű',
                0xDC => 'Ü',
                0xDD => 'Ý',
                0xDE => 'Ţ',
                0xDF => 'ß',
                0xE0 => 'ŕ',
                0xE1 => 'á',
                0xE2 => 'â',
                0xE3 => 'ă',
                0xE4 => 'ä',
                0xE5 => 'ĺ',
                0xE6 => 'ć',
                0xE7 => 'ç',
                0xE8 => 'č',
                0xE9 => 'é',
                0xEA => 'ę',
                0xEB => 'ë',
                0xEC => 'ě',
                0xED => 'í',
                0xEE => 'î',
                0xEF => 'ď',
                0xF0 => 'đ',
                0xF1 => 'ń',
                0xF2 => 'ň',
                0xF3 => 'ó',
                0xF4 => 'ô',
                0xF5 => 'ő',
                0xF6 => 'ö',
                0xF7 => '÷',
                0xF8 => 'ř',
                0xF9 => 'ů',
                0xFA => 'ú',
                0xFB => 'ű',
                0xFC => 'ü',
                0xF0..=0xFF => b as char,
                _ => '?',
            };
            out.push(ch);
        }
    }
    out
}

/// Eksport C FFI dla skryptow i mostka Pythona
#[no_mangle]
pub extern "C" fn m2_decode_cp1250_to_utf8(
    in_bytes: *const u8,
    in_len: usize,
    out_buf: *mut u8,
    out_max_len: usize,
) -> usize {
    if in_bytes.is_null() || out_buf.is_null() || in_len == 0 || out_max_len == 0 {
        return 0;
    }

    let slice = unsafe { std::slice::from_raw_parts(in_bytes, in_len) };
    let decoded = decode_metin2_text(slice);
    let bytes = decoded.as_bytes();
    let to_copy = bytes.len().min(out_max_len - 1);

    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf, to_copy);
        *out_buf.add(to_copy) = 0;
    }

    to_copy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8_passthrough() {
        let input = "Zwykly tekst UTF-8".as_bytes();
        assert_eq!(decode_metin2_text(input), "Zwykly tekst UTF-8");
    }

    #[test]
    fn test_cp1250_polish_characters() {
        // "Zażółć gęślą jaźń" w kodowaniu Windows-1250:
        // Z=0x5A, a=0x61, ż=0xBF, ó=0xF3, ł=0xB3, ć=0xE6, ' '=0x20,
        // g=0x67, ę=0xEA, ś=0x9C, l=0x6C, ą=0xB9, ' '=0x20,
        // j=0x6A, a=0x61, ź=0x9F, ń=0xF1
        let cp1250_bytes: [u8; 17] = [
            0x5A, 0x61, 0xBF, 0xF3, 0xB3, 0xE6, 0x20,
            0x67, 0xEA, 0x9C, 0x6C, 0xB9, 0x20,
            0x6A, 0x61, 0x9F, 0xF1,
        ];
        let decoded = decode_metin2_text(&cp1250_bytes);
        assert_eq!(decoded, "Zażółć gęślą jaźń");
    }
}
