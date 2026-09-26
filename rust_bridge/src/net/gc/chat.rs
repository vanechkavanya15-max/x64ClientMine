#![allow(non_snake_case)]

// Modul obslugujacy wiadomosci czatu przychodzace od serwera (Game -> Client)
// Zgodny z E:\m2dev-client-src-main\src\UserInterface\Packet.h (TPacketGCChat)

pub const HEADER_GC_CHAT: u16 = 0x0603;
pub const HEADER_GC_CHAT_SIZE: usize = 10;

// Struktura stalego naglowka czatu TPacketGCChat
// Rozmiar: header(2) + length(2) + type(1) + dwVID(4) + bEmpire(1) = 10 bajtow
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct TPacketGCChat {
    pub header: u16,
    pub length: u16,
    pub type_: u8,
    pub dwVID: u32,
    pub bEmpire: u8,
}

impl TPacketGCChat {
    pub const PACKET_SIZE: usize = HEADER_GC_CHAT_SIZE;

    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < Self::PACKET_SIZE {
            return Err("Bufor jest za krotki aby zawierac naglowek pakietu czatu".to_string());
        }

        let header = u16::from_le_bytes(buf[0..2].try_into().unwrap());
        let length = u16::from_le_bytes(buf[2..4].try_into().unwrap());
        let type_ = buf[4];
        let dwVID = u32::from_le_bytes(buf[5..9].try_into().unwrap());
        let bEmpire = buf[9];

        Ok(Self {
            header,
            length,
            type_,
            dwVID,
            bEmpire,
        })
    }
}

// Struktura reprezentujaca pelny odebrany pakiet czatu lacznie z tekstem
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GcChatPacket {
    pub header: u16,
    pub length: u16,
    pub chat_type: u8,
    pub vid: u32,
    pub empire: u8,
    pub message: String,
}

impl GcChatPacket {
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < HEADER_GC_CHAT_SIZE {
            return Err("Bufor jest za krotki aby zawierac naglowek pakietu czatu".to_string());
        }

        let header = u16::from_le_bytes(buf[0..2].try_into().unwrap());
        let length = u16::from_le_bytes(buf[2..4].try_into().unwrap());

        if (length as usize) < HEADER_GC_CHAT_SIZE {
            return Err("Zgloszona dlugosc pakietu czatu jest mniejsza niz rozmiar naglowka".to_string());
        }

        if buf.len() < length as usize {
            return Err("Bufor jest krotszy niz deklarowany rozmiar pakietu".to_string());
        }

        let chat_type = buf[4];
        let vid = u32::from_le_bytes(buf[5..9].try_into().unwrap());
        let empire = buf[9];

        let message_len = (length as usize) - HEADER_GC_CHAT_SIZE;
        let message_bytes = &buf[HEADER_GC_CHAT_SIZE..HEADER_GC_CHAT_SIZE + message_len];
        let message = String::from_utf8_lossy(message_bytes).into_owned();

        Ok(GcChatPacket {
            header,
            length,
            chat_type,
            vid,
            empire,
            message,
        })
    }

    // Akcesor dla zachowania zgodnosci wstecznej z polem size
    pub fn size(&self) -> u16 {
        self.length
    }
}
