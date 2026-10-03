#[path = "../src/input_mouse_double_click_handler.rs"]
mod input_mouse_double_click_handler;

use input_mouse_double_click_handler::{DoubleClickDetector, Position, DoubleClickAction};
use std::time::{Duration, Instant};

#[test]
fn test_double_click_use_potion() {
    let mut detector = DoubleClickDetector::default();
    let start = Instant::now();
    let pos = Position { x: 100, y: 100 };
    
    // First click
    let a1 = detector.on_left_click(start, pos, None);
    assert_eq!(a1, DoubleClickAction::None);
    
    // Second click within timeout and distance
    let t2 = start + Duration::from_millis(100);
    let a2 = detector.on_left_click(t2, pos, Some(DoubleClickAction::UsePotion));
    assert_eq!(a2, DoubleClickAction::UsePotion);
}

#[test]
fn test_double_click_equip_weapon() {
    let mut detector = DoubleClickDetector::default();
    let start = Instant::now();
    let pos = Position { x: 50, y: 50 };
    
    detector.on_left_click(start, pos, None);
    
    let t2 = start + Duration::from_millis(300);
    let a2 = detector.on_left_click(t2, pos, Some(DoubleClickAction::EquipWeapon));
    assert_eq!(a2, DoubleClickAction::EquipWeapon);
}

#[test]
fn test_double_click_timeout_exceeded() {
    let mut detector = DoubleClickDetector::new(Duration::from_millis(500), 25);
    let start = Instant::now();
    let pos = Position { x: 10, y: 10 };
    
    detector.on_left_click(start, pos, None);
    
    // Second click after timeout
    let t2 = start + Duration::from_millis(600);
    let a2 = detector.on_left_click(t2, pos, Some(DoubleClickAction::UsePotion));
    assert_eq!(a2, DoubleClickAction::None);
}

#[test]
fn test_double_click_distance_exceeded() {
    let mut detector = DoubleClickDetector::new(Duration::from_millis(500), 25); // max dist sq = 25 (5 pixels)
    let start = Instant::now();
    
    let pos1 = Position { x: 10, y: 10 };
    detector.on_left_click(start, pos1, None);
    
    // Second click within time, but 10 pixels away (100 > 25)
    let t2 = start + Duration::from_millis(100);
    let pos2 = Position { x: 20, y: 10 };
    let a2 = detector.on_left_click(t2, pos2, Some(DoubleClickAction::UsePotion));
    assert_eq!(a2, DoubleClickAction::None);
}

#[test]
fn test_triple_click_handling() {
    let mut detector = DoubleClickDetector::default();
    let start = Instant::now();
    let pos = Position { x: 10, y: 10 };
    
    // First click
    detector.on_left_click(start, pos, None);
    
    // Second click (triggers double click)
    let t2 = start + Duration::from_millis(100);
    let a2 = detector.on_left_click(t2, pos, Some(DoubleClickAction::EquipWeapon));
    assert_eq!(a2, DoubleClickAction::EquipWeapon);
    
    // Third click (should act as a first click again, returning None)
    let t3 = start + Duration::from_millis(200);
    let a3 = detector.on_left_click(t3, pos, Some(DoubleClickAction::EquipWeapon));
    assert_eq!(a3, DoubleClickAction::None);
}
