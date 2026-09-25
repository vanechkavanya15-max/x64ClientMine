use crate::ffi_guard::safe_ffi_call;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGShoot {
    pub header: u8,
    pub b_type: u8,
}

impl TPacketCGShoot {
    pub const HEADER: u8 = 54;

    pub fn new(b_type: u8) -> Self {
        Self {
            header: Self::HEADER,
            b_type,
        }
    }

    pub fn to_bytes(&self) -> [u8; 2] {
        [self.header, self.b_type]
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_shoot(b_type: u8) -> bool {
    safe_ffi_call(false, || {
        let pkt = TPacketCGShoot::new(b_type);
        let _bytes = pkt.to_bytes();
        true
    })
}
