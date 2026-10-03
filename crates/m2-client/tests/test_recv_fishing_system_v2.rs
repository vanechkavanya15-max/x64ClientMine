use m2_client::recv_fishing_system_v2::*;
use std::sync::atomic::Ordering;
use std::sync::Mutex;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

fn reset_hit_window() {
    HIT_WINDOW_MS.store(0, Ordering::Relaxed);
}

#[test]
fn test_fishing_get_hit_window() {
    let _lock = TEST_MUTEX.lock().unwrap();
    reset_hit_window();
    let packet = TPacketGCFishing {
        sub_header: FISHING_SUB_HEADER_HIT,
        vid: 123,
        hit_window_ms: 1000,
    };
    handle_fishing_packet(&packet);
    assert_eq!(get_hit_window(), 1000);
}

#[test]
fn test_fishing_ignore_other_sub_headers() {
    let _lock = TEST_MUTEX.lock().unwrap();
    reset_hit_window();
    let packet = TPacketGCFishing {
        sub_header: FISHING_SUB_HEADER_START,
        vid: 123,
        hit_window_ms: 2000,
    };
    handle_fishing_packet(&packet);
    assert_eq!(get_hit_window(), 0); 
}

#[test]
fn test_fishing_success() {
    let _lock = TEST_MUTEX.lock().unwrap();
    reset_hit_window();
    let packet = TPacketGCFishing {
        sub_header: FISHING_SUB_HEADER_SUCCESS,
        vid: 123,
        hit_window_ms: 3000,
    };
    handle_fishing_packet(&packet);
    assert_eq!(get_hit_window(), 0); 
}

#[test]
fn test_fishing_fail() {
    let _lock = TEST_MUTEX.lock().unwrap();
    reset_hit_window();
    let packet = TPacketGCFishing {
        sub_header: FISHING_SUB_HEADER_FAIL,
        vid: 123,
        hit_window_ms: 4000,
    };
    handle_fishing_packet(&packet);
    assert_eq!(get_hit_window(), 0); 
}

#[test]
fn test_fishing_hit_updates_window() {
    let _lock = TEST_MUTEX.lock().unwrap();
    reset_hit_window();
    let packet = TPacketGCFishing {
        sub_header: FISHING_SUB_HEADER_HIT,
        vid: 123,
        hit_window_ms: 5000,
    };
    handle_fishing_packet(&packet);
    assert_eq!(get_hit_window(), 5000);
}
