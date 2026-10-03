#![allow(non_snake_case)]

use std::mem::size_of;
use std::ptr;
use crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET;
use std::io::Write;

pub const HEADER_CG_ITEM_USE: u16 = 0x0501;
pub const HEADER_CG_ITEM_MOVE: u16 = 0x0504;
pub const HEADER_CG_ITEM_DROP: u16 = 0x0503;
pub const HEADER_CG_ITEM_PICKUP: u16 = 0x0505;
pub const HEADER_CG_SAFEBOX_CHECKOUT: u16 = 0x0821;
pub const HEADER_CG_SAFEBOX_CHECKIN: u16 = 0x0820;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TPacketCGItemUse {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TPacketCGItemMove {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
    pub change_pos: TItemPos,
    pub num: u8,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TPacketCGItemDrop {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
    pub gold: u32,
    pub count: u8,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TPacketCGItemPickUp {
    pub header: u16,
    pub length: u16,
    pub vid: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TPacketCGSafeboxCheckout {
    pub header: u16,
    pub length: u16,
    pub bSafePos: u8,
    pub ItemPos: TItemPos,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TPacketCGSafeboxCheckin {
    pub header: u16,
    pub length: u16,
    pub bSafePos: u8,
    pub ItemPos: TItemPos,
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
pub unsafe extern "C" fn m2_net_send_use_item_v2(
    window_type: u8,
    cell: u16,
    out_buf: *mut u8,
) -> usize {
    let packet = TPacketCGItemUse {
        header: HEADER_CG_ITEM_USE,
        length: size_of::<TPacketCGItemUse>() as u16,
        pos: TItemPos {
            window_type,
            cell,
        },
    };

    let packet_bytes: [u8; std::mem::size_of::<TPacketCGItemUse>()] = std::mem::transmute(packet);
    send_packet(&packet_bytes);

    if out_buf.is_null() {
        return 0;
    }

    ptr::copy_nonoverlapping(
        &packet as *const _ as *const u8,
        out_buf,
        size_of::<TPacketCGItemUse>(),
    );

    size_of::<TPacketCGItemUse>()
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_item_move_v2(
    from_window: u8,
    from_cell: u16,
    to_window: u8,
    to_cell: u16,
    count: u8,
    out_buf: *mut u8,
) -> usize {
    let packet = TPacketCGItemMove {
        header: HEADER_CG_ITEM_MOVE,
        length: size_of::<TPacketCGItemMove>() as u16,
        pos: TItemPos {
            window_type: from_window,
            cell: from_cell,
        },
        change_pos: TItemPos {
            window_type: to_window,
            cell: to_cell,
        },
        num: count,
    };

    let packet_bytes: [u8; std::mem::size_of::<TPacketCGItemMove>()] = std::mem::transmute(packet);
    send_packet(&packet_bytes);

    if out_buf.is_null() {
        return 0;
    }

    ptr::copy_nonoverlapping(
        &packet as *const _ as *const u8,
        out_buf,
        size_of::<TPacketCGItemMove>(),
    );

    size_of::<TPacketCGItemMove>()
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_item_drop_v2(
    window_type: u8,
    cell: u16,
    gold: u32,
    count: u8,
    out_buf: *mut u8,
) -> usize {
    let packet = TPacketCGItemDrop {
        header: HEADER_CG_ITEM_DROP,
        length: size_of::<TPacketCGItemDrop>() as u16,
        pos: TItemPos {
            window_type,
            cell,
        },
        gold,
        count,
    };

    let packet_bytes: [u8; std::mem::size_of::<TPacketCGItemDrop>()] = std::mem::transmute(packet);
    send_packet(&packet_bytes);

    if out_buf.is_null() {
        return 0;
    }

    ptr::copy_nonoverlapping(
        &packet as *const _ as *const u8,
        out_buf,
        size_of::<TPacketCGItemDrop>(),
    );

    size_of::<TPacketCGItemDrop>()
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_pickup_v2(
    vid: u32,
    out_buf: *mut u8,
) -> usize {
    let packet = TPacketCGItemPickUp {
        header: HEADER_CG_ITEM_PICKUP,
        length: size_of::<TPacketCGItemPickUp>() as u16,
        vid,
    };

    let packet_bytes: [u8; std::mem::size_of::<TPacketCGItemPickUp>()] = std::mem::transmute(packet);
    send_packet(&packet_bytes);

    if out_buf.is_null() {
        return 0;
    }

    ptr::copy_nonoverlapping(
        &packet as *const _ as *const u8,
        out_buf,
        size_of::<TPacketCGItemPickUp>(),
    );

    size_of::<TPacketCGItemPickUp>()
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_safebox_checkout(
    safebox_pos: u8,
    inven_window: u8,
    inven_cell: u16,
    out_buf: *mut u8,
) -> usize {
    let packet = TPacketCGSafeboxCheckout {
        header: HEADER_CG_SAFEBOX_CHECKOUT,
        length: size_of::<TPacketCGSafeboxCheckout>() as u16,
        bSafePos: safebox_pos,
        ItemPos: TItemPos {
            window_type: inven_window,
            cell: inven_cell,
        },
    };

    let packet_bytes: [u8; std::mem::size_of::<TPacketCGSafeboxCheckout>()] = std::mem::transmute(packet);
    send_packet(&packet_bytes);

    if out_buf.is_null() {
        return 0;
    }

    ptr::copy_nonoverlapping(
        &packet as *const _ as *const u8,
        out_buf,
        size_of::<TPacketCGSafeboxCheckout>(),
    );

    size_of::<TPacketCGSafeboxCheckout>()
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_send_safebox_checkin(
    safebox_pos: u8,
    inven_window: u8,
    inven_cell: u16,
    out_buf: *mut u8,
) -> usize {
    let packet = TPacketCGSafeboxCheckin {
        header: HEADER_CG_SAFEBOX_CHECKIN,
        length: size_of::<TPacketCGSafeboxCheckin>() as u16,
        bSafePos: safebox_pos,
        ItemPos: TItemPos {
            window_type: inven_window,
            cell: inven_cell,
        },
    };

    let packet_bytes: [u8; std::mem::size_of::<TPacketCGSafeboxCheckin>()] = std::mem::transmute(packet);
    send_packet(&packet_bytes);

    if out_buf.is_null() {
        return 0;
    }

    ptr::copy_nonoverlapping(
        &packet as *const _ as *const u8,
        out_buf,
        size_of::<TPacketCGSafeboxCheckin>(),
    );

    size_of::<TPacketCGSafeboxCheckin>()
}