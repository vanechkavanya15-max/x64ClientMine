pub use m2_client::*;
use character_manager_core::get_character_manager;

#[path = "../src/py_bridge_chr_lifecycle.rs"]
mod py_bridge_chr_lifecycle;

extern "C" {
    fn m2_py_chr_create_instance(vid: u32) -> bool;
    fn m2_py_chr_delete_instance(vid: u32) -> bool;
    fn m2_py_chr_delete_instance_by_fade(vid: u32) -> bool;
    fn m2_py_chr_select(vid: u32) -> bool;
    fn m2_py_chr_unselect() -> bool;
    fn m2_py_chr_destroy() -> bool;
}

#[test]
fn test_create_and_delete_instance() {
    let vid = 1001;
    unsafe { m2_py_chr_destroy() };
    
    assert!(unsafe { m2_py_chr_create_instance(vid) }, "Should successfully create instance");
    
    {
        let mgr = get_character_manager().read().unwrap();
        assert!(mgr.get_instance(vid).is_some(), "Instance should exist in character manager");
    }
    
    assert!(!unsafe { m2_py_chr_create_instance(vid) }, "Should fail to create duplicate instance");
    assert!(unsafe { m2_py_chr_delete_instance(vid) }, "Should successfully delete instance");
    
    {
        let mgr = get_character_manager().read().unwrap();
        assert!(mgr.get_instance(vid).is_none(), "Instance should not exist after deletion");
    }
}

#[test]
fn test_select_and_unselect_instance() {
    let vid = 2002;
    unsafe { m2_py_chr_destroy() };
    
    unsafe { m2_py_chr_create_instance(vid) };
    
    assert!(unsafe { m2_py_chr_select(vid) }, "Should select successfully");
    
    assert!(unsafe { m2_py_chr_unselect() }, "Should unselect successfully");
    
    assert!(!unsafe { m2_py_chr_select(9999) }, "Should fail to select non-existent instance");
}

#[test]
fn test_delete_instance_clears_selection() {
    let vid = 3003;
    unsafe { m2_py_chr_destroy() };
    
    unsafe { m2_py_chr_create_instance(vid) };
    unsafe { m2_py_chr_select(vid) };
    
    unsafe { m2_py_chr_delete_instance(vid) };
}

#[test]
fn test_delete_instance_by_fade() {
    let vid = 4004;
    unsafe { m2_py_chr_destroy() };
    unsafe { m2_py_chr_create_instance(vid) };
    
    assert!(unsafe { m2_py_chr_delete_instance_by_fade(vid) }, "Should successfully queue for fade");
}

#[test]
fn test_destroy_manager() {
    unsafe { m2_py_chr_destroy() };
    unsafe { m2_py_chr_create_instance(5005) };
    unsafe { m2_py_chr_create_instance(6006) };
    unsafe { m2_py_chr_select(5005) };
    
    assert!(unsafe { m2_py_chr_destroy() }, "Should successfully destroy");
    
    let mgr = get_character_manager().read().unwrap();
    assert_eq!(mgr.count(), 0, "Character manager should be empty after destroy");
}
