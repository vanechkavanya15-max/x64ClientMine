use crate::ffi_guard::safe_ffi_call;

pub const HEADER_CG_TARGET: u8 = 61;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C, packed)]
pub struct TPacketCGTarget {
    pub header: u8,
    pub dw_vid: u32,
}

impl TPacketCGTarget {
    pub fn new(dw_vid: u32) -> Self {
        Self {
            header: HEADER_CG_TARGET,
            dw_vid,
        }
    }

    pub fn to_bytes(&self) -> [u8; 5] {
        let mut bytes = [0u8; 5];
        bytes[0] = self.header;
        bytes[1..5].copy_from_slice(&self.dw_vid.to_le_bytes());
        bytes
    }
}

// Funkcja C-FFI wysylajaca pakiet CG target.
// Zabezpieczona przed panika w C++.
#[no_mangle]
pub extern "C" fn m2_net_send_target(dw_vid: u32) -> bool {
    safe_ffi_call(false, || {
        let _packet = TPacketCGTarget::new(dw_vid);
        // Tymczasowo zwraca true w symulacji wysylania. Docelowo podlaczona bedzie tu faktyczna logika.
        true
    })
}
