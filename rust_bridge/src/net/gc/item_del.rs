// Struktura reprezentujaca pakiet usuniecia itemu ze slotu (HEADER_GC_ITEM_DEL)
#[derive(Debug, PartialEq)]
pub struct GcItemDel {
    pub header: u8,
    pub pos: u8,
}

impl GcItemDel {
    // Deserializuje pakiet z bufora bajtow
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        // Zabezpieczenie przed za krotkim buforem
        if buf.len() < 2 {
            return Err("Bufor jest za krotki aby zdeserializowac GcItemDel".to_string());
        }

        Ok(GcItemDel {
            header: buf[0],
            pos: buf[1],
        })
    }
}
