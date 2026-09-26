use std::fs::File;
use std::io::Write;
use std::os::raw::c_char;
use std::ffi::CString;
use std::ptr;

use m2_client_bridge::vfs::epk_reader::{
    m2_epk_reader_open, m2_epk_reader_read, m2_epk_reader_close, EpkReader,
};

#[test]
fn test_epk_reader_basic_read() {
    // 1. Stworz plik testowy.
    let path = "test_epk_basic.epk";
    {
        let mut f = File::create(path).unwrap();
        f.write_all(b"HELLO_METIN2_WORLD").unwrap();
    }

    // 2. Przetestuj otwieranie pliku.
    let reader = EpkReader::open(path).expect("Nie udalo sie otworzyc pliku testowego.");
    
    // 3. Sprawdz odczyt.
    let mut reader = reader;
    let mut buf = vec![0u8; 6];
    
    // Odczytaj "METIN2" z offsetu 6
    let bytes_read = reader.read_block(6, &mut buf).expect("Blad odczytu");
    assert_eq!(bytes_read, 6);
    assert_eq!(&buf, b"METIN2");

    // Odczytaj "WORLD" z offsetu 13
    let mut buf_world = vec![0u8; 5];
    let bytes_read_world = reader.read_block(13, &mut buf_world).expect("Blad odczytu");
    assert_eq!(bytes_read_world, 5);
    assert_eq!(&buf_world, b"WORLD");

    // 4. Usun plik testowy.
    std::fs::remove_file(path).unwrap();
}

#[test]
fn test_epk_reader_ffi() {
    // 1. Stworz plik testowy.
    let path_str = "test_epk_ffi.epk";
    {
        let mut f = File::create(path_str).unwrap();
        f.write_all(b"RUST_FFI_TESTING").unwrap();
    }

    let c_path = CString::new(path_str).unwrap();

    // 2. Przetestuj m2_epk_reader_open.
    let reader_ptr = m2_epk_reader_open(c_path.as_ptr());
    assert!(!reader_ptr.is_null());

    // 3. Przetestuj m2_epk_reader_read.
    let mut buf = vec![0u8; 3];
    // Odczytaj "FFI" z offsetu 5
    let bytes_read = m2_epk_reader_read(reader_ptr, 5, buf.as_mut_ptr(), 3);
    assert_eq!(bytes_read, 3);
    assert_eq!(&buf, b"FFI");
    
    // Odczytaj poza plikiem.
    let mut big_buf = vec![0u8; 100];
    let bytes_read_end = m2_epk_reader_read(reader_ptr, 10, big_buf.as_mut_ptr(), 100);
    // Dostepnych bajtow jest od indeksu 10 do 16 ("ESTING"), co daje 6 bajtow.
    assert_eq!(bytes_read_end, 6);
    assert_eq!(&big_buf[0..6], b"ESTING");

    // 4. Przetestuj zamykanie.
    m2_epk_reader_close(reader_ptr);
    
    // Zabezpieczenie przed uzyciem zamknietego pointera zalezy od uzytkownika C++,
    // my tylko czyscimy plik testowy.
    let _ = std::fs::remove_file(path_str);
}

#[test]
fn test_epk_reader_ffi_invalid_args() {
    // 1. Null w open.
    let null_ptr: *const c_char = ptr::null();
    let reader_ptr = m2_epk_reader_open(null_ptr);
    assert!(reader_ptr.is_null());

    // 2. Nieistniejacy plik w open.
    let c_path_invalid = CString::new("this_file_does_not_exist_123.epk").unwrap();
    let reader_ptr_invalid = m2_epk_reader_open(c_path_invalid.as_ptr());
    assert!(reader_ptr_invalid.is_null());

    // 3. Null argumenty w read.
    // Tworzymy dummy plik do poprawnego otwarcia (by reader byl nienull).
    let path_str = "test_epk_ffi_invalid.epk";
    {
        let mut f = File::create(path_str).unwrap();
        f.write_all(b"123").unwrap();
    }
    let c_path = CString::new(path_str).unwrap();
    let valid_reader = m2_epk_reader_open(c_path.as_ptr());
    assert!(!valid_reader.is_null());

    let mut buf = vec![0u8; 10];
    
    // Null reader
    let read_null_reader = m2_epk_reader_read(ptr::null_mut(), 0, buf.as_mut_ptr(), 10);
    assert_eq!(read_null_reader, 0);

    // Null buffer
    let read_null_buf = m2_epk_reader_read(valid_reader, 0, ptr::null_mut(), 10);
    assert_eq!(read_null_buf, 0);
    
    // Rozmiar 0
    let read_zero_len = m2_epk_reader_read(valid_reader, 0, buf.as_mut_ptr(), 0);
    assert_eq!(read_zero_len, 0);

    m2_epk_reader_close(valid_reader);
    
    // Null reader podany do close nie powinien craslowac
    m2_epk_reader_close(ptr::null_mut());

    let _ = std::fs::remove_file(path_str);
}
