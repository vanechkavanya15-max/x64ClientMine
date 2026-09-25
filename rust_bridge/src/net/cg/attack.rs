// Struktura reprezentujaca pakiet ataku wysylany do serwera (Client to Game).
// Zawiera podstawowe informacje o ataku, w tym VID ofiary oraz czesci CRC do walidacji.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCgAttack {
    pub header: u8,
    pub b_type: u8,
    pub dw_victim_vid: u32,
    pub b_crc_magic_cube_proc_piece: u8,
    pub b_crc_magic_cube_file_piece: u8,
}

impl TPacketCgAttack {
    // Naglowek dla pakietu ataku w komunikacji CG.
    pub const HEADER: u8 = 2;

    // Konstruktor tworzacy nowy pakiet ataku z zainicjalizowanym naglowkiem.
    pub fn new(
        b_type: u8,
        dw_victim_vid: u32,
        b_crc_magic_cube_proc_piece: u8,
        b_crc_magic_cube_file_piece: u8,
    ) -> Self {
        Self {
            header: Self::HEADER,
            b_type,
            dw_victim_vid,
            b_crc_magic_cube_proc_piece,
            b_crc_magic_cube_file_piece,
        }
    }

    // Serializuje strukture do formatu binarnego.
    // Uzywa kolejnosci bajtow Little-Endian dla pol wielobajtowych (jak dw_victim_vid).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8);
        buf.push(self.header);
        buf.push(self.b_type);
        buf.extend_from_slice(&self.dw_victim_vid.to_le_bytes());
        buf.push(self.b_crc_magic_cube_proc_piece);
        buf.push(self.b_crc_magic_cube_file_piece);
        buf
    }
}
