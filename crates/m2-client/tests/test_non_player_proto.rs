use m2_client::non_player_proto::{NonPlayerProto, MobTable, ClickEvent};

#[test]
fn test_integration_non_player_proto() {
    let instance = NonPlayerProto::new();
    assert!(instance.is_ready());
}

#[test]
fn test_non_player_proto_getters() {
    let mut instance = NonPlayerProto::new();
    
    let mut dummy_mob = MobTable::default();
    dummy_mob.vnum = 101;
    let name_str = b"Wild Dog\0";
    dummy_mob.locale_name[..name_str.len()].copy_from_slice(name_str);
    dummy_mob.mob_type = 1; // Monster
    dummy_mob.on_click_type = ClickEvent::Battle as u8;
    dummy_mob.monster_color = 12345;
    
    instance.mob_tables.insert(dummy_mob.vnum, dummy_mob);

    assert_eq!(instance.get_name(101).unwrap(), "Wild Dog");
    assert_eq!(instance.get_monster_name(101), "Wild Dog");
    assert_eq!(instance.get_instance_type(101).unwrap(), 1);
    assert_eq!(instance.get_event_type(101), ClickEvent::Battle as u8);
    assert_eq!(instance.get_monster_color(101), 12345);
    
    assert!(instance.get_name(999).is_none());
    assert_eq!(instance.get_monster_name(999), "");
    assert!(instance.get_instance_type(999).is_none());
    assert_eq!(instance.get_event_type(999), ClickEvent::None as u8);
    assert_eq!(instance.get_monster_color(999), 0);
    
    instance.clear();
    assert!(instance.get_table(101).is_none());
}

#[test]
fn test_non_player_proto_load_from_buffer() {
    let mut instance = NonPlayerProto::new();
    
    let mut dummy_mob = MobTable::default();
    dummy_mob.vnum = 102;
    let name_str = b"Wolf\0";
    dummy_mob.locale_name[..name_str.len()].copy_from_slice(name_str);
    dummy_mob.mob_type = 2; // NPC
    
    let size = std::mem::size_of::<MobTable>();
    let mut buffer = vec![0u8; size];
    unsafe {
        std::ptr::copy_nonoverlapping(
            &dummy_mob as *const MobTable as *const u8,
            buffer.as_mut_ptr(),
            size,
        );
    }
    
    assert!(instance.load_from_buffer(&buffer));
    
    assert_eq!(instance.get_name(102).unwrap(), "Wolf");
    assert_eq!(instance.get_instance_type(102).unwrap(), 2);
}

#[test]
fn test_non_player_proto_load_from_buffer_invalid() {
    let mut instance = NonPlayerProto::new();
    let invalid_buffer = vec![0u8; 15]; // Invalid size for MobTable
    
    assert!(!instance.load_from_buffer(&invalid_buffer));
}
