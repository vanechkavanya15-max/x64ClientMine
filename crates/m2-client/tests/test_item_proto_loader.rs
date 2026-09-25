use m2_client::item_proto_loader::*;

#[test]
fn test_integration_item_proto_loader() {
    let mut loader = ItemProtoLoader::new();
    assert!(loader.is_ready());

    let mut table = ItemTable {
        dw_vnum: 1234,
        dw_vnum_range: 0,
        sz_name: [0; 25],
        sz_locale_name: [0; 25],
        b_type: ItemType::Weapon as u8,
        b_sub_type: WeaponSubTypes::Sword as u8,
        b_weight: 0,
        b_size: 2,
        dw_anti_flags: 1,
        dw_flags: 2,
        dw_wear_flags: 4,
        dw_immune_flag: 0,
        dw_ibuy_item_price: 100,
        dw_isell_item_price: 50,
        a_limits: [ItemLimit { b_type: 0, l_value: 0 }; ITEM_LIMIT_MAX_NUM],
        a_applies: [ItemApply { b_type: 0, l_value: 0 }; ITEM_APPLY_MAX_NUM],
        al_values: [0; ITEM_VALUES_MAX_NUM],
        al_sockets: [0; ITEM_SOCKET_MAX_NUM],
        dw_refined_vnum: 1235,
        w_refine_set: 0,
        b_alter_to_magic_item_pct: 0,
        b_specular: 50,
        b_gain_socket_pct: 3,
    };
    
    // Ustawienie testowej nazwy
    let name = b"Sword\0";
    for i in 0..name.len() {
        table.sz_locale_name[i] = name[i];
    }

    assert!(loader.load_item(table));

    let item_data = loader.get_item_data(1234).unwrap();

    assert_eq!(item_data.get_index(), 1234);
    assert_eq!(item_data.get_name(), "Sword");
    assert_eq!(item_data.get_type(), ItemType::Weapon as u8);
    assert_eq!(item_data.get_sub_type(), WeaponSubTypes::Sword as u8);
    assert_eq!(item_data.get_use_type_string(), "NOT_USE_TYPE");
    assert_eq!(item_data.get_weapon_type(), WeaponSubTypes::Sword as u32);
    assert_eq!(item_data.get_size(), 2);
    
    assert!(item_data.is_anti_flag(1));
    assert!(!item_data.is_anti_flag(2));
    
    assert!(item_data.is_flag(2));
    assert!(!item_data.is_flag(1));
    
    assert!(item_data.is_wearable_flag(4));
    assert!(!item_data.is_wearable_flag(1));
    
    assert!(item_data.has_next_grade());
    assert_eq!(item_data.get_wear_flags(), 4);
    assert_eq!(item_data.get_ibuy_item_price(), 100);
    assert_eq!(item_data.get_isell_item_price(), 50);

    assert!(item_data.get_limit(0).is_some());
    assert!(item_data.get_limit(2).is_none());

    assert!(item_data.get_apply(0).is_some());
    assert!(item_data.get_apply(3).is_none());

    assert_eq!(item_data.get_value(0), 0);
    assert_eq!(item_data.get_value(6), 0); // Out of bounds value

    assert_eq!(item_data.get_socket(0), 0);
    assert_eq!(item_data.get_socket(3), -1); // Out of bounds socket

    assert_eq!(item_data.get_socket_count(), 3);
    assert_eq!(item_data.get_icon_number(), 1234);
    
    assert_eq!(item_data.get_specular_power_u(), 50);
    assert_eq!(item_data.get_specular_power_f(), 0.5);
    
    assert_eq!(item_data.get_refine(), 4); // 1234 % 10 = 4
    
    assert!(item_data.is_equipment());

    // Test modyfikacji socketu
    let mut_item = loader.get_item_data_mut(1234).unwrap();
    assert_eq!(mut_item.set_socket(0, 100), 100);
    assert_eq!(mut_item.get_socket(0), 100);
    assert_eq!(mut_item.set_socket(3, 100), -1); // Out of bounds

    // Dodatkowy test typu USE
    let table_use = ItemTable {
        dw_vnum: 2000,
        dw_vnum_range: 0,
        sz_name: [0; 25],
        sz_locale_name: [0; 25],
        b_type: ItemType::Use as u8,
        b_sub_type: UseSubTypes::Potion as u8,
        b_weight: 0,
        b_size: 1,
        dw_anti_flags: 0,
        dw_flags: 0,
        dw_wear_flags: 0,
        dw_immune_flag: 0,
        dw_ibuy_item_price: 10,
        dw_isell_item_price: 5,
        a_limits: [ItemLimit { b_type: 0, l_value: 0 }; ITEM_LIMIT_MAX_NUM],
        a_applies: [ItemApply { b_type: 0, l_value: 0 }; ITEM_APPLY_MAX_NUM],
        al_values: [0; ITEM_VALUES_MAX_NUM],
        al_sockets: [0; ITEM_SOCKET_MAX_NUM],
        dw_refined_vnum: 0,
        w_refine_set: 0,
        b_alter_to_magic_item_pct: 0,
        b_specular: 0,
        b_gain_socket_pct: 0,
    };
    loader.load_item(table_use);
    let use_item = loader.get_item_data(2000).unwrap();
    assert_eq!(use_item.get_use_type_string(), "USE_UNKNOWN_TYPE"); // Potion sub_type == 0 is unknown by that function in CPP

    let mut table_use_tuning = table_use.clone();
    table_use_tuning.dw_vnum = 2001;
    table_use_tuning.b_sub_type = UseSubTypes::Tuning as u8;
    loader.load_item(table_use_tuning);
    assert_eq!(loader.get_item_data(2001).unwrap().get_use_type_string(), "USE_TUNING");
    assert!(!loader.get_item_data(2001).unwrap().is_equipment());
}
