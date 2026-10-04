#[path = "../src/vfs_virtual_filesystem_manager.rs"]
mod vfs_virtual_filesystem_manager;

#[path = "../src/vfs_pck_memory_mapped.rs"]
mod vfs_pck_memory_mapped;

#[path = "../src/vfs_security_decrypt.rs"]
mod vfs_security_decrypt;

use vfs_virtual_filesystem_manager::VfsManager;
use vfs_pck_memory_mapped::{TPackFileHeader, TPackFileEntry, PckArchive};
use std::collections::HashMap;

fn create_dummy_archive(name: &str, files: Vec<(&str, Vec<u8>)>) -> PckArchive {
    let mut name_map = HashMap::new();
    let mut index_map = HashMap::new();
    let mut entries = Vec::new();
    let mut data_mmap = Vec::new();
    
    let mut offset = 0;
    for (i, (file_name, data)) in files.into_iter().enumerate() {
        let mut entry = TPackFileEntry::new();
        entry.set_file_name(file_name);
        entry.file_size = data.len() as u64;
        entry.compressed_size = data.len() as u64;
        entry.offset = offset as u64;
        
        name_map.insert(file_name.to_lowercase(), i);
        
        // Wypełniamy bufor danymi z testów 
        data_mmap.extend_from_slice(&data);
        offset += data.len();
        entries.push(entry);
    }
    
    PckArchive {
        file_path: name.to_string(),
        header: Some(TPackFileHeader { entry_num: 0, data_begin: 0, nonce: [0; 24] }), // uproszczenie
        entries,
        name_map,
        index_map,
        data_mmap,
    }
}

#[test]
fn test_vfs_lru_cache() {
    let mut vfs = VfsManager::new();
    vfs.max_cache_size = 10; // small cache
    
    // Ustawienie base path na istniejący w systemie plik (może to być sam plik testowy)
    let test_file = "Cargo.toml"; 
    // to przeczyta rzeczywisty plik "Cargo.toml" jak jest na dysku
    
    // Ale w tescie cache stworzymy własne pakiety, zeby to z nich korzystać
    let pack = create_dummy_archive("test.pck", vec![
        ("file1.txt", vec![1, 2, 3]),
        ("file2.txt", vec![4, 5, 6]),
        ("file3.txt", vec![7, 8, 9, 10, 11]), // 5 bajtów
    ]);
    vfs.std_packs.push(pack);
    
    // File 1 cache (3 bajty, w cache)
    let d1 = vfs.get_file_bytes("file1.txt").unwrap();
    assert_eq!(vfs.current_cache_size, 3);
    
    // File 2 cache (3 bajty, lacznie 6)
    let d2 = vfs.get_file_bytes("file2.txt").unwrap();
    assert_eq!(vfs.current_cache_size, 6);
    
    // File 3 cache (5 bajtow, przekroczy 10 bajtow)
    // 6 + 5 = 11 > 10, usunie file1 (najstarszy)
    let d3 = vfs.get_file_bytes("file3.txt").unwrap();
    assert_eq!(vfs.current_cache_size, 8); // 3 (file2) + 5 (file3)
    
    assert!(vfs.file_cache.contains_key("file2.txt"));
    assert!(vfs.file_cache.contains_key("file3.txt"));
    assert!(!vfs.file_cache.contains_key("file1.txt"));
}

#[test]
fn test_vfs_priority() {
    let mut vfs = VfsManager::new();
    
    let std_pack = create_dummy_archive("std.pck", vec![
        ("test.txt", vec![1, 1, 1]),
    ]);
    
    let mod_pack = create_dummy_archive("mod.pck", vec![
        ("test.txt", vec![2, 2, 2]),
    ]);
    
    vfs.std_packs.push(std_pack);
    
    // powinno zwrocic ze std_pack
    let d1 = vfs.get_file_bytes("test.txt").unwrap();
    assert_eq!(d1.as_ref(), &vec![1, 1, 1]);
    
    vfs.clear_cache(); // czyscimy cache zeby nie korzystalo
    
    vfs.mod_packs.push(mod_pack);
    
    // teraz priorytet ma mod_pack
    let d2 = vfs.get_file_bytes("test.txt").unwrap();
    assert_eq!(d2.as_ref(), &vec![2, 2, 2]);
}

#[test]
fn test_vfs_file_case_insensitivity() {
    let mut vfs = VfsManager::new();
    let pack = create_dummy_archive("std.pck", vec![
        ("UI/Test.txt", vec![9, 9, 9]),
    ]);
    vfs.std_packs.push(pack);
    
    // Test should find file with lowercase even if stored with uppercase
    // Wait, the dummy archive stores exactly what we pass to `set_file_name`. 
    // And `name_map` keys are `to_lowercase()`.
    
    let d = vfs.get_file_bytes("ui/test.txt");
    assert!(d.is_some());
    assert_eq!(d.unwrap().as_ref(), &vec![9, 9, 9]);
    
    vfs.clear_cache();
    let d2 = vfs.get_file_bytes("UI/TEST.TXT"); // should also normalize
    assert!(d2.is_some());
    assert_eq!(d2.unwrap().as_ref(), &vec![9, 9, 9]);
}

