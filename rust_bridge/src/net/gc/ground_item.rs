// Struktura reprezentujaca pakiet HEADER_GC_ITEM_GROUND_ADD
// Uzywana do obslugi dodawania przedmiotu lezacego na ziemi.
// Zgodnie z paczka C++ zpragma pack(1).
// typedef struct packet_ground_add_item
// {
//     BYTE        bHeader;
//     long        lX;
//     long        lY;
//     long        lZ;
//     DWORD       dwVID;
//     DWORD       dwVnum;
// } TPacketGCItemGroundAdd;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GcItemGroundAddPacket {
    pub header: u8,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub vid: u32,
    pub vnum: u32,
}

impl GcItemGroundAddPacket {
    // Rozmiar pakietu w bajtach (1 + 4 + 4 + 4 + 4 + 4) = 21 bajtow.
    pub const PACKET_SIZE: usize = 21;

    // Deserializuje pakiet z bufora bajtow.
    // Zwraca blad jesli bufor jest mniejszy niz oczekiwany rozmiar pakietu.
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < Self::PACKET_SIZE {
            return Err(format!(
                "Zbyt krotki bufor. Oczekiwano {}, otrzymano {}",
                Self::PACKET_SIZE,
                buf.len()
            ));
        }

        let header = buf[0];
        
        let mut offset = 1;
        let x = i32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap());
        offset += 4;
        
        let y = i32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap());
        offset += 4;
        
        let z = i32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap());
        offset += 4;
        
        let vid = u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap());
        offset += 4;
        
        let vnum = u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap());

        Ok(Self {
            header,
            x,
            y,
            z,
            vid,
            vnum,
        })
    }
}
