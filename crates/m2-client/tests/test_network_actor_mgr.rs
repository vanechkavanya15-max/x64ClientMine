use m2_client::network_actor_mgr::{
    NetworkActorData, NetworkActorMgr, NetworkMoveActorData, NetworkUpdateActorData,
};

#[test]
fn test_append_and_queue_actor() {
    let mut mgr = NetworkActorMgr::new();
    let mut data1 = NetworkActorData::new();
    data1.vid = 101;
    data1.name = "Actor1".to_string();

    let mut data2 = NetworkActorData::new();
    data2.vid = 102;
    data2.name = "Actor2".to_string();

    mgr.append_actor(data1);
    mgr.append_actor(data2);

    assert!(mgr.get_actor_data(101).is_some());
    assert!(mgr.get_actor_data(102).is_some());

    // Process spawn queue with limit 1
    let ready = mgr.process_spawn_queue(1);
    assert_eq!(ready.len(), 1);
    assert_eq!(ready[0].vid, 101);

    // Process again
    let ready = mgr.process_spawn_queue(1);
    assert_eq!(ready.len(), 1);
    assert_eq!(ready[0].vid, 102);

    // Should be empty now
    let ready = mgr.process_spawn_queue(1);
    assert_eq!(ready.len(), 0);
}

#[test]
fn test_remove_actor() {
    let mut mgr = NetworkActorMgr::new();
    let mut data = NetworkActorData::new();
    data.vid = 999;
    
    mgr.append_actor(data);
    assert!(mgr.get_actor_data(999).is_some());

    // Should remove from dict and queue
    mgr.remove_actor(999);
    assert!(mgr.get_actor_data(999).is_none());

    let ready = mgr.process_spawn_queue(10);
    assert_eq!(ready.len(), 0);
}

#[test]
fn test_update_actor() {
    let mut mgr = NetworkActorMgr::new();
    let mut data = NetworkActorData::new();
    data.vid = 200;
    mgr.append_actor(data);

    let mut update_data = NetworkUpdateActorData::new();
    update_data.vid = 200;
    update_data.mov_spd = 150;
    update_data.atk_spd = 120;
    update_data.armor = 111;

    mgr.update_actor(&update_data);

    let updated_actor = mgr.get_actor_data(200).unwrap();
    assert_eq!(updated_actor.mov_spd, 150);
    assert_eq!(updated_actor.atk_spd, 120);
    assert_eq!(updated_actor.armor, 111);
}

#[test]
fn test_move_actor() {
    let mut mgr = NetworkActorMgr::new();
    let mut data = NetworkActorData::new();
    data.vid = 300;
    data.cur_x = 1000;
    data.cur_y = 2000;
    mgr.append_actor(data);

    let mut move_data = NetworkMoveActorData::new();
    move_data.vid = 300;
    move_data.time = 5000;
    move_data.pos_x = 1500;
    move_data.pos_y = 2500;
    move_data.rot = 45.0;
    move_data.duration = 1000;

    mgr.move_actor(&move_data);

    let actor = mgr.get_actor_data(300).unwrap();
    assert_eq!(actor.src_x, 1000);
    assert_eq!(actor.src_y, 2000);
    assert_eq!(actor.dst_x, 1500);
    assert_eq!(actor.dst_y, 2500);
    assert_eq!(actor.server_src_time, 5000);
    assert_eq!(actor.duration, 1000);
    assert_eq!(actor.rot, 45.0);
}

#[test]
fn test_sync_actor() {
    let mut mgr = NetworkActorMgr::new();
    let mut data = NetworkActorData::new();
    data.vid = 400;
    mgr.append_actor(data);

    mgr.sync_actor(400, 500, 600, 0);

    let actor = mgr.get_actor_data(400).unwrap();
    assert_eq!(actor.cur_x, 500);
    assert_eq!(actor.cur_y, 600);
    assert_eq!(actor.src_x, 500);
    assert_eq!(actor.dst_x, 500);
}

#[test]
fn test_set_actor_owner() {
    let mut mgr = NetworkActorMgr::new();
    let mut data = NetworkActorData::new();
    data.vid = 500;
    mgr.append_actor(data);

    mgr.set_actor_owner(9999, 500);

    let actor = mgr.get_actor_data(500).unwrap();
    assert_eq!(actor.owner_vid, 9999);
}

#[test]
fn test_edge_case_update_missing_actor() {
    let mut mgr = NetworkActorMgr::new();
    let mut update_data = NetworkUpdateActorData::new();
    update_data.vid = 12345;
    update_data.mov_spd = 100;

    // Should not panic or crash
    mgr.update_actor(&update_data);
    assert!(mgr.get_actor_data(12345).is_none());
}
