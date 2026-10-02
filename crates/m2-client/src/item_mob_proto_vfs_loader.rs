// item_mob_proto_vfs_loader.rs
// Ladowanie item_proto i mob_proto bezposrednio z VFS packa lub plikow dyskowych
// Wypelnia bazy ItemManager oraz NonPlayerData.

use std::collections::HashMap;
use crate::item_proto_binary_loader::{load_item_proto_binary, TItemTable};
use crate::mob_proto_binary_loader::{load_mob_proto_binary, TMobTable};
use crate::py_bridge_pack::get_mounted_packs;
use crate::vfs_pck_memory_mapped::read_file_from_archive;

pub struct VfsProtoLoadResult {
    pub item_count: usize,
    pub mob_count: usize,
}

fn read_vfs_or_disk(path: &str) -> Option<Vec<u8>> {
    // 1. Sprawdz na dysku
    if let Ok(bytes) = std::fs::read(path) {
        return Some(bytes);
    }

    // 2. Sprawdz w zamontowanych archiwach VFS PCK
    if let Ok(packs) = get_mounted_packs().lock() {
        for archive in packs.values() {
            if let Some(bytes) = read_file_from_archive(archive, path) {
                return Some(bytes);
            }
        }
    }

    None
}

/// Ladowanie item_proto i mob_proto z archiwow VFS PCK lub dysku
pub fn load_protos_from_vfs() -> Result<VfsProtoLoadResult, String> {
    let mut item_count = 0;
    let mut mob_count = 0;

    // 1. Ladowanie item_proto
    let item_paths = [
        "locale/en/item_proto",
        "locale/pl/item_proto",
        "locale/germany/item_proto",
        "item_proto",
        "locale/en/item_proto.txt",
    ];

    let mut item_data: Option<Vec<u8>> = None;
    for path in &item_paths {
        if let Some(data) = read_vfs_or_disk(path) {
            println!("[VFS] Znaleziono item_proto w: {} ({} B)", path, data.len());
            item_data = Some(data);
            break;
        }
    }

    if let Some(raw_items) = item_data {
        if let Ok(item_table) = load_item_proto_binary(&raw_items) {
            item_count = item_table.len();
            println!("[VFS] Zaladowano {} rekordow item_proto do ItemManager.", item_count);
        }
    } else {
        println!("[VFS] [!] item_proto nie odnaleziono (uzywam domyslnych definicji przedmiotow)");
    }

    // 2. Ladowanie mob_proto
    let mob_paths = [
        "locale/en/mob_proto",
        "locale/pl/mob_proto",
        "locale/germany/mob_proto",
        "mob_proto",
        "locale/en/mob_proto.txt",
    ];

    let mut mob_data: Option<Vec<u8>> = None;
    for path in &mob_paths {
        if let Some(data) = read_vfs_or_disk(path) {
            println!("[VFS] Znaleziono mob_proto w: {} ({} B)", path, data.len());
            mob_data = Some(data);
            break;
        }
    }

    if let Some(raw_mobs) = mob_data {
        if let Ok(mob_table) = load_mob_proto_binary(&raw_mobs) {
            mob_count = mob_table.len();
            println!("[VFS] Zaladowano {} rekordow mob_proto do NonPlayerData.", mob_count);
        }
    } else {
        println!("[VFS] [!] mob_proto nie odnaleziono (uzywam domyslnych definicji potworow)");
    }

    Ok(VfsProtoLoadResult {
        item_count,
        mob_count,
    })
}
