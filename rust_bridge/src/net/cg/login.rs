// Struktury pakietow logowania CG na podstawie Packet.h: command_login2, command_login3, TPacketCGLoginSecure
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::net::constants::cg::{LOGIN2, LOGIN3, LOGIN_SECURE};

pub const ID_MAX_NUM: usize = 30;
pub const PASS_MAX_NUM: usize = 16;

pub const HEADER_CG_LOGIN2: u16 = LOGIN2;
pub const HEADER_CG_LOGIN3: u16 = LOGIN3;
pub const HEADER_CG_LOGIN_SECURE: u16 = LOGIN_SECURE;

// Pakiet logowania Login2 (autoryzacja kluczem)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGLogin2 {
    pub header: u16,
    pub length: u16,
    pub name: [u8; ID_MAX_NUM + 1],
    pub login_key: u32,
}

impl TPacketCGLogin2 {
    pub const HEADER: u16 = LOGIN2;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    pub fn new(name: &str, login_key: u32) -> Self {
        let mut name_arr = [0u8; ID_MAX_NUM + 1];
        let bytes = name.as_bytes();
        let len = bytes.len().min(ID_MAX_NUM);
        name_arr[..len].copy_from_slice(&bytes[..len]);

        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            name: name_arr,
            login_key,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.extend_from_slice(&self.name);
        buf.extend_from_slice(&self.login_key.to_le_bytes());
        buf
    }
}

// Pakiet logowania Login3 (nazwa uzytkownika i haslo)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGLogin3 {
    pub header: u16,
    pub length: u16,
    pub name: [u8; ID_MAX_NUM + 1],
    pub pwd: [u8; PASS_MAX_NUM + 1],
}

// Zachowanie kompatybilnosci z wczesniejsza nazwa typu
pub type CgLogin3 = TPacketCGLogin3;

impl TPacketCGLogin3 {
    pub const HEADER: u16 = LOGIN3;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    pub fn new(name: &str, pwd: &str) -> Self {
        let mut name_arr = [0u8; ID_MAX_NUM + 1];
        let n_bytes = name.as_bytes();
        let n_len = n_bytes.len().min(ID_MAX_NUM);
        name_arr[..n_len].copy_from_slice(&n_bytes[..n_len]);

        let mut pwd_arr = [0u8; PASS_MAX_NUM + 1];
        let p_bytes = pwd.as_bytes();
        let p_len = p_bytes.len().min(PASS_MAX_NUM);
        pwd_arr[..p_len].copy_from_slice(&p_bytes[..p_len]);

        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            name: name_arr,
            pwd: pwd_arr,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.extend_from_slice(&self.name);
        buf.extend_from_slice(&self.pwd);
        buf
    }
}

// Pakiet bezpiecznego logowania TPacketCGLoginSecure (libsodium/XChaCha20-Poly1305)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGLoginSecure {
    pub header: u16,
    pub length: u16,
    pub name: [u8; ID_MAX_NUM + 1],
    pub pwd: [u8; PASS_MAX_NUM + 1],
    pub session_token: [u8; 32],
}

impl TPacketCGLoginSecure {
    pub const HEADER: u16 = LOGIN_SECURE;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    pub fn new(name: &str, pwd: &str, session_token: [u8; 32]) -> Self {
        let mut name_arr = [0u8; ID_MAX_NUM + 1];
        let n_bytes = name.as_bytes();
        let n_len = n_bytes.len().min(ID_MAX_NUM);
        name_arr[..n_len].copy_from_slice(&n_bytes[..n_len]);

        let mut pwd_arr = [0u8; PASS_MAX_NUM + 1];
        let p_bytes = pwd.as_bytes();
        let p_len = p_bytes.len().min(PASS_MAX_NUM);
        pwd_arr[..p_len].copy_from_slice(&p_bytes[..p_len]);

        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            name: name_arr,
            pwd: pwd_arr,
            session_token,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.extend_from_slice(&self.name);
        buf.extend_from_slice(&self.pwd);
        buf.extend_from_slice(&self.session_token);
        buf
    }
}
