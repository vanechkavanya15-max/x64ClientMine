use m2_client_bridge::vfs::eix_entry::{EixEntry, FILENAME_MAX_LEN};
use std::mem;

#[test]
fn test_eix_entry_layout() {
    // Sprawdz rozmiar i wyrownanie struktury (musi odpowiadac #pragma pack(4) i kompilacji z g++ 32-bit).
    assert_eq!(mem::size_of::<EixEntry>(), 192, "Rozmiar EixEntry musi wynosic 192 bajty");
    assert_eq!(mem::align_of::<EixEntry>(), 4, "Wyrownanie EixEntry musi wynosic 4 bajty");
}

#[test]
fn test_eix_entry_default() {
    let entry = EixEntry::default();
    assert_eq!(entry.id, 0);
    assert_eq!(entry.filename_crc, 0);
    assert_eq!(entry.real_data_size, 0);
    assert_eq!(entry.data_size, 0);
    assert_eq!(entry.data_crc, 0);
    assert_eq!(entry.data_position, 0);
    assert_eq!(entry.compressed_type, 0);
    assert_eq!(entry.filename[0], 0);
    assert_eq!(entry.get_filename(), "");
}

#[test]
fn test_eix_entry_filename() {
    let mut entry = EixEntry::default();
    
    // Normalna nazwa pliku
    let name = "d_ymir work/test_file.dds";
    entry.set_filename(name);
    assert_eq!(entry.get_filename(), name);
    
    // Zbyt dluga nazwa pliku (powinna zostac obcieta)
    let long_name = "a".repeat(200);
    entry.set_filename(&long_name);
    let expected_name = "a".repeat(FILENAME_MAX_LEN);
    assert_eq!(entry.get_filename(), expected_name);
    
    // Upewnij sie, ze znak null zostal ustawiony
    assert_eq!(entry.filename[FILENAME_MAX_LEN], 0);
}
