// vfs_pck_memory_mapped.rs
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use crate::vfs_security_decrypt::{
    compress_zstd, decompress_zstd, decrypt_xchacha20, encrypt_xchacha20,
    PACK_KEY, PACK_KEY_SIZE, PACK_NONCE_SIZE,
};

pub const FILENAME_MAX_LEN: usize = 261;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPackFileHeader {
    pub entry_num: u64,
    pub data_begin: u64,
    pub nonce: [u8; PACK_NONCE_SIZE],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPackFileEntry {
    pub file_name: [u8; FILENAME_MAX_LEN],
    pub offset: u64,
    pub file_size: u64,
    pub compressed_size: u64,
    pub encryption: u8,
    pub nonce: [u8; PACK_NONCE_SIZE],
}

impl TPackFileEntry {
    pub fn new() -> Self {
        Self {
            file_name: [0u8; FILENAME_MAX_LEN],
            offset: 0,
            file_size: 0,
            compressed_size: 0,
            encryption: 0,
            nonce: [0u8; PACK_NONCE_SIZE],
        }
    }

    pub fn get_file_name(&self) -> String {
        let pos = self.file_name.iter().position(|&c| c == 0).unwrap_or(self.file_name.len());
        String::from_utf8_lossy(&self.file_name[..pos]).to_string()
    }

    pub fn set_file_name(&mut self, name: &str) {
        self.file_name = [0u8; FILENAME_MAX_LEN];
        let bytes = name.as_bytes();
        let len = bytes.len().min(FILENAME_MAX_LEN - 1);
        self.file_name[..len].copy_from_slice(&bytes[..len]);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PckIndexEntry {
    pub path_hash: u32,
    pub offset: u64,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
}

#[derive(Debug, Clone)]
pub struct PckArchive {
    pub file_path: String,
    pub header: Option<TPackFileHeader>,
    pub entries: Vec<TPackFileEntry>,
    pub name_map: HashMap<String, usize>,
    pub index_map: HashMap<u32, PckIndexEntry>,
    pub data_mmap: Vec<u8>,
}

pub fn calculate_crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFFFFFFu32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }
    }
    crc ^ 0xFFFFFFFFu32
}

pub fn normalize_pack_path(path: &str) -> String {
    let mut norm = path.trim().to_lowercase().replace('\\', "/");
    let ymir_prefix = "ymir work/";
    if norm.starts_with(ymir_prefix) {
        norm = format!("d:/ymir work/{}", &norm[ymir_prefix.len()..]);
    }
    norm
}

