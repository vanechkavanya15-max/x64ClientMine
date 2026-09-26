use m2_client::pvp_mode_manager::{PvpModeManager, PvpMode, AlignmentType};

#[test]
fn test_peace_mode_same_empire() {
    // Cannot attack same empire in Peace mode
    assert_eq!(
        PvpModeManager::can_attack_player(
            1, 0, PvpMode::Peace, 0, 1, 0, PvpMode::Peace, 0, false, false, false, false
        ),
        false
    );
}

#[test]
fn test_peace_mode_different_empire() {
    // Can attack different empire even in Peace mode
    assert_eq!(
        PvpModeManager::can_attack_player(
            1, 0, PvpMode::Peace, 0, 2, 0, PvpMode::Peace, 0, false, false, false, false
        ),
        true
    );
}

#[test]
fn test_free_mode() {
    // Free mode can attack same empire (if not protect/party)
    assert_eq!(
        PvpModeManager::can_attack_player(
            1, 0, PvpMode::Free, 0, 1, 0, PvpMode::Peace, 0, false, false, false, false
        ),
        true
    );
}

#[test]
fn test_free_mode_protect_target() {
    // Free mode cannot attack Protect mode
    assert_eq!(
        PvpModeManager::can_attack_player(
            1, 0, PvpMode::Free, 0, 1, 0, PvpMode::Protect, 0, false, false, false, false
        ),
        false
    );
}

#[test]
fn test_revenge_mode_dark_target() {
    // Revenge mode can attack same empire target with negative alignment (Dark)
    assert_eq!(
        PvpModeManager::can_attack_player(
            1, 0, PvpMode::Revenge, 1000, 1, 0, PvpMode::Peace, -1000, false, false, false, false
        ),
        true
    );
}

#[test]
fn test_revenge_mode_white_target() {
    // Revenge mode cannot attack same empire target with positive alignment
    assert_eq!(
        PvpModeManager::can_attack_player(
            1, 0, PvpMode::Revenge, 1000, 1, 0, PvpMode::Peace, 1000, false, false, false, false
        ),
        false
    );
}

#[test]
fn test_guild_mode_same_guild() {
    // Guild mode cannot attack same guild
    assert_eq!(
        PvpModeManager::can_attack_player(
            1, 10, PvpMode::Guild, 0, 1, 10, PvpMode::Peace, 0, false, false, false, false
        ),
        false
    );
}

#[test]
fn test_guild_mode_different_guild() {
    // Guild mode can attack different guild
    assert_eq!(
        PvpModeManager::can_attack_player(
            1, 10, PvpMode::Guild, 0, 1, 20, PvpMode::Peace, 0, false, false, false, false
        ),
        true
    );
}

#[test]
fn test_same_party() {
    // Cannot attack party member even in Free mode
    assert_eq!(
        PvpModeManager::can_attack_player(
            1, 0, PvpMode::Free, 0, 1, 0, PvpMode::Peace, 0, true, false, false, false
        ),
        false
    );
}

#[test]
fn test_alignment_grading() {
    assert_eq!(PvpModeManager::get_alignment_grade(15000), 0);
    assert_eq!(PvpModeManager::get_alignment_grade(12000), 0);
    assert_eq!(PvpModeManager::get_alignment_grade(8000), 1);
    assert_eq!(PvpModeManager::get_alignment_grade(4000), 2);
    assert_eq!(PvpModeManager::get_alignment_grade(1000), 3);
    assert_eq!(PvpModeManager::get_alignment_grade(0), 4);
    assert_eq!(PvpModeManager::get_alignment_grade(-1000), 5);
    assert_eq!(PvpModeManager::get_alignment_grade(-5000), 6);
    assert_eq!(PvpModeManager::get_alignment_grade(-9000), 7);
    assert_eq!(PvpModeManager::get_alignment_grade(-15000), 8);
}

#[test]
fn test_alignment_type() {
    assert_eq!(PvpModeManager::get_alignment_type(15000), AlignmentType::White);
    assert_eq!(PvpModeManager::get_alignment_type(1000), AlignmentType::White);
    assert_eq!(PvpModeManager::get_alignment_type(500), AlignmentType::Normal);
    assert_eq!(PvpModeManager::get_alignment_type(0), AlignmentType::Normal);
    assert_eq!(PvpModeManager::get_alignment_type(-1000), AlignmentType::Dark);
    assert_eq!(PvpModeManager::get_alignment_type(-15000), AlignmentType::Dark);
}

#[test]
fn test_killer() {
    // Non-party killer can be attacked in Peace mode (same empire)
    assert_eq!(
        PvpModeManager::can_attack_player(
            1, 0, PvpMode::Peace, 0, 1, 0, PvpMode::Peace, 0, false, true, false, false
        ),
        true
    );
    // Party killer cannot be attacked
    assert_eq!(
        PvpModeManager::can_attack_player(
            1, 0, PvpMode::Peace, 0, 1, 0, PvpMode::Peace, 0, true, true, false, false
        ),
        false
    );
}

#[test]
fn test_duel_active() {
    // Duel active overrides other rules
    assert_eq!(
        PvpModeManager::can_attack_player(
            1, 0, PvpMode::Peace, 0, 1, 0, PvpMode::Protect, 0, true, false, true, false
        ),
        true
    );
}

#[test]
fn test_pvp_instance() {
    // PVP instance active overrides same-empire rules
    assert_eq!(
        PvpModeManager::can_attack_player(
            1, 0, PvpMode::Peace, 0, 1, 0, PvpMode::Peace, 0, false, false, false, true
        ),
        true
    );
}

#[test]
fn test_conflict_alignment() {
    assert_eq!(PvpModeManager::is_conflict_alignment(1000, -1000, PvpMode::Peace), true);
    assert_eq!(PvpModeManager::is_conflict_alignment(1000, 1000, PvpMode::Peace), false);
    assert_eq!(PvpModeManager::is_conflict_alignment(-1000, 1000, PvpMode::Peace), true);
    assert_eq!(PvpModeManager::is_conflict_alignment(-1000, -1000, PvpMode::Peace), false);
    assert_eq!(PvpModeManager::is_conflict_alignment(1000, -1000, PvpMode::Protect), false); // Target is Protect
}

#[test]
fn test_alignment_penalty() {
    // Killing innocent player in FREE mode applies penalty
    assert_eq!(PvpModeManager::calculate_alignment_penalty(PvpMode::Free, 50, 1000), -2000);
    
    // Killing dark player in FREE mode does not apply penalty
    assert_eq!(PvpModeManager::calculate_alignment_penalty(PvpMode::Free, 50, -1000), 0);
    
    // Killing innocent player in GUILD mode does not apply Free penalty
    assert_eq!(PvpModeManager::calculate_alignment_penalty(PvpMode::Guild, 50, 1000), 0);
}
