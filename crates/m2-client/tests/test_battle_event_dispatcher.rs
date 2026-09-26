#[path = "../src/battle_event_dispatcher.rs"]
mod battle_event_dispatcher;

use battle_event_dispatcher::{BattleEventDispatcher, BattleHitEvent};
use std::time::Duration;
use std::thread;

#[test]
fn test_enqueue_and_flush() {
    let mut dispatcher = BattleEventDispatcher::new();
    
    let event1 = BattleHitEvent {
        attacker_vid: 1,
        victim_vid: 2,
        hit_type: 0,
        motion_code: 10,
    };
    
    let event2 = BattleHitEvent {
        attacker_vid: 1,
        victim_vid: 3,
        hit_type: 1,
        motion_code: 11,
    };
    
    dispatcher.push_hit(event1.clone());
    dispatcher.push_hit(event2.clone());
    
    let flushed = dispatcher.flush_hits();
    assert_eq!(flushed.len(), 2);
    assert_eq!(flushed[0], event1);
    assert_eq!(flushed[1], event2);
    
    // Test that the queue is actually empty after flush
    let flushed_again = dispatcher.flush_hits();
    assert_eq!(flushed_again.len(), 0);
}

#[test]
fn test_flood_protection() {
    let mut dispatcher = BattleEventDispatcher::new();
    
    // Add 10 events rapidly, they should all be accepted
    for i in 0..10 {
        dispatcher.push_hit(BattleHitEvent {
            attacker_vid: 1,
            victim_vid: 100 + i,
            hit_type: 0,
            motion_code: i,
        });
    }
    
    // 11th event should be dropped due to flood protection
    let dropped_event = BattleHitEvent {
        attacker_vid: 1,
        victim_vid: 999,
        hit_type: 0,
        motion_code: 999,
    };
    dispatcher.push_hit(dropped_event);
    
    let flushed = dispatcher.flush_hits();
    assert_eq!(flushed.len(), 10);
    assert!(flushed.iter().all(|e| e.victim_vid != 999));
}

#[test]
fn test_flood_protection_reset() {
    let mut dispatcher = BattleEventDispatcher::new();
    
    // Add 10 events rapidly to fill the quota
    for i in 0..10 {
        dispatcher.push_hit(BattleHitEvent {
            attacker_vid: 1,
            victim_vid: 100 + i,
            hit_type: 0,
            motion_code: i,
        });
    }
    
    // Flush them out
    let flushed = dispatcher.flush_hits();
    assert_eq!(flushed.len(), 10);
    
    // Try adding another one immediately, should be dropped
    dispatcher.push_hit(BattleHitEvent {
        attacker_vid: 1,
        victim_vid: 999,
        hit_type: 0,
        motion_code: 999,
    });
    
    let flushed2 = dispatcher.flush_hits();
    assert_eq!(flushed2.len(), 0);
    
    // Sleep for a bit over 1 second to let the timestamps expire
    thread::sleep(Duration::from_millis(1100));
    
    // Now we should be able to add new events
    let new_event = BattleHitEvent {
        attacker_vid: 2,
        victim_vid: 888,
        hit_type: 1,
        motion_code: 888,
    };
    dispatcher.push_hit(new_event.clone());
    
    let flushed3 = dispatcher.flush_hits();
    assert_eq!(flushed3.len(), 1);
    assert_eq!(flushed3[0], new_event);
}

#[test]
fn test_empty_flush() {
    let mut dispatcher = BattleEventDispatcher::new();
    let flushed = dispatcher.flush_hits();
    assert_eq!(flushed.len(), 0);
}

#[test]
fn test_interleaved_push_and_flush() {
    let mut dispatcher = BattleEventDispatcher::new();
    
    let event1 = BattleHitEvent { attacker_vid: 1, victim_vid: 2, hit_type: 0, motion_code: 1 };
    let event2 = BattleHitEvent { attacker_vid: 1, victim_vid: 3, hit_type: 0, motion_code: 2 };
    let event3 = BattleHitEvent { attacker_vid: 1, victim_vid: 4, hit_type: 0, motion_code: 3 };
    
    dispatcher.push_hit(event1.clone());
    let flushed = dispatcher.flush_hits();
    assert_eq!(flushed.len(), 1);
    assert_eq!(flushed[0], event1);
    
    dispatcher.push_hit(event2.clone());
    dispatcher.push_hit(event3.clone());
    let flushed = dispatcher.flush_hits();
    assert_eq!(flushed.len(), 2);
    assert_eq!(flushed[0], event2);
    assert_eq!(flushed[1], event3);
}
