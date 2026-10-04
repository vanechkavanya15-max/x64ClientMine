use m2_client::game_notice_banner_announcement::{BannerType, AnimationState, NoticeBannerSystem};

#[test]
fn test_append_notice_and_update() {
    let mut system = NoticeBannerSystem::new();
    system.append_notice("Test Notice 1".to_string(), 1000);
    
    assert_eq!(system.active_banners.len(), 1);
    assert_eq!(system.active_banners[0].text, "Test Notice 1");
    assert_eq!(system.active_banners[0].banner_type, BannerType::Notice);
    
    // Test expiration
    // Display duration is 5000, fade in is 500, fade out is 500. Total = 6000
    system.update(1000 + 6000); 
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
    // Display duration is 7000, fade in 1000, fade out 1000. Total = 9000
    system.update(2000 + 9000); 
    assert_eq!(system.active_banners.len(), 0);
}

#[test]
fn test_banner_animation_states() {
    let mut system = NoticeBannerSystem::new();
    system.append_notice("Animated Notice".to_string(), 1000);
    
    // Initial state: FadeIn
    assert_eq!(system.active_banners[0].state, AnimationState::FadeIn);
    assert_eq!(system.active_banners[0].opacity, 0.0);
    
    // Halfway through fade-in (250ms)
    system.update(1250);
    assert_eq!(system.active_banners[0].state, AnimationState::FadeIn);
    assert_eq!(system.active_banners[0].opacity, 0.5);
    assert_eq!(system.active_banners[0].position_y, -10.0);
    
    // Completed fade-in, entering Display (500ms)
    system.update(1500);
    assert_eq!(system.active_banners[0].state, AnimationState::Display);
    assert_eq!(system.active_banners[0].opacity, 1.0);
    assert_eq!(system.active_banners[0].position_y, 0.0);
    
    // Displaying (2000ms)
    system.update(3000);
    assert_eq!(system.active_banners[0].state, AnimationState::Display);
    assert_eq!(system.active_banners[0].opacity, 1.0);
    
    // Fade out start (5000 display + 500 fade-in = 5500 duration -> at 6500)
    system.update(6500);
    assert_eq!(system.active_banners[0].state, AnimationState::FadeOut);
    
    // Halfway through fade-out (6750)
    system.update(6750);
    assert_eq!(system.active_banners[0].state, AnimationState::FadeOut);
    assert_eq!(system.active_banners[0].opacity, 0.5);
    assert_eq!(system.active_banners[0].position_y, -10.0);
    
    // Finished (7000)
    system.update(7000);
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
    system.update(1000 + 6000); // 5000 duration + 1000 fades
    
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
    
    system.update(1000 + 9000); // 7000 duration + 2000 fades
    
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

#[test]
fn test_clear_system() {
    let mut system = NoticeBannerSystem::new();
    system.append_notice("Notice 1".to_string(), 1000);
    system.append_big_notice("Big 1".to_string(), 1000);
    
    system.max_active_notices = 1;
    system.append_notice("Notice 2".to_string(), 1000); // Queue this
    
    assert_eq!(system.active_banners.len(), 2);
    assert_eq!(system.notice_queue.len(), 1);
    
    system.clear();
    
    assert_eq!(system.active_banners.len(), 0);
    assert_eq!(system.notice_queue.len(), 0);
    assert_eq!(system.big_notice_queue.len(), 0);
}
