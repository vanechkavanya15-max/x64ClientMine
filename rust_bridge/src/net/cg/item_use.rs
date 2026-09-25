// Reprezentacja pozycji przedmiotu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemPos {
    pub window_type: u8,
    pub cell: u16,
}

// Reprezentacja pakietu HEADER_CG_ITEM_USE
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketCgItemUse {
    pub header: u8,
    pub pos: ItemPos,
}

impl PacketCgItemUse {
    // Stale wartosci
    pub const HEADER: u8 = 11;

    // Metoda sluzaca do serializacji pakietu, konwertujaca go na bajty.
    // Pakiet na serwer jest w formacie Little-Endian i ma #pragma pack(1) w C++ dla ItemPos.
    // Wynikowe bajty to: header (1), window_type (1), cell (2). Razem 4 bajty.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(4);
        
        // Zapis naglowka pakietu
        buf.push(self.header);
        
        // Zapis window_type
        buf.push(self.pos.window_type);
        
        // Zapis cell w formacie Little-Endian
        let cell_bytes = self.pos.cell.to_le_bytes();
        buf.push(cell_bytes[0]);
        buf.push(cell_bytes[1]);
        
        buf
    }
}
