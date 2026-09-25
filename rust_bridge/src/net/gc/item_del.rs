use crate::ffi_guard::safe_ffi_call;

#[derive(Debug, PartialEq)]
pub enum PacketError {
    TooShort,
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TPacketGCItemDel {
    pub header: u8,
    pub pos: TItemPos,
}

impl TPacketGCItemDel {
    pub fn from_bytes(data: &[u8]) -> Result<Self, PacketError> {
        let expected_size = std::mem::size_of::<Self>();
        if data.len() < expected_size {
            return Err(PacketError::TooShort);
        }

        let header = data[0];
        let window_type = data[1];
        let cell = u16::from_le_bytes([data[2], data[3]]);

        Ok(Self {
            header,
            pos: TItemPos {
                window_type,
                cell,
            },
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_parse_item_del(
    data: *const u8,
    len: u32,
    out_win: *mut u8,
    out_cell: *mut u16,
) -> bool {
    safe_ffi_call(false, || {
        if data.is_null() || out_win.is_null() || out_cell.is_null() {
            return false;
        }

        let slice = std::slice::from_raw_parts(data, len as usize);
        match TPacketGCItemDel::from_bytes(slice) {
            Ok(packet) => {
                *out_win = packet.pos.window_type;
                *out_cell = packet.pos.cell;
                true
            }
            Err(_) => false,
        }
    })
}
