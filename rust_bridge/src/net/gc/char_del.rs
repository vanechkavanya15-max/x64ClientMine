#![allow(non_snake_case)]

// Struktura pakietu usuniecia postaci z widoku (Game -> Client)
// Zgodna z E:\m2dev-client-src-main\src\UserInterface\Packet.h (TPacketGCCharacterDelete)

use std::ptr;

pub const HEADER_GC_CHARACTER_DEL: u16 = 0x0208;

// Struktura pakietu usuniecia postaci
// Rozmiar: header(2) + length(2) + dwVID(4) = 8 bajtow
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct TPacketGCCharacterDelete {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
}

pub type PacketGcCharacterDelete = TPacketGCCharacterDelete;

impl TPacketGCCharacterDelete {
    pub const PACKET_SIZE: usize = 8;

    // Deserializuje pakiet z bufora bajtow
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < Self::PACKET_SIZE {
            return Err("Bufor jest za krotki".to_string());
        }

        let header = u16::from_le_bytes(buf[0..2].try_into().unwrap());
        let length = u16::from_le_bytes(buf[2..4].try_into().unwrap());
        let dwVID = u32::from_le_bytes(buf[4..8].try_into().unwrap());

        Ok(Self {
            header,
            length,
            dwVID,
        })
    }

    // Akcesor dla kompatybilnosci z vid
    pub fn vid(&self) -> u32 {
        unsafe { ptr::addr_of!(self.dwVID).read_unaligned() }
    }
}
