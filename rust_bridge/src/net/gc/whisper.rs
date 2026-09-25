use crate::ffi_guard::safe_ffi_call;
use std::ffi::{c_char, CString};

use std::slice;

pub const HEADER_GC_WHISPER: u8 = 34;
pub const CHARACTER_NAME_MAX_LEN: usize = 24;

/// Struktura odpowiadajaca pakietowi C++ TPacketGCWhisper.
/// BYTE bHeader
/// WORD wSize
/// BYTE bType
/// char szNameFrom[CHARACTER_NAME_MAX_LEN + 1]
/// reszta bajtow to tekst wiadomosci
#[derive(Debug, PartialEq)]
pub struct TPacketGCWhisper {
    pub header: u8,
    pub size: u16,
    pub b_type: u8,
    pub sz_name_from: String,
    pub message: String,
}

impl TPacketGCWhisper {
    /// Minimalny rozmiar pakietu (bez tresci szeptu): 1 + 2 + 1 + 25 = 29 bajtow
    pub const MIN_PACKET_SIZE: usize = 29;

    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() < Self::MIN_PACKET_SIZE {
            return Err("Bufor jest za krotki".to_string());
        }

        let header = data[0];
        if header != HEADER_GC_WHISPER {
            return Err("Nieprawidlowy naglowek szeptu".to_string());
        }

        let mut size_bytes = [0u8; 2];
        size_bytes.copy_from_slice(&data[1..3]);
        let size = u16::from_le_bytes(size_bytes);

        if (size as usize) < Self::MIN_PACKET_SIZE {
            return Err("Rozmiar pakietu zgloszony w naglowku jest mniejszy niz minimum".to_string());
        }

        if data.len() < size as usize {
            return Err("Bufor nie zawiera calego pakietu zadeklarowanego w naglowku".to_string());
        }

        let b_type = data[3];

        // Odczytanie nazwy postaci nadawcy. Null-terminated, max 24 char + 1 null.
        let name_slice = &data[4..4 + CHARACTER_NAME_MAX_LEN + 1];
        let name_end = name_slice.iter().position(|&c| c == 0).unwrap_or(CHARACTER_NAME_MAX_LEN + 1);
        let sz_name_from = String::from_utf8_lossy(&name_slice[..name_end]).into_owned();

        // Reszta to wiadomosc
        let message_len = (size as usize) - Self::MIN_PACKET_SIZE;
        let message_slice = &data[Self::MIN_PACKET_SIZE..Self::MIN_PACKET_SIZE + message_len];
        // Szept moze byc null-terminated w kliencie C++, wiec ucinamy
        let message_end = message_slice.iter().position(|&c| c == 0).unwrap_or(message_len);
        let message = String::from_utf8_lossy(&message_slice[..message_end]).into_owned();

        Ok(TPacketGCWhisper {
            header,
            size,
            b_type,
            sz_name_from,
            message,
        })
    }
}

// Struktura na potrzeby C-FFI, poniewaz std::string z Rusta nie moze byc bezpiecznie uzyty w C++
#[repr(C)]
pub struct C_TPacketGCWhisper {
    pub header: u8,
    pub size: u16,
    pub b_type: u8,
    pub sz_name_from: *mut c_char,
    pub message: *mut c_char,
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_parse_whisper(
    data: *const u8,
    len: usize,
    out_packet: *mut *mut C_TPacketGCWhisper,
) -> bool {
    safe_ffi_call(false, || {
        if data.is_null() || out_packet.is_null() || len == 0 {
            return false;
        }

        let slice = unsafe { slice::from_raw_parts(data, len) };
        match TPacketGCWhisper::from_bytes(slice) {
            Ok(rust_packet) => {
                let name_c = CString::new(rust_packet.sz_name_from).unwrap_or_default();
                let message_c = CString::new(rust_packet.message).unwrap_or_default();

                let c_packet = Box::new(C_TPacketGCWhisper {
                    header: rust_packet.header,
                    size: rust_packet.size,
                    b_type: rust_packet.b_type,
                    sz_name_from: name_c.into_raw(),
                    message: message_c.into_raw(),
                });

                unsafe {
                    *out_packet = Box::into_raw(c_packet);
                }
                true
            }
            Err(_) => false,
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_free_whisper(packet: *mut C_TPacketGCWhisper) {
    safe_ffi_call((), || {
        if packet.is_null() {
            return;
        }

        unsafe {
            let p = Box::from_raw(packet);
            if !p.sz_name_from.is_null() {
                let _ = CString::from_raw(p.sz_name_from);
            }
            if !p.message.is_null() {
                let _ = CString::from_raw(p.message);
            }
        }
    })
}
