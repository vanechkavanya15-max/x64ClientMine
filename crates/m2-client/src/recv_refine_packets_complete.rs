#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const REFINE_MATERIAL_MAX_NUM: usize = 5;

// Network opcodes
pub const HEADER_GC_REFINE_INFORMATION: u16 = 0x051D;
pub const HEADER_GC_REFINE_INFORMATION_NEW: u16 = 0x051E;
pub const HEADER_GC_SPECIAL_EFFECT: u16 = 0x0A30;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TMaterial {
    pub vnum: u32,
    pub count: i32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TRefineTable {
    pub src_vnum: u32,
    pub result_vnum: u32,
    pub material_count: u8,
    pub cost: i32,
    pub prob: i32,
    pub materials: [TMaterial; REFINE_MATERIAL_MAX_NUM],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCRefineInformation {
    pub header: u16,
    pub length: u16,
    pub type_: u8,
    pub pos: u8,
    pub refine_table: TRefineTable,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCRefineInformationNew {
    pub header: u16,
    pub length: u16,
    pub type_: u8,
    pub pos: u8,
    pub refine_table: TRefineTable,
}

extern "C" {
    pub fn m2_recv_refine_open_dialog(pos: u8, result_vnum: u32, cost: i32, prob: i32);
    pub fn m2_recv_refine_open_dialog_new(pos: u8, result_vnum: u32, cost: i32, prob: i32, type_: u8);
    pub fn m2_recv_refine_append_material(vnum: u32, count: i32);
}

#[no_mangle]
pub unsafe extern "C" fn m2_recv_refine_information_complete(packet_ptr: *const TPacketGCRefineInformation) -> bool {
    if packet_ptr.is_null() {
        return false;
    }

    let packet = &*packet_ptr;
    if packet.header != HEADER_GC_REFINE_INFORMATION {
        return false;
    }

    let table = &packet.refine_table;
    m2_recv_refine_open_dialog(
        packet.pos,
        table.result_vnum,
        table.cost,
        table.prob,
    );

    for i in 0..(table.material_count as usize) {
        if i < REFINE_MATERIAL_MAX_NUM {
            let material = &table.materials[i];
            m2_recv_refine_append_material(material.vnum, material.count);
        }
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn m2_recv_refine_information_new_complete(packet_ptr: *const TPacketGCRefineInformationNew) -> bool {
    if packet_ptr.is_null() {
        return false;
    }

    let packet = &*packet_ptr;
    if packet.header != HEADER_GC_REFINE_INFORMATION_NEW {
        return false;
    }

    let table = &packet.refine_table;
    m2_recv_refine_open_dialog_new(
        packet.pos,
        table.result_vnum,
        table.cost,
        table.prob,
        packet.type_,
    );

    for i in 0..(table.material_count as usize) {
        if i < REFINE_MATERIAL_MAX_NUM {
            let material = &table.materials[i];
            m2_recv_refine_append_material(material.vnum, material.count);
        }
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn m2_recv_special_effect_complete(packet_ptr: *const TPacketGCSpecialEffect) -> bool {
    if packet_ptr.is_null() {
        return false;
    }

    let packet = &*packet_ptr;
    if packet.header != 0x0A30 {
        return false;
    }

    match packet.type_ {
        11 => { // SE_SUCCESS
            // effect = CInstanceBase::EFFECT_SUCCESS; bAttachEffect = false;
        }
        12 => { // SE_FAIL
            // effect = CInstanceBase::EFFECT_FAIL;
        }
        _ => {}
    }

    true
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCSpecialEffect {
    pub header: u16,
    pub length: u16,
    pub type_: u8,
    pub vid: u32,
}
