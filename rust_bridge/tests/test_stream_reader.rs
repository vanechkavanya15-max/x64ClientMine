// Testy modulu stream_reader - test fragmentacji TCP i sklejania pakietow.
// Komentarze polskie bez polskich znakow diakrytycznych.

use m2_client_bridge::net::stream_reader::{PacketStreamReader, STREAM_READER};
use m2_client_bridge::tea::tea_encrypt;

#[test]
fn test_tcp_fragmentation() {
    let mut reader = PacketStreamReader::new();
    let tea_key = [0x11111111, 0x22222222, 0x33333333, 0x44444444];
    reader.set_tea_key(tea_key);

    // Pakiet STREAM_READER o rozmiarze 13 bajtow
    let mut dummy_packet = vec![STREAM_READER];
    dummy_packet.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    
    // Zgodnie z formatem TEA wyrownujemy do wielokrotnosci 8 bajtow -> 16 bajtow
    let mut padded = dummy_packet.clone();
    padded.extend_from_slice(&[0, 0, 0]);
    
    // Szyfrujemy calosc
    let mut encrypted = vec![0u8; 16];
    tea_encrypt(&mut encrypted, &padded, &tea_key);

    // Test fragmentacji - dzielimy na 5, 5 i 6 bajtow
    reader.append_bytes(&encrypted[0..5]);
    assert!(reader.read_next_packet().is_none(), "Nie powinno byc pakietu, za malo bajtow");
    
    reader.append_bytes(&encrypted[5..10]);
    assert!(reader.read_next_packet().is_none(), "Nadal brakuje bajtow do pelnego zdekodowania pakietu");

    reader.append_bytes(&encrypted[10..16]);
    let packet = reader.read_next_packet();
    
    assert!(packet.is_some(), "Pakiet powinien zostac poprawnie odczytany");
    let packet_data = packet.unwrap();
    assert_eq!(packet_data.len(), 13, "Rozmiar pakietu musi wynosic dokladnie 13 bajtow");
    assert_eq!(packet_data, dummy_packet, "Odczytane dane musza byc zgodne z oryginalnym pakietem");
}

#[test]
fn test_two_packets_concat() {
    let mut reader = PacketStreamReader::new();
    let tea_key = [0xAAAAAAAA, 0xBBBBBBBB, 0xCCCCCCCC, 0xDDDDDDDD];
    reader.set_tea_key(tea_key);

    // Dwa pakiety STREAM_READER po 13 bajtow kazdy
    let mut packet1 = vec![STREAM_READER];
    packet1.extend_from_slice(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120]);
    
    let mut packet2 = vec![STREAM_READER];
    packet2.extend_from_slice(&[15, 25, 35, 45, 55, 65, 75, 85, 95, 105, 115, 125]);
    
    let mut both_packets = packet1.clone();
    both_packets.extend_from_slice(&packet2);
    
    // Obie wiadomosci daja lacznie 26 bajtow. Wyrownujemy do wielokrotnosci 8 -> 32 bajty.
    let mut padded = both_packets.clone();
    padded.extend_from_slice(&[0, 0, 0, 0, 0, 0]);
    
    let mut encrypted = vec![0u8; 32];
    tea_encrypt(&mut encrypted, &padded, &tea_key);

    // Symulujemy odebranie 32 bajtow w jednym bloku
    reader.append_bytes(&encrypted);

    // Sprawdzamy pierwszy pakiet
    let first = reader.read_next_packet();
    assert!(first.is_some(), "Pierwszy pakiet musi zostac odczytany");
    let first_data = first.unwrap();
    assert_eq!(first_data, packet1, "Zawartosc pierwszego pakietu musi byc prawidlowa");

    // Sprawdzamy drugi pakiet
    let second = reader.read_next_packet();
    assert!(second.is_some(), "Drugi pakiet musi zostac odczytany");
    let second_data = second.unwrap();
    assert_eq!(second_data, packet2, "Zawartosc drugiego pakietu musi byc prawidlowa");
    
    // Nie powinno byc juz nic wiecej (mimo nadmiarowych zdekodowanych danych TEA,
    // nastepny read zwroci None przez brak wystarczajacej dlugosci czy zly naglowek 0)
    // - poniewaz naglowek w wyrownaniu bedzie 0 (co oczekuje 13 bajtow), a zostalo 6 bajtow.
    assert!(reader.read_next_packet().is_none(), "Brak kolejnych pakietow");
}
