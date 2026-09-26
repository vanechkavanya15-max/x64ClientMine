// Struktura pakietu ataku CG (Client -> Game) na podstawie Packet.h: command_attack
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::net::constants::cg::ATTACK;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGAttack {
    pub header: u16,
    pub length: u16,
    pub b_type: u8,
    pub dw_victim_vid: u32,
    pub b_crc_magic_cube_proc_piece: u8,
    pub b_crc_magic_cube_file_piece: u8,
}

impl TPacketCGAttack {
    pub const HEADER: u16 = ATTACK;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    // Tworzy nowy pakiet ataku z pelnymi parametrami.
    pub fn new(
        b_type: u8,
        dw_victim_vid: u32,
        b_crc_magic_cube_proc_piece: u8,
        b_crc_magic_cube_file_piece: u8,
    ) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            b_type,
            dw_victim_vid,
            b_crc_magic_cube_proc_piece,
            b_crc_magic_cube_file_piece,
        }
    }

    // Tworzy pakiet ataku z zerowymi sumami kontrolnymi.
    pub fn new_simple(b_type: u8, dw_victim_vid: u32) -> Self {
        Self::new(b_type, dw_victim_vid, 0, 0)
    }

    // Serializuje strukture do wektora bajtow (Little-Endian).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.push(self.b_type);
        buf.extend_from_slice(&self.dw_victim_vid.to_le_bytes());
        buf.push(self.b_crc_magic_cube_proc_piece);
        buf.push(self.b_crc_magic_cube_file_piece);
        buf
    }
}

// Funkcja pomocnicza budujaca pakiet do podanego bufora wyjsciowego C FFI.
#[no_mangle]
pub unsafe extern "C" fn m2_net_build_cg_attack(
    victim_vid: u32,
    attack_type: u8,
    out_buf: *mut u8,
) -> bool {
    crate::ffi_guard::safe_ffi_call(false, || {
        if out_buf.is_null() {
            return false;
        }

        let packet = TPacketCGAttack::new_simple(attack_type, victim_vid);
        let bytes = packet.to_bytes();
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf, bytes.len());

        true
    })
}
