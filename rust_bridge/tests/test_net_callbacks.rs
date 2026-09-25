use std::sync::atomic::{AtomicU32, Ordering};
use m2_client_bridge::net::callbacks::{M2NetCallbacks, m2_net_set_callbacks, dispatch_char_spawn};

static CALLED_VID: AtomicU32 = AtomicU32::new(0);

extern "C" fn mock_on_char_spawn(vid: u32, _x: i32, _y: i32, _race: u16) {
    CALLED_VID.store(vid, Ordering::SeqCst);
}

#[test]
fn test_register_and_dispatch() {
    let mut cbs = M2NetCallbacks::default();
    cbs.on_char_spawn = Some(mock_on_char_spawn);
    
    unsafe {
        assert!(m2_net_set_callbacks(&cbs));
    }
    
    dispatch_char_spawn(42, 100, 200, 1);
    assert_eq!(CALLED_VID.load(Ordering::SeqCst), 42);
}

#[test]
fn test_null_pointer_protection() {
    unsafe {
        assert_eq!(m2_net_set_callbacks(std::ptr::null()), false);
    }
}
