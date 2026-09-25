use crate::ffi_guard::safe_ffi_call;
use std::slice;

pub const HEADER_GC_ITEM_SET: u8 = 21;

#[derive(Debug, PartialEq)]
pub enum PacketError {
    BufferTooSmall,
    InvalidHeader,
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TPlayerItemAttribute {
    pub b_type: u8,
    pub s_value: i16,
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(non_snake_case)]
pub struct TPacketGCItemSet {
    pub header: u8,
    pub Cell: TItemPos,
    pub vnum: u32,
    pub count: u8,
    pub al_sockets: [u32; 3],
    pub a_attrs: [TPlayerItemAttribute; 7],
}

impl TPacketGCItemSet {
    pub const EXPECTED_SIZE: usize = 42;

    pub fn from_bytes(data: &[u8]) -> Result<Self, PacketError> {
        if data.len() < Self::EXPECTED_SIZE {
            return Err(PacketError::BufferTooSmall);
        }
        
        let header = data[0];
        if header != HEADER_GC_ITEM_SET {
            return Err(PacketError::InvalidHeader);
        }

        let window_type = data[1];
        let cell = u16::from_le_bytes([data[2], data[3]]);
        
        let vnum = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        let count = data[8];
        
        let mut al_sockets = [0u32; 3];
        for i in 0..3 {
            let offset = 9 + i * 4;
            al_sockets[i] = u32::from_le_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]);
        }
        
        let mut a_attrs = [TPlayerItemAttribute { b_type: 0, s_value: 0 }; 7];
        for i in 0..7 {
            let offset = 21 + i * 3;
            let b_type = data[offset];
            let s_value = i16::from_le_bytes([data[offset+1], data[offset+2]]);
            a_attrs[i] = TPlayerItemAttribute { b_type, s_value };
        }
        
        Ok(Self {
            header,
            Cell: TItemPos { window_type, cell },
            vnum,
            count,
            al_sockets,
            a_attrs,
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_parse_item_set(
    data: *const u8,
    len: usize,
    out_packet: *mut *mut TPacketGCItemSet,
) -> bool {
    safe_ffi_call(false, || {
        if data.is_null() || out_packet.is_null() || len == 0 {
            return false;
        }

        let slice = unsafe { slice::from_raw_parts(data, len) };
        
        match TPacketGCItemSet::from_bytes(slice) {
            Ok(packet) => {
                let boxed = Box::new(packet);
                unsafe {
                    *out_packet = Box::into_raw(boxed);
                }
                true
            }
            Err(_) => false,
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_free_item_set(packet: *mut TPacketGCItemSet) {
    safe_ffi_call((), || {
        if !packet.is_null() {
            unsafe {
                let _ = Box::from_raw(packet);
            }
        }
    });
}
