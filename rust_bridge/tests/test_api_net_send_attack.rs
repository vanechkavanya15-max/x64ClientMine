use m2_client_bridge::api::net_send_attack::m2_net_send_attack;

#[test]
fn test_m2_net_send_attack() {
    // Sprawdzamy poprawne dzialanie funkcji
    // Uzywamy u32 (DWORD w C++) oraz u8 (BYTE w C++)
    let target_vid: u32 = 12345;
    let attack_type: u8 = 2; // np. BATTLEMODE_ATTACK / HEADER_CG_ATTACK = 2 (ale jako typ ataku)

    let result = m2_net_send_attack(target_vid, attack_type);
    
    // Funkcja w obecnej wersji mock'owej powinna zwracac wartosc true
    assert_eq!(result, true);
}
