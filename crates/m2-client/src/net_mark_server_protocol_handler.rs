//! Net Mark Server Protocol Handler Module
//!
//! Klient protokolu sieciowego TCP do serwera symboli gildii (Mark Server, domyslny port 10020).
//! Realizuje pobieranie indeksow symboli, sum kontrolnych CRC oraz blokow ikon 16x12 pikseli.

use std::collections::HashMap;

pub const DEFAULT_MARK_SERVER_PORT: u16 = 10020;
pub const MARK_IMAGE_WIDTH: usize = 16;
pub const MARK_IMAGE_HEIGHT: usize = 12;
pub const MARK_IMAGE_BYTES: usize = MARK_IMAGE_WIDTH * MARK_IMAGE_HEIGHT * 4; // 16x12x4 (RGBA32)

// Opcody protokolu Mark Server (CG = Client to Mark Server, GC = Mark Server to Client)
pub const HEADER_CG_MARK_LOGIN: u8 = 100;
pub const HEADER_CG_MARK_CRCLIST: u8 = 101;
pub const HEADER_CG_MARK_IDX_LIST: u8 = 102;
pub const HEADER_CG_MARK_UPLOAD: u8 = 103;

pub const HEADER_GC_MARK_IDX_LIST: u8 = 200;
pub const HEADER_GC_MARK_BLOCK: u8 = 201;

/// Zadanie pobrania listy symboli wysylane do serwera Mark.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarkLoginPacket {
    pub header: u8,
    pub handle: u32,
    pub random_key: u32,
}

impl MarkLoginPacket {
    pub fn new(handle: u32, random_key: u32) -> Self {
        Self {
            header: HEADER_CG_MARK_LOGIN,
            handle,
            random_key,
        }
    }

    pub fn to_bytes(&self) -> [u8; 9] {
        let mut buf = [0u8; 9];
        buf[0] = self.header;
        buf[1..5].copy_from_slice(&self.handle.to_le_bytes());
        buf[5..9].copy_from_slice(&self.random_key.to_le_bytes());
        buf
    }
}

/// Pojedynczy wpis indeksu symbolu gildii.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuildMarkIndexEntry {
    pub guild_id: u32,
    pub mark_id: u32,
}

/// Pakiet bloku danych symboli (obraz 16x12) odebrany z serwera.
#[derive(Debug, Clone)]
pub struct MarkBlockData {
    pub image_index: u32,
    pub pixel_data: Vec<u8>,
}

/// Handler stanu polaczenia i protokolu Mark Server.
#[derive(Debug, Default, Clone)]
pub struct NetMarkServerProtocolHandler {
    pub server_ip: String,
    pub server_port: u16,
    pub is_connected: bool,
    pub mark_index_map: HashMap<u32, u32>, // guild_id -> mark_id
    pub downloaded_blocks: HashMap<u32, Vec<u8>>, // image_index -> raw RGBA
}

impl NetMarkServerProtocolHandler {
    pub fn new(ip: &str, port: u16) -> Self {
        Self {
            server_ip: ip.to_string(),
            server_port: if port == 0 { DEFAULT_MARK_SERVER_PORT } else { port },
            is_connected: false,
            mark_index_map: HashMap::new(),
            downloaded_blocks: HashMap::new(),
        }
    }

    /// Generuje pakiet zapytania o liste indeksow symboli (HEADER_CG_MARK_IDX_LIST).
    pub fn build_request_idx_list_packet(&self) -> Vec<u8> {
        vec![HEADER_CG_MARK_IDX_LIST]
    }

    /// Deserializuje odpowiedz serwera z lista indeksow (HEADER_GC_MARK_IDX_LIST).
    pub fn parse_idx_list_response(&mut self, payload: &[u8]) -> Result<usize, String> {
        if payload.is_empty() {
            return Err("Pusty bufor indeksow mark".to_string());
        }

        if payload[0] != HEADER_GC_MARK_IDX_LIST {
            return Err(format!("Nieznany naglowek: {}", payload[0]));
        }

        if payload.len() < 5 {
            return Err("Bufor za krotki na naglowek z licznikiem".to_string());
        }

        let count = u32::from_le_bytes(payload[1..5].try_into().unwrap()) as usize;
        let entry_size = 8; // 4 bajty guild_id + 4 bajty mark_id
        let expected_len = 5 + count * entry_size;

        if payload.len() < expected_len {
            return Err(format!("Oczekiwano {} bajtow, otrzymano {}", expected_len, payload.len()));
        }

        for i in 0..count {
            let offset = 5 + i * entry_size;
            let guild_id = u32::from_le_bytes(payload[offset..offset + 4].try_into().unwrap());
            let mark_id = u32::from_le_bytes(payload[offset + 4..offset + 8].try_into().unwrap());
            self.mark_index_map.insert(guild_id, mark_id);
        }

        Ok(count)
    }

    /// Deserializuje pakiet bloku grafiki symbolu (HEADER_GC_MARK_BLOCK).
    pub fn parse_mark_block_response(&mut self, payload: &[u8]) -> Result<u32, String> {
        if payload.len() < 5 {
            return Err("Bufor bloku mark za krotki".to_string());
        }

        if payload[0] != HEADER_GC_MARK_BLOCK {
            return Err(format!("Nieprawidlowy naglowek bloku: {}", payload[0]));
        }

        let image_index = u32::from_le_bytes(payload[1..5].try_into().unwrap());
        let raw_pixels = &payload[5..];

        self.downloaded_blocks.insert(image_index, raw_pixels.to_vec());
        Ok(image_index)
    }

    /// Sprawdza czy dany symbol gildii jest juz w pamieci podręcznej.
    pub fn has_mark_for_guild(&self, guild_id: u32) -> bool {
        self.mark_index_map.contains_key(&guild_id)
    }

    /// Pobiera identyfikator symbolu dla gildii.
    pub fn get_mark_id(&self, guild_id: u32) -> Option<u32> {
        self.mark_index_map.get(&guild_id).copied()
    }
}
