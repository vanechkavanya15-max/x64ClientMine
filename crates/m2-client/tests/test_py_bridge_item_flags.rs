use m2_client::py_bridge_item_flags_complete::{
    get_affect, get_limit, is_anti_flag, is_wear_flag, select_item_table, ItemApply, ItemLimit,
    ItemTableData, ITEM_ANTIFLAG_DROP, ITEM_ANTIFLAG_FEMALE, ITEM_ANTIFLAG_SELL,
    ITEM_ANTIFLAG_WARRIOR, WEARFLAG_BODY, WEARFLAG_WEAPON,
};

#[test]
fn test_item_flags_bitmask_operations() {
    let mut data = ItemTableData::default();
    data.vnum = 11299; // Zbroja z czarnej stali
    data.anti_flags = ITEM_ANTIFLAG_FEMALE | ITEM_ANTIFLAG_SELL;
    data.wear_flags = WEARFLAG_BODY;
    data.limits[0] = ItemLimit { limit_type: 1, limit_value: 66 }; // LEVEL 66
    data.applies[0] = ItemApply { apply_type: 54, apply_value: 20 }; // Max HP

    select_item_table(data);

    // Sprawdz anti-flags
    assert!(is_anti_flag(ITEM_ANTIFLAG_FEMALE));
    assert!(is_anti_flag(ITEM_ANTIFLAG_SELL));
    assert!(!is_anti_flag(ITEM_ANTIFLAG_DROP));
    assert!(!is_anti_flag(ITEM_ANTIFLAG_WARRIOR));

    // Sprawdz wear-flags
    assert!(is_wear_flag(WEARFLAG_BODY));
    assert!(!is_wear_flag(WEARFLAG_WEAPON));

    // Sprawdz limit i affect
    assert_eq!(get_limit(0), (1, 66));
    assert_eq!(get_affect(0), (54, 20));
}
