//! Binary Item Proto Reader Module
//!
//! Dekoduje binarne pliki `item_proto` z naglowkiem FourCC 'MIPX',
//! dekompresja LZO oraz opcjonalnym deszyfrowaniem TEA. Parsuje struktury `TItemTable`.

use std::collections::HashMap;
use std::mem::size_of;

pub use crate::item_proto_binary_loader::{
    TItemApply, TItemLimit, TItemTable, ITEM_APPLY_MAX_NUM, ITEM_LIMIT_MAX_NUM,
    ITEM_NAME_MAX_LEN, ITEM_SOCKET_MAX_NUM, ITEM_VALUES_MAX_NUM,
};
use crate::vfs_security_decrypt::{decompress_lzo, decrypt_tea, SecurityKey};

pub const ITEM_PROTO_FOURCC_MIPX: u32 = 0x5850494D; // 'MIPX' w little-endian

/// Czytnik bazy danych przedmiotow z formatu binarnego item_proto.
#[derive(Debug, Default, Clone)]
pub struct BinaryProtoItemReader {
    pub items: HashMap<u32, TItemTable>,
}

impl BinaryProtoItemReader {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn get_item(&self, vnum: u32) -> Option<&TItemTable> {
        self.items.get(&vnum)
    }

    pub fn register_item(&mut self, item: TItemTable) {
        self.items.insert(item.dwVnum, item);
    }

    /// Ladowanie z bufora pamieci z weryfikacja naglowka MIPX.
    pub fn load_from_memory(&mut self, data: &[u8], key: Option<&SecurityKey>) -> Result<usize, String> {
        if data.len() < 12 {
            return Err("Bufor danych item_proto jest zbyt maly na naglowek".to_string());
        }

        let fourcc = u32::from_le_bytes(data[0..4].try_into().unwrap());
        if fourcc != ITEM_PROTO_FOURCC_MIPX {
            return Err(format!("Nieprawidlowy naglowek FourCC: {:#X}, oczekiwano MIPX ({:#X})", fourcc, ITEM_PROTO_FOURCC_MIPX));
        }

        let elements = u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize;
        let data_size = u32::from_le_bytes(data[8..12].try_into().unwrap()) as usize;

        let payload = &data[12..];
        let decompressed = if let Some(k) = key {
            let mut decrypted = payload.to_vec();
            decrypt_tea(&mut decrypted, k);
            decompress_lzo(&decrypted, data_size)?
        } else {
            if payload.len() == data_size {
                payload.to_vec()
            } else {
                decompress_lzo(payload, data_size)?
            }
        };

        let item_size = size_of::<TItemTable>();
        let count = decompressed.len() / item_size;
        let parse_count = count.min(elements);

        for i in 0..parse_count {
            let offset = i * item_size;
            let chunk = &decompressed[offset..offset + item_size];
            let item: TItemTable = unsafe {
                std::ptr::read_unaligned(chunk.as_ptr() as *const TItemTable)
            };
            self.items.insert(item.dwVnum, item);
        }

        Ok(self.items.len())
    }
}
