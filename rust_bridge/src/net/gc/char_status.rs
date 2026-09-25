#[derive(Debug, PartialEq)]
pub enum PacketError {
    BufferTooShort,
    InvalidHeader,
}

pub const HEADER_GC_PLAYER_POINT_CHANGE: u8 = 17;

#[repr(C, packed)]
#[derive(Debug)]
pub struct TPacketGCPointChange {
    pub header: u8,
    pub b_type: u8,
    pub value: i64,
    pub amount: i64,
}

impl TPacketGCPointChange {
    pub fn from_bytes(data: &[u8]) -> Result<Self, PacketError> {
        let size = std::mem::size_of::<Self>();
        if data.len() < size {
            return Err(PacketError::BufferTooShort);
        }

        if data[0] != HEADER_GC_PLAYER_POINT_CHANGE {
            return Err(PacketError::InvalidHeader);
        }

        let mut buf = [0u8; std::mem::size_of::<Self>()];
        buf.copy_from_slice(&data[..size]);
        
        let packet: Self = unsafe { std::ptr::read_unaligned(buf.as_ptr() as *const Self) };
        Ok(packet)
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_parse_point_change(
    data: *const u8,
    len: u32,
    out_type: *mut u8,
    out_val: *mut i64,
) -> bool {
    if data.is_null() || out_type.is_null() || out_val.is_null() {
        return false;
    }

    let result = std::panic::catch_unwind(|| {
        let slice = std::slice::from_raw_parts(data, len as usize);
        match TPacketGCPointChange::from_bytes(slice) {
            Ok(packet) => {
                *out_type = packet.b_type;
                *out_val = std::ptr::read_unaligned(std::ptr::addr_of!(packet.value));
                true
            }
            Err(_) => false,
        }
    });

    result.unwrap_or(false)
}
