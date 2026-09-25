use crate::ffi_guard::safe_ffi_call;

// Struktura reprezentujaca pozycje przedmiotu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C, packed)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

// Struktura pakietu HEADER_CG_ITEM_DROP
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C, packed)]
pub struct TPacketCGItemDrop {
    pub header: u8,
    pub pos: TItemPos,
    pub count: u32,
}

impl TPacketCGItemDrop {
    // Stala reprezentujaca naglowek pakietu CG_ITEM_DROP
    pub const HEADER_CG_ITEM_DROP: u8 = 12;

    pub fn new(window_type: u8, cell: u16, count: u32) -> Self {
        Self {
            header: Self::HEADER_CG_ITEM_DROP,
            pos: TItemPos { window_type, cell },
            count,
        }
    }

    // Metoda serializujaca strukture do tablicy bajtow (Little-Endian)
    // Zgodnie z `#pragma pack(1)` struktura ma 8 bajtow:
    // header: 1 bajt
    // pos.window_type: 1 bajt
    // pos.cell: 2 bajty
    // count: 4 bajty
    pub fn to_bytes(&self) -> [u8; 8] {
        let mut buf = [0u8; 8];
        
        // Zapisujemy naglowek
        buf[0] = self.header;
        
        // Zapisujemy window_type
        buf[1] = self.pos.window_type;
        
        // Zapisujemy cell (2 bajty, Little-Endian)
        let cell_bytes = self.pos.cell.to_le_bytes();
        buf[2] = cell_bytes[0];
        buf[3] = cell_bytes[1];
        
        // Zapisujemy count (4 bajty, Little-Endian)
        let count_bytes = self.count.to_le_bytes();
        buf[4] = count_bytes[0];
        buf[5] = count_bytes[1];
        buf[6] = count_bytes[2];
        buf[7] = count_bytes[3];
        
        buf
    }
}

/// FFI Bridge dla wysylania pakietu wyrzucenia przedmiotu lub zlota.
/// 
/// Argumenty:
/// window_type - identyfikator okna (np. INVENTORY)
/// slot - indeks komorki w ekwipunku
/// count - ilosc sztuk lub zlota
#[no_mangle]
pub unsafe extern "C" fn m2_net_send_item_drop(_window_type: u8, _slot: u16, _count: u32) -> bool {
    safe_ffi_call(false, || {
        // Implementacja zablokowana do kolejnej fazy integracji z C++.
        // Zwracamy true, zeby potwierdzic symulowane wyslanie pakietu.
        
        true
    })
}
