use m2_client_bridge::api::player_status::m2_player_get_status;

#[test]
fn test_m2_player_get_status_normal_call() {
    // Normalne wywolanie nie powoduje paniki
    let result = m2_player_get_status(1);
    assert_eq!(result, 0);
}

#[test]
fn test_m2_player_get_status_catches_panic() {
    // Specjalna wartosc status_type = 999 symuluje panike i sprawdza, czy safe_ffi_call wylapie ja i zwroci domyslne 0
    let result = m2_player_get_status(999);
    assert_eq!(result, 0);
}
