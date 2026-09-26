// Struktura pakietu podniesienia przedmiotu CG na podstawie Packet.h: command_item_pickup
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::net::constants::cg::ITEM_PICKUP;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGItemPickUp {
    pub header: u16,
    pub length: u16,
    pub vid: u32,
}

// Alias dla zgodnosci z rozna wielkoscia liter
pub type TPacketCGItemPickup = TPacketCGItemPickUp;

impl TPacketCGItemPickUp {
    pub const HEADER: u16 = ITEM_PICKUP;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    // Tworzy nowy pakiet podniesienia przedmiotu dla zadanego VID.
    pub fn new(vid: u32) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            vid,
        }
    }

    // Serializuje strukture do wektora bajtow (Little-Endian).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.extend_from_slice(&self.vid.to_le_bytes());
        buf
    }
}
