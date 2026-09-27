use m2_client_bridge::ffi_guard::safe_ffi_call;

pub const HEADER_CG_SHOP: u16 = 0x0801;
pub const HEADER_CG_EXCHANGE: u16 = 0x0508;

pub const SHOP_SUB_BUY: u8 = 1;
pub const SHOP_SUB_SELL: u8 = 2;
pub const SHOP_SUB_SELL2: u8 = 3;

pub const EXCHANGE_SUB_START: u8 = 0;
pub const EXCHANGE_SUB_ITEM_ADD: u8 = 1;
pub const EXCHANGE_SUB_ITEM_DEL: u8 = 2;
pub const EXCHANGE_SUB_ELK_ADD: u8 = 3;
pub const EXCHANGE_SUB_ACCEPT: u8 = 4;
pub const EXCHANGE_SUB_CANCEL: u8 = 5;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TPacketCGShop {
    pub header: u16,
    pub length: u16,
    pub subheader: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TPacketCGExchange {
    pub header: u16,
    pub length: u16,
    pub subheader: u8,
    pub arg1: u32,
    pub arg2: u8,
    pub pos: TItemPos,
}

impl TPacketCGExchange {
    pub fn new(subheader: u8) -> Self {
        Self {
            header: HEADER_CG_EXCHANGE,
            length: std::mem::size_of::<Self>() as u16,
            subheader,
            arg1: 0,
            arg2: 0,
            pos: TItemPos {
                window_type: 0,
                cell: 0xffff,
            },
        }
    }
}

#[no_mangle]
pub extern "C" fn m2_send_shop_buy(pos: u8, count: u8, out_buf: *mut u8) -> usize {
    safe_ffi_call(0, || {
        if out_buf.is_null() {
            return 0;
        }

        let packet = TPacketCGShop {
            header: HEADER_CG_SHOP,
            length: (std::mem::size_of::<TPacketCGShop>() + 2) as u16,
            subheader: SHOP_SUB_BUY,
        };

        unsafe {
            std::ptr::copy_nonoverlapping(
                &packet as *const _ as *const u8,
                out_buf,
                std::mem::size_of::<TPacketCGShop>(),
            );
            
            let mut offset = std::mem::size_of::<TPacketCGShop>();
            *out_buf.add(offset) = count;
            offset += 1;
            *out_buf.add(offset) = pos;
            offset += 1;

            offset
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_send_exchange_start(vid: u32, out_buf: *mut u8) -> usize {
    safe_ffi_call(0, || {
        if out_buf.is_null() {
            return 0;
        }

        let mut packet = TPacketCGExchange::new(EXCHANGE_SUB_START);
        packet.arg1 = vid;

        unsafe {
            std::ptr::copy_nonoverlapping(
                &packet as *const _ as *const u8,
                out_buf,
                std::mem::size_of::<TPacketCGExchange>(),
            );
        }
        std::mem::size_of::<TPacketCGExchange>()
    })
}

#[no_mangle]
pub extern "C" fn m2_send_exchange_elk_add(elk: u32, out_buf: *mut u8) -> usize {
    safe_ffi_call(0, || {
        if out_buf.is_null() {
            return 0;
        }

        let mut packet = TPacketCGExchange::new(EXCHANGE_SUB_ELK_ADD);
        packet.arg1 = elk;

        unsafe {
            std::ptr::copy_nonoverlapping(
                &packet as *const _ as *const u8,
                out_buf,
                std::mem::size_of::<TPacketCGExchange>(),
            );
        }
        std::mem::size_of::<TPacketCGExchange>()
    })
}

#[no_mangle]
pub extern "C" fn m2_send_exchange_item_add(window_type: u8, cell: u16, display_pos: u8, out_buf: *mut u8) -> usize {
    safe_ffi_call(0, || {
        if out_buf.is_null() {
            return 0;
        }

        let mut packet = TPacketCGExchange::new(EXCHANGE_SUB_ITEM_ADD);
        packet.pos = TItemPos { window_type, cell };
        packet.arg2 = display_pos;

        unsafe {
            std::ptr::copy_nonoverlapping(
                &packet as *const _ as *const u8,
                out_buf,
                std::mem::size_of::<TPacketCGExchange>(),
            );
        }
        std::mem::size_of::<TPacketCGExchange>()
    })
}

#[no_mangle]
pub extern "C" fn m2_send_exchange_item_del(pos: u8, out_buf: *mut u8) -> usize {
    safe_ffi_call(0, || {
        if out_buf.is_null() {
            return 0;
        }

        let mut packet = TPacketCGExchange::new(EXCHANGE_SUB_ITEM_DEL);
        packet.arg1 = pos as u32;

        unsafe {
            std::ptr::copy_nonoverlapping(
                &packet as *const _ as *const u8,
                out_buf,
                std::mem::size_of::<TPacketCGExchange>(),
            );
        }
        std::mem::size_of::<TPacketCGExchange>()
    })
}

#[no_mangle]
pub extern "C" fn m2_send_exchange_accept(out_buf: *mut u8) -> usize {
    safe_ffi_call(0, || {
        if out_buf.is_null() {
            return 0;
        }

        let packet = TPacketCGExchange::new(EXCHANGE_SUB_ACCEPT);

        unsafe {
            std::ptr::copy_nonoverlapping(
                &packet as *const _ as *const u8,
                out_buf,
                std::mem::size_of::<TPacketCGExchange>(),
            );
        }
        std::mem::size_of::<TPacketCGExchange>()
    })
}

#[no_mangle]
pub extern "C" fn m2_send_exchange_exit(out_buf: *mut u8) -> usize {
    safe_ffi_call(0, || {
        if out_buf.is_null() {
            return 0;
        }

        let packet = TPacketCGExchange::new(EXCHANGE_SUB_CANCEL);

        unsafe {
            std::ptr::copy_nonoverlapping(
                &packet as *const _ as *const u8,
                out_buf,
                std::mem::size_of::<TPacketCGExchange>(),
            );
        }
        std::mem::size_of::<TPacketCGExchange>()
    })
}
