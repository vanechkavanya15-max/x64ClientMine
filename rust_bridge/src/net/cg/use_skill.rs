pub const HEADER_CG_USE_SKILL: u8 = 52;

// Struktura reprezentujaca pakiet uzycia umiejetnosci (CG - Client to Game).
// Zgodnie z C++ jest to:
// typedef struct command_use_skill
// {
//     BYTE                bHeader;
//     DWORD               dwVnum;
//     DWORD               dwTargetVID;
// } TPacketCGUseSkill;
pub struct CgUseSkill {
    pub header: u8,
    pub vnum: u32,
    pub target_vid: u32,
}

impl CgUseSkill {
    // Tworzy nowy pakiet uzycia umiejetnosci.
    pub fn new(vnum: u32, target_vid: u32) -> Self {
        Self {
            header: HEADER_CG_USE_SKILL,
            vnum,
            target_vid,
        }
    }

    // Serializuje strukture do wektora bajtow (Little-Endian).
    pub fn to_bytes(&self) -> Vec<u8> {
        // Calkowity rozmiar: 1 bajt (header) + 4 bajty (vnum) + 4 bajty (target_vid) = 9 bajtow.
        let mut buf = Vec::with_capacity(9);
        
        // Zapisujemy naglowek (1 bajt).
        buf.push(self.header);
        
        // Zapisujemy vnum (4 bajty, Little-Endian).
        buf.extend_from_slice(&self.vnum.to_le_bytes());
        
        // Zapisujemy target_vid (4 bajty, Little-Endian).
        buf.extend_from_slice(&self.target_vid.to_le_bytes());
        
        buf
    }
}
