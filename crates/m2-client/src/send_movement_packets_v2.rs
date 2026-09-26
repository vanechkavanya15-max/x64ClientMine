use std::mem::size_of;
use std::ptr;

// Header for CG::MOVE based on Packet.h
const CG_MOVE_HEADER: u16 = 0x0301; // As per the first block in Packet.h

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGMove {
    pub header: u16,
    pub length: u16,
    pub b_func: u8,
    pub b_arg: u8,
    pub b_rot: u8,
    pub l_x: i32,
    pub l_y: i32,
    pub dw_time: u32,
}

impl TPacketCGMove {
    pub const SIZE: usize = size_of::<Self>();

    pub fn new(func: u8, arg: u8, rot_normalized: u8, x: i32, y: i32, time: u32) -> Self {
        Self {
            header: CG_MOVE_HEADER,
            length: Self::SIZE as u16,
            b_func: func,
            b_arg: arg,
            b_rot: rot_normalized,
            l_x: x,
            l_y: y,
            dw_time: time,
        }
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        buf[0..2].copy_from_slice(&self.header.to_le_bytes());
        buf[2..4].copy_from_slice(&self.length.to_le_bytes());
        buf[4] = self.b_func;
        buf[5] = self.b_arg;
        buf[6] = self.b_rot;
        buf[7..11].copy_from_slice(&self.l_x.to_le_bytes());
        buf[11..15].copy_from_slice(&self.l_y.to_le_bytes());
        buf[15..19].copy_from_slice(&self.dw_time.to_le_bytes());
        buf
    }
}

/// Serializuje pakiet ruchu klienta do podanego bufora i zwraca zapisana ilosc bajtow.
/// m2_send_move(func: u8, x: i32, y: i32, rot: f32, time: u32, out_buf: *mut u8) -> usize
#[no_mangle]
pub extern "C" fn m2_send_move(func: u8, x: i32, y: i32, rot: f32, time: u32, out_buf: *mut u8) -> usize {
    if out_buf.is_null() {
        return 0;
    }

    // Normalizacja rotacji zgodnie z logiką C++
    let mut f_dst_rot = rot;
    if f_dst_rot < 0.0 {
        f_dst_rot = 360.0 + f_dst_rot;
    } else if f_dst_rot > 360.0 {
        f_dst_rot = f_dst_rot % 360.0;
    }

    // Dodatkowy bezpiecznik w przypadku bardzo dużych ujemnych wartości
    if f_dst_rot < 0.0 {
        f_dst_rot = f_dst_rot.rem_euclid(360.0);
    }

    // Zamiana na bRot
    let b_rot = (f_dst_rot / 5.0) as u8;

    // Utworzenie pakietu. bArg jest zawsze 0 w tym kontekście (zgodnie z uArg w wywołaniach, choć tam uArg mogłoby być podawane).
    // Ponieważ interfejs FFI ma tylko func, x, y, rot, time; bArg uznajemy za 0, by dopasować sygnaturę z zadania.
    let packet = TPacketCGMove::new(func, 0, b_rot, x, y, time);
    let bytes = packet.to_bytes();

    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf, bytes.len());
    }

    bytes.len()
}
