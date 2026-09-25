use m2_client::ui_login_screen::{UiLoginScreen, LoginState};

#[test]
fn test_ui_login_screen_initialization() {
    let instance = UiLoginScreen::new();
    assert!(instance.is_ready());
    assert_eq!(instance.login_input, "");
    assert_eq!(instance.password_input, "");
    assert_eq!(instance.selected_server_id, None);
    assert_eq!(instance.selected_channel_id, None);
    assert_eq!(instance.login_state, LoginState::Disconnected);
    assert_eq!(instance.empire_id, 0);
    assert_eq!(instance.login_key, 0);
    assert_eq!(instance.is_wait_login_key, false);
    assert_eq!(instance.is_direct_enter_mode, false);
    assert_eq!(instance.chr_slot, 0);
}

#[test]
fn test_server_channel_selection() {
    let mut instance = UiLoginScreen::new();
    
    instance.select_server(1);
    assert_eq!(instance.selected_server_id, Some(1));
    
    instance.select_channel(2);
    assert_eq!(instance.selected_channel_id, Some(2));
}

#[test]
fn test_set_login_phase_clears_password() {
    let mut instance = UiLoginScreen::new();
    instance.set_password("secret_password");
    assert_eq!(instance.password_input, "secret_password");
    
    instance.set_login_phase();
    
    assert_eq!(instance.login_state, LoginState::Authenticating);
    assert_eq!(instance.password_input, ""); // Haslo powinno byc wyczyszczone
}

#[test]
fn test_network_packet_handlers() {
    let mut instance = UiLoginScreen::new();
    
    // Ustawmy login phase recznie na autoryzacje zeby miec inny stan
    instance.login_state = LoginState::Authenticating;
    
    // Testowanie odbierania empire
    assert!(instance.recv_empire_packet(2));
    assert_eq!(instance.empire_id, 2);
    
    // Testowanie login success (3 i 4)
    assert!(instance.recv_login_success_packet3());
    assert_eq!(instance.login_state, LoginState::Disconnected);
    
    instance.login_state = LoginState::Authenticating;
    assert!(instance.recv_login_success_packet4());
    assert_eq!(instance.login_state, LoginState::Disconnected);
    
    // Testowanie on_connect_failure
    instance.login_state = LoginState::Authenticating;
    instance.is_direct_enter_mode = false;
    instance.on_connect_failure();
    assert_eq!(instance.login_state, LoginState::Disconnected);
    
    instance.login_state = LoginState::Authenticating;
    instance.is_direct_enter_mode = true;
    instance.on_connect_failure();
    assert_eq!(instance.login_state, LoginState::Disconnected);
    
    // Testowanie login failure
    instance.login_state = LoginState::Authenticating;
    assert!(instance.recv_login_failure_packet());
    assert_eq!(instance.login_state, LoginState::Disconnected);
    
    // Testowanie login key
    instance.is_wait_login_key = true;
    assert!(instance.recv_login_key_packet(12345));
    assert_eq!(instance.login_key, 12345);
    assert_eq!(instance.is_wait_login_key, false);
}

#[test]
fn test_direct_enter_packet_sends() {
    let mut instance = UiLoginScreen::new();
    instance.set_password("direct_pass");
    
    assert!(instance.send_direct_enter_packet());
    
    // Po poprawnym wyslaniu pakietu (u nas symulacja zawsze true) powinno wyczyscic haslo
    assert_eq!(instance.password_input, "");
}
