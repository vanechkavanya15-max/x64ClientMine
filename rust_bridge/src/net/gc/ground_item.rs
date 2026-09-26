#![allow(non_snake_case)]

// Struktura pakietu dodania przedmiotu na ziemi (Game -> Client)
// Zgodna z E:\m2dev-client-src-main\src\UserInterface\Packet.h (TPacketGCItemGroundAdd)

use std::ptr;

pub const HEADER_GC_ITEM_GROUND_ADD: u16 = 0x0515;

// Struktura pakietu TPacketGCItemGroundAdd
// Rozmiar: header(2) + length(2) + lX(4) + lY(4) + lZ(4) + dwVID(4) + dwVnum(4) = 24 bajty
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct TPacketGCItemGroundAdd {
    pub header: u16,
    pub length: u16,
    pub lX: i32,
    pub lY: i32,
    pub lZ: i32,
    pub dwVID: u32,
    pub dwVnum: u32,
}

pub type GcItemGroundAddPacket = TPacketGCItemGroundAdd;

impl TPacketGCItemGroundAdd {
    pub const PACKET_SIZE: usize = 24;

    // Deserializuje pakiet z bufora bajtow
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < Self::PACKET_SIZE {
            return Err(format!(
                "Zbyt krotki bufor. Oczekiwano {}, otrzymano {}",
                Self::PACKET_SIZE,
                buf.len()
            ));
        }

        let header = u16::from_le_bytes(buf[0..2].try_into().unwrap());
        let length = u16::from_le_bytes(buf[2..4].try_into().unwrap());
        let lX = i32::from_le_bytes(buf[4..8].try_into().unwrap());
        let lY = i32::from_le_bytes(buf[8..12].try_into().unwrap());
        let lZ = i32::from_le_bytes(buf[12..16].try_into().unwrap());
        let dwVID = u32::from_le_bytes(buf[16..20].try_into().unwrap());
        let dwVnum = u32::from_le_bytes(buf[20..24].try_into().unwrap());

        Ok(Self {
            header,
            length,
            lX,
            lY,
            lZ,
            dwVID,
            dwVnum,
        })
    }

    // Akcesory dla kompatybilnosci wstecznej
    pub fn x(&self) -> i32 {
        unsafe { ptr::addr_of!(self.lX).read_unaligned() }
    }

    pub fn y(&self) -> i32 {
        unsafe { ptr::addr_of!(self.lY).read_unaligned() }
    }

    pub fn z(&self) -> i32 {
        unsafe { ptr::addr_of!(self.lZ).read_unaligned() }
    }

    pub fn vid(&self) -> u32 {
        unsafe { ptr::addr_of!(self.dwVID).read_unaligned() }
    }

    pub fn vnum(&self) -> u32 {
        unsafe { ptr::addr_of!(self.dwVnum).read_unaligned() }
    }
}
