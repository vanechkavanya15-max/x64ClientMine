// Struktura pakietu czatu CG (Client -> Game) na podstawie Packet.h: command_chat
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::net::constants::cg::CHAT;

pub const HEADER_CG_CHAT: u16 = CHAT;
pub const MAX_CHAT_MSG_LEN: usize = 512;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGChat {
    pub header: u16,
    pub length: u16,
    pub chat_type: u8,
}

impl TPacketCGChat {
    pub const HEADER: u16 = CHAT;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    // Tworzy naglowek pakietu czatu z podanym typem oraz calkowita dlugoscia ramki.
    pub fn new(chat_type: u8, length: u16) -> Self {
        Self {
            header: Self::HEADER,
            length,
            chat_type,
        }
    }

    // Serializuje sam naglowek pakietu czatu (5 bajtow).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.push(self.chat_type);
        buf
    }

    // Buduje pelny zserializowany pakiet z uwzglednieniem wiadomosci tekstowej i ograniczenia dlugosci.
    pub fn build_with_message(chat_type: u8, msg: &str) -> Vec<u8> {
        let bytes = msg.as_bytes();
        let mut truncated_len = bytes.len();

        if truncated_len > MAX_CHAT_MSG_LEN {
            let mut cut_idx = MAX_CHAT_MSG_LEN;
            while cut_idx > 0 && !msg.is_char_boundary(cut_idx) {
                cut_idx -= 1;
            }
            truncated_len = cut_idx;
        }

        let msg_slice = &bytes[..truncated_len];
        // Calkowity rozmiar ramki: naglowek (5 bajtow) + tresc tekstu + null terminator (1 bajt)
        let total_length = (Self::SIZE as usize + msg_slice.len() + 1) as u16;

        let packet = Self::new(chat_type, total_length);
        let mut out = packet.to_bytes();
        out.extend_from_slice(msg_slice);
        out.push(0); // null terminator
        out
    }
}

// Re-eksport funkcji C FFI z api::net_send_chat
pub use crate::api::net_send_chat::m2_net_send_chat;

