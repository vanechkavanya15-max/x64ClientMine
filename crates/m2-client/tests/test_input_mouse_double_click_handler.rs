#[path = "../src/input_mouse_double_click_handler.rs"]
mod input_mouse_double_click_handler;

use input_mouse_double_click_handler::{DoubleClickDetector, Position, DoubleClickAction};
use std::time::{Duration, Instant};

#[test]
fn test_single_click() {
    let mut detector = DoubleClickDetector::default();
    let time = Instant::now();
    let pos = Position { x: 10, y: 10 };
    
    let action = detector.on_left_click(time, pos, None);
    assert_eq!(action, DoubleClickAction::None);
}

#[test]
fn test_valid_double_click() {
    let mut detector = DoubleClickDetector::default();
    let t1 = Instant::now();
    let pos = Position { x: 10, y: 10 };
    
    detector.on_left_click(t1, pos, None);
    
    let t2 = t1 + Duration::from_millis(200); // 200ms < 500ms default
    let action = detector.on_left_click(t2, pos, Some(DoubleClickAction::UsePotion));
    assert_eq!(action, DoubleClickAction::UsePotion);
}
