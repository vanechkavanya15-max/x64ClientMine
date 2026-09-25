// Struktura reprezentujaca pozycje przedmiotu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemPos {
    pub window_type: u8,
    pub cell: u16,
}

// Struktura pakietu HEADER_CG_ITEM_DROP
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketCgItemDrop {
    pub header: u8,
    pub pos: ItemPos,
    pub elk: u32,
}

impl PacketCgItemDrop {
    // Stala reprezentujaca naglowek pakietu CG_ITEM_DROP
    pub const HEADER: u8 = 12;

    // Metoda serializujaca strukture do wektora bajtow (Little-Endian)
    // Zgodnie z `#pragma pack(1)` struktura ma 8 bajtow:
    // header: 1 bajt
    // pos.window_type: 1 bajt
    // pos.cell: 2 bajty
    // elk: 4 bajty
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8);
        
        // Zapisujemy naglowek
        buf.push(self.header);
        
        // Zapisujemy window_type
        buf.push(self.pos.window_type);
        
        // Zapisujemy cell (2 bajty, Little-Endian)
        buf.extend_from_slice(&self.pos.cell.to_le_bytes());
        
        // Zapisujemy elk (4 bajty, Little-Endian)
        buf.extend_from_slice(&self.elk.to_le_bytes());
        
        buf
    }
}
