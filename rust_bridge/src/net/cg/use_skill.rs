pub const HEADER_CG_USE_SKILL: u8 = 52;

// Struktura reprezentujaca pakiet uzycia umiejetnosci (CG - Client to Game).
// Zgodnie z C++ jest to:
// typedef struct command_use_skill
// {
//     BYTE                bHeader;
//     DWORD               dwVnum;
//     DWORD               dwTargetVID;
// } TPacketCGUseSkill;
#[repr(C, packed)]
pub struct TPacketCGUseSkill {
    pub header: u8,
    pub dw_vnum: u32,
    pub dw_vid: u32,
}

impl TPacketCGUseSkill {
    // Tworzy nowy pakiet uzycia umiejetnosci.
    pub fn new(dw_vnum: u32, dw_vid: u32) -> Self {
        Self {
            header: HEADER_CG_USE_SKILL,
            dw_vnum,
            dw_vid,
        }
    }

    // Serializuje strukture do tablicy bajtow (Little-Endian).
    pub fn to_bytes(&self) -> [u8; 9] {
        let mut buf = [0u8; 9];
        
        // Zapisujemy naglowek (1 bajt).
        buf[0] = self.header;
        
        // Poniewaz struct jest packed, bezpieczniej jest skopiowac wartosci
        let vnum = self.dw_vnum;
        let vid = self.dw_vid;
        
        // Zapisujemy vnum (4 bajty, Little-Endian).
        buf[1..5].copy_from_slice(&vnum.to_le_bytes());
        
        // Zapisujemy target_vid (4 bajty, Little-Endian).
        buf[5..9].copy_from_slice(&vid.to_le_bytes());
        
        buf
    }
}
