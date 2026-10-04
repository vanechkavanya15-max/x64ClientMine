#[path = "../src/guild_protocol_manager.rs"]
mod guild_protocol_manager;

use guild_protocol_manager::*;

#[test]
fn test_enable_guild() {
    let mut manager = GuildProtocolManager::default();
    assert!(!manager.is_enabled);
    manager.enable_guild();
    assert!(manager.is_enabled);
}

#[test]
fn test_set_guild_money() {
    let mut manager = GuildProtocolManager::default();
    assert_eq!(manager.info.guild_money, 0);
    manager.set_guild_money(1000);
    assert_eq!(manager.info.guild_money, 1000);
}

#[test]
fn test_set_guild_exp() {
    let mut manager = GuildProtocolManager::default();
    assert_eq!(manager.info.guild_level, 0);
    assert_eq!(manager.info.current_experience, 0);
    manager.set_guild_exp(5, 5000);
    assert_eq!(manager.info.guild_level, 5);
    assert_eq!(manager.info.current_experience, 5000);
}

#[test]
fn test_set_grade_data_and_updates() {
    let mut manager = GuildProtocolManager::default();
    
    // Add Grade
    manager.set_grade_data(1, GuildGradeData {
        authority_flag: 0,
        name: "Member".to_string(),
    });
    
    assert_eq!(manager.grades.get(&1).unwrap().name, "Member");
    assert_eq!(manager.grades.get(&1).unwrap().authority_flag, 0);

    // Update Name
    manager.set_grade_name(1, "Officer");
    assert_eq!(manager.grades.get(&1).unwrap().name, "Officer");

    // Update Authority
    manager.set_grade_authority(1, GUILD_AUTH_ADD_MEMBER | GUILD_AUTH_REMOVE_MEMBER);
    assert_eq!(manager.grades.get(&1).unwrap().authority_flag, GUILD_AUTH_ADD_MEMBER | GUILD_AUTH_REMOVE_MEMBER);
}

#[test]
fn test_register_and_clear_comments() {
    let mut manager = GuildProtocolManager::default();
    assert!(manager.board_comments.is_empty());

    // Register a valid comment
    manager.register_comment(1, "Player1", "Hello Guild!");
    assert_eq!(manager.board_comments.len(), 1);
    assert_eq!(manager.board_comments[0].comment_id, 1);
    assert_eq!(manager.board_comments[0].name, "Player1");
    assert_eq!(manager.board_comments[0].comment, "Hello Guild!");

    // Register empty comment - should be ignored
    manager.register_comment(2, "Player2", "");
    assert_eq!(manager.board_comments.len(), 1);

    // Clear comments
    manager.clear_comment();
    assert!(manager.board_comments.is_empty());
}

#[test]
fn test_member_management() {
    let mut manager = GuildProtocolManager::default();

    let member1 = GuildMemberData {
        pid: 100,
        name: "Alice".to_string(),
        grade: 2,
        job: 0,
        level: 30,
        general_flag: 0,
        offer: 1000,
    };

    let member2 = GuildMemberData {
        pid: 101,
        name: "Bob".to_string(),
        grade: 3,
        job: 1,
        level: 20,
        general_flag: 0,
        offer: 500,
    };

    manager.register_member(member1.clone());
    manager.register_member(member2.clone());

    assert_eq!(manager.members.len(), 2);
    assert_eq!(manager.member_level_summary, 50);
    assert_eq!(manager.member_level_average, 25);
    assert_eq!(manager.member_experience_summary, 1500);

    // Update member1
    let mut member1_update = member1.clone();
    member1_update.level = 40;
    member1_update.offer = 2000;
    manager.register_member(member1_update);

    assert_eq!(manager.members.len(), 2);
    assert_eq!(manager.members.iter().find(|m| m.pid == 100).unwrap().level, 40);
    assert_eq!(manager.member_level_summary, 60);
    assert_eq!(manager.member_level_average, 30);
    assert_eq!(manager.member_experience_summary, 2500);

    // Change grade and general flag
    manager.change_guild_member_grade(101, 1);
    assert_eq!(manager.members.iter().find(|m| m.pid == 101).unwrap().grade, 1);

    manager.change_guild_member_general_flag(101, 1);
    assert_eq!(manager.members.iter().find(|m| m.pid == 101).unwrap().general_flag, 1);

    // Remove member
    manager.remove_member(100);
    assert_eq!(manager.members.len(), 1);
    assert_eq!(manager.members[0].pid, 101);
}

#[test]
fn test_skill_data_mut() {
    let mut manager = GuildProtocolManager::default();
    
    let skills = manager.get_guild_skill_data_mut();
    skills.skill_point = 5;
    skills.guild_point = 100;
    skills.max_guild_point = 200;

    assert_eq!(manager.skill_data.skill_point, 5);
    assert_eq!(manager.skill_data.guild_point, 100);
    assert_eq!(manager.skill_data.max_guild_point, 200);
}

#[test]
fn test_guild_names() {
    let mut manager = GuildProtocolManager::default();
    manager.register_guild_name(10, "BestGuild");
    assert_eq!(manager.names.get(&10).unwrap(), "BestGuild");
}
