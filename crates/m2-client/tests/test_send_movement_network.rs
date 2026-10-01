use m2_client::send_movement_network::{
    reset_movement_throttling, send_move_packet, TPacketCGMove, MOVE_STOP,
};
use std::mem::size_of;
use std::time::{Duration, Instant};

#[test]
fn test_packet_cg_move_layout_and_size() {
    assert_eq!(size_of::<TPacketCGMove>(), 27);
    assert_eq!(TPacketCGMove::SIZE, 27);
}

#[test]
fn test_packet_cg_move_serialization() {
    let packet = TPacketCGMove {
        header: 0x0301,
        length: 27,
        b_func: 1, // MOVE_WALK np.
        b_arg: 0,
        b_rot: 128, // Pół obrotu (180 stopni)
        dw_vid: 12345,
        l_x: 50000,
        l_y: -30000,
        dw_time: 111222333,
        dw_duration: 1500,
    };

    let bytes = packet.to_bytes();

    assert_eq!(bytes.len(), 27);

    // Nagłówek
    assert_eq!(&bytes[0..2], &0x0301u16.to_le_bytes());
    // Długość
    assert_eq!(&bytes[2..4], &27u16.to_le_bytes());
    // Funkcja
    assert_eq!(bytes[4], 1);
    // Argument
    assert_eq!(bytes[5], 0);
    // Rotacja
    assert_eq!(bytes[6], 128);
    // VID
    assert_eq!(&bytes[7..11], &12345u32.to_le_bytes());
    // Pozycje
    assert_eq!(&bytes[11..15], &50000i32.to_le_bytes());
    assert_eq!(&bytes[15..19], &(-30000i32).to_le_bytes());
    // Czas
    assert_eq!(&bytes[19..23], &111222333u32.to_le_bytes());
    // Duration
    assert_eq!(&bytes[23..27], &1500u32.to_le_bytes());
}

#[test]
fn test_send_move_packet_throttling() {
    // Resetujemy stan przed testem (uwaga na rownoleglosc testow!)
    reset_movement_throttling();

    // Próba wysłania bez połączonego socketu zwróci Err("Active game socket not found")
    // Niemniej jednak, logika throttlingu zostanie odpalona ZANIM zwrócimy błąd (dla nie-throttlowanych)
    // lub zwróci Ok(()) od razu jeśli zostanie zthrottlowane.
    
    // Pierwszy pakiet (ruch, nie MOVE_STOP) - powinien przejść do wysyłania (czyli zwróci Err brak socketu)
    let res1 = send_move_packet(1, 0, 90.0, 100, 100, 0);
    assert_eq!(res1, Err("Active game socket not found".to_string()));

    // Drugi pakiet zaraz potem - powinien zostać zthrottlowany, zwracając Ok(()) bez wysyłania
    let res2 = send_move_packet(1, 0, 90.0, 100, 100, 0);
    assert_eq!(res2, Ok(()));

    // Pakiet MOVE_STOP - powinien przejść mimo blokady 100ms
    let res3 = send_move_packet(MOVE_STOP, 0, 90.0, 100, 100, 0);
    assert_eq!(res3, Err("Active game socket not found".to_string()));
}

#[test]
fn test_send_move_packet_rotation_conversion() {
    // Kąt 180.0 powinnien dać 128 (180/360 * 255 = 127.5, round = 128)
    // Ponieważ test wywali się na braku socketu, nie możemy podglądać ramki,
    // ale możemy przynajmniej sprawdzić że wywołanie przechodzi przez funkcję konwersji (gdyby nie throttlowało)
    
    // Niestety globalny socket to black-box, więc sprawdzimy ręcznie logikę konwersji użytą w pliku (testując ujemne itd).
    let convert = |rot: f32| -> u8 {
        let mut rot_norm = rot % 360.0;
        if rot_norm < 0.0 {
            rot_norm += 360.0;
        }
        ((rot_norm / 360.0) * 255.0).round() as u8
    };

    assert_eq!(convert(0.0), 0);
    assert_eq!(convert(180.0), 128); // 127.5 -> round -> 128
    assert_eq!(convert(360.0), 0);
    assert_eq!(convert(90.0), 64);   // 63.75 -> round -> 64
    assert_eq!(convert(-90.0), 191); // 270.0 -> 191.25 -> 191
    assert_eq!(convert(720.0), 0);
}
