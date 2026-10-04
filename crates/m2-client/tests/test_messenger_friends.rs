use m2_client::game_messenger_friends_manager::{
    FriendOnlineStatus, GameMessengerFriendsManager,
};

#[test]
fn test_messenger_friends_lifecycle() {
    let mut manager = GameMessengerFriendsManager::new();
    assert_eq!(manager.count_friends(), 0);

    // Dodanie znajomego
    manager.add_friend("Janek", false);
    assert_eq!(manager.count_friends(), 1);
    assert_eq!(manager.count_online(), 0);

    // Logowanie znajomego
    assert!(manager.update_status("Janek", FriendOnlineStatus::Online));
    assert_eq!(manager.count_online(), 1);

    // Zablokowanie (przeniesienie do blacklisty)
    manager.block_player("Janek");
    assert_eq!(manager.count_friends(), 0);
    assert!(manager.is_blocked("Janek"));

    // Odblokowanie
    assert!(manager.unblock_player("Janek"));
    assert!(!manager.is_blocked("Janek"));
}
