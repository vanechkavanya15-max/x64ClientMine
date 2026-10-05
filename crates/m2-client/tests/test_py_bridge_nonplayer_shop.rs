// crates/m2-client/tests/test_py_bridge_nonplayer_shop.rs
use m2_client::py_bridge_nonplayer_shop::*;

#[test]
fn test_monster_grade_classification() {
    assert_eq!(m2_nonplayer_get_monster_grade(101), MONSTER_GRADE_PAWN);
    assert_eq!(m2_nonplayer_get_monster_grade(591), MONSTER_GRADE_BOSS);
    assert_eq!(m2_nonplayer_get_monster_grade(1093), MONSTER_GRADE_KING);

    assert!(m2_nonplayer_is_monster(101));
    assert!(!m2_nonplayer_is_npc(101));

    assert!(m2_nonplayer_is_npc(20016));
    assert!(!m2_nonplayer_is_monster(20016));
}

#[test]
fn test_shop_sockets_and_attributes() {
    let slot = 5;
    let socket_idx = 1;
    let socket_val = 28430; // Kamien Duszy Smierci +4

    assert!(m2_shop_set_item_metin_socket(slot, socket_idx, socket_val));
    assert_eq!(m2_shop_get_item_metin_socket(slot, socket_idx), socket_val);

    let attr_idx = 0;
    let attr_type = 71; // Srednie Obrazenia
    let attr_val = 45;  // 45% sr

    assert!(m2_shop_set_item_attribute(slot, attr_idx, attr_type, attr_val));

    let mut out_type = 0u8;
    let mut out_val = 0i16;

    assert!(m2_shop_get_item_attribute(slot, attr_idx, &mut out_type, &mut out_val));
    assert_eq!(out_type, attr_type);
    assert_eq!(out_val, attr_val);
}
