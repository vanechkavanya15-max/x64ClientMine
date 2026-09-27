use m2_client::vfs_pck_memory_mapped::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use serial_test::serial;

fn get_temp_pck_path() -> PathBuf {
    let mut dir = std::env::temp_dir();
    dir.push("test_archive.pck");
    dir
}

fn create_dummy_pck(path: &PathBuf) {
    let mut file = File::create(path).expect("Failed to create dummy PCK");
    
    // Magic bytes
    file.write_all(b"PCK1").unwrap();
    
    // Num entries: 2
    let num_entries: u32 = 2;
    file.write_all(&num_entries.to_le_bytes()).unwrap();
    
    // Entries: 20 bytes each
    // Entry 1: file1.txt
    let hash1 = calculate_crc32(b"file1.txt");
    let offset1: u64 = 48; // 4 + 4 + (2 * 20) = 48
    let comp_size1: u32 = 12; // "Hello World!"
    let uncomp_size1: u32 = 12;
    
    file.write_all(&hash1.to_le_bytes()).unwrap();
    file.write_all(&offset1.to_le_bytes()).unwrap();
    file.write_all(&comp_size1.to_le_bytes()).unwrap();
    file.write_all(&uncomp_size1.to_le_bytes()).unwrap();
    
    // Entry 2: file2.txt
    let hash2 = calculate_crc32(b"file2.txt");
    let offset2: u64 = 60; // 48 + 12
    let comp_size2: u32 = 11; // "Rust Engine"
    let uncomp_size2: u32 = 11;
    
    file.write_all(&hash2.to_le_bytes()).unwrap();
    file.write_all(&offset2.to_le_bytes()).unwrap();
    file.write_all(&comp_size2.to_le_bytes()).unwrap();
    file.write_all(&uncomp_size2.to_le_bytes()).unwrap();
    
    // Data blobs
    file.write_all(b"Hello World!").unwrap();
    file.write_all(b"Rust Engine").unwrap();
}

#[test]
#[serial]
fn test_crc32_calculation() {
    let hash = calculate_crc32(b"test_string");
    assert_eq!(hash, 0x0967B587);
    
    let hash2 = calculate_crc32(b"file1.txt");
    assert_eq!(hash2, 0x2D212004); // Verified standard CRC32
}

#[test]
#[serial]
fn test_mount_valid_archive() {
    let path = get_temp_pck_path();
    create_dummy_pck(&path);
    
    let archive = mount_archive(path.to_str().unwrap()).expect("Failed to mount valid archive");
    
    assert_eq!(archive.index_map.len(), 2);
    assert!(archive.index_map.contains_key(&calculate_crc32(b"file1.txt")));
    assert!(archive.index_map.contains_key(&calculate_crc32(b"file2.txt")));
    
    let _ = std::fs::remove_file(path);
}

#[test]
#[serial]
fn test_read_existing_file() {
    let path = get_temp_pck_path();
    create_dummy_pck(&path);
    
    let archive = mount_archive(path.to_str().unwrap()).unwrap();
    
    let data1 = read_file_from_archive(&archive, "file1.txt").expect("Should find file1.txt");
    assert_eq!(data1, b"Hello World!");
    
    let data2 = read_file_from_archive(&archive, "file2.txt").expect("Should find file2.txt");
    assert_eq!(data2, b"Rust Engine");
    
    let _ = std::fs::remove_file(path);
}

#[test]
#[serial]
fn test_read_non_existent_file() {
    let path = get_temp_pck_path();
    create_dummy_pck(&path);
    
    let archive = mount_archive(path.to_str().unwrap()).unwrap();
    
    let missing = read_file_from_archive(&archive, "missing.txt");
    assert!(missing.is_none());
    
    let _ = std::fs::remove_file(path);
}

#[test]
#[serial]
fn test_mount_invalid_magic() {
    let path = get_temp_pck_path();
    let mut file = File::create(&path).unwrap();
    file.write_all(b"BAD1").unwrap(); // wrong magic
    file.write_all(&[0, 0, 0, 0]).unwrap();
    
    let result = mount_archive(path.to_str().unwrap());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid PCK magic bytes");
    
    let _ = std::fs::remove_file(path);
}

#[test]
#[serial]
fn test_mount_non_existent_file() {
    let result = mount_archive("/tmp/definitely_not_a_real_file.pck");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to open file"));
}

