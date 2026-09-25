pub const HEADER_CG_CHAT: u8 = 3;
pub const MAX_CHAT_MSG_LEN: usize = 512;

/// Struktura reprezentujaca pakiet czatu od klienta do serwera (CG)
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct TPacketCGChat {
    pub header: u8,
    pub size: u16,
    pub chat_type: u8,
    pub message: Vec<u8>,
}

impl TPacketCGChat {
    /// Konstruktor tworzacy nowy pakiet czatu.
    /// Zgodnie z formatem wiadomosc jest ograniczana do maksymalnie 512 bajtow (wlacznie, bez ewentualnych prob uszkodzenia kodu w polowie znaku).
    pub fn new(chat_type: u8, msg: &str) -> Self {
        // Upewniamy sie, ze dlugosc w bajtach nie przekroczy 512
        let bytes = msg.as_bytes();
        let mut truncated_len = bytes.len();
        
        if truncated_len > MAX_CHAT_MSG_LEN {
            // Musimy obciac na granicy bajtow bedacej prawidlowym UTF-8, 
            // by nie stworzyc "poszarpanej" reprezentacji wielobajtowego znaku.
            let mut cut_idx = MAX_CHAT_MSG_LEN;
            while !msg.is_char_boundary(cut_idx) {
                cut_idx -= 1;
            }
            truncated_len = cut_idx;
        }

        let mut message_vec = bytes[..truncated_len].to_vec();
        message_vec.push(0); // dodajemy null terminator

        let size = 4 + message_vec.len(); // 1(header) + 2(size) + 1(chat_type) + ... string ...

        Self {
            header: HEADER_CG_CHAT,
            size: size as u16,
            chat_type,
            message: message_vec,
        }
    }

    /// Serializuje pakiet do tablicy bajtow (Little-Endian).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.size as usize);
        buf.push(self.header);
        buf.extend_from_slice(&self.size.to_le_bytes());
        buf.push(self.chat_type);
        buf.extend_from_slice(&self.message);
        buf
    }
}

use std::os::raw::c_char;
use std::ffi::CStr;
use crate::ffi_guard::safe_ffi_call;

/// Funkcja C FFI obslugujaca wysylanie wiadomosci czatu do serwera (CG).
/// Zwraca true, jezeli pakiet utworzyl sie bezblednie i wywolanie doszlo do skutku.
#[no_mangle]
pub unsafe extern "C" fn m2_net_send_chat(chat_type: u8, msg: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if msg.is_null() {
            return false;
        }

        let c_str = unsafe { CStr::from_ptr(msg) };
        let msg_str = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };

        let packet = TPacketCGChat::new(chat_type, msg_str);
        
        // Pakiet sformatowany (na teraz symulujemy pomyslne zwrocenie, w rzeczywistosci pakiet wysylany bylby przez kanal)
        let _ = packet.to_bytes();

        true
    })
}
