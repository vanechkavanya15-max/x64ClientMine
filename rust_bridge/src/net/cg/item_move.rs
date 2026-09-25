use crate::ffi_guard::safe_ffi_call;
use crate::net::cg::item_use::TItemPos;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGItemMove {
    pub header: u8,
    pub pos: TItemPos,
    pub change_pos: TItemPos,
    pub count: u8,
}

impl TPacketCGItemMove {
    pub const HEADER: u8 = 13;

    pub fn new(src_win: u8, src_cell: u16, dst_win: u8, dst_cell: u16, count: u8) -> Self {
        Self {
            header: Self::HEADER,
            pos: TItemPos {
                window_type: src_win,
                cell: src_cell,
            },
            change_pos: TItemPos {
                window_type: dst_win,
                cell: dst_cell,
            },
            count,
        }
    }

    pub fn to_bytes(&self) -> [u8; 8] {
        let mut buf = [0u8; 8];
        buf[0] = self.header;
        buf[1] = self.pos.window_type;
        let sc = self.pos.cell.to_le_bytes();
        buf[2] = sc[0];
        buf[3] = sc[1];
        buf[4] = self.change_pos.window_type;
        let dc = self.change_pos.cell.to_le_bytes();
        buf[5] = dc[0];
        buf[6] = dc[1];
        buf[7] = self.count;
        buf
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_item_move(
    src_win: u8,
    src_cell: u16,
    dst_win: u8,
    dst_cell: u16,
    count: u8,
) -> bool {
    safe_ffi_call(false, || {
        let pkt = TPacketCGItemMove::new(src_win, src_cell, dst_win, dst_cell, count);
        let _bytes = pkt.to_bytes();
        true
    })
}