pub fn mount_archive(path: &str) -> Result<PckArchive, String> {
    let mut file = File::open(path).map_err(|e| format!("Failed to open file '{}': {}", path, e))?;
    let mut data_mmap = Vec::new();
    file.read_to_end(&mut data_mmap).map_err(|e| format!("Failed to read file '{}': {}", path, e))?;

    if data_mmap.len() < 8 {
        return Err("File too small to contain valid PCK header".to_string());
    }

    // Obsluga formatu PCK1 (legacy / testy)
    if data_mmap.len() >= 4 && &data_mmap[0..4] == b"PCK1" {
        return mount_legacy_pck1(path, data_mmap);
    }

    let header_size = std::mem::size_of::<TPackFileHeader>();
    if data_mmap.len() < header_size {
        return Err("Invalid PCK magic bytes".to_string());
    }

    // Odczyt TPackFileHeader
    let entry_num = u64::from_le_bytes(data_mmap[0..8].try_into().unwrap());
    let data_begin = u64::from_le_bytes(data_mmap[8..16].try_into().unwrap());
    let mut nonce = [0u8; PACK_NONCE_SIZE];
    nonce.copy_from_slice(&data_mmap[16..16 + PACK_NONCE_SIZE]);

    let header = TPackFileHeader {
        entry_num,
        data_begin,
        nonce,
    };

    let entry_size = std::mem::size_of::<TPackFileEntry>();
    let expected_entries_size = match (entry_num as usize).checked_mul(entry_size) {
        Some(s) => s,
        None => return Err("Invalid PCK magic bytes".to_string()),
    };
    let expected_min_size = match header_size.checked_add(expected_entries_size) {
        Some(s) => s,
        None => return Err("Invalid PCK magic bytes".to_string()),
    };
    if data_mmap.len() < expected_min_size {
        return Err("Invalid PCK magic bytes".to_string());
    }

    let mut entries = Vec::with_capacity(entry_num as usize);
    let mut name_map = HashMap::new();
    let mut index_map = HashMap::new();

    let mut cursor = header_size;
    for idx in 0..entry_num {
        let mut entry_bytes = [0u8; std::mem::size_of::<TPackFileEntry>()];
        entry_bytes.copy_from_slice(&data_mmap[cursor..cursor + entry_size]);
        cursor += entry_size;

        // Deszyfrowanie wpisu indeksu za pomoca XChaCha20 i nonce naglowka
        decrypt_xchacha20(&mut entry_bytes, &header.nonce, &PACK_KEY);

        let mut entry = TPackFileEntry::new();
        entry.file_name.copy_from_slice(&entry_bytes[0..FILENAME_MAX_LEN]);
        entry.offset = u64::from_le_bytes(entry_bytes[261..269].try_into().unwrap());
        entry.file_size = u64::from_le_bytes(entry_bytes[269..277].try_into().unwrap());
        entry.compressed_size = u64::from_le_bytes(entry_bytes[277..285].try_into().unwrap());
        entry.encryption = entry_bytes[285];
        entry.nonce.copy_from_slice(&entry_bytes[286..286 + PACK_NONCE_SIZE]);

        // Sprawdzenie poprawnosci zakresu danych wpisu wzgledem rozmiaru archiwum
        let entry_data_end = header.data_begin
            .checked_add(entry.offset)
            .and_then(|o| o.checked_add(entry.compressed_size));
        if let Some(end) = entry_data_end {
            if end > data_mmap.len() as u64 {
                return Err("Invalid PCK magic bytes".to_string());
            }
        } else {
            return Err("Invalid PCK magic bytes".to_string());
        }

        let raw_name = entry.get_file_name();
        let norm_name = normalize_pack_path(&raw_name);

        name_map.insert(raw_name.clone(), idx as usize);
        name_map.insert(norm_name.clone(), idx as usize);

        let hash = calculate_crc32(raw_name.as_bytes());
        let entry_offset = match header.data_begin.checked_add(entry.offset) {
            Some(o) => o,
            None => return Err("Invalid PCK magic bytes".to_string()),
        };

        index_map.insert(
            hash,
            PckIndexEntry {
                path_hash: hash,
                offset: entry_offset,
                compressed_size: entry.compressed_size as u32,
                uncompressed_size: entry.file_size as u32,
            },
        );

        let norm_hash = calculate_crc32(norm_name.as_bytes());
        index_map.insert(
            norm_hash,
            PckIndexEntry {
                path_hash: norm_hash,
                offset: entry_offset,
                compressed_size: entry.compressed_size as u32,
                uncompressed_size: entry.file_size as u32,
            },
        );

        entries.push(entry);
    }

    Ok(PckArchive {
        file_path: path.to_string(),
        header: Some(header),
        entries,
        name_map,
        index_map,
        data_mmap,
    })
}

fn mount_legacy_pck1(path: &str, data_mmap: Vec<u8>) -> Result<PckArchive, String> {
    let mut cursor = 4;
    let mut num_entries_bytes = [0u8; 4];
    num_entries_bytes.copy_from_slice(&data_mmap[cursor..cursor + 4]);
    let num_entries = u32::from_le_bytes(num_entries_bytes);
    cursor += 4;

    let mut index_map = HashMap::new();

    for _ in 0..num_entries {
        if cursor + 20 > data_mmap.len() {
            return Err("Unexpected end of file while reading PCK entries".to_string());
        }

        let mut hash_bytes = [0u8; 4];
        hash_bytes.copy_from_slice(&data_mmap[cursor..cursor + 4]);
        let path_hash = u32::from_le_bytes(hash_bytes);
        cursor += 4;

        let mut offset_bytes = [0u8; 8];
        offset_bytes.copy_from_slice(&data_mmap[cursor..cursor + 8]);
        let offset = u64::from_le_bytes(offset_bytes);
        cursor += 8;

        let mut comp_bytes = [0u8; 4];
        comp_bytes.copy_from_slice(&data_mmap[cursor..cursor + 4]);
        let compressed_size = u32::from_le_bytes(comp_bytes);
        cursor += 4;

        let mut uncomp_bytes = [0u8; 4];
        uncomp_bytes.copy_from_slice(&data_mmap[cursor..cursor + 4]);
        let uncompressed_size = u32::from_le_bytes(uncomp_bytes);
        cursor += 4;

        let entry = PckIndexEntry {
            path_hash,
            offset,
            compressed_size,
            uncompressed_size,
        };

        index_map.insert(path_hash, entry);
    }

    Ok(PckArchive {
        file_path: path.to_string(),
        header: None,
        entries: Vec::new(),
        name_map: HashMap::new(),
        index_map,
        data_mmap,
    })
}

