//! Testy jednostkowe dla serializatorow pakietow CG Handshake
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use m2_client_bridge::net::cg::handshake::{PacketCgPong, PacketCgHandshake};

#[test]
fn test_cg_pong_serialization() {
    let packet = PacketCgPong::new();
    let bytes = packet.to_bytes();
    
    // Sprawdzenie rozmiaru (header u16 + length u16 = 4 bajty)
    assert_eq!(bytes.len(), 4, "Rozmiar pakietu PONG powinien wynosic 4 bajty.");
    
    // Sprawdzenie poprawnosci naglowka (0x0006)
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), 0x0006);
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 4);
}

#[test]
fn test_cg_handshake_serialization() {
    // Generowanie pakietu z typowymi testowymi wartosciami
    let packet = PacketCgHandshake::new(0x11223344, 0xAABBCCDD, -500);
    let bytes = packet.to_bytes();
    
    // Rozmiar = 2 (header) + 2 (length) + 4 (handshake) + 4 (time) + 4 (delta) = 16
    assert_eq!(bytes.len(), 16, "Rozmiar pakietu HANDSHAKE powinien wynosic 16 bajtow.");
    
    // Sprawdzenie naglowka
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), 0x00FF);
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 16);
    
    // Sprawdzenie pola handshake (Little-Endian)
    assert_eq!(&bytes[4..8], &0x11223344u32.to_le_bytes());
    
    // Sprawdzenie pola time (Little-Endian)
    assert_eq!(&bytes[8..12], &0xAABBCCDDu32.to_le_bytes());
    
    // Sprawdzenie pola delta (Little-Endian)
    assert_eq!(&bytes[12..16], &(-500i32).to_le_bytes());
}

#[test]
fn test_cg_handshake_time_sync_serialization() {
    let packet = PacketCgHandshake::new_time_sync(0x99887766, 0x55443322, 100);
    let bytes = packet.to_bytes();
    
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), 0x00FC);
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 16);
    assert_eq!(&bytes[4..8], &0x99887766u32.to_le_bytes());
    assert_eq!(&bytes[8..12], &0x55443322u32.to_le_bytes());
    assert_eq!(&bytes[12..16], &100i32.to_le_bytes());
}
