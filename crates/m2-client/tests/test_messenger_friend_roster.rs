use std::ffi::CString;

#[path = "../src/messenger_friend_roster.rs"]
mod messenger_friend_roster;

use messenger_friend_roster::*;

#[test]
fn test_friend_operations() {
    let mut roster = MessengerFriendRoster::new();
    
    // Test add_friend
    roster.add_friend("Player1");
    assert!(roster.is_friend_by_key("Player1"));
    assert_eq!(roster.friend_state_map.get("Player1"), Some(&MESSENGER_CONNECTED_STATE_OFFLINE));

    // Test on_friend_login
    roster.on_friend_login("Player1");
    assert!(roster.is_friend_by_key("Player1"));
    assert!(roster.is_friend_by_name("Player1"));
    assert_eq!(roster.friend_state_map.get("Player1"), Some(&MESSENGER_CONNECTED_STATE_ONLINE));
    
    // Test on_friend_logout
    roster.on_friend_logout("Player2");
    assert!(roster.is_friend_by_key("Player2"));
    assert_eq!(roster.friend_state_map.get("Player2"), Some(&MESSENGER_CONNECTED_STATE_OFFLINE));
    
    // Test remove_friend
    roster.remove_friend("Player1");
    assert!(!roster.is_friend_by_key("Player1"));
}

#[test]
fn test_guild_member_operations() {
    let mut roster = MessengerFriendRoster::new();
    
    // Test append_guild_member
    roster.append_guild_member("GuildMember1");
    assert_eq!(roster.guild_member_state_map.get("GuildMember1"), Some(&MESSENGER_CONNECTED_STATE_OFFLINE));
    
    // Test login_guild_member
    roster.login_guild_member("GuildMember1");
    assert_eq!(roster.guild_member_state_map.get("GuildMember1"), Some(&MESSENGER_CONNECTED_STATE_ONLINE));
    
    // Test logout_guild_member
    roster.logout_guild_member("GuildMember1");
    assert_eq!(roster.guild_member_state_map.get("GuildMember1"), Some(&MESSENGER_CONNECTED_STATE_OFFLINE));
    
    // Test remove_guild_member
    roster.remove_guild_member("GuildMember1");
    assert!(!roster.guild_member_state_map.contains_key("GuildMember1"));
    
    // Test remove_all_guild_member
    roster.append_guild_member("GuildMember2");
    roster.append_guild_member("GuildMember3");
    roster.remove_all_guild_member();
    assert!(roster.guild_member_state_map.is_empty());
}

#[test]
fn test_destroy() {
    let mut roster = MessengerFriendRoster::new();
    roster.on_friend_login("Friend1");
    roster.append_guild_member("GuildMember1");
    
    roster.destroy();
    
    assert!(roster.friend_state_map.is_empty());
    assert!(roster.guild_member_state_map.is_empty());
}

#[test]
fn test_ffi_functions() {
    // Reset global state for reliable testing
    m2_messenger_destroy();

    let name1 = CString::new("FFIFriend1").unwrap();
    let name2 = CString::new("FFIFriend2").unwrap();
    
    // Add via rust code for setup
    if let Ok(mut roster) = MESSENGER_FRIEND_ROSTER.write() {
        roster.on_friend_login("FFIFriend1");
    }
    
    // Test m2_messenger_is_friend_by_name
    assert_eq!(m2_messenger_is_friend_by_name(name1.as_ptr()), 1);
    assert_eq!(m2_messenger_is_friend_by_name(name2.as_ptr()), 0);
    
    // Test m2_messenger_roster_remove_friend
    m2_messenger_roster_remove_friend(name1.as_ptr());
    assert_eq!(m2_messenger_is_friend_by_name(name1.as_ptr()), 0);
    
    // Clean up
    m2_messenger_destroy();
}
