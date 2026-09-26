#![allow(non_snake_case)]

// Modul pakietow kontrolnych i nawiazywania polaczenia (Handshake / Key Exchange)
// Zgodny z E:\m2dev-client-src-main\src\UserInterface\Packet.h oraz EterLib\ControlPackets.h

use std::mem::size_of;
use std::ptr;

pub const HEADER_GC_PING: u16 = 0x0007;
pub const HEADER_GC_PHASE: u16 = 0x0008;
pub const HEADER_GC_KEY_CHALLENGE: u16 = 0x000B;
pub const HEADER_GC_KEY_COMPLETE: u16 = 0x000C;
pub const HEADER_GC_HANDSHAKE: u16 = 0x00FF;

// Struktura pakietu TPacketGCHandShake w ramce sieciowej [header: u16, length: u16, ...]
// Rozmiar: header(2) + length(2) + dwHandshake(4) + dwTime(4) + lDelta(4) = 16 bajtow
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct TPacketGCHandShake {
    pub header: u16,
    pub length: u16,
    pub dwHandshake: u32,
    pub dwTime: u32,
    pub lDelta: i32,
}

pub type HandshakePacket = TPacketGCHandShake;

impl TPacketGCHandShake {
    pub const PACKET_SIZE: usize = size_of::<Self>();

    // Deserializacja z bufora bajtow
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < Self::PACKET_SIZE {
            return Err(format!(
                "Za krotki bufor. Oczekiwano {}, otrzymano {}",
                Self::PACKET_SIZE,
                buf.len()
            ));
        }

        let header = u16::from_le_bytes(buf[0..2].try_into().unwrap());
        let length = u16::from_le_bytes(buf[2..4].try_into().unwrap());
        let dwHandshake = u32::from_le_bytes(buf[4..8].try_into().unwrap());
        let dwTime = u32::from_le_bytes(buf[8..12].try_into().unwrap());
        let lDelta = i32::from_le_bytes(buf[12..16].try_into().unwrap());

        Ok(TPacketGCHandShake {
            header,
            length,
            dwHandshake,
            dwTime,
            lDelta,
        })
    }

    // Akcesory dla kompatybilnosci wstecznej
    pub fn dw_handshake(&self) -> u32 {
        unsafe { ptr::addr_of!(self.dwHandshake).read_unaligned() }
    }

    pub fn dw_time(&self) -> u32 {
        unsafe { ptr::addr_of!(self.dwTime).read_unaligned() }
    }

    pub fn l_delta(&self) -> i32 {
        unsafe { ptr::addr_of!(self.lDelta).read_unaligned() }
    }
}

// Struktura pakietu zmiany fazy (EterLib/ControlPackets.h)
// Rozmiar: header(2) + length(2) + phase(1) = 5 bajtow
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCPhase {
    pub header: u16,
    pub length: u16,
    pub phase: u8,
}

impl TPacketGCPhase {
    pub const PACKET_SIZE: usize = size_of::<Self>();

    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < Self::PACKET_SIZE {
            return Err("Bufor za krotki dla TPacketGCPhase".to_string());
        }

        Ok(Self {
            header: u16::from_le_bytes(buf[0..2].try_into().unwrap()),
            length: u16::from_le_bytes(buf[2..4].try_into().unwrap()),
            phase: buf[4],
        })
    }
}

// Struktura pakietu PING (EterLib/ControlPackets.h)
// Rozmiar: header(2) + length(2) + server_time(4) = 8 bajtow
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCPing {
    pub header: u16,
    pub length: u16,
    pub server_time: u32,
}

impl TPacketGCPing {
    pub const PACKET_SIZE: usize = size_of::<Self>();

    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < Self::PACKET_SIZE {
            return Err("Bufor za krotki dla TPacketGCPing".to_string());
        }

        Ok(Self {
            header: u16::from_le_bytes(buf[0..2].try_into().unwrap()),
            length: u16::from_le_bytes(buf[2..4].try_into().unwrap()),
            server_time: u32::from_le_bytes(buf[4..8].try_into().unwrap()),
        })
    }
}

// Struktura wyzwania klucza (EterLib/ControlPackets.h)
// Rozmiar: header(2) + length(2) + server_pk(32) + challenge(32) + server_time(4) = 72 bajty
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCKeyChallenge {
    pub header: u16,
    pub length: u16,
    pub server_pk: [u8; 32],
    pub challenge: [u8; 32],
    pub server_time: u32,
}

impl TPacketGCKeyChallenge {
    pub const PACKET_SIZE: usize = size_of::<Self>();

    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < Self::PACKET_SIZE {
            return Err("Bufor za krotki dla TPacketGCKeyChallenge".to_string());
        }

        let mut server_pk = [0u8; 32];
        server_pk.copy_from_slice(&buf[4..36]);

        let mut challenge = [0u8; 32];
        challenge.copy_from_slice(&buf[36..68]);

        Ok(Self {
            header: u16::from_le_bytes(buf[0..2].try_into().unwrap()),
            length: u16::from_le_bytes(buf[2..4].try_into().unwrap()),
            server_pk,
            challenge,
            server_time: u32::from_le_bytes(buf[68..72].try_into().unwrap()),
        })
    }
}

// Struktura zakonczenia wymiany kluczy (EterLib/ControlPackets.h)
// Rozmiar: header(2) + length(2) + encrypted_token(48) + nonce(24) = 76 bajtow
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCKeyComplete {
    pub header: u16,
    pub length: u16,
    pub encrypted_token: [u8; 48],
    pub nonce: [u8; 24],
}

impl TPacketGCKeyComplete {
    pub const PACKET_SIZE: usize = size_of::<Self>();

    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < Self::PACKET_SIZE {
            return Err("Bufor za krotki dla TPacketGCKeyComplete".to_string());
        }

        let mut encrypted_token = [0u8; 48];
        encrypted_token.copy_from_slice(&buf[4..52]);

        let mut nonce = [0u8; 24];
        nonce.copy_from_slice(&buf[52..76]);

        Ok(Self {
            header: u16::from_le_bytes(buf[0..2].try_into().unwrap()),
            length: u16::from_le_bytes(buf[2..4].try_into().unwrap()),
            encrypted_token,
            nonce,
        })
    }
}
