use m2_client::recv_refine_blacksmith_v2::*;
use std::mem::size_of;

#[test]
fn test_struct_sizes() {
    assert_eq!(size_of::<TMaterial>(), 5);
    // target_pos (2) + result_vnum (4) + cost (8) + prob (1) + mat_count (1) + 5 * 5 (25) = 41
    assert_eq!(size_of::<TPacketGCRefineInformation>(), 41);
}

#[test]
fn test_parse_refine_information_valid() {
    let mut packet = TPacketGCRefineInformation {
        target_pos: 2,
        result_vnum: 11,
        cost: 1000,
        prob: 80,
        material_count: 2,
        materials: [
            TMaterial { vnum: 100, count: 1 },
            TMaterial { vnum: 200, count: 2 },
            TMaterial { vnum: 0, count: 0 },
            TMaterial { vnum: 0, count: 0 },
            TMaterial { vnum: 0, count: 0 },
        ],
    };

    let data = unsafe {
        std::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<TPacketGCRefineInformation>())
    };

    let mut out_target_pos = 0;
    let mut out_result_vnum = 0;
    let mut out_cost = 0;
    let mut out_prob = 0;
    let mut out_material_count = 0;
    let mut out_materials = [TMaterial { vnum: 0, count: 0 }; 5];

    let result = m2_refine_get_info(
        data.as_ptr(),
        data.len(),
        &mut out_target_pos,
        &mut out_result_vnum,
        &mut out_cost,
        &mut out_prob,
        &mut out_material_count,
        out_materials.as_mut_ptr(),
    );

    assert!(result);
    assert_eq!(out_target_pos, 2);
    assert_eq!(out_result_vnum, 11);
    assert_eq!(out_cost, 1000);
    assert_eq!(out_prob, 80);
    assert_eq!(out_material_count, 2);
    assert_eq!(out_materials[0], TMaterial { vnum: 100, count: 1 });
    assert_eq!(out_materials[1], TMaterial { vnum: 200, count: 2 });
}

#[test]
fn test_parse_invalid_length() {
    let data = [0u8; 10]; // Too short
    assert!(!m2_refine_get_info(
        data.as_ptr(),
        data.len(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut()
    ));
}

#[test]
fn test_parse_material_count_out_of_bounds() {
    let mut packet = TPacketGCRefineInformation {
        target_pos: 2,
        result_vnum: 11,
        cost: 1000,
        prob: 80,
        material_count: 6, // Exceeds REFINE_MATERIAL_MAX_NUM
        materials: [TMaterial { vnum: 0, count: 0 }; 5],
    };

    let data = unsafe {
        std::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<TPacketGCRefineInformation>())
    };

    assert!(!m2_refine_get_info(
        data.as_ptr(),
        data.len(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut()
    ));
}

#[test]
fn test_ffi_null_pointers_allowed_for_out() {
    let mut packet = TPacketGCRefineInformation {
        target_pos: 2,
        result_vnum: 11,
        cost: 1000,
        prob: 80,
        material_count: 1,
        materials: [TMaterial { vnum: 0, count: 0 }; 5],
    };

    let data = unsafe {
        std::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<TPacketGCRefineInformation>())
    };

    let result = m2_refine_get_info(
        data.as_ptr(),
        data.len(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );

    assert!(result);
}
