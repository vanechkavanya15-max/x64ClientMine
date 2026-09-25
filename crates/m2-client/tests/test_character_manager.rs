use m2_client::character_manager::{CharacterManager, CreateData};

#[test]
fn test_integration_character_manager() {
    let mut manager = CharacterManager::new();
    assert!(manager.is_ready());

    // Test creating instances
    let create_data1 = CreateData {
        vid: 1,
        is_main: false,
        name: "Test1".to_string(),
        race: 0,
    };
    
    let create_data2 = CreateData {
        vid: 2,
        is_main: true,
        name: "Test2".to_string(),
        race: 1,
    };

    assert_eq!(manager.create_instance(create_data1), Some(1));
    assert_eq!(manager.create_instance(create_data2), Some(2));
    
    // Test duplicate creation fails
    let duplicate_data = CreateData {
        vid: 1,
        is_main: false,
        name: "TestDup".to_string(),
        race: 0,
    };
    assert_eq!(manager.create_instance(duplicate_data), None);

    // Test get instance ptr
    assert!(manager.get_instance_ptr(1).is_some());
    assert!(manager.get_instance_ptr(3).is_none());
    
    // Test get main instance
    assert_eq!(manager.main_inst_vid, Some(2));
    assert!(manager.get_main_instance_ptr().is_some());

    // Test get selected instance
    assert_eq!(manager.bind_inst_vid, Some(2));
    assert!(manager.get_selected_instance_ptr().is_some());
    
    // Test delete instance
    manager.delete_instance(1);
    assert!(manager.get_instance_ptr(1).is_none());
    
    // Test clear main instance
    manager.clear_main_instance();
    assert_eq!(manager.main_inst_vid, None);
    assert!(manager.get_main_instance_ptr().is_none());

    // Test set main instance
    manager.set_main_instance(2);
    assert_eq!(manager.main_inst_vid, Some(2));
    
    // Test delete main instance resets references
    manager.delete_instance(2);
    assert_eq!(manager.main_inst_vid, None);
    assert_eq!(manager.bind_inst_vid, None);
    
    // Recreate
    let create_data_main = CreateData {
        vid: 10,
        is_main: true,
        name: "Main".to_string(),
        race: 0,
    };
    manager.create_instance(create_data_main);

    // Test delete instance by fade
    manager.delete_instance_by_fade(10);
    assert!(manager.get_instance_ptr(10).is_none());
    assert_eq!(manager.dead_inst_list.len(), 1);

    manager.update_deleting();
    // In our dummy implementation, update_deleting returns false for instance base, so it should be removed
    assert_eq!(manager.dead_inst_list.len(), 0);
    
    // Test delete all instances
    manager.create_instance(CreateData { vid: 20, is_main: false, name: "A".to_string(), race: 0 });
    manager.create_instance(CreateData { vid: 21, is_main: false, name: "B".to_string(), race: 0 });
    manager.delete_instance_by_fade(20);
    manager.delete_all_instances();
    assert_eq!(manager.alive_inst_map.len(), 0);
    assert_eq!(manager.dead_inst_list.len(), 0);
    assert_eq!(manager.main_inst_vid, None);
}

#[test]
fn test_pick_logic() {
    let mut manager = CharacterManager::new();
    
    manager.create_instance(CreateData {
        vid: 100,
        is_main: true,
        name: "Main".to_string(),
        race: 0,
    });
    
    manager.create_instance(CreateData {
        vid: 101,
        is_main: false,
        name: "NPC1".to_string(),
        race: 0,
    });
    
    manager.create_instance(CreateData {
        vid: 102,
        is_main: false,
        name: "NPC2".to_string(),
        race: 0,
    });
    
    // old_pick will not pick anything since our dummy `intersect_defending_sphere` returns false
    manager.old_pick();
    assert_eq!(manager.pick_inst_vid, None);

    manager.new_pick();
    // Since dummy `can_pick_instance` is true, but `intersect_bounding_box` is false, and picking list includes main when exact picking fails.
    // However, our dummy `intersect_defending_sphere` returns false, so picked_inst_vids will be empty.
    assert_eq!(manager.pick_inst_vid, None);
}

#[test]
fn test_helper_methods() {
    let mut manager = CharacterManager::new();
    manager.create_instance(CreateData {
        vid: 1,
        is_main: false,
        name: "Player1".to_string(),
        race: 0,
    });

    assert_eq!(manager.pick_all(), -1);
    
    manager.refresh_all_pc_text_tail();
    manager.refresh_all_guild_mark();
    
    let close = manager.get_close_instance(1);
    assert_eq!(close, None);
    
    manager.create_instance(CreateData {
        vid: 2,
        is_main: false,
        name: "Player2".to_string(),
        race: 0,
    });
    
    let close = manager.get_close_instance(1);
    assert_eq!(close, Some(2));
}
