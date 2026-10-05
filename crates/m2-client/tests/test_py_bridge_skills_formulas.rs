// crates/m2-client/tests/test_py_bridge_skills_formulas.rs
use std::ffi::{CStr, CString};
use m2_client::py_bridge_skills_formulas::*;

#[test]
fn test_skill_require_stat() {
    let mut point = 0u8;
    let mut level = 0u8;

    // Skill 1 (Aura Miecza)
    assert!(m2_skill_get_require_stat_data(1, 0, &mut point, &mut level));
    assert_eq!(point, 1);
    assert_eq!(level, 5);

    // Nieistniejacy stat index zwraca 0, 0
    assert!(m2_skill_get_require_stat_data(1, 99, &mut point, &mut level));
    assert_eq!(point, 0);
    assert_eq!(level, 0);
}

#[test]
fn test_skill_new_affect_formula() {
    let mut type_buf = [0i8; 64];
    let mut min_val = 0.0f32;
    let mut max_val = 0.0f32;

    // Skill 1 (Aura Miecza), level 10: 50 + 2 * 10 = 70.0
    unsafe {
        assert!(m2_skill_get_new_affect_data(
            1,
            0,
            10.0,
            type_buf.as_mut_ptr(),
            type_buf.len(),
            &mut min_val,
            &mut max_val,
        ));
        let type_str = CStr::from_ptr(type_buf.as_ptr()).to_str().unwrap();
        assert_eq!(type_str, "ATT_GRADE_BONUS");
        assert!((min_val - 70.0).abs() < 0.001);
    }
}

#[test]
fn test_quest_last_time_countdown() {
    let quest_id = 7;
    let c_name = CString::new("Misja Polowania").unwrap();

    let now_sec = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i32;

    unsafe {
        assert!(m2_quest_set_clock(quest_id, c_name.as_ptr(), now_sec, 600)); // 10 minut = 600s

        let mut name_buf = [0i8; 64];
        let mut last_time = 0i32;

        assert!(m2_quest_get_last_time(
            quest_id,
            name_buf.as_mut_ptr(),
            name_buf.len(),
            &mut last_time,
        ));

        let retrieved_name = CStr::from_ptr(name_buf.as_ptr()).to_str().unwrap();
        assert_eq!(retrieved_name, "Misja Polowania");
        assert!(last_time >= 598 && last_time <= 600);
    }
}
