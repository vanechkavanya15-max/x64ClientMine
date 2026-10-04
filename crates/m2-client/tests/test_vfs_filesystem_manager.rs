use m2_client::vfs_virtual_filesystem_manager::VfsManager;
use m2_client::vfs_pck_memory_mapped::{TPackFileHeader, TPackFileEntry, PckArchive};
use std::collections::HashMap;

fn create_dummy_archive(name: &str, files: Vec<(&str, Vec<u8>)>) -> PckArchive {
    let mut name_map = HashMap::new();
    let index_map = HashMap::new();
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
        
        data_mmap.extend_from_slice(&data);
        offset += data.len();
        entries.push(entry);
    }
    
    PckArchive {
        file_path: name.to_string(),
        header: Some(TPackFileHeader { entry_num: 0, data_begin: 0, nonce: [0; 24] }), 
        entries,
        name_map,
        index_map,
        data_mmap,
    }
}

#[test]
fn test_vfs_lru_cache() {
    let mut vfs = VfsManager::new();
    vfs.max_cache_size = 10; 
    
    let pack = create_dummy_archive("test.pck", vec![
        ("file1.txt", vec![1, 2, 3]),
        ("file2.txt", vec![4, 5, 6]),
        ("file3.txt", vec![7, 8, 9, 10, 11]),
    ]);
    vfs.std_packs.push(pack);
    
    let _d1 = vfs.get_file_bytes("file1.txt").unwrap();
    assert_eq!(vfs.current_cache_size, 3);
    
    let _d2 = vfs.get_file_bytes("file2.txt").unwrap();
    assert_eq!(vfs.current_cache_size, 6);
    
    let _d3 = vfs.get_file_bytes("file3.txt").unwrap();
    assert_eq!(vfs.current_cache_size, 8); 
    
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
    
    let d1 = vfs.get_file_bytes("test.txt").unwrap();
    assert_eq!(d1.as_ref(), &vec![1, 1, 1]);
    
    vfs.clear_cache(); 
    
    vfs.mod_packs.push(mod_pack);
    
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
    
    let d = vfs.get_file_bytes("ui/test.txt");
    assert!(d.is_some());
    assert_eq!(d.unwrap().as_ref(), &vec![9, 9, 9]);
    
    vfs.clear_cache();
    let d2 = vfs.get_file_bytes("UI/TEST.TXT");
    assert!(d2.is_some());
    assert_eq!(d2.unwrap().as_ref(), &vec![9, 9, 9]);
}

#[test]
fn test_vfs_disk_priority() {
    use std::io::Write;
    let mut vfs = VfsManager::new();
    let file_path = "test_disk_priority.txt";
    
    let mut f = std::fs::File::create(file_path).unwrap();
    f.write_all(&[5, 5, 5]).unwrap();
    
    let std_pack = create_dummy_archive("std.pck", vec![
        ("test_disk_priority.txt", vec![1, 1, 1]),
    ]);
    vfs.std_packs.push(std_pack);
    
    let d = vfs.get_file_bytes("test_disk_priority.txt");
    assert!(d.is_some());
    assert_eq!(d.unwrap().as_ref(), &vec![5, 5, 5]);
    
    std::fs::remove_file(file_path).unwrap();
}
