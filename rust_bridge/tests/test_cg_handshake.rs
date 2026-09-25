//! Testy jednostkowe dla serializatorow pakietow CG Handshake

use m2_client_bridge::net::cg::handshake::{PacketCgPong, PacketCgHandshake};

#[test]
fn test_cg_pong_serialization() {
    let packet = PacketCgPong::new();
    let bytes = packet.to_bytes();
    
    // Sprawdzenie ogolnego rozmiaru
    assert_eq!(bytes.len(), 1, "Rozmiar pakietu PONG powinien wynosic 1 bajt.");
    
    // Sprawdzenie poprawnosci naglowka
    assert_eq!(bytes[0], 0xfe, "Naglowek PONG powinien miec wartosc 0xfe.");
}

#[test]
fn test_cg_handshake_serialization() {
    // Generowanie pakietu z typowymi testowymi wartosciami
    let packet = PacketCgHandshake::new(0x11223344, 0xAABBCCDD, -500);
    let bytes = packet.to_bytes();
    
    // Rozmiar = 1 (header) + 4 (handshake) + 4 (time) + 4 (delta) = 13
    assert_eq!(bytes.len(), 13, "Rozmiar pakietu HANDSHAKE powinien wynosic 13 bajtow.");
    
    // Sprawdzenie naglowka
    assert_eq!(bytes[0], 0xff, "Naglowek HANDSHAKE powinien miec wartosc 0xff.");
    
    // Sprawdzenie pola handshake (Little-Endian)
    assert_eq!(&bytes[1..5], &0x11223344u32.to_le_bytes(), "Pole handshake nie jest poprawne w formacie Little-Endian.");
    
    // Sprawdzenie pola time (Little-Endian)
    assert_eq!(&bytes[5..9], &0xAABBCCDDu32.to_le_bytes(), "Pole time nie jest poprawne w formacie Little-Endian.");
    
    // Sprawdzenie pola delta (Little-Endian z uwzglednieniem znaku)
    assert_eq!(&bytes[9..13], &(-500i32).to_le_bytes(), "Pole delta nie jest poprawne w formacie Little-Endian.");
}

#[test]
fn test_cg_handshake_time_sync_serialization() {
    // Pakiet typu Time Sync uzywa innego naglowka
    let packet = PacketCgHandshake::new_time_sync(0x99887766, 0x55443322, 100);
    let bytes = packet.to_bytes();
    
    // Sprawdzenie naglowka (HEADER_CG_TIME_SYNC = 0xfc)
    assert_eq!(bytes[0], 0xfc, "Naglowek TIME_SYNC powinien miec wartosc 0xfc.");
    
    // Sprawdzenie pola handshake
    assert_eq!(&bytes[1..5], &0x99887766u32.to_le_bytes());
    
    // Sprawdzenie pola time
    assert_eq!(&bytes[5..9], &0x55443322u32.to_le_bytes());
    
    // Sprawdzenie pola delta
    assert_eq!(&bytes[9..13], &100i32.to_le_bytes());
}
