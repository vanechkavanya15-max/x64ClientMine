use std::mem::size_of;
use std::slice;

pub const REFINE_MATERIAL_MAX_NUM: usize = 5;

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TMaterial {
    pub vnum: u32,
    pub count: u8,
}

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TPacketGCRefineInformation {
    pub target_pos: u16,
    pub result_vnum: u32,
    pub cost: u64,
    pub prob: u8,
    pub material_count: u8,
    pub materials: [TMaterial; REFINE_MATERIAL_MAX_NUM],
}

#[no_mangle]
pub extern "C" fn m2_refine_get_info(
    buf: *const u8,
    len: usize,
    out_target_pos: *mut u16,
    out_result_vnum: *mut u32,
    out_cost: *mut u64,
    out_prob: *mut u8,
    out_material_count: *mut u8,
    out_materials: *mut TMaterial,
) -> bool {
    if buf.is_null() || len < size_of::<TPacketGCRefineInformation>() {
        return false;
    }

    let slice = unsafe { slice::from_raw_parts(buf, len) };
    let packet = unsafe { (slice.as_ptr() as *const TPacketGCRefineInformation).read_unaligned() };

    let mat_count = packet.material_count as usize;
    if mat_count > REFINE_MATERIAL_MAX_NUM {
        return false;
    }

    if !out_target_pos.is_null() {
        unsafe { *out_target_pos = packet.target_pos; }
    }
    if !out_result_vnum.is_null() {
        unsafe { *out_result_vnum = packet.result_vnum; }
    }
    if !out_cost.is_null() {
        unsafe { *out_cost = packet.cost; }
    }
    if !out_prob.is_null() {
        unsafe { *out_prob = packet.prob; }
    }
    if !out_material_count.is_null() {
        unsafe { *out_material_count = packet.material_count; }
    }
    if !out_materials.is_null() {
        let out_mats = unsafe { slice::from_raw_parts_mut(out_materials, REFINE_MATERIAL_MAX_NUM) };
        for i in 0..mat_count {
            out_mats[i] = packet.materials[i];
        }
    }

    true
}
