#[path = "../src/recv_refine_packets_complete.rs"]
mod recv_refine_packets_complete;

use recv_refine_packets_complete::*;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref CALLS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

#[no_mangle]
pub extern "C" fn m2_recv_refine_open_dialog(pos: u8, result_vnum: u32, cost: i32, prob: i32) {
    CALLS.lock().unwrap().push(format!("open_refine_dialog({}, {}, {}, {})", pos, result_vnum, cost, prob));
}

#[no_mangle]
pub extern "C" fn m2_recv_refine_open_dialog_new(pos: u8, result_vnum: u32, cost: i32, prob: i32, type_: u8) {
    CALLS.lock().unwrap().push(format!("open_refine_dialog_new({}, {}, {}, {}, {})", pos, result_vnum, cost, prob, type_));
}

#[no_mangle]
pub extern "C" fn m2_recv_refine_append_material(vnum: u32, count: i32) {
    CALLS.lock().unwrap().push(format!("append_material({}, {})", vnum, count));
}

#[test]
fn test_refine_information_packet_size() {
    assert_eq!(std::mem::size_of::<TMaterial>(), 8);
    assert_eq!(std::mem::size_of::<TRefineTable>(), 57);
}

#[test]
fn test_recv_refine_information() {
    CALLS.lock().unwrap().clear();

    let mut refine_table = TRefineTable {
        src_vnum: 1,
        result_vnum: 100,
        material_count: 2,
        cost: 500,
        prob: 75,
        materials: [TMaterial { vnum: 0, count: 0 }; 5],
    };
    refine_table.materials[0] = TMaterial { vnum: 10, count: 1 };
    refine_table.materials[1] = TMaterial { vnum: 20, count: 5 };

    let packet = TPacketGCRefineInformation {
        header: HEADER_GC_REFINE_INFORMATION,
        length: std::mem::size_of::<TPacketGCRefineInformation>() as u16,
        type_: 0,
        pos: 3,
        refine_table,
    };

    unsafe {
        assert!(m2_recv_refine_information_complete(&packet));
    }

    let calls = CALLS.lock().unwrap();
    assert_eq!(calls.len(), 3);
    assert_eq!(calls[0], "open_refine_dialog(3, 100, 500, 75)");
    assert_eq!(calls[1], "append_material(10, 1)");
    assert_eq!(calls[2], "append_material(20, 5)");
}

#[test]
fn test_recv_refine_information_new() {
    CALLS.lock().unwrap().clear();

    let mut refine_table = TRefineTable {
        src_vnum: 2,
        result_vnum: 200,
        material_count: 1,
        cost: 1000,
        prob: 50,
        materials: [TMaterial { vnum: 0, count: 0 }; 5],
    };
    refine_table.materials[0] = TMaterial { vnum: 30, count: 10 };

    let packet = TPacketGCRefineInformationNew {
        header: HEADER_GC_REFINE_INFORMATION_NEW,
        length: std::mem::size_of::<TPacketGCRefineInformationNew>() as u16,
        type_: 5,
        pos: 1,
        refine_table,
    };

    unsafe {
        assert!(m2_recv_refine_information_new_complete(&packet));
    }

    let calls = CALLS.lock().unwrap();
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0], "open_refine_dialog_new(1, 200, 1000, 50, 5)");
    assert_eq!(calls[1], "append_material(30, 10)");
}

#[test]
fn test_invalid_header() {
    let packet = TPacketGCRefineInformation {
        header: 0x9999,
        length: 0,
        type_: 0,
        pos: 0,
        refine_table: TRefineTable {
            src_vnum: 0,
            result_vnum: 0,
            material_count: 0,
            cost: 0,
            prob: 0,
            materials: [TMaterial { vnum: 0, count: 0 }; 5],
        },
    };
    unsafe {
        assert!(!m2_recv_refine_information_complete(&packet));
    }
}

#[test]
fn test_recv_special_effect() {
    let mut packet = TPacketGCSpecialEffect {
        header: HEADER_GC_SPECIAL_EFFECT,
        length: std::mem::size_of::<TPacketGCSpecialEffect>() as u16,
        type_: 11, // SE_SUCCESS
        vid: 12345,
    };
    unsafe {
        assert!(m2_recv_special_effect_complete(&packet));
    }
    
    packet.type_ = 12; // SE_FAIL
    unsafe {
        assert!(m2_recv_special_effect_complete(&packet));
    }

    packet.header = 0x9999;
    unsafe {
        assert!(!m2_recv_special_effect_complete(&packet));
    }
}

#[test]
fn test_null_pointers() {
    unsafe {
        assert!(!m2_recv_refine_information_complete(std::ptr::null()));
        assert!(!m2_recv_refine_information_new_complete(std::ptr::null()));
        assert!(!m2_recv_special_effect_complete(std::ptr::null()));
    }
}
