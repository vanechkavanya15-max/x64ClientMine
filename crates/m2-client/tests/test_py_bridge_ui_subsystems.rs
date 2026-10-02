use m2_client::py_bridge_ui_subsystems::*;

#[test]
fn test_guild_methods_no_crash() {
    assert_eq!(m2_py_ui_guild_get_guild_id(), 0);
    assert_eq!(m2_py_ui_guild_get_member_count(), 0);
    assert_eq!(m2_py_ui_guild_has_grade_auth(0, 1), false);
    
    let mut buf = vec![0u8; 64];
    assert_eq!(m2_py_ui_guild_get_guild_name(buf.as_mut_ptr(), buf.len()), false);
    assert_eq!(m2_py_ui_guild_get_guild_master_name(buf.as_mut_ptr(), buf.len()), false);
}

#[test]
fn test_shop_methods_no_crash() {
    assert_eq!(m2_py_ui_shop_get_item_vnum(0), 0);
    assert_eq!(m2_py_ui_shop_get_item_count(0), 0);
    assert_eq!(m2_py_ui_shop_get_item_price(0), 0);
    assert_eq!(m2_py_ui_shop_is_open(), false);
}

#[test]
fn test_skill_methods_no_crash() {
    let name_ptr = m2_py_ui_skill_get_skill_name(1);
    assert!(name_ptr.is_null());
    
    let desc_ptr = m2_py_ui_skill_get_skill_description(1);
    assert!(desc_ptr.is_null());
    
    assert_eq!(m2_py_ui_skill_get_skill_type(1), 0);
    assert_eq!(m2_py_ui_skill_can_use_skill(1), false);
}

#[test]
fn test_safebox_methods_no_crash() {
    assert_eq!(m2_py_ui_safebox_get_item_id(0), 0);
    assert_eq!(m2_py_ui_safebox_get_item_count(0), 0);
    assert_eq!(m2_py_ui_safebox_get_money(), 0);
    assert_eq!(m2_py_ui_safebox_get_size(), 0);
}

#[test]
fn test_exchange_methods_no_crash() {
    assert_eq!(m2_py_ui_exchange_get_item_vnum_from_self(0), 0);
    assert_eq!(m2_py_ui_exchange_get_item_vnum_from_target(0), 0);
    assert_eq!(m2_py_ui_exchange_get_elk_from_self(), 0);
    assert_eq!(m2_py_ui_exchange_get_elk_from_target(), 0);
    assert_eq!(m2_py_ui_exchange_is_accepted_from_self(), false);
    assert_eq!(m2_py_ui_exchange_is_accepted_from_target(), false);
}

#[test]
fn test_chat_methods_no_crash() {
    assert_eq!(m2_py_ui_chat_append_chat(1, std::ptr::null()), false);
    
    let msg = std::ffi::CString::new("test message").unwrap();
    assert_eq!(m2_py_ui_chat_append_chat(1, msg.as_ptr()), true);
    
    assert!(m2_py_ui_chat_get_line_count() > 0);
}

#[test]
fn test_minimap_methods_no_crash() {
    assert_eq!(m2_py_ui_minimap_set_scale(1.5), true);
    assert_eq!(m2_py_ui_minimap_show_atlas(), true);
    assert_eq!(m2_py_ui_minimap_is_atlas_show(), true);
    assert_eq!(m2_py_ui_minimap_hide_atlas(), true);
    assert_eq!(m2_py_ui_minimap_is_atlas_show(), false);
}
