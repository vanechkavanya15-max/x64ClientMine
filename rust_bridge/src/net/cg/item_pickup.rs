// Struktura reprezentujaca pakiet HEADER_CG_ITEM_PICKUP
// Sluzy do podnoszenia przedmiotow z ziemi
#[derive(Debug, Clone, PartialEq)]
pub struct CGItemPickup {
    // Naglowek pakietu
    pub header: u8,
    // Identyfikator wirtualny (vid) przedmiotu
    pub vid: u32,
}

impl CGItemPickup {
    // Konstruktor nowego pakietu podnoszenia przedmiotu
    pub fn new(vid: u32) -> Self {
        Self {
            header: 15, // HEADER_CG_ITEM_PICKUP = 15
            vid,
        }
    }

    // Serializuje strukture do wektora bajtow (Little-Endian)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(5);
        buf.push(self.header);
        buf.extend_from_slice(&self.vid.to_le_bytes());
        buf
    }
}
