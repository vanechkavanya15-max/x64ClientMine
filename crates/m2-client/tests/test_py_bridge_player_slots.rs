#[path = "../src/py_bridge_player_slots.rs"]
mod py_bridge_player_slots;

use py_bridge_player_slots::*;
use m2_client_bridge::state::game_state::GLOBAL_GAME_STATE;

#[test]
fn test_is_equipment_slot() {
    assert_eq!(m2_py_player_is_equipment_slot(EQUIPMENT_START as i32), 1);
    assert_eq!(m2_py_player_is_equipment_slot(DRAGON_SOUL_EQUIP_END as i32), 1);
    
    assert_eq!(m2_py_player_is_equipment_slot((EQUIPMENT_START - 1) as i32), 0);
    assert_eq!(m2_py_player_is_equipment_slot((DRAGON_SOUL_EQUIP_END + 1) as i32), 0);
}

#[test]
fn test_is_costume_slot() {
    assert_eq!(m2_py_player_is_costume_slot(COSTUME_SLOT_START as i32), 1);
    assert_eq!(m2_py_player_is_costume_slot(COSTUME_SLOT_END as i32), 1);
    
    assert_eq!(m2_py_player_is_costume_slot((COSTUME_SLOT_START - 1) as i32), 0);
    assert_eq!(m2_py_player_is_costume_slot((COSTUME_SLOT_END + 1) as i32), 0);
}

#[test]
fn test_is_belt_inventory_slot() {
    assert_eq!(m2_py_player_is_belt_inventory_slot(BELT_INVENTORY_SLOT_START as i32), 1);
    assert_eq!(m2_py_player_is_belt_inventory_slot((BELT_INVENTORY_SLOT_END - 1) as i32), 1);
    
    assert_eq!(m2_py_player_is_belt_inventory_slot((BELT_INVENTORY_SLOT_START - 1) as i32), 0);
    assert_eq!(m2_py_player_is_belt_inventory_slot(BELT_INVENTORY_SLOT_END as i32), 0);
}

#[test]
fn test_is_ds_equipment_slot() {
    assert_eq!(m2_py_player_is_ds_equipment_slot(WINDOW_INVENTORY, DRAGON_SOUL_EQUIP_START as i32), 1);
    assert_eq!(m2_py_player_is_ds_equipment_slot(WINDOW_INVENTORY, DRAGON_SOUL_EQUIP_END as i32), 1);
    
    assert_eq!(m2_py_player_is_ds_equipment_slot(WINDOW_EQUIPMENT, DRAGON_SOUL_EQUIP_START as i32), 0);
    assert_eq!(m2_py_player_is_ds_equipment_slot(WINDOW_INVENTORY, (DRAGON_SOUL_EQUIP_START - 1) as i32), 0);
    assert_eq!(m2_py_player_is_ds_equipment_slot(WINDOW_INVENTORY, (DRAGON_SOUL_EQUIP_END + 1) as i32), 0);
}

#[test]
fn test_slot_type_to_inven_type() {
    assert_eq!(m2_py_player_slot_type_to_inven_type(SLOT_TYPE_INVENTORY), WINDOW_INVENTORY as i32);
    assert_eq!(m2_py_player_slot_type_to_inven_type(SLOT_TYPE_DRAGON_SOUL_INVENTORY), WINDOW_DRAGON_SOUL_INVENTORY as i32);
    
    // Safebox is mapped to RESERVED_WINDOW in C++ as legacy behavior
    assert_eq!(m2_py_player_slot_type_to_inven_type(SLOT_TYPE_SAFEBOX), WINDOW_RESERVED as i32);
    
    assert_eq!(m2_py_player_slot_type_to_inven_type(SLOT_TYPE_MAX), WINDOW_RESERVED as i32);
}

#[test]
fn test_is_available_belt_inventory_cell_no_belt() {
    // Clear global game state equipment belt first just in case
    {
        let mut state = GLOBAL_GAME_STATE.write().unwrap();
        state.player.inventory.remove(&(WINDOW_EQUIPMENT, EQUIPMENT_BELT));
    }
    
    assert_eq!(m2_py_player_is_available_belt_inventory_cell(BELT_INVENTORY_SLOT_START as i32), false);
}
