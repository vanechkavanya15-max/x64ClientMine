#[path = "../src/guild_war_declaration_state_v2.rs"]
mod guild_war_declaration_state_v2;

use guild_war_declaration_state_v2::{
    GuildWarDeclarationStateV2, GuildWarType, GuildWarState, GuildRole, ENEMY_GUILD_SLOT_MAX_COUNT
};

#[test]
fn test_start_end_war() {
    let mut state = GuildWarDeclarationStateV2::new();
    
    // Start a war
    let res = state.start_guild_war(100, GuildWarType::Field);
    assert!(res.is_ok());
    assert_eq!(state.get_enemy_guild_id(0), Some(100));
    assert!(state.is_doing_guild_war());

    // Try starting the same war again
    let res2 = state.start_guild_war(100, GuildWarType::Arena);
    assert!(res2.is_err());

    // End the war
    state.end_guild_war(100);
    assert_eq!(state.get_enemy_guild_id(0), None);
    assert!(!state.is_doing_guild_war());
}

#[test]
fn test_max_enemy_guild_slots() {
    let mut state = GuildWarDeclarationStateV2::new();
    
    // Fill up all slots
    for i in 0..ENEMY_GUILD_SLOT_MAX_COUNT {
        let res = state.start_guild_war(100 + i as u32, GuildWarType::Arena);
        assert!(res.is_ok());
    }

    // Try adding one more
    let res = state.start_guild_war(999, GuildWarType::Flag);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Enemy guild slot is full");
}

#[test]
fn test_is_doing_guild_war() {
    let mut state = GuildWarDeclarationStateV2::new();
    
    assert!(!state.is_doing_guild_war());
    
    let _ = state.start_guild_war(10, GuildWarType::Field);
    assert!(state.is_doing_guild_war());
    
    state.end_guild_war(10);
    assert!(!state.is_doing_guild_war());
}

#[test]
fn test_war_point_tracking() {
    let mut state = GuildWarDeclarationStateV2::new();
    let _ = state.start_guild_war(42, GuildWarType::Flag);
    
    // Ensure scores start at 0
    let war = state.get_war_mut(42).unwrap();
    assert_eq!(war.score_self, 0);
    assert_eq!(war.score_opp, 0);

    // Update score
    let res = state.update_score(42, 10, 5);
    assert!(res.is_ok());

    let war_updated = state.get_war_mut(42).unwrap();
    assert_eq!(war_updated.score_self, 10);
    assert_eq!(war_updated.score_opp, 5);

    // Update score for non-existent war
    let res = state.update_score(99, 100, 100);
    assert!(res.is_err());
}

#[test]
fn test_permission_checks() {
    assert!(GuildRole::Leader.can_declare_war());
    assert!(GuildRole::General.can_declare_war());
    assert!(!GuildRole::Soldier.can_declare_war());
    assert!(!GuildRole::Member.can_declare_war());
}

#[test]
fn test_countdown_and_start_war() {
    let mut state = GuildWarDeclarationStateV2::new();
    let _ = state.start_guild_war(55, GuildWarType::Arena);
    
    let war = state.get_war_mut(55).unwrap();
    assert_eq!(war.state, GuildWarState::OnWar); // Starts as OnWar according to C++ implementation logic provided
    
    // Test countdown specifically
    war.state = GuildWarState::WaitStart;
    assert_eq!(war.state, GuildWarState::WaitStart);
    
    war.start_countdown(60);
    assert!(war.countdown_end.is_some());
    
    war.start_war();
    assert_eq!(war.state, GuildWarState::OnWar);
    assert!(war.war_end_time.is_some());
}
