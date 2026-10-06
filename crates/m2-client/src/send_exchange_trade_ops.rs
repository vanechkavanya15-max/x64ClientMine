use m2_client_bridge::ffi_guard::safe_ffi_call;
use m2_client::ui_bridge_ffi::ACTIVE_GAME_SOCKET;
use std::io::Write;

pub const HEADER_CG_EXCHANGE: u16 = 0x0508;

pub const EXCHANGE_SUBHEADER_CG_START: u8 = 0;
pub const EXCHANGE_SUBHEADER_CG_ITEM_ADD: u8 = 1;
pub const EXCHANGE_SUBHEADER_CG_ITEM_DEL: u8 = 2;
pub const EXCHANGE_SUBHEADER_CG_ELK_ADD: u8 = 3;
pub const EXCHANGE_SUBHEADER_CG_ACCEPT: u8 = 4;
pub const EXCHANGE_SUBHEADER_CG_CANCEL: u8 = 5;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
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

fn send_packet(bytes: &[u8]) {
    if let Ok(mut guard) = ACTIVE_GAME_SOCKET.lock() {
        if let Some((stream, cipher)) = guard.as_mut() {
            let mut encrypted_bytes = bytes.to_vec();
            cipher.encrypt_in_place(&mut encrypted_bytes);
            let _ = stream.write_all(&encrypted_bytes);
        }
    }
}

#[no_mangle]
pub extern "C" fn m2_send_exchange_ops_start(vid: u32, out_buf: *mut u8) -> usize {
    safe_ffi_call(0, || {
        let mut packet = TPacketCGExchange::new(EXCHANGE_SUBHEADER_CG_START);
        packet.arg1 = vid;

        let packet_bytes: [u8; std::mem::size_of::<TPacketCGExchange>()] = unsafe { std::mem::transmute(packet) };
        send_packet(&packet_bytes);

        if out_buf.is_null() {
            return 0;
        }

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
pub extern "C" fn m2_send_exchange_ops_item_add(window_type: u8, cell: u16, display_pos: u8, out_buf: *mut u8) -> usize {
    safe_ffi_call(0, || {
        let mut packet = TPacketCGExchange::new(EXCHANGE_SUBHEADER_CG_ITEM_ADD);
        packet.pos = TItemPos { window_type, cell };
        packet.arg2 = display_pos;

        let packet_bytes: [u8; std::mem::size_of::<TPacketCGExchange>()] = unsafe { std::mem::transmute(packet) };
        send_packet(&packet_bytes);

        if out_buf.is_null() {
            return 0;
        }

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
pub extern "C" fn m2_send_exchange_ops_item_del(pos: u8, out_buf: *mut u8) -> usize {
    safe_ffi_call(0, || {
        let mut packet = TPacketCGExchange::new(EXCHANGE_SUBHEADER_CG_ITEM_DEL);
        packet.arg1 = pos as u32;

        let packet_bytes: [u8; std::mem::size_of::<TPacketCGExchange>()] = unsafe { std::mem::transmute(packet) };
        send_packet(&packet_bytes);

        if out_buf.is_null() {
            return 0;
        }

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
pub extern "C" fn m2_send_exchange_ops_elk_add(elk: u32, out_buf: *mut u8) -> usize {
    safe_ffi_call(0, || {
        let mut packet = TPacketCGExchange::new(EXCHANGE_SUBHEADER_CG_ELK_ADD);
        packet.arg1 = elk;

        let packet_bytes: [u8; std::mem::size_of::<TPacketCGExchange>()] = unsafe { std::mem::transmute(packet) };
        send_packet(&packet_bytes);

        if out_buf.is_null() {
            return 0;
        }

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
pub extern "C" fn m2_send_exchange_ops_accept(out_buf: *mut u8) -> usize {
    safe_ffi_call(0, || {
        let packet = TPacketCGExchange::new(EXCHANGE_SUBHEADER_CG_ACCEPT);

        let packet_bytes: [u8; std::mem::size_of::<TPacketCGExchange>()] = unsafe { std::mem::transmute(packet) };
        send_packet(&packet_bytes);

        if out_buf.is_null() {
            return 0;
        }

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
pub extern "C" fn m2_send_exchange_ops_cancel(out_buf: *mut u8) -> usize {
    safe_ffi_call(0, || {
        let packet = TPacketCGExchange::new(EXCHANGE_SUBHEADER_CG_CANCEL);

        let packet_bytes: [u8; std::mem::size_of::<TPacketCGExchange>()] = unsafe { std::mem::transmute(packet) };
        send_packet(&packet_bytes);

        if out_buf.is_null() {
            return 0;
        }

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
