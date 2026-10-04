use m2_client::game_notice_banner_announcement::{BannerType, NoticeBannerSystem};

#[test]
fn test_append_notice_and_update() {
    let mut system = NoticeBannerSystem::new();
    system.append_notice("Test Notice 1".to_string(), 1000);
    
    assert_eq!(system.active_banners.len(), 1);
    assert_eq!(system.active_banners[0].text, "Test Notice 1");
    assert_eq!(system.active_banners[0].banner_type, BannerType::Notice);
    
    // Test expiration
    system.update(1000 + 5000); // 5000 is display duration
    assert_eq!(system.active_banners.len(), 0);
}

#[test]
fn test_append_big_notice_and_update() {
    let mut system = NoticeBannerSystem::new();
    system.append_big_notice("Big Notice Event!".to_string(), 2000);
    
    assert_eq!(system.active_banners.len(), 1);
    assert_eq!(system.active_banners[0].text, "Big Notice Event!");
    assert_eq!(system.active_banners[0].banner_type, BannerType::BigNotice);
    
    // Test expiration
    system.update(2000 + 7000); // 7000 is display duration for BigNotice
    assert_eq!(system.active_banners.len(), 0);
}

#[test]
fn test_queue_limits() {
    let mut system = NoticeBannerSystem::new();
    system.max_active_notices = 2;
    
    system.append_notice("Notice 1".to_string(), 1000);
    system.append_notice("Notice 2".to_string(), 1000);
    system.append_notice("Notice 3".to_string(), 1000); // Should go to queue
    
    assert_eq!(system.active_banners.len(), 2);
    assert_eq!(system.notice_queue.len(), 1);
    
    // Expire the first two
    system.update(1000 + 5000);
    
    // The queued one should now be active
    assert_eq!(system.active_banners.len(), 1);
    assert_eq!(system.active_banners[0].text, "Notice 3");
    assert_eq!(system.notice_queue.len(), 0);
}

#[test]
fn test_big_notice_queue_limits() {
    let mut system = NoticeBannerSystem::new();
    system.max_active_big_notices = 1;
    
    system.append_big_notice("Big 1".to_string(), 1000);
    system.append_big_notice("Big 2".to_string(), 1000);
    
    assert_eq!(system.active_banners.len(), 1);
    assert_eq!(system.big_notice_queue.len(), 1);
    
    system.update(1000 + 7000);
    
    assert_eq!(system.active_banners.len(), 1);
    assert_eq!(system.active_banners[0].text, "Big 2");
}

#[test]
fn test_independent_queues() {
    let mut system = NoticeBannerSystem::new();
    system.max_active_notices = 1;
    system.max_active_big_notices = 1;
    
    system.append_notice("Notice 1".to_string(), 1000);
    system.append_notice("Notice 2".to_string(), 1000); // Blocked
    
    // Even though notice queue is blocked, big notice should still get through
    system.append_big_notice("Big 1".to_string(), 1000);
    
    assert_eq!(system.active_banners.len(), 2); // 1 Notice, 1 BigNotice
    assert_eq!(system.notice_queue.len(), 1);
    assert_eq!(system.big_notice_queue.len(), 0);
}
