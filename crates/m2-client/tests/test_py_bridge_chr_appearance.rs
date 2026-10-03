#[path = "../src/py_bridge_chr_appearance.rs"]
mod py_bridge_chr_appearance;

use py_bridge_chr_appearance::*;
use m2_client::character_manager_core::get_character_manager;
use m2_client::actor_appearance::PART_MAIN;

// A helper macro for synchronizing tests that mutate global state.
use std::sync::OnceLock;
use std::sync::Mutex;
static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn setup_test_manager() -> std::sync::MutexGuard<'static, ()> {
    let guard = TEST_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap();
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
    mgr.create_instance(100, 0, 0.0, 0.0, 0.0);
    mgr.main_actor_vid = Some(100);
    guard
}

fn clear_test_manager() {
    let mut mgr = get_character_manager().write().unwrap();
    mgr.clear();
}

#[test]
fn test_m2_py_chr_set_race() {
    let _guard = setup_test_manager();
    
    let result = m2_py_chr_set_race(1);
    assert!(result);
    
    let mgr = get_character_manager().read().unwrap();
    let inst = mgr.get_instance(100).unwrap();
    assert_eq!(inst.race, 1);
    
    clear_test_manager();
}

#[test]
fn test_m2_py_chr_set_armor() {
    let _guard = setup_test_manager();
    
    let result = m2_py_chr_set_armor(12010);
    assert!(result);
    
    let mgr = get_character_manager().read().unwrap();
    let inst = mgr.get_instance(100).unwrap();
    assert_eq!(inst.appearance.armor_vnum, 12010);
    
    clear_test_manager();
}

#[test]
fn test_m2_py_chr_change_shape() {
    let _guard = setup_test_manager();
    
    let result = m2_py_chr_change_shape(11299);
    assert!(result);
    
    let mgr = get_character_manager().read().unwrap();
    let inst = mgr.get_instance(100).unwrap();
    assert_eq!(inst.appearance.armor_vnum, 11299);
    
    clear_test_manager();
}

#[test]
fn test_m2_py_chr_set_weapon() {
    let _guard = setup_test_manager();
    
    let result = m2_py_chr_set_weapon(299);
    assert!(result);
    
    let mgr = get_character_manager().read().unwrap();
    let inst = mgr.get_instance(100).unwrap();
    assert_eq!(inst.appearance.weapon_vnum, 299);
    
    clear_test_manager();
}

#[test]
fn test_m2_py_chr_set_hair() {
    let _guard = setup_test_manager();
    
    let result = m2_py_chr_set_hair(2010);
    assert!(result);
    
    let mgr = get_character_manager().read().unwrap();
    let inst = mgr.get_instance(100).unwrap();
    assert_eq!(inst.appearance.hair_vnum, 2010);
    
    clear_test_manager();
}

#[test]
fn test_m2_py_chr_change_hair() {
    let _guard = setup_test_manager();
    
    let result = m2_py_chr_change_hair(2020);
    assert!(result);
    
    let mgr = get_character_manager().read().unwrap();
    let inst = mgr.get_instance(100).unwrap();
    assert_eq!(inst.appearance.hair_vnum, 2020);
    
    clear_test_manager();
}

#[test]
fn test_no_main_actor() {
    let _guard = setup_test_manager();
    
    // Clear main actor
    {
        let mut mgr = get_character_manager().write().unwrap();
        mgr.main_actor_vid = None;
    }
    
    let result = m2_py_chr_set_race(2);
    assert!(!result);
    
    clear_test_manager();
}

#[test]
fn test_main_actor_not_found_in_instances() {
    let _guard = setup_test_manager();
    
    // Set main actor vid to a non-existent vid
    {
        let mut mgr = get_character_manager().write().unwrap();
        mgr.main_actor_vid = Some(999);
    }
    
    let result = m2_py_chr_set_armor(12010);
    assert!(!result);
    
    clear_test_manager();
}