#[test]
#[serial]
fn test_tpack_structures_and_xchacha20_zstd_archive() {
    let header_size = std::mem::size_of::<TPackFileHeader>();
    assert_eq!(header_size, 40); // 8 + 8 + 24

    let entry_size = std::mem::size_of::<TPackFileEntry>();
    assert_eq!(entry_size, 310); // 261 + 8 + 8 + 8 + 1 + 24

    let mut entry = TPackFileEntry::new();
    entry.set_file_name("root/ui.py");
    assert_eq!(entry.get_file_name(), "root/ui.py");

    let mut path = std::env::temp_dir();
    path.push("test_xchacha_zstd.pck");

    let files = [
        ("d:/ymir work/ui/intro.sub", b"Sample Unencrypted ZSTD texture data 12345".as_slice(), false),
        ("root/game.py", b"def GameMain(): print('Hello XChaCha20 Encrypted Python')".as_slice(), true),
    ];

    write_pck_archive(&files, path.to_str().unwrap()).expect("Should write official PCK archive");

    let archive = mount_archive(path.to_str().unwrap()).expect("Should mount official PCK archive");
    assert_eq!(archive.entries.len(), 2);
    let header = archive.header.unwrap();
    let entry_num = header.entry_num;
    assert_eq!(entry_num, 2);

    let data1 = read_file_from_archive(&archive, "d:/ymir work/ui/intro.sub").expect("Find intro.sub");
    assert_eq!(data1, b"Sample Unencrypted ZSTD texture data 12345");

    let data2 = read_file_from_archive(&archive, "root/game.py").expect("Find game.py");
    assert_eq!(data2, b"def GameMain(): print('Hello XChaCha20 Encrypted Python')");

    // Test lookup with backslashes and uppercase
    let data_norm1 = read_file_from_archive(&archive, "ROOT\\GAME.PY").expect("Find root\\game.py with backslash and case");
    assert_eq!(data_norm1, data2);

    let data_norm2 = read_file_from_archive(&archive, "D:\\YMIR WORK\\ui\\intro.sub").expect("Find intro.sub with backslash and case");
    assert_eq!(data_norm2, data1);

    let _ = std::fs::remove_file(path);
}

#[test]
#[serial]
fn test_corrupted_pck_overflow_protection() {
    let mut path = std::env::temp_dir();
    path.push("test_overflow_corrupted.pck");

    let mut file = File::create(&path).unwrap();
    // entry_num = u64::MAX, data_begin = 40, nonce = 24 zeros
    file.write_all(&u64::MAX.to_le_bytes()).unwrap();
    file.write_all(&40u64.to_le_bytes()).unwrap();
    file.write_all(&[0u8; 24]).unwrap();

    let res = mount_archive(path.to_str().unwrap());
    assert!(res.is_err(), "Should safely reject archive with huge entry_num without panicking on overflow");

    let _ = std::fs::remove_file(path);
}

#[test]
#[serial]
fn test_truncated_pck_header() {
    let mut path = std::env::temp_dir();
    path.push("test_truncated_pck.pck");

    let mut file = File::create(&path).unwrap();
    // Write 20 bytes (less than 40B header)
    file.write_all(&[0xAB; 20]).unwrap();

    let res = mount_archive(path.to_str().unwrap());
    assert!(res.is_err(), "Should reject file smaller than 40-byte header");

    let _ = std::fs::remove_file(path);
}

#[test]
#[serial]
fn test_pck_out_of_bounds_entry() {
    let mut path = std::env::temp_dir();
    path.push("test_oob_entry.pck");

    let mut file = File::create(&path).unwrap();
    // Header: 1 entry, data_begin = 350, nonce
    let header_nonce = [0x11u8; 24];
    file.write_all(&1u64.to_le_bytes()).unwrap(); // entry_num = 1
    file.write_all(&350u64.to_le_bytes()).unwrap(); // data_begin = 350
    file.write_all(&header_nonce).unwrap();

    // Entry: offset = 0, file_size = 100, compressed_size = 100
    // Total file size will be 40 + 310 = 350 bytes, but entry data starts at 350 and length 100 -> end 450 > 350!
    let mut entry = TPackFileEntry::new();
    entry.set_file_name("test_oob.txt");
    entry.offset = 0;
    entry.file_size = 100;
    entry.compressed_size = 100;

    let mut entry_copy = entry;
    let entry_slice = unsafe {
        std::slice::from_raw_parts_mut(
            &mut entry_copy as *mut TPackFileEntry as *mut u8,
            std::mem::size_of::<TPackFileEntry>(),
        )
    };
    m2_client::vfs_security_decrypt::encrypt_xchacha20(entry_slice, &header_nonce, &m2_client::vfs_security_decrypt::PACK_KEY);
    file.write_all(entry_slice).unwrap();

    let res = mount_archive(path.to_str().unwrap());
    assert!(res.is_err(), "Should reject archive where entry data extends beyond file bounds");

    let _ = std::fs::remove_file(path);
}
