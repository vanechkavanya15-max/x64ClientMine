#![allow(non_snake_case)]

// Modul obslugujacy wiadomosci prywatne szeptu (Game -> Client)
// Zgodny z E:\m2dev-client-src-main\src\UserInterface\Packet.h (TPacketGCWhisper)

use crate::ffi_guard::safe_ffi_call;
use std::ffi::{c_char, CString};
use std::slice;

pub const HEADER_GC_WHISPER: u16 = 0x0604;
pub const CHARACTER_NAME_MAX_LEN: usize = 64;

// Struktura pakietu TPacketGCWhisper
// Rozmiar: header(2) + length(2) + bType(1) + szNameFrom(65) = 70 bajtow
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct TPacketGCWhisper {
    pub header: u16,
    pub length: u16,
    pub bType: u8,
    pub szNameFrom: [u8; CHARACTER_NAME_MAX_LEN + 1],
}

impl TPacketGCWhisper {
    pub const PACKET_SIZE: usize = 70;
    pub const MIN_PACKET_SIZE: usize = 70;

    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() < Self::PACKET_SIZE {
            return Err("Bufor jest za krotki aby zawierac naglowek szeptu".to_string());
        }

        let header = u16::from_le_bytes(data[0..2].try_into().unwrap());
        let length = u16::from_le_bytes(data[2..4].try_into().unwrap());
        let bType = data[4];
        let mut szNameFrom = [0u8; CHARACTER_NAME_MAX_LEN + 1];
        szNameFrom.copy_from_slice(&data[5..70]);

        Ok(Self {
            header,
            length,
            bType,
            szNameFrom,
        })
    }
}

// Struktura uzytkowa ze zdeserializowanymi stringami
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GcWhisperPacket {
    pub header: u16,
    pub length: u16,
    pub b_type: u8,
    pub sz_name_from: String,
    pub message: String,
}

impl GcWhisperPacket {
    pub const MIN_PACKET_SIZE: usize = 70;

    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() < Self::MIN_PACKET_SIZE {
            return Err("Bufor jest za krotki".to_string());
        }

        let header = u16::from_le_bytes(data[0..2].try_into().unwrap());
        let length = u16::from_le_bytes(data[2..4].try_into().unwrap());

        if (length as usize) < Self::MIN_PACKET_SIZE {
            return Err("Rozmiar pakietu zgloszony w naglowku jest mniejszy niz minimum".to_string());
        }

        if data.len() < length as usize {
            return Err("Bufor nie zawiera calego pakietu zadeklarowanego w naglowku".to_string());
        }

        let b_type = data[4];

        // Odczytanie nazwy postaci nadawcy (null-terminated)
        let name_slice = &data[5..70];
        let name_end = name_slice.iter().position(|&c| c == 0).unwrap_or(CHARACTER_NAME_MAX_LEN + 1);
        let sz_name_from = String::from_utf8_lossy(&name_slice[..name_end]).into_owned();

        // Reszta bajtow stanowi tresc wiadomosci
        let message_len = (length as usize) - Self::MIN_PACKET_SIZE;
        let message_slice = &data[Self::MIN_PACKET_SIZE..Self::MIN_PACKET_SIZE + message_len];
        let message_end = message_slice.iter().position(|&c| c == 0).unwrap_or(message_len);
        let message = String::from_utf8_lossy(&message_slice[..message_end]).into_owned();

        Ok(GcWhisperPacket {
            header,
            length,
            b_type,
            sz_name_from,
            message,
        })
    }

    // Akcesory pomocnicze
    pub fn size(&self) -> u16 {
        self.length
    }
}

// Struktura na potrzeby C-FFI
#[repr(C)]
pub struct C_TPacketGCWhisper {
    pub header: u16,
    pub length: u16,
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
        match GcWhisperPacket::from_bytes(slice) {
            Ok(rust_packet) => {
                let name_c = CString::new(rust_packet.sz_name_from).unwrap_or_default();
                let message_c = CString::new(rust_packet.message).unwrap_or_default();

                let c_packet = Box::new(C_TPacketGCWhisper {
                    header: rust_packet.header,
                    length: rust_packet.length,
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
