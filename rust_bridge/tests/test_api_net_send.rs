use m2_client_bridge::api::net_send::m2_net_send_chat_ex;
use std::ffi::CString;

#[test]
fn test_net_send_chat_ffi() {
    unsafe {
        let msg = CString::new("Wiadomosc testowa").unwrap();
        assert!(m2_net_send_chat_ex(0, msg.as_ptr()));
        assert!(!m2_net_send_chat_ex(0, std::ptr::null()));
    }
}
