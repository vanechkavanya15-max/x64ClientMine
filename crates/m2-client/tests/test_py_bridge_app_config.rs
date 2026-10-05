// crates/m2-client/tests/test_py_bridge_app_config.rs
use m2_client::py_bridge_app_config::*;

#[test]
fn test_frame_skip_toggle() {
    m2_app_set_frame_skip(1);
    assert!(m2_app_get_frame_skip());

    m2_app_set_frame_skip(0);
    assert!(!m2_app_get_frame_skip());
}

#[test]
fn test_hair_color_toggle() {
    m2_app_set_hair_color_enable(1);
    assert!(m2_app_get_hair_color_enable());

    m2_app_set_hair_color_enable(0);
    assert!(!m2_app_get_hair_color_enable());
}

#[test]
fn test_global_timestamp() {
    let ts = m2_app_get_global_time_stamp();
    assert!(ts > 1_000_000_000); // Sensowny unix timestamp
}

#[test]
fn test_get_random_range() {
    // Standardowy zakres
    for _ in 0..50 {
        let val = m2_app_get_random(10, 20);
        assert!((10..=20).contains(&val));
    }

    // Odwrotny zakres (from > to)
    for _ in 0..50 {
        let val = m2_app_get_random(100, 50);
        assert!((50..=100).contains(&val));
    }

    // Identyczny zakres (from == to)
    assert_eq!(m2_app_get_random(42, 42), 42);
}
