#[path = "../src/equipment_slots.rs"]
mod equipment_slots;

use equipment_slots::{
    EquipmentManager, ItemData, SLOT_ARMOR,  SLOT_WEAPON, WEARABLE_BODY,
    WEARABLE_SHIELD, WEARABLE_WEAPON,
};

#[test]
fn test_equip_weapon_and_armor_valid() {
    let mut manager = EquipmentManager::new();

    let weapon = ItemData {
        vnum: 10,
        wearable_flag: WEARABLE_WEAPON,
    };
    let armor = ItemData {
        vnum: 11299,
        wearable_flag: WEARABLE_BODY,
    };

    // Equipping should return None because the slot was previously empty
    assert_eq!(manager.equip_item(SLOT_WEAPON, weapon.clone()), None);
    assert_eq!(manager.equip_item(SLOT_ARMOR, armor.clone()), None);

    assert_eq!(manager.get_item(SLOT_WEAPON), Some(&weapon));
    assert_eq!(manager.get_item(SLOT_ARMOR), Some(&armor));

    // Equip another weapon should return the previous weapon
    let new_weapon = ItemData {
        vnum: 20,
        wearable_flag: WEARABLE_WEAPON,
    };
    assert_eq!(manager.equip_item(SLOT_WEAPON, new_weapon.clone()), Some(weapon));
    assert_eq!(manager.get_item(SLOT_WEAPON), Some(&new_weapon));
}

#[test]
fn test_equip_shield_in_weapon_slot_invalid() {
    let mut manager = EquipmentManager::new();

    let shield = ItemData {
        vnum: 13009, // Battle Shield
        wearable_flag: WEARABLE_SHIELD,
    };

    // Attempting to equip a shield in the weapon slot should fail
    // and it should return the shield back.
    let returned_item = manager.equip_item(SLOT_WEAPON, shield.clone());
    assert_eq!(returned_item, Some(shield.clone()));

    // The slot should still be empty
    assert_eq!(manager.get_item(SLOT_WEAPON), None);
}

#[test]
fn test_unequip_item() {
    let mut manager = EquipmentManager::new();

    let weapon = ItemData {
        vnum: 10,
        wearable_flag: WEARABLE_WEAPON,
    };
    manager.equip_item(SLOT_WEAPON, weapon.clone());
    
    // Unequip should return the weapon
    assert_eq!(manager.unequip_item(SLOT_WEAPON), Some(weapon));

    // The slot should now be empty
    assert_eq!(manager.get_item(SLOT_WEAPON), None);

    // Unequipping an empty slot should return None
    assert_eq!(manager.unequip_item(SLOT_WEAPON), None);
}

#[test]
fn test_out_of_bounds_slots() {
    let mut manager = EquipmentManager::new();

    let invalid_item = ItemData {
        vnum: 99999,
        wearable_flag: WEARABLE_WEAPON,
    };

    // Equipping to slot 32 (out of bounds) should reject and return the item
    assert_eq!(manager.equip_item(32, invalid_item.clone()), Some(invalid_item.clone()));

    // Unequipping from slot 32 should safely return None
    assert_eq!(manager.unequip_item(32), None);

    // Getting item from slot 32 should safely return None
    assert_eq!(manager.get_item(32), None);
}

#[test]
fn test_equip_invalid_flag() {
    let mut manager = EquipmentManager::new();
    let item = ItemData {
        vnum: 12345,
        wearable_flag: 0, // No valid flag
    };

    assert_eq!(manager.equip_item(SLOT_ARMOR, item.clone()), Some(item));
}

#[test]
fn test_equip_extended_slots() {
    use equipment_slots::{
        SLOT_ARROW, SLOT_BELT, SLOT_COSTUME_BODY, SLOT_COSTUME_HAIR, SLOT_UNIQUE1,
        WEARABLE_ARROW, WEARABLE_BELT, WEARABLE_COSTUME_BODY, WEARABLE_COSTUME_HAIR, WEARABLE_UNIQUE,
    };

    let mut manager = EquipmentManager::new();

    let arrow = ItemData { vnum: 5000, wearable_flag: WEARABLE_ARROW };
    let belt = ItemData { vnum: 18000, wearable_flag: WEARABLE_BELT };
    let unique = ItemData { vnum: 71001, wearable_flag: WEARABLE_UNIQUE };
    let costume = ItemData { vnum: 41001, wearable_flag: WEARABLE_COSTUME_BODY };
    let hair = ItemData { vnum: 45001, wearable_flag: WEARABLE_COSTUME_HAIR };

    assert_eq!(manager.equip_item(SLOT_ARROW, arrow.clone()), None);
    assert_eq!(manager.equip_item(SLOT_BELT, belt.clone()), None);
    assert_eq!(manager.equip_item(SLOT_UNIQUE1, unique.clone()), None);
    assert_eq!(manager.equip_item(SLOT_COSTUME_BODY, costume.clone()), None);
    assert_eq!(manager.equip_item(SLOT_COSTUME_HAIR, hair.clone()), None);

    assert_eq!(manager.get_item(SLOT_ARROW), Some(&arrow));
    assert_eq!(manager.get_item(SLOT_BELT), Some(&belt));
    assert_eq!(manager.get_item(SLOT_UNIQUE1), Some(&unique));
    assert_eq!(manager.get_item(SLOT_COSTUME_BODY), Some(&costume));
    assert_eq!(manager.get_item(SLOT_COSTUME_HAIR), Some(&hair));
}
