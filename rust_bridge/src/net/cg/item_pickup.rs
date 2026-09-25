// Struktura reprezentujaca pakiet HEADER_CG_ITEM_PICKUP
// Sluzy do podnoszenia przedmiotow z ziemi
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TPacketCGItemPickup {
    // Naglowek pakietu
    pub header: u8,
    // Identyfikator wirtualny (vid) przedmiotu
    pub dw_vid: u32,
}

impl TPacketCGItemPickup {
    // Konstruktor nowego pakietu podnoszenia przedmiotu
    pub fn new(dw_vid: u32) -> Self {
        Self {
            header: 15, // HEADER_CG_ITEM_PICKUP = 15
            dw_vid,
        }
    }

    // Serializuje strukture do tablicy 5 bajtow (Little-Endian)
    pub fn to_bytes(&self) -> [u8; 5] {
        let mut buf = [0u8; 5];
        buf[0] = self.header;
        let vid_bytes = self.dw_vid.to_le_bytes();
        buf[1] = vid_bytes[0];
        buf[2] = vid_bytes[1];
        buf[3] = vid_bytes[2];
        buf[4] = vid_bytes[3];
        buf
    }
}
