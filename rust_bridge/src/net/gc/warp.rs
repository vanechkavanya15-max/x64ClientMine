#![allow(non_snake_case)]

// Struktura pakietu teleportacji (Warp) (Game -> Client)
// Zgodna z E:\m2dev-client-src-main\src\UserInterface\Packet.h (TPacketGCWarp)

use crate::ffi_guard::safe_ffi_call;
use std::ptr;

pub const HEADER_GC_WARP: u16 = 0x0306;

// Struktura pakietu teleportacji
// Rozmiar: header(2) + length(2) + lX(4) + lY(4) + lAddr(4) + wPort(2) = 18 bajtow
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct TPacketGCWarp {
    pub header: u16,
    pub length: u16,
    pub lX: i32,
    pub lY: i32,
    pub lAddr: i32,
    pub wPort: u16,
}

impl TPacketGCWarp {
    pub const PACKET_SIZE: usize = 18;

    // Deserializacja pakietu z bufora bajtow
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < Self::PACKET_SIZE {
            return Err(format!(
                "Bufor jest za krotki. Oczekiwano {}, otrzymano {}",
                Self::PACKET_SIZE,
                buf.len()
            ));
        }

        let header = u16::from_le_bytes(buf[0..2].try_into().unwrap());
        let length = u16::from_le_bytes(buf[2..4].try_into().unwrap());
        let lX = i32::from_le_bytes(buf[4..8].try_into().unwrap());
        let lY = i32::from_le_bytes(buf[8..12].try_into().unwrap());
        let lAddr = i32::from_le_bytes(buf[12..16].try_into().unwrap());
        let wPort = u16::from_le_bytes(buf[16..18].try_into().unwrap());

        Ok(TPacketGCWarp {
            header,
            length,
            lX,
            lY,
            lAddr,
            wPort,
        })
    }

    // Akcesory dla kompatybilnosci wstecznej
    pub fn l_x(&self) -> i32 {
        unsafe { ptr::addr_of!(self.lX).read_unaligned() }
    }

    pub fn l_y(&self) -> i32 {
        unsafe { ptr::addr_of!(self.lY).read_unaligned() }
    }

    pub fn l_addr(&self) -> i32 {
        unsafe { ptr::addr_of!(self.lAddr).read_unaligned() }
    }

    pub fn w_port(&self) -> u16 {
        unsafe { ptr::addr_of!(self.wPort).read_unaligned() }
    }
}

// Funkcja C-FFI do parsowania pakietu warp z surowego bufora bajtow
#[no_mangle]
pub unsafe extern "C" fn m2_net_parse_warp(
    buf: *const u8,
    len: usize,
    out_packet: *mut *mut TPacketGCWarp,
) -> bool {
    safe_ffi_call(false, || {
        if buf.is_null() || out_packet.is_null() {
            return false;
        }

        let slice = std::slice::from_raw_parts(buf, len);

        match TPacketGCWarp::from_bytes(slice) {
            Ok(packet) => {
                let boxed_packet = Box::new(packet);
                *out_packet = Box::into_raw(boxed_packet);
                true
            }
            Err(_) => false,
        }
    })
}

// Zwalnia pamiec zajmowana przez strukture TPacketGCWarp
#[no_mangle]
pub unsafe extern "C" fn m2_net_free_warp(packet: *mut TPacketGCWarp) {
    safe_ffi_call((), || {
        if !packet.is_null() {
            let _ = Box::from_raw(packet);
        }
    });
}
