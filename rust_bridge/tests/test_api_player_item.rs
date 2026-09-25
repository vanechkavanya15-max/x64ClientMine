use m2_client_bridge::api::player_item::{m2_player_get_item_vnum, m2_player_get_item_count};

#[test]
fn test_get_item_vnum_safe_call() {
    // Sprawdza czy m2_player_get_item_vnum poprawnie zwraca domyslna wartosc
    let vnum = m2_player_get_item_vnum(1, 1);
    assert_eq!(vnum, 0);
}

#[test]
fn test_get_item_count_safe_call() {
    // Sprawdza czy m2_player_get_item_count poprawnie zwraca domyslna wartosc
    let count = m2_player_get_item_count(1, 1);
    assert_eq!(count, 0);
}
