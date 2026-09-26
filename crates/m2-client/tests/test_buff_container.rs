use m2_client::buff_container::{BuffContainer};

#[test]
fn test_buff_container_apply_and_get() {
    let mut container = BuffContainer::new();
    
    // Add a buff
    container.apply_buff(1, 5000, 10, 20);
    
    let buff = container.get_buff(1).expect("Buff should exist");
    assert_eq!(buff.affect_id, 1);
    assert_eq!(buff.duration_ms, 5000);
    assert_eq!(buff.elapsed_ms, 0);
    assert_eq!(buff.value1, 10);
    assert_eq!(buff.value2, 20);
    
    // Non-existent buff
    assert!(container.get_buff(999).is_none());
}

#[test]
fn test_buff_container_update_expiration() {
    let mut container = BuffContainer::new();
    
    container.apply_buff(1, 2000, 5, 5);
    container.apply_buff(2, 5000, 10, 10);
    
    // Update by 1000ms
    let expired = container.update(1000);
    assert!(expired.is_empty(), "No buffs should expire yet");
    
    let buff1 = container.get_buff(1).unwrap();
    assert_eq!(buff1.elapsed_ms, 1000);
    
    // Update by another 1500ms
    let expired = container.update(1500);
    assert_eq!(expired.len(), 1, "One buff should expire");
    assert_eq!(expired[0].affect_id, 1);
    
    // Buff 1 should be gone
    assert!(container.get_buff(1).is_none());
    
    // Buff 2 should still be there
    let buff2 = container.get_buff(2).unwrap();
    assert_eq!(buff2.elapsed_ms, 2500);
}

#[test]
fn test_buff_container_overwrite_buff() {
    let mut container = BuffContainer::new();
    
    container.apply_buff(1, 3000, 10, 0);
    container.update(1000);
    
    let buff = container.get_buff(1).unwrap();
    assert_eq!(buff.elapsed_ms, 1000);
    
    // Overwrite the buff
    container.apply_buff(1, 5000, 20, 5);
    
    let buff = container.get_buff(1).unwrap();
    assert_eq!(buff.duration_ms, 5000);
    assert_eq!(buff.elapsed_ms, 0, "elapsed_ms should be reset");
    assert_eq!(buff.value1, 20);
    assert_eq!(buff.value2, 5);
}

#[test]
fn test_buff_container_exact_expiration() {
    let mut container = BuffContainer::new();
    
    container.apply_buff(1, 3000, 10, 0);
    let expired = container.update(3000); // Exactly at duration
    
    assert_eq!(expired.len(), 1);
    assert_eq!(expired[0].affect_id, 1);
    assert!(container.get_buff(1).is_none());
}

#[test]
fn test_buff_container_clear() {
    let mut container = BuffContainer::new();
    
    container.apply_buff(1, 1000, 0, 0);
    container.apply_buff(2, 2000, 0, 0);
    
    container.clear();
    
    assert!(container.get_buff(1).is_none());
    assert!(container.get_buff(2).is_none());
    assert!(container.active_buffs.is_empty());
}
