use m2_client_bridge::state::chat_filter::*;
use std::ffi::CString;

#[test]
fn test_chat_filter_basics() {
    let mut filter = ChatFilter::new();
    
    assert!(!filter.is_blocked("Player1"));
    
    filter.block_user("Player1");
    assert!(filter.is_blocked("Player1"));
    assert!(filter.is_blocked("player1"));
    assert!(filter.is_blocked("PLAYER1"));
    
    filter.unblock_user("pLaYeR1");
    assert!(!filter.is_blocked("Player1"));
}

#[test]
fn test_chat_filter_ffi() {
    // Czyszczenie na wypadek uruchamiania testow wielowatkowo (choc w Cargo.toml jest catch_unwind).
    unsafe {
        m2_chat_filter_clear();
    }

    let p1 = CString::new("Spammer").unwrap();
    let p2 = CString::new("Friend").unwrap();

    unsafe {
        assert!(!m2_chat_is_user_blocked(p1.as_ptr()));
        
        m2_chat_block_user(p1.as_ptr());
        
        assert!(m2_chat_is_user_blocked(p1.as_ptr()));
        let p1_lower = CString::new("spammer").unwrap();
        assert!(m2_chat_is_user_blocked(p1_lower.as_ptr()));
        assert!(!m2_chat_is_user_blocked(p2.as_ptr()));
        
        m2_chat_unblock_user(p1.as_ptr());
        
        assert!(!m2_chat_is_user_blocked(p1.as_ptr()));
    }
}
