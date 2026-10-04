//! Binary Mob Proto Reader Module
//!
//! Dekoduje binarne pliki `mob_proto` gry Metin2 z naglowkiem FourCC 'MOPX',
//! dekompresja LZO oraz deszyfrowaniem TEA. Parsuje struktury `TMobTable`.

use std::collections::HashMap;
use std::mem::size_of;

pub use crate::mob_proto_binary_loader::{
    CHARACTER_NAME_MAX_LEN, MOB_ENCHANTS_MAX_NUM, MOB_RESISTS_MAX_NUM, MOB_SKILL_MAX_NUM,
    TMobSkillLevel, TMobTable,
};
use crate::vfs_security_decrypt::{decompress_lzo, decrypt_tea, SecurityKey};

pub const MOB_PROTO_FOURCC_MOPX: u32 = 0x58504F4D; // 'MOPX' w little-endian

/// Czytnik bazy danych potworow i NPC z formatu binarnego mob_proto.
#[derive(Debug, Default, Clone)]
pub struct BinaryProtoMobReader {
    pub mobs: HashMap<u32, TMobTable>,
}

impl BinaryProtoMobReader {
    pub fn new() -> Self {
        Self {
            mobs: HashMap::new(),
        }
    }

    /// Pobiera liczbe zaladowanych wpisow mobow.
    pub fn len(&self) -> usize {
        self.mobs.len()
    }

    /// Sprawdza czy baza jest pusta.
    pub fn is_empty(&self) -> bool {
        self.mobs.is_empty()
    }

    /// Wyszukuje moba po vnum.
    pub fn get_mob(&self, vnum: u32) -> Option<&TMobTable> {
        self.mobs.get(&vnum)
    }

    /// Parsuje surowe bajty mob_proto (po ewentualnym deszyfrowaniu i dekompresji).
    pub fn load_from_memory(&mut self, data: &[u8], key: Option<&SecurityKey>) -> Result<usize, String> {
        if data.len() < 12 {
            return Err("Bufor danych mob_proto jest zbyt maly na naglowek".to_string());
        }

        let fourcc = u32::from_le_bytes(data[0..4].try_into().unwrap());
        if fourcc != MOB_PROTO_FOURCC_MOPX {
            return Err(format!("Nieprawidlowy naglowek FourCC: {:#X}, oczekiwano MOPX ({:#X})", fourcc, MOB_PROTO_FOURCC_MOPX));
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

        let mob_size = size_of::<TMobTable>();
        let count = decompressed.len() / mob_size;
        let parse_count = count.min(elements);

        for i in 0..parse_count {
            let offset = i * mob_size;
            let chunk = &decompressed[offset..offset + mob_size];
            // Bezpieczne odczytanie struktury
            let mob: TMobTable = unsafe {
                std::ptr::read_unaligned(chunk.as_ptr() as *const TMobTable)
            };
            self.mobs.insert(mob.dwVnum, mob);
        }

        Ok(self.mobs.len())
    }

    /// Pomocnicza metoda do bezposredniego dodania wpisu moba (np. do testow i rejestru runtime).
    pub fn register_mob(&mut self, mob: TMobTable) {
        self.mobs.insert(mob.dwVnum, mob);
    }
}
