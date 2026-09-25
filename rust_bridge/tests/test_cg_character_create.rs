// Testy jednostkowe serializatora tworzenia postaci

use m2_client_bridge::net::cg::character_create::{TPacketCGPlayerCreate, m2_net_build_cg_player_create, m2_net_free_cg_player_create};
use std::ffi::CString;

#[test]
fn test_character_create_packet_serialization() {
    let mut name = [0u8; 25];
    let name_str = b"TestName";
    name[..name_str.len()].copy_from_slice(name_str);

    let packet = TPacketCGPlayerCreate {
        header: 4, // HEADER_CG_PLAYER_CREATE
        index: 1,
        name: name,
        job: 2,
        shape: 3,
        con: 10,
        int: 20,
        str: 30,
        dex: 40,
    };

    let bytes = packet.to_bytes();
    
    // Sprawdzenie ogolnego rozmiaru bufora (34 bajty)
    assert_eq!(bytes.len(), 34);
    
    // Sprawdzenie pol 1-bajtowych
    assert_eq!(bytes[0], 4);
    assert_eq!(bytes[1], 1);
    
    // Sprawdzenie tablicy znakow (nazwy postaci) zakonczonej zerami
    for i in 0..8 {
        assert_eq!(bytes[2 + i], name_str[i]);
    }
    for i in 8..25 {
        assert_eq!(bytes[2 + i], 0);
    }
    
    // Sprawdzenie pola 2-bajtowego w Little-Endian
    assert_eq!(bytes[27], 2);
    assert_eq!(bytes[28], 0);
    
    // Sprawdzenie pozostalych pol
    assert_eq!(bytes[29], 3);
    assert_eq!(bytes[30], 10);
    assert_eq!(bytes[31], 20);
    assert_eq!(bytes[32], 30);
    assert_eq!(bytes[33], 40);
}

#[test]
fn test_character_create_packet_new() {
    let packet = TPacketCGPlayerCreate::new(2, "TestNameLongerThanExpected", 1, 4, 15, 25, 35, 45);
    let bytes = packet.to_bytes();
    
    // Sprawdzenie ogolnego rozmiaru bufora (34 bajty)
    assert_eq!(bytes.len(), 34);
    
    assert_eq!(bytes[0], 4);
    assert_eq!(bytes[1], 2);
    
    // "TestNameLongerThanExpected" ma 26 znakow. `new` powinno uciac do 24 znakow.
    let expected_name = b"TestNameLongerThanExpect";
    for i in 0..24 {
        assert_eq!(bytes[2 + i], expected_name[i]);
    }
    assert_eq!(bytes[26], 0); // Null terminator
    
    // Sprawdzenie innych pol
    assert_eq!(bytes[27], 1);
    assert_eq!(bytes[28], 0);
    assert_eq!(bytes[29], 4);
    assert_eq!(bytes[30], 15);
    assert_eq!(bytes[31], 25);
    assert_eq!(bytes[32], 35);
    assert_eq!(bytes[33], 45);
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
    assert_eq!(out_len, 34);

    let bytes = unsafe { std::slice::from_raw_parts(out_buf, out_len) };
    assert_eq!(bytes[0], 4); // HEADER_CG_PLAYER_CREATE
    assert_eq!(bytes[1], 0); // index

    // Sprawdzenie nazwy
    let name_str = b"Warrior\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
    assert_eq!(&bytes[2..27], name_str);

    assert_eq!(bytes[27], 0); // job
    assert_eq!(bytes[28], 0);
    assert_eq!(bytes[29], 1); // shape
    assert_eq!(bytes[30], 90); // con
    assert_eq!(bytes[31], 10); // int
    assert_eq!(bytes[32], 80); // str
    assert_eq!(bytes[33], 20); // dex

    unsafe {
        m2_net_free_cg_player_create(out_buf, out_len);
    }
}

#[test]
fn test_character_create_packet_stat_ranges() {
    // Stat ranges are implicitly enforced by u8, but testing boundary logic
    let packet_min = TPacketCGPlayerCreate::new(0, "Min", 0, 0, 0, 0, 0, 0);
    assert_eq!(packet_min.con, 0);
    assert_eq!(packet_min.int, 0);
    assert_eq!(packet_min.str, 0);
    assert_eq!(packet_min.dex, 0);

    let packet_max = TPacketCGPlayerCreate::new(3, "Max", 3, 255, 255, 255, 255, 255);
    assert_eq!(packet_max.con, 255);
    assert_eq!(packet_max.int, 255);
    assert_eq!(packet_max.str, 255);
    assert_eq!(packet_max.dex, 255);
}
