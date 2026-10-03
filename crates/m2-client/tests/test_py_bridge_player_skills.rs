use m2_client::py_bridge_player_skills::{
    m2_py_player_get_skill_current_efficient_percentage, m2_py_player_get_skill_grade,
    m2_py_player_get_skill_index, m2_py_player_get_skill_next_efficient_percentage,
    m2_py_player_get_skill_slot_index, m2_py_player_is_skill_active,
    m2_py_player_is_skill_cool_time, PLAYER_SKILLS,
};

#[test]
fn test_get_skill_grade() {
    {
        let mut skills = PLAYER_SKILLS.write().unwrap();
        skills[1].iGrade = 3; // Perfect Master
    }
    
    assert_eq!(m2_py_player_get_skill_grade(1), 3);
    assert_eq!(m2_py_player_get_skill_grade(-1), 0); // Out of bounds
    assert_eq!(m2_py_player_get_skill_grade(255), 0); // Out of bounds
}

#[test]
fn test_get_skill_index() {
    {
        let mut skills = PLAYER_SKILLS.write().unwrap();
        skills[5].dwIndex = 100;
    }
    
    assert_eq!(m2_py_player_get_skill_index(5), 100);
    assert_eq!(m2_py_player_get_skill_index(-1), 0);
    assert_eq!(m2_py_player_get_skill_index(300), 0);
}

#[test]
fn test_get_skill_slot_index() {
    {
        let mut skills = PLAYER_SKILLS.write().unwrap();
        skills[10].dwIndex = 1234;
    }
    
    assert_eq!(m2_py_player_get_skill_slot_index(1234), 10);
    assert_eq!(m2_py_player_get_skill_slot_index(9999), 0); // Not found
}

#[test]
fn test_get_skill_current_efficient_percentage() {
    {
        let mut skills = PLAYER_SKILLS.write().unwrap();
        skills[2].fcurEfficientPercentage = 0.75;
    }
    
    assert_eq!(m2_py_player_get_skill_current_efficient_percentage(2), 0.75);
    assert_eq!(m2_py_player_get_skill_current_efficient_percentage(300), 0.0);
}

#[test]
fn test_get_skill_next_efficient_percentage() {
    {
        let mut skills = PLAYER_SKILLS.write().unwrap();
        skills[3].fnextEfficientPercentage = 1.0;
    }
    
    assert_eq!(m2_py_player_get_skill_next_efficient_percentage(3), 1.0);
    assert_eq!(m2_py_player_get_skill_next_efficient_percentage(-5), 0.0);
}

#[test]
fn test_is_skill_active() {
    {
        let mut skills = PLAYER_SKILLS.write().unwrap();
        skills[4].bActive = true;
        skills[5].bActive = false;
    }
    
    assert_eq!(m2_py_player_is_skill_active(4), 1);
    assert_eq!(m2_py_player_is_skill_active(5), 0);
    assert_eq!(m2_py_player_is_skill_active(300), 0);
}

#[test]
fn test_is_skill_cool_time() {
    {
        let mut skills = PLAYER_SKILLS.write().unwrap();
        // Since we can't mock m2_get_time() easily, we set a huge cool time to ensure it is in cool time.
        // Wait, m2_get_time returns an increasing value, so if fLastUsedTime is very high, elapsed is negative
        // But elapsed = current - last_used
        // To be in cool time, elapsed < fCoolTime
        skills[6].fLastUsedTime = 9999999999.0;
        skills[6].fCoolTime = 100.0;
        
        // Ready
        skills[7].fLastUsedTime = 0.0;
        skills[7].fCoolTime = 0.0;
    }
    
    // 0 = FALSE (in cooltime), 1 = TRUE (not in cooltime, ready)
    // Actually, in C++:
    // fElapsedTime = CTimer::Instance().GetCurrentSecond() - m_playerStatus.aSkill[dwSlotIndex].fLastUsedTime;
    // if (fElapsedTime >= m_playerStatus.aSkill[dwSlotIndex].fCoolTime) return false; (Not in cool time)
    // return true; (In cool time)
    // Wait, my FFI implementation returns 0 (FALSE) if elapsed >= fCoolTime, which matches C++ boolean meaning!
    // So 1 means it IS in cooltime, 0 means it IS NOT in cooltime.
    
    assert_eq!(m2_py_player_is_skill_cool_time(6), 1); // True, still cooling
    assert_eq!(m2_py_player_is_skill_cool_time(7), 0); // False, not cooling
    assert_eq!(m2_py_player_is_skill_cool_time(-1), 0); // Out of bounds
}
