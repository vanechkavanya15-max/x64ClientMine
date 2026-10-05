// crates/m2-client/tests/test_py_bridge_chr_transforms.rs
use std::ffi::CString;
use m2_client::py_bridge_chr_transforms::*;

#[test]
fn test_chr_transforms_and_lifecycle_suite() {
    // 1. Wybor instancji i przypisanie vnum
    let vid = 10050;
    assert!(m2_chr_select_instance_by_id(vid));
    assert_eq!(m2_chr_get_selected_instance_vid(), vid);

    let vnum = 101; // Dziki pies
    assert!(m2_chr_set_virtual_number_by_vid(vid, vnum));
    assert_eq!(m2_chr_get_virtual_number_by_vid(vid), vnum);
    let _ = m2_chr_get_virtual_number(vid);

    // 2. Nazwa i typ instancji
    let npc_vid = 20016;
    assert!(m2_chr_select_instance_by_id(npc_vid));
    assert_eq!(m2_chr_get_selected_instance_vid(), npc_vid);

    let c_name = CString::new("Kowal").unwrap();
    unsafe {
        assert!(m2_chr_set_name_string(c_name.as_ptr()));
    }
    assert!(m2_chr_set_instance_type(1)); // TYPE_NPC

    // 3. Refresh i deform
    let mob_vid = 50001;
    assert!(m2_chr_select_instance_by_id(mob_vid));
    assert!(m2_chr_refresh_selected());
    assert!(m2_chr_deform_selected());
    assert!(m2_chr_refresh(mob_vid));
    assert!(m2_chr_deform(mob_vid));
}
