use m2_client::race_manager::{get_race_resource_pathes, is_guild_race, is_npc_race, RaceManager};

#[test]
fn test_integration_race_manager() {
    let instance = RaceManager::new();
    assert!(instance.is_ready());
}

#[test]
fn test_is_guild_race() {
    assert!(is_guild_race(14000));
    assert!(is_guild_race(14999));
    assert!(is_guild_race(20043));
    assert!(!is_guild_race(13999));
    assert!(!is_guild_race(15000));
}

#[test]
fn test_is_npc_race() {
    assert!(is_npc_race(9001));
    assert!(!is_npc_race(9000));
}

#[test]
fn test_get_race_resource_pathes() {
    // Guild Race
    let pathes = get_race_resource_pathes(14000);
    assert_eq!(pathes[0], "d:/ymir work/guild/");

    // NPC Race >= 30000
    let pathes = get_race_resource_pathes(30000);
    assert_eq!(pathes[0], "d:/ymir work/npc2/");

    // NPC Race < 30000
    let pathes = get_race_resource_pathes(9001);
    assert_eq!(pathes[0], "d:/ymir work/npc/");

    // Special event monsters
    let pathes = get_race_resource_pathes(8507);
    assert_eq!(pathes[0], "d:/ymir work/monster2/");

    // Race > 8000
    let pathes = get_race_resource_pathes(8001);
    assert_eq!(pathes[0], "d:/ymir work/monster/");

    // Race > 2000
    let pathes = get_race_resource_pathes(2001);
    assert_eq!(pathes[0], "d:/ymir work/monster2/");

    // Race 1400..=1700
    let pathes = get_race_resource_pathes(1500);
    assert_eq!(pathes[0], "d:/ymir work/monster2/");

    // Other races
    let pathes = get_race_resource_pathes(1);
    assert_eq!(pathes[0], "d:/ymir work/monster/");
}

#[test]
fn test_race_manager_methods() {
    let mut manager = RaceManager::new();
    
    manager.set_path_name("base_path/");
    assert_eq!(manager.get_full_path_file_name("file.txt"), "base_path/file.txt");
    assert_eq!(manager.get_full_path_file_name(".hidden.txt"), ".hidden.txt");

    manager.register_race_name(10, "warrior");
    manager.register_race_src_name("warrior", "warrior_src");

    assert_eq!(manager.race_key_race_name_map.get(&10).unwrap(), "warrior");
    assert_eq!(manager.race_name_src_name_map.get("warrior").unwrap(), "warrior_src");

    manager.create_race(10);
    assert!(manager.race_data_map.contains_key(&10));

    manager.select_race(10);
    assert!(manager.get_selected_race_data_pointer().is_some());
    
    // Testing lazy loading (get_race_data_pointer)
    manager.register_race_name(99, "sura");
    let race_data = manager.get_race_data_pointer(99);
    assert!(race_data.is_some());
}
