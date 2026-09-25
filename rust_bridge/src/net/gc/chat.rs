// Plik modulu obslugujacego wiadomosci czatu przychodzace od serwera (GC = Game Client)

/// Zgodnie z C++ struktura TPacketGCChat wyglada tak (z uwzglednieniem #pragma pack(1)):
/// BYTE    header;    // 1 bajt (zawsze 4 - HEADER_GC_CHAT)
/// WORD    size;      // 2 bajty (rozmiar calego pakietu lacznie z headerem i wiadomoscia)
/// BYTE    type;      // 1 bajt  (typ wiadomosci)
/// DWORD   dwVID;     // 4 bajty (ID aktora, np. mowiacego gracza)
/// BYTE    bEmpire;   // 1 bajt  (imperium)
/// Nastepnie przychodza stringi zawierajace tresc wiadomosci
/// Calosc stalej wielkosci zajmuje 9 bajtow.
const HEADER_GC_CHAT_SIZE: usize = 9;
const HEADER_GC_CHAT: u8 = 4;

#[derive(Debug, PartialEq)]
pub struct GcChatPacket {
    pub header: u8,
    pub size: u16,
    pub chat_type: u8,
    pub vid: u32,
    pub empire: u8,
    pub message: String,
}

impl GcChatPacket {
    /// Deserializator wiadomosci czatu HEADER_GC_CHAT (type, message)
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        // Sprawdz czy bufor zawiera przynajmniej staly naglowek
        if buf.len() < HEADER_GC_CHAT_SIZE {
            return Err("Bufor jest za krotki aby zawierac naglowek pakietu czatu".to_string());
        }

        let header = buf[0];
        
        // Nie musimy ale mozemy sprawdzic czy naglowek odpowiada HEADER_GC_CHAT
        if header != HEADER_GC_CHAT {
            return Err("Nieprawidlowy naglowek pakiety czatu".to_string());
        }

        // Dekodowanie WORD size z uzyciem little endian
        let size = u16::from_le_bytes([buf[1], buf[2]]);
        
        // Sprawdz czy bufor zawiera wystarczajaca ilosc bajtow zdefiniowana w size
        if buf.len() < size as usize {
            return Err("Bufor jest krotszy niz deklarowany rozmiar pakietu".to_string());
        }

        let chat_type = buf[3];
        let vid = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
        let empire = buf[8];

        // Wyciagnij wiadomosc, pomijajac naglowek. Wiadomosc moze zajmowac reszte deklarowanego rozmiaru
        let message_len = (size as usize) - HEADER_GC_CHAT_SIZE;
        let message_bytes = &buf[HEADER_GC_CHAT_SIZE..HEADER_GC_CHAT_SIZE + message_len];
        
        // Zamien bajty na string (ignorujemy bledy utf-8, mozna zamieniac bledne znaki na zastepcze lub uzyc from_utf8)
        let message = String::from_utf8_lossy(message_bytes).into_owned();

        Ok(GcChatPacket {
            header,
            size,
            chat_type,
            vid,
            empire,
            message,
        })
    }
}
