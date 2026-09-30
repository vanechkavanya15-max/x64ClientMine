// tests/test_live_server_login.rs - Test polaczenia klienta Pure Rust z lokalnym serwerem Auth (Port 11000)
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

#[test]
fn test_live_auth_server_handshake_and_login() {
    let addr = "127.0.0.1:11000";
    let stream_res = TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_secs(2));
    if let Err(e) = stream_res {
        println!("[-] Serwer nie dziala pod {}: {}, pomijam test", addr, e);
        return;
    }
    let mut stream = stream_res.unwrap();
    stream.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
    stream.set_write_timeout(Some(Duration::from_secs(2))).unwrap();

    // 1. Serwer wysyla pakiet powitalny: PHASE_HANDSHAKE (0xFD 0x01) + PacketHandshake (0xFF, 13B) = 15B
    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf).expect("Failed to read server welcome packet");
    assert!(n >= 15, "Server welcome packet must be at least 15 bytes");
    assert_eq!(buf[0], 0xFD, "Header must be PHASE_HANDSHAKE (0xFD)");
    assert_eq!(buf[1], 0x01, "Phase must be Handshake (0x01)");
    assert_eq!(buf[2], 0xFF, "Packet header must be HEADER_HANDSHAKE (0xFF)");
    println!("[+] Odebrano naglowek powitalny serwera ({} bajtow): {:02X?}", n, &buf[..n]);

    // 2. Klient odsyla 13-bajtowy Handshake (0xFF, seed, time, delta)
    let mut hs = [0u8; 13];
    hs[0] = 0xFF; // HEADER_HANDSHAKE
    hs[1..5].copy_from_slice(&0x13374242u32.to_le_bytes());
    hs[5..9].copy_from_slice(&0u32.to_le_bytes());
    hs[9..13].copy_from_slice(&0i32.to_le_bytes());
    stream.write_all(&hs).expect("Failed to send handshake response");
    println!("[+] Odeslano 13-bajtowa odpowiedz Handshake");

    // 3. Serwer potwierdza zmiane fazy na PHASE_AUTH (0xFD 0x0A)
    let n_phase = stream.read(&mut buf).expect("Failed to read phase switch");
    assert_eq!(n_phase, 2);
    assert_eq!(buf[0], 0xFD);
    assert_eq!(buf[1], 0x0A); // PHASE_AUTH
    println!("[+] Potwierdzenie fazy serwera: PHASE_AUTH (0xFD, 0x0A)");

    // 4. Klient wysyla pakiet logowania PacketCGLogin (81 bajtow)
    let mut login_pkt = [0u8; 81];
    login_pkt[0] = 0x6F; // HEADER_CG_LOGIN
    let id_bytes = b"player1";
    login_pkt[1..1 + id_bytes.len()].copy_from_slice(id_bytes);
    let pwd_bytes = b"123456789";
    login_pkt[32..32 + pwd_bytes.len()].copy_from_slice(pwd_bytes);
    stream.write_all(&login_pkt).expect("Failed to send login packet");
    println!("[+] Wyslano pakiet logowania PacketCGLogin (81 bajtow, login: player1)");

    // 5. Serwer odpowiada PacketGCLoginSuccess (header: 0x96, login_key: u32, result: u8)
    let n_resp = stream.read(&mut buf).expect("Failed to read login success");
    println!("[+] Odpowiedz serwera na logowanie ({} bajtow): {:02X?}", n_resp, &buf[..n_resp]);
    assert!(n_resp >= 6);
    assert_eq!(buf[0], 0x96, "Header must be HEADER_GC_LOGIN_SUCCESS (0x96)");
    let login_key = u32::from_le_bytes(buf[1..5].try_into().unwrap());
    let result = buf[5];
    println!("[+] SUKCES! Zalogowano do serwera gry! login_key={}, result={}", login_key, result);
    assert_eq!(result, 1);
}
