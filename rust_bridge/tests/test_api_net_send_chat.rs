use m2_client_bridge::api::net_send_chat::m2_net_send_chat;
use std::ptr;
use std::ffi::CString;

#[test]
fn test_m2_net_send_chat_null_ptr() {
    let result = m2_net_send_chat(1, ptr::null());
    assert_eq!(result, false, "Oczekiwano false dla null pointera");
}

#[test]
fn test_m2_net_send_chat_valid_ptr() {
    let msg = CString::new("Testowa wiadomosc").unwrap();
    let result = m2_net_send_chat(1, msg.as_ptr());
    assert_eq!(result, true, "Oczekiwano true dla poprawnego wskaznika na C-string");
}
