use m2_client::item_anti_flags::{
    can_equip_by_player, Item, PlayerClass, PlayerSex, ITEM_ANTIFLAG_ASSASSIN,
    ITEM_ANTIFLAG_FEMALE, ITEM_ANTIFLAG_MALE, ITEM_ANTIFLAG_SHAMAN, ITEM_ANTIFLAG_SURA,
    ITEM_ANTIFLAG_WARRIOR,
};

#[test]
fn test_warrior_armor_cannot_be_equipped_by_ninja() {
    let item = Item {
        anti_flags: ITEM_ANTIFLAG_ASSASSIN | ITEM_ANTIFLAG_SURA | ITEM_ANTIFLAG_SHAMAN,
        limit_level: 1,
    };
    
    // Warrior CAN equip
    assert!(can_equip_by_player(&item, PlayerClass::Warrior, PlayerSex::Male, 10));
    
    // Assassin CANNOT equip
    assert!(!can_equip_by_player(&item, PlayerClass::Assassin, PlayerSex::Male, 10));
}

#[test]
fn test_item_with_higher_level_requirement_blocked() {
    let item = Item {
        anti_flags: 0,
        limit_level: 50,
    };
    
    // Level 49 cannot equip
    assert!(!can_equip_by_player(&item, PlayerClass::Warrior, PlayerSex::Male, 49));
    
    // Level 50 can equip
    assert!(can_equip_by_player(&item, PlayerClass::Warrior, PlayerSex::Male, 50));
}

#[test]
fn test_item_can_be_equipped_when_all_conditions_met() {
    let item = Item {
        anti_flags: 0,
        limit_level: 1,
    };
    
    assert!(can_equip_by_player(&item, PlayerClass::Shaman, PlayerSex::Female, 1));
}

#[test]
fn test_sex_anti_flags() {
    let male_only_item = Item {
        anti_flags: ITEM_ANTIFLAG_FEMALE,
        limit_level: 1,
    };
    
    assert!(can_equip_by_player(&male_only_item, PlayerClass::Warrior, PlayerSex::Male, 10));
    assert!(!can_equip_by_player(&male_only_item, PlayerClass::Warrior, PlayerSex::Female, 10));

    let female_only_item = Item {
        anti_flags: ITEM_ANTIFLAG_MALE,
        limit_level: 1,
    };

    assert!(!can_equip_by_player(&female_only_item, PlayerClass::Warrior, PlayerSex::Male, 10));
    assert!(can_equip_by_player(&female_only_item, PlayerClass::Warrior, PlayerSex::Female, 10));
}

#[test]
fn test_all_class_flags() {
    let item = Item {
        anti_flags: ITEM_ANTIFLAG_WARRIOR | ITEM_ANTIFLAG_ASSASSIN | ITEM_ANTIFLAG_SURA | ITEM_ANTIFLAG_SHAMAN,
        limit_level: 1,
    };

    assert!(!can_equip_by_player(&item, PlayerClass::Warrior, PlayerSex::Male, 10));
    assert!(!can_equip_by_player(&item, PlayerClass::Assassin, PlayerSex::Male, 10));
    assert!(!can_equip_by_player(&item, PlayerClass::Sura, PlayerSex::Male, 10));
    assert!(!can_equip_by_player(&item, PlayerClass::Shaman, PlayerSex::Male, 10));
}

#[test]
fn test_level_zero_edge_cases() {
    let item = Item {
        anti_flags: 0,
        limit_level: 0,
    };

    assert!(can_equip_by_player(&item, PlayerClass::Warrior, PlayerSex::Male, 0));
    assert!(can_equip_by_player(&item, PlayerClass::Warrior, PlayerSex::Male, 1));
}

#[test]
fn test_level_requirement() {
    use m2_client::item_anti_flags::can_equip;
    assert!(!can_equip(0, 0, 0, 49, 50));
    assert!(can_equip(0, 0, 0, 50, 50));
    assert!(can_equip(0, 0, 0, 51, 50));
}
