use m2_client::race_data::{RaceData, make_combo_key, combo_key_get_motion_mode, combo_key_get_combo_type, EParts};

#[test]
fn test_integration_race_data() {
    let mut instance = RaceData::new();
    assert!(instance.is_ready());

    instance.set_race(1);
    assert_eq!(instance.race_index, 1);

    instance.register_attaching_bone_name(EParts::Weapon as u32, "Bip01 R Hand");
    assert_eq!(instance.get_attaching_bone_name(1).unwrap(), "Bip01 R Hand");

    instance.register_motion_mode(1);
    assert!(instance.register_motion_data(1, 10, "motion1.gr2", 100));
    assert!(instance.set_motion_random_weight(1, 10, 0, 50));
    
    let vector = instance.get_motion_vector_pointer(1, 10).unwrap();
    assert_eq!(vector.len(), 1);
    assert_eq!(vector[0].percentage, 50);
    assert_eq!(vector[0].motion_file_name, "motion1.gr2");

    instance.register_normal_attack(1, 20);
    assert_eq!(instance.get_normal_attack_index(1), Some(20));
    assert_eq!(instance.get_normal_attack_index(99), None);

    instance.reserve_combo_attack(1, 2, 3);
    instance.register_combo_attack(1, 2, 0, 10);
    instance.register_combo_attack(1, 2, 1, 11);
    instance.register_combo_attack(1, 2, 2, 12);

    let combo = instance.get_combo_data_pointer(1, 2).unwrap();
    assert_eq!(combo.combo_index_vector.len(), 3);
    assert_eq!(combo.combo_index_vector[0], 10);
    assert_eq!(combo.combo_index_vector[1], 11);
    assert_eq!(combo.combo_index_vector[2], 12);

    instance.set_shape_model(1, "shape1.gr2");
    instance.append_shape_skin(1, 0, "src.dds", "dst.dds");
    let shape = instance.find_shape(1).unwrap();
    assert_eq!(shape.model_file_name, "shape1.gr2");
    assert_eq!(shape.skins.len(), 1);
    assert_eq!(shape.skins[0].src_file_name, "src.dds");

    instance.set_hair_skin(1, EParts::Hair as u32, "hair1.gr2", "h_src.dds", "h_dst.dds");
    let hair = instance.find_hair(1).unwrap();
    assert_eq!(hair.model_file_name, "hair1.gr2");
    assert_eq!(hair.skins.len(), 1);
    assert_eq!(hair.skins[0].dst_file_name, "h_dst.dds");

    instance.destroy();
    assert_eq!(instance.race_index, 0);
    assert!(instance.find_hair(1).is_none());
    assert!(instance.find_shape(1).is_none());
    assert!(instance.get_combo_data_pointer(1, 2).is_none());
}

#[test]
fn test_combo_key_bits() {
    let key = make_combo_key(0xABCD, 0x1234);
    assert_eq!(combo_key_get_motion_mode(key), 0xABCD);
    assert_eq!(combo_key_get_combo_type(key), 0x1234);
    
    let key2 = make_combo_key(0xFFFF, 0xFFFF);
    assert_eq!(combo_key_get_motion_mode(key2), 0xFFFF);
    assert_eq!(combo_key_get_combo_type(key2), 0xFFFF);
}

#[test]
fn test_invalid_fetches() {
    let mut instance = RaceData::new();
    assert!(instance.get_attaching_bone_name(99).is_none());
    assert!(instance.get_motion_vector_pointer(99, 99).is_none());
    
    assert_eq!(instance.register_motion_data(99, 99, "none", 100), false);
    assert_eq!(instance.set_motion_random_weight(99, 99, 0, 100), false);
    
    instance.register_motion_mode(1);
    // motion_index 10 does not exist
    assert_eq!(instance.set_motion_random_weight(1, 10, 0, 100), false);
}
