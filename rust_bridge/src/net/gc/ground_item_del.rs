#![allow(non_snake_case)]

// Struktura pakietu usuniecia przedmiotu z ziemi (Game -> Client)
// Zgodna z E:\m2dev-client-src-main\src\UserInterface\Packet.h (TPacketGCItemGroundDel)

use std::ptr;

pub const HEADER_GC_ITEM_GROUND_DEL: u16 = 0x0516;

// Struktura pakietu TPacketGCItemGroundDel
// Rozmiar: header(2) + length(2) + vid(4) = 8 bajtow
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCItemGroundDel {
    pub header: u16,
    pub length: u16,
    pub vid: u32,
}

pub type GroundItemDelPacket = TPacketGCItemGroundDel;

impl TPacketGCItemGroundDel {
    pub const PACKET_SIZE: usize = 8;

    // Deserializuje pakiet z bufora bajtow
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < Self::PACKET_SIZE {
            return Err("Bufor jest zbyt krotki aby zdeserializowac GroundItemDelPacket".to_string());
        }

        let header = u16::from_le_bytes(buf[0..2].try_into().unwrap());
        let length = u16::from_le_bytes(buf[2..4].try_into().unwrap());
        let vid = u32::from_le_bytes(buf[4..8].try_into().unwrap());

        Ok(TPacketGCItemGroundDel {
            header,
            length,
            vid,
        })
    }

    pub fn get_vid(&self) -> u32 {
        unsafe { ptr::addr_of!(self.vid).read_unaligned() }
    }
}