pub fn read_file_from_archive(archive: &PckArchive, file_name: &str) -> Option<Vec<u8>> {
    // 1. Sprawdz po nazwie w oficjalnym formacie TPackFileHeader
    let norm = normalize_pack_path(file_name);
    let entry_idx = archive.name_map.get(file_name).or_else(|| archive.name_map.get(&norm));

    if let (Some(&idx), Some(header)) = (entry_idx, archive.header) {
        if let Some(entry) = archive.entries.get(idx) {
            let offset = match (header.data_begin as usize).checked_add(entry.offset as usize) {
                Some(o) => o,
                None => return None,
            };
            let comp_size = entry.compressed_size as usize;
            let file_size = entry.file_size as usize;

            let end_offset = match offset.checked_add(comp_size) {
                Some(e) => e,
                None => return None,
            };

            if end_offset <= archive.data_mmap.len() {
                let compressed_slice = &archive.data_mmap[offset..end_offset];

                match entry.encryption {
                    0 => {
                        // Nieszyfrowane, skompresowane ZSTD
                        if let Ok(decomp) = decompress_zstd(compressed_slice, file_size) {
                            return Some(decomp);
                        } else if comp_size == file_size {
                            return Some(compressed_slice.to_vec());
                        }
                    }
                    1 => {
                        // Szyfrowane XChaCha20 + skompresowane ZSTD
                        let mut decrypted = compressed_slice.to_vec();
                        decrypt_xchacha20(&mut decrypted, &entry.nonce, &PACK_KEY);
                        if let Ok(decomp) = decompress_zstd(&decrypted, file_size) {
                            return Some(decomp);
                        } else if comp_size == file_size {
                            return Some(decrypted);
                        }
                    }
                    _ => return None,
                }
            }
        }
    }

    // 2. Fallback na CRC32 (dla kompatybilnosci ze starymi archiwami testowymi)
    let hash = calculate_crc32(file_name.as_bytes());
    let norm_hash = calculate_crc32(norm.as_bytes());
    let entry_opt = archive.index_map.get(&hash).or_else(|| archive.index_map.get(&norm_hash));
    if let Some(entry) = entry_opt {
        let start = entry.offset as usize;
        if let Some(end) = start.checked_add(entry.compressed_size as usize) {
            if end <= archive.data_mmap.len() {
                let slice = &archive.data_mmap[start..end];
                if let Ok(decomp) = decompress_zstd(slice, entry.uncompressed_size as usize) {
                    return Some(decomp);
                }
                return Some(slice.to_vec());
            }
        }
    }

    None
}

/// Narzedzie do tworzenia archiwum PCK z XChaCha20 i ZSTD
pub fn write_pck_archive(
    files: &[(&str, &[u8], bool)],
    output_path: &str,
) -> Result<(), String> {
    let mut entries_meta = Vec::new();
    let mut compressed_blobs = Vec::new();

    let mut current_offset = 0u64;
    for (name, data, encrypt) in files {
        let compressed = compress_zstd(data, 3).map_err(|e| e.to_string())?;
        let comp_len = compressed.len() as u64;

        let mut entry = TPackFileEntry::new();
        entry.set_file_name(name);
        entry.offset = current_offset;
        entry.file_size = data.len() as u64;
        entry.compressed_size = comp_len;
        entry.encryption = if *encrypt { 1 } else { 0 };

        let mut blob = compressed;
        if *encrypt {
            // Generuj deterministyczny lub pseudo-losowy nonce
            let nonce_hash = calculate_crc32(name.as_bytes());
            let mut nonce = [0u8; PACK_NONCE_SIZE];
            for i in 0..PACK_NONCE_SIZE {
                nonce[i] = (nonce_hash.wrapping_add(i as u32) & 0xFF) as u8;
            }
            entry.nonce = nonce;
            encrypt_xchacha20(&mut blob, &entry.nonce, &PACK_KEY);
        }

        compressed_blobs.push(blob);
        entries_meta.push(entry);
        current_offset += comp_len;
    }

    let header_size = std::mem::size_of::<TPackFileHeader>() as u64;
    let entries_size = (entries_meta.len() * std::mem::size_of::<TPackFileEntry>()) as u64;
    let data_begin = header_size + entries_size;

    let header_nonce = [0x5Au8; PACK_NONCE_SIZE];
    let header = TPackFileHeader {
        entry_num: entries_meta.len() as u64,
        data_begin,
        nonce: header_nonce,
    };

    let mut out_file = File::create(output_path).map_err(|e| e.to_string())?;

    // Zapis naglowka
    let header_bytes = unsafe {
        std::slice::from_raw_parts(
            &header as *const TPackFileHeader as *const u8,
            std::mem::size_of::<TPackFileHeader>(),
        )
    };
    out_file.write_all(header_bytes).map_err(|e| e.to_string())?;

    // Zapis zaszyfrowanych wpisow
    for entry in &entries_meta {
        let mut entry_copy = *entry;
        let entry_slice = unsafe {
            std::slice::from_raw_parts_mut(
                &mut entry_copy as *mut TPackFileEntry as *mut u8,
                std::mem::size_of::<TPackFileEntry>(),
            )
        };
        encrypt_xchacha20(entry_slice, &header.nonce, &PACK_KEY);
        out_file.write_all(entry_slice).map_err(|e| e.to_string())?;
    }

    // Zapis blobów danych
    for blob in &compressed_blobs {
        out_file.write_all(blob).map_err(|e| e.to_string())?;
    }

    Ok(())
}
