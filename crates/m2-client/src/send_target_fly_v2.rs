use std::ptr;

const HEADER_CG_TARGET: u16 = 0x0A01;
const HEADER_CG_FLY_TARGETING: u16 = 0x0404;

#[repr(C, packed)]
struct TPacketCGTarget {
    header: u16,
    length: u16,
    dw_vid: u32,
}

#[repr(C, packed)]
struct TPacketCGFlyTargeting {
    header: u16,
    length: u16,
    dw_target_vid: u32,
    l_x: i32,
    l_y: i32,
}

/// Serializes target packet (HEADER_CG_TARGET).
/// Equivalent to CPythonNetworkStream::SendTargetPacket.
/// 
/// `vid`: The target victim ID.
/// `out_buf`: Pointer to output buffer (must be at least 8 bytes).
/// Returns: Number of bytes written (8), or 0 on error.
#[no_mangle]
pub extern "C" fn m2_send_target(vid: u32, out_buf: *mut u8) -> usize {
    if out_buf.is_null() {
        return 0;
    }

    let size = std::mem::size_of::<TPacketCGTarget>();
    
    let packet = TPacketCGTarget {
        header: HEADER_CG_TARGET,
        length: size as u16,
        dw_vid: vid,
    };

    unsafe {
        ptr::copy_nonoverlapping(
            &packet as *const _ as *const u8,
            out_buf,
            size,
        );
    }

    size
}

/// Serializes fly target packet (HEADER_CG_FLY_TARGETING).
/// Equivalent to CPythonNetworkStream::SendFlyTargetingPacket.
/// 
/// `vid`: The target victim ID.
/// `x`: The global target X position.
/// `y`: The global target Y position.
/// `out_buf`: Pointer to output buffer (must be at least 16 bytes).
/// Returns: Number of bytes written (16), or 0 on error.
#[no_mangle]
pub extern "C" fn m2_send_fly_target(vid: u32, x: i32, y: i32, out_buf: *mut u8) -> usize {
    if out_buf.is_null() {
        return 0;
    }

    let size = std::mem::size_of::<TPacketCGFlyTargeting>();
    
    let packet = TPacketCGFlyTargeting {
        header: HEADER_CG_FLY_TARGETING,
        length: size as u16,
        dw_target_vid: vid,
        l_x: x,
        l_y: y,
    };

    unsafe {
        ptr::copy_nonoverlapping(
            &packet as *const _ as *const u8,
            out_buf,
            size,
        );
    }

    size
}
