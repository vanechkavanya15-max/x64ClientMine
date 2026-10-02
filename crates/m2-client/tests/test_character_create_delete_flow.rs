use m2_client::character_create_delete_flow::{
    create_character, delete_character, CreateCharacterError, DeleteCharacterError, Job,
    TPacketCGCreateCharacter, TPacketCGDestroyCharacter, TPacketGCPlayerCreateSuccess,
    TPacketGCCreateFailure, TPacketGCDestroyCharacterSuccess, TPacketGCBlank,
    TSimplePlayerInformation, CHARACTER_NAME_MAX_LEN, PRIVATE_CODE_LENGTH,
};
use std::mem::size_of;

#[test]
fn test_create_character_valid() {
    let result = create_character(1, "ValidName1", Job::WarriorM, 0);
    assert!(result.is_ok());
    let packet = result.unwrap();

    let header = { packet.header };
    let length = { packet.length };
    let index = { packet.index };
    let job = { packet.job };
    let shape = { packet.shape };
    let con = { packet.con };
    let int = { packet.int };
    let str = { packet.str };
    let dex = { packet.dex };

    assert_eq!(header, 0x0201);
    assert_eq!(length, size_of::<TPacketCGCreateCharacter>() as u16);
    assert_eq!(index, 1);
    assert_eq!(job, Job::WarriorM as u16);
    assert_eq!(shape, 0);

    let mut expected_name = [0u8; CHARACTER_NAME_MAX_LEN + 1];
    expected_name[..10].copy_from_slice(b"ValidName1");
    let name_bytes = { packet.name };
    assert_eq!(name_bytes, expected_name);

    assert_eq!(con, 4);
    assert_eq!(int, 3);
    assert_eq!(str, 6);
    assert_eq!(dex, 3);
}

#[test]
fn test_create_character_invalid_name_length() {
    let result_short = create_character(0, "ab", Job::NinjaF, 1);
    match result_short {
        Err(CreateCharacterError::InvalidNameLength) => {},
        _ => panic!("Expected InvalidNameLength"),
    }

    let result_long = create_character(0, "VeryLongName123", Job::SuraM, 0);
    match result_long {
        Err(CreateCharacterError::InvalidNameLength) => {},
        _ => panic!("Expected InvalidNameLength"),
    }
}

#[test]
fn test_create_character_invalid_name_characters() {
    let result = create_character(2, "Test!@#", Job::ShamanF, 0);
    match result {
        Err(CreateCharacterError::InvalidNameCharacters) => {},
        _ => panic!("Expected InvalidNameCharacters"),
    }
}

#[test]
fn test_delete_character_valid() {
    let result = delete_character(3, "1234567");
    assert!(result.is_ok());
    let packet = result.unwrap();

    let header = { packet.header };
    let length = { packet.length };
    let index = { packet.index };

    assert_eq!(header, 0x0202);
    assert_eq!(length, size_of::<TPacketCGDestroyCharacter>() as u16);
    assert_eq!(index, 3);

    let mut expected_code = [0u8; PRIVATE_CODE_LENGTH];
    expected_code[..7].copy_from_slice(b"1234567");
    
    let sz_code = { packet.szPrivateCode };
    assert_eq!(sz_code, expected_code);
}

#[test]
fn test_delete_character_invalid_pin_length() {
    let result_short = delete_character(0, "123456");
    match result_short {
        Err(DeleteCharacterError::InvalidPinLength) => {},
        _ => panic!("Expected InvalidPinLength"),
    }

    let result_long = delete_character(0, "12345678");
    match result_long {
        Err(DeleteCharacterError::InvalidPinLength) => {},
        _ => panic!("Expected InvalidPinLength"),
    }
}

#[test]
fn test_packet_sizes() {
    assert_eq!(size_of::<TPacketCGCreateCharacter>(), 77);
    assert_eq!(size_of::<TPacketCGDestroyCharacter>(), 13);
    assert_eq!(size_of::<TSimplePlayerInformation>(), 103);
    assert_eq!(size_of::<TPacketGCPlayerCreateSuccess>(), 108);
    assert_eq!(size_of::<TPacketGCCreateFailure>(), 5);
    assert_eq!(size_of::<TPacketGCDestroyCharacterSuccess>(), 5);
    assert_eq!(size_of::<TPacketGCBlank>(), 4);
}
