use m2_client::game_equipment_gem_socket_engine::*;

#[test]
fn test_socket_insertion_and_shard_cleaning() {
    let mut state = EquipmentSocketState::default();

    // 100% success rate -> Success
    let res1 = state.insert_metin(0, 28430, 100, 10);
    assert_eq!(res1, SocketInsertionResult::Success);
    assert_eq!(state.get_socket(0), 28430);

    // 0% success rate -> Broken Shard
    let res2 = state.insert_metin(1, 28431, 0, 10);
    assert_eq!(res2, SocketInsertionResult::BrokenShard);
    assert_eq!(state.get_socket(1), ITEM_BROKEN_METIN_VNUM);

    // Clean shards
    let cleaned = state.clean_shards();
    assert_eq!(cleaned, 1);
    assert_eq!(state.get_socket(1), 0); // Socket 1 is empty again!
    assert_eq!(state.get_socket(0), 28430); // Socket 0 still intact!
}
