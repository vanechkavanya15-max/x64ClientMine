// Pakiety kontrolne handshake oraz pong CG na podstawie ControlPackets.h: TPacketCGPong, TPacketCGKeyResponse
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::net::constants::cg::{PONG, KEY_RESPONSE};

// Pakiet TPacketCGPong
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGPong {
    pub header: u16,
    pub length: u16,
}

// Zachowanie kompatybilnosci z wczesniejsza nazwa typu
pub type PacketCgPong = TPacketCGPong;

impl TPacketCGPong {
    pub const HEADER: u16 = PONG;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    pub fn new() -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
        }
    }

    // Serializuje pakiet do tablicy bajtow.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf
    }
}

impl Default for TPacketCGPong {
    fn default() -> Self {
        Self::new()
    }
}

// Pakiet TPacketCGKeyResponse do wymiany kluczy kryptograficznych
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGKeyResponse {
    pub header: u16,
    pub length: u16,
    pub client_pk: [u8; 32],
    pub response: [u8; 32],
}

impl TPacketCGKeyResponse {
    pub const HEADER: u16 = KEY_RESPONSE;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    pub fn new(client_pk: [u8; 32], response: [u8; 32]) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            client_pk,
            response,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.extend_from_slice(&self.client_pk);
        buf.extend_from_slice(&self.response);
        buf
    }
}

// Struktura pakietu Handshake / TimeSync klienta (framing z header u16 i length u16)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketCgHandshake {
    pub header: u16,
    pub length: u16,
    pub handshake: u32,
    pub time: u32,
    pub delta: i32,
}

impl PacketCgHandshake {
    pub const HEADER: u16 = 0x00FF;
    pub const HEADER_TIME_SYNC: u16 = 0x00FC;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    pub fn new(handshake: u32, time: u32, delta: i32) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            handshake,
            time,
            delta,
        }
    }

    pub fn new_time_sync(handshake: u32, time: u32, delta: i32) -> Self {
        Self {
            header: Self::HEADER_TIME_SYNC,
            length: Self::SIZE,
            handshake,
            time,
            delta,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.extend_from_slice(&self.handshake.to_le_bytes());
        buf.extend_from_slice(&self.time.to_le_bytes());
        buf.extend_from_slice(&self.delta.to_le_bytes());
        buf
    }
}
