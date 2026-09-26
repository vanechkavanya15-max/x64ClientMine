#![allow(non_snake_case)]

// Struktura pakietu informacji o obrazeniach (Game -> Client)
// Zgodna z E:\m2dev-client-src-main\src\UserInterface\Packet.h (TPacketGCDamageInfo)

use crate::ffi_guard::safe_ffi_call;
use std::slice;
use std::ptr;

pub const HEADER_GC_DAMAGE_INFO: u16 = 0x0410;

// Struktura pakietu informacji o obrazeniach
// Rozmiar: header(2) + length(2) + dwVID(4) + flag(1) + damage(4) = 13 bajtow
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct TPacketGCDamageInfo {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub flag: u8,
    pub damage: i32,
}

pub type TPacketGCDamage = TPacketGCDamageInfo;

impl TPacketGCDamageInfo {
    pub const PACKET_SIZE: usize = 13;

    // Deserializuje pakiet z bufora bajtow
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < Self::PACKET_SIZE {
            return Err(format!(
                "Bufor jest za krotki. Wymagane {}, otrzymano {}",
                Self::PACKET_SIZE,
                buf.len()
            ));
        }

        let header = u16::from_le_bytes(buf[0..2].try_into().unwrap());
        let length = u16::from_le_bytes(buf[2..4].try_into().unwrap());
        let dwVID = u32::from_le_bytes(buf[4..8].try_into().unwrap());
        let flag = buf[8];
        let damage = i32::from_le_bytes(buf[9..13].try_into().unwrap());

        Ok(Self {
            header,
            length,
            dwVID,
            flag,
            damage,
        })
    }

    // Akcesory dla kompatybilnosci wstecznej
    pub fn dw_victim_vid(&self) -> u32 {
        unsafe { ptr::addr_of!(self.dwVID).read_unaligned() }
    }

    pub fn b_flag(&self) -> u8 {
        self.flag
    }

    pub fn l_damage(&self) -> i32 {
        unsafe { ptr::addr_of!(self.damage).read_unaligned() }
    }
}

// Funkcja C-FFI do parsowania pakietu HEADER_GC_DAMAGE
#[no_mangle]
pub unsafe extern "C" fn m2_net_parse_damage(
    buf: *const u8,
    len: usize,
    out_packet: *mut *mut TPacketGCDamageInfo,
) -> bool {
    safe_ffi_call(false, || {
        if buf.is_null() || out_packet.is_null() {
            return false;
        }

        let slice = unsafe { slice::from_raw_parts(buf, len) };

        match TPacketGCDamageInfo::from_bytes(slice) {
            Ok(packet) => {
                let boxed_packet = Box::new(packet);
                unsafe {
                    *out_packet = Box::into_raw(boxed_packet);
                }
                true
            }
            Err(e) => {
                eprintln!("[m2_client_bridge] Blad parsowania pakietu DAMAGE_INFO: {}", e);
                false
            }
        }
    })
}

// Funkcja C-FFI do zwalniania pamieci
#[no_mangle]
pub unsafe extern "C" fn m2_net_free_damage(packet: *mut TPacketGCDamageInfo) {
    safe_ffi_call((), || {
        if !packet.is_null() {
            unsafe {
                drop(Box::from_raw(packet));
            }
        }
    });
}
