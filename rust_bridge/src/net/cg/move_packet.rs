// Struktura pakietu ruchu klienta CG (Client -> Game) na podstawie Packet.h: command_move
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::net::constants::cg::MOVE;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGMove {
    pub header: u16,
    pub length: u16,
    pub b_func: u8,
    pub b_arg: u8,
    pub b_rot: u8,
    pub l_x: i32,
    pub l_y: i32,
    pub dw_time: u32,
}

// Zachowanie kompatybilnosci z wczesniejsza nazwa typu
pub type MovePacket = TPacketCGMove;

impl TPacketCGMove {
    pub const HEADER: u16 = MOVE;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    // Tworzy nowy pakiet ruchu z domyslnym naglowkiem i rozmiarem.
    pub fn new(b_func: u8, b_arg: u8, b_rot: u8, l_x: i32, l_y: i32, dw_time: u32) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            b_func,
            b_arg,
            b_rot,
            l_x,
            l_y,
            dw_time,
        }
    }

    // Serializuje strukture do wektora bajtow (Little-Endian).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.push(self.b_func);
        buf.push(self.b_arg);
        buf.push(self.b_rot);
        buf.extend_from_slice(&self.l_x.to_le_bytes());
        buf.extend_from_slice(&self.l_y.to_le_bytes());
        buf.extend_from_slice(&self.dw_time.to_le_bytes());
        buf
    }
}
