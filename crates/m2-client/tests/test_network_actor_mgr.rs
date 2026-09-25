use m2_client::network_actor_mgr::*;

#[test]
fn test_integration_network_actor_mgr() {
    let instance = NetworkActorMgr::new();
    assert!(instance.is_ready());
}

#[test]
fn test_affect_flag_container() {
    let mut flags = AffectFlagContainer::new();
    assert!(!flags.is_set(0));
    assert!(!flags.is_set(AFFECT_SHOW_ALWAYS));

    flags.set(AFFECT_SHOW_ALWAYS, true);
    assert!(flags.is_set(AFFECT_SHOW_ALWAYS));

    flags.set(AFFECT_SHOW_ALWAYS, false);
    assert!(!flags.is_set(AFFECT_SHOW_ALWAYS));

    flags.set(63, true);
    assert!(flags.is_set(63));
    
    // Out of bounds check
    flags.set(64, true);
    assert!(!flags.is_set(64));
}

#[test]
fn test_main_actor_vid() {
    let mut mgr = NetworkActorMgr::new();
    mgr.set_main_actor_vid(100);
    assert!(mgr.is_main_actor_vid(100));
    assert!(!mgr.is_main_actor_vid(200));

    mgr.destroy();
    assert!(!mgr.is_main_actor_vid(100));
}

#[test]
fn test_visibility_pos() {
    let mut mgr = NetworkActorMgr::new();
    mgr.set_main_actor_vid(1);
    mgr.main_pos_x = 0;
    mgr.main_pos_y = 0;

    // Inside bounds
    assert!(mgr.is_visible_pos(1000, 1000));
    
    // Outside bounds
    assert!(!mgr.is_visible_pos(CHAR_STAGE_VIEW_BOUND + 1, 0));
}

#[test]
fn test_visibility_actor() {
    let mut mgr = NetworkActorMgr::new();
    mgr.set_main_actor_vid(10);
    mgr.main_pos_x = 0;
    mgr.main_pos_y = 0;

    // 1. Main actor
    let mut main_actor = NetworkActorData::new();
    main_actor.vid = 10;
    main_actor.set_position(30000, 30000); // outside
    assert!(mgr.is_visible_actor(&main_actor));

    // 2. Far actor, no show_always
    let mut far_actor = NetworkActorData::new();
    far_actor.vid = 20;
    far_actor.set_position(30000, 30000); // outside
    assert!(!mgr.is_visible_actor(&far_actor));

    // 3. Far actor, show_always
    far_actor.affect_flags.set(AFFECT_SHOW_ALWAYS, true);
    assert!(mgr.is_visible_actor(&far_actor));

    // 4. Close actor
    let mut close_actor = NetworkActorData::new();
    close_actor.vid = 30;
    close_actor.set_position(1000, 1000); // inside
    assert!(mgr.is_visible_actor(&close_actor));
}

#[test]
fn test_lifecycle_actor() {
    let mut mgr = NetworkActorMgr::new();
    mgr.set_main_actor_vid(100);

    let mut data = NetworkActorData::new();
    data.vid = 200;
    data.set_position(0, 0); // Visible pos

    // Append actor
    mgr.append_actor(data);
    assert!(mgr.net_actor_dict.contains_key(&200));

    // Update actor
    let mut update_data = NetworkUpdateActorData::new();
    update_data.vid = 200;
    update_data.guild_id = 99;
    update_data.armor = 1234;
    mgr.update_actor(&update_data);
    
    let updated_actor = mgr.net_actor_dict.get(&200).unwrap();
    assert_eq!(updated_actor.guild_id, 99);
    assert_eq!(updated_actor.armor, 1234);

    // Remove actor
    mgr.remove_actor(200);
    assert!(!mgr.net_actor_dict.contains_key(&200));
}

#[test]
fn test_sync_and_move_actor() {
    let mut mgr = NetworkActorMgr::new();
    mgr.set_main_actor_vid(100);

    let mut data = NetworkActorData::new();
    data.vid = 200;
    data.set_position(0, 0);
    mgr.append_actor(data);

    // Sync actor
    mgr.sync_actor(200, 1000, 2000);
    let actor = mgr.net_actor_dict.get(&200).unwrap();
    assert_eq!(actor.cur_x, 1000);
    assert_eq!(actor.cur_y, 2000);

    // Move actor
    let mut move_data = NetworkMoveActorData::new();
    move_data.vid = 200;
    move_data.pos_x = 3000;
    move_data.pos_y = 4000;
    move_data.duration = 1000;
    move_data.time = 5000; // Server time
    
    let current_time = 10000;
    mgr.move_actor(&move_data, current_time);

    let actor = mgr.net_actor_dict.get(&200).unwrap();
    assert_eq!(actor.dst_x, 3000);
    assert_eq!(actor.dst_y, 4000);
    assert_eq!(actor.client_src_time, 10000);

    // Interpolation (update) at half duration
    mgr.update(10500); // 500ms elapsed -> 50%
    let actor = mgr.net_actor_dict.get(&200).unwrap();
    assert_eq!(actor.cur_x, 2000); // 1000 + 50% * 2000
    assert_eq!(actor.cur_y, 3000); // 2000 + 50% * 2000

    // Interpolation (update) at full duration
    mgr.update(11000);
    let actor = mgr.net_actor_dict.get(&200).unwrap();
    assert_eq!(actor.cur_x, 3000);
    assert_eq!(actor.cur_y, 4000);
}
