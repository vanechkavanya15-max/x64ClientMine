use crate::ffi_guard::safe_ffi_call;

// Reprezentacja pozycji przedmiotu (kompatybilna z C++ MSVC, pragma pack(1))
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

// Reprezentacja pakietu HEADER_CG_ITEM_USE
#[repr(C, packed)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TPacketCGItemUse {
    pub header: u8,
    pub pos: TItemPos,
}

impl TPacketCGItemUse {
    // Stale wartosci
    pub const HEADER_CG_ITEM_USE: u8 = 11;

    // Inicjuje nowa strukture z zadanymi parametrami.
    pub fn new(window_type: u8, cell: u16) -> Self {
        Self {
            header: Self::HEADER_CG_ITEM_USE,
            pos: TItemPos {
                window_type,
                cell,
            },
        }
    }

    // Metoda sluzaca do serializacji pakietu, konwertujaca go na tablice 4 bajtow.
    // Pakiet na serwer jest w formacie Little-Endian dla wartosci cell.
    pub fn to_bytes(&self) -> [u8; 4] {
        let mut buf = [0u8; 4];
        buf[0] = self.header;
        buf[1] = self.pos.window_type;
        
        let cell_bytes = self.pos.cell.to_le_bytes();
        buf[2] = cell_bytes[0];
        buf[3] = cell_bytes[1];
        
        buf
    }
}

// Eksport FFI symulujacy wyslanie uzycia przedmiotu
#[no_mangle]
pub unsafe extern "C" fn m2_net_send_item_use(window_type: u8, slot: u16) -> bool {
    safe_ffi_call(false, || {
        let _packet = TPacketCGItemUse::new(window_type, slot);
        let _bytes = _packet.to_bytes();
        
        // Zgodnie z wymaganiami funkcji FFI na ten moment jest tylko budowa, ewentualnie wysylka
        // symulowana jest tak jak w api/net_send_use_item.rs, i zwracamy true.
        true
    })
}
