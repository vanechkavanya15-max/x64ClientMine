// Testy jednostkowe serializatora tworzenia postaci
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use m2_client_bridge::net::cg::character_create::{
    TPacketCGCreateCharacter, m2_net_build_cg_player_create, m2_net_free_cg_player_create,
    CHARACTER_NAME_MAX_LEN,
};
use std::ffi::CString;

#[test]
fn test_character_create_packet_serialization() {
    let mut name = [0u8; CHARACTER_NAME_MAX_LEN + 1];
    let name_str = b"TestName";
    name[..name_str.len()].copy_from_slice(name_str);

    let packet = TPacketCGCreateCharacter {
        header: 0x0201, // CG::CHARACTER_CREATE
        length: 77,
        index: 1,
        name,
        job: 2,
        shape: 3,
        con: 10,
        int_stat: 20,
        str_stat: 30,
        dex: 40,
    };

    let bytes = packet.to_bytes();
    
    // Sprawdzenie ogolnego rozmiaru bufora (77 bajtow)
    assert_eq!(bytes.len(), 77);
    
    // Naglowek i dlugosc (u16 LE)
    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    assert_eq!(header, 0x0201);
    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    assert_eq!(length, 77);
    
    assert_eq!(bytes[4], 1); // index
    
    // Sprawdzenie tablicy znakow (nazwy postaci) zakonczonej zerami
    for i in 0..8 {
        assert_eq!(bytes[5 + i], name_str[i]);
    }
    for i in 8..65 {
        assert_eq!(bytes[5 + i], 0);
    }
    
    // Sprawdzenie pola 2-bajtowego w Little-Endian (job)
    let job = u16::from_le_bytes([bytes[70], bytes[71]]);
    assert_eq!(job, 2);
    
    // Sprawdzenie pozostalych pol
    assert_eq!(bytes[72], 3); // shape
    assert_eq!(bytes[73], 10); // con
    assert_eq!(bytes[74], 20); // int_stat
    assert_eq!(bytes[75], 30); // str_stat
    assert_eq!(bytes[76], 40); // dex
}

#[test]
fn test_character_create_packet_new() {
    let packet = TPacketCGCreateCharacter::new(2, "TestNameLongerThanExpected", 1, 4, 15, 25, 35, 45);
    let bytes = packet.to_bytes();
    
    // Sprawdzenie ogolnego rozmiaru bufora (77 bajtow)
    assert_eq!(bytes.len(), 77);
    
    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    assert_eq!(header, 0x0201);
    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    assert_eq!(length, 77);
    assert_eq!(bytes[4], 2);
    
    // Nazwa postaci
    let expected_name = b"TestNameLongerThanExpected";
    for i in 0..expected_name.len() {
        assert_eq!(bytes[5 + i], expected_name[i]);
    }
    
    // Sprawdzenie innych pol
    let job = u16::from_le_bytes([bytes[70], bytes[71]]);
    assert_eq!(job, 1);
    assert_eq!(bytes[72], 4);
    assert_eq!(bytes[73], 15);
    assert_eq!(bytes[74], 25);
    assert_eq!(bytes[75], 35);
    assert_eq!(bytes[76], 45);
}

#[test]
fn test_character_create_ffi() {
    let c_name = CString::new("Warrior").unwrap();
    let mut out_buf: *mut u8 = std::ptr::null_mut();
    let mut out_len: usize = 0;

    let success = unsafe {
        m2_net_build_cg_player_create(
            0,
            c_name.as_ptr(),
            0, // job: Warrior
            1, // shape
            90, // con
            10, // int
            80, // str
            20, // dex
            &mut out_buf,
            &mut out_len,
        )
    };

    assert!(success);
    assert!(!out_buf.is_null());
    assert_eq!(out_len, 77);

    let bytes = unsafe { std::slice::from_raw_parts(out_buf, out_len) };
    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    assert_eq!(header, 0x0201);
    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    assert_eq!(length, 77);
    assert_eq!(bytes[4], 0); // index

    // Sprawdzenie nazwy
    assert_eq!(&bytes[5..12], b"Warrior");
    assert_eq!(bytes[12], 0);

    let job = u16::from_le_bytes([bytes[70], bytes[71]]);
    assert_eq!(job, 0);
    assert_eq!(bytes[72], 1); // shape
    assert_eq!(bytes[73], 90); // con
    assert_eq!(bytes[74], 10); // int
    assert_eq!(bytes[75], 80); // str
    assert_eq!(bytes[76], 20); // dex

    unsafe {
        m2_net_free_cg_player_create(out_buf, out_len);
    }
}

#[test]
fn test_character_create_packet_stat_ranges() {
    let packet_min = TPacketCGCreateCharacter::new(0, "Min", 0, 0, 0, 0, 0, 0);
    let con_min = { packet_min.con };
    let int_min = { packet_min.int_stat };
    let str_min = { packet_min.str_stat };
    let dex_min = { packet_min.dex };
    assert_eq!(con_min, 0);
    assert_eq!(int_min, 0);
    assert_eq!(str_min, 0);
    assert_eq!(dex_min, 0);

    let packet_max = TPacketCGCreateCharacter::new(3, "Max", 3, 255, 255, 255, 255, 255);
    let con_max = { packet_max.con };
    let int_max = { packet_max.int_stat };
    let str_max = { packet_max.str_stat };
    let dex_max = { packet_max.dex };
    assert_eq!(con_max, 255);
    assert_eq!(int_max, 255);
    assert_eq!(str_max, 255);
    assert_eq!(dex_max, 255);
}
