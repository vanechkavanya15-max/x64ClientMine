use m2_client::warp_phase_reconnect::{process_warp_packet, TPacketGCWarp};
use m2_client::ui_bridge_ffi::ACTIVE_GAME_SOCKET;
use m2_client_bridge::state::phase::{ClientPhase, GLOBAL_PHASE_MANAGER};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

#[test]
fn test_process_warp_packet_success() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    
    // Uruchomienie mock serwera w tle
    let _server_thread = thread::spawn(move || {
        if let Ok((mut stream, _addr)) = listener.accept() {
            // Serwer wysyla 77 bajtow handshake (naglowek ulozylmy testowy)
            let mut handshake_buf = vec![0u8; 77];
            // Pierwsze bajty w 77-bajtowym handshake to np. wersja, opcode itp.
            // Klucz publiczny serwera to 32 bajty od indeksu 9.
            let server_pk = [1u8; 32];
            handshake_buf[9..41].copy_from_slice(&server_pk);
            // Challenge to 32 bajty od indeksu 41
            let challenge = [2u8; 32];
            handshake_buf[41..73].copy_from_slice(&challenge);
            
            stream.write_all(&handshake_buf).unwrap();
            
            // Oczekiwanie na odpowiedz klienta (64 bajty klucza publicznego)
            let mut client_resp = [0u8; 64];
            stream.read_exact(&mut client_resp).unwrap();
            
            // Oczekiwanie na pakiet ENTERGAME
            // Szyfrowanie po stronie klienta utrudnia precyzyjne odczytanie w tecie (poniewaz serwer
            // w tecie nie oblicza prawidlowych kluczy sesji uzywajac wlasnego sk), wiec
            // wystarczy sprawdzic, ze klient zdołał wysłać 4 bajty i zamknął/otworzył nową fazę.
            let mut enter_pkt = [0u8; 4];
            stream.read_exact(&mut enter_pkt).unwrap();
        }
    });

    let ip_int = u32::from_be_bytes([127, 0, 0, 1]);

    let warp_pkt = TPacketGCWarp {
        header: 0x0306,
        length: 18,
        lX: 1000,
        lY: 2000,
        lAddr: ip_int,
        wPort: port,
    };
    
    let pkt_bytes: [u8; 18] = unsafe { std::mem::transmute(warp_pkt) };
    
    // Poczatkowy stan - nie jestesmy w Loading
    {
        let mut pm = GLOBAL_PHASE_MANAGER.lock().unwrap();
        let _ = pm.set_phase(ClientPhase::Game);
    }
    
    // Wywolanie testowanej funkcji
    let result = process_warp_packet(&pkt_bytes);
    
    assert!(result.is_ok(), "Blad podczas przetwarzania pakietu warp: {:?}", result.err());
    
    // Weryfikacja fazy
    {
        let pm = GLOBAL_PHASE_MANAGER.lock().unwrap();
        assert_eq!(pm.get_phase(), ClientPhase::Loading, "Faza powinna zostac zmieniona na Loading");
    }
    
    // Weryfikacja ze nowe gniazdo zostalo podpiete
    {
        let sock_guard = ACTIVE_GAME_SOCKET.lock().unwrap();
        assert!(sock_guard.is_some(), "ACTIVE_GAME_SOCKET powinno zawierac nowe polaczenie");
        
        let (_, cipher) = sock_guard.as_ref().unwrap();
        assert!(cipher.is_activated(), "Szyfr powinien byc aktywowany");
    }
}
