#[path = "../src/input_key_modifiers_tracker.rs"]
mod input_key_modifiers_tracker;

use input_key_modifiers_tracker::InputKeyModifiersTracker;

#[test]
fn test_default_state() {
    let tracker = InputKeyModifiersTracker::new();
    assert!(!tracker.is_ctrl_pressed());
    assert!(!tracker.is_shift_pressed());
    assert!(!tracker.is_alt_pressed());
}

#[test]
fn test_ctrl_key_pressing() {
    let mut tracker = InputKeyModifiersTracker::new();
    
    // Press main ctrl
    tracker.on_key_down(InputKeyModifiersTracker::VK_CONTROL);
    assert!(tracker.is_ctrl_pressed());
    assert!(!tracker.is_shift_pressed());
    assert!(!tracker.is_alt_pressed());
    
    // Release main ctrl
    tracker.on_key_up(InputKeyModifiersTracker::VK_CONTROL);
    assert!(!tracker.is_ctrl_pressed());
    
    // Press left ctrl
    tracker.on_key_down(InputKeyModifiersTracker::VK_LCONTROL);
    assert!(tracker.is_ctrl_pressed());
    
    // Release left ctrl
    tracker.on_key_up(InputKeyModifiersTracker::VK_LCONTROL);
    assert!(!tracker.is_ctrl_pressed());
    
    // Press right ctrl
    tracker.on_key_down(InputKeyModifiersTracker::VK_RCONTROL);
    assert!(tracker.is_ctrl_pressed());
    
    // Release right ctrl
    tracker.on_key_up(InputKeyModifiersTracker::VK_RCONTROL);
    assert!(!tracker.is_ctrl_pressed());
}

#[test]
fn test_shift_key_pressing() {
    let mut tracker = InputKeyModifiersTracker::new();
    
    // Press main shift
    tracker.on_key_down(InputKeyModifiersTracker::VK_SHIFT);
    assert!(tracker.is_shift_pressed());
    assert!(!tracker.is_ctrl_pressed());
    assert!(!tracker.is_alt_pressed());
    
    // Release main shift
    tracker.on_key_up(InputKeyModifiersTracker::VK_SHIFT);
    assert!(!tracker.is_shift_pressed());
    
    // Press left shift
    tracker.on_key_down(InputKeyModifiersTracker::VK_LSHIFT);
    assert!(tracker.is_shift_pressed());
    
    // Release left shift
    tracker.on_key_up(InputKeyModifiersTracker::VK_LSHIFT);
    assert!(!tracker.is_shift_pressed());
    
    // Press right shift
    tracker.on_key_down(InputKeyModifiersTracker::VK_RSHIFT);
    assert!(tracker.is_shift_pressed());
    
    // Release right shift
    tracker.on_key_up(InputKeyModifiersTracker::VK_RSHIFT);
    assert!(!tracker.is_shift_pressed());
}

#[test]
fn test_alt_key_pressing() {
    let mut tracker = InputKeyModifiersTracker::new();
    
    // Press main alt
    tracker.on_key_down(InputKeyModifiersTracker::VK_MENU);
    assert!(tracker.is_alt_pressed());
    assert!(!tracker.is_ctrl_pressed());
    assert!(!tracker.is_shift_pressed());
    
    // Release main alt
    tracker.on_key_up(InputKeyModifiersTracker::VK_MENU);
    assert!(!tracker.is_alt_pressed());
    
    // Press left alt
    tracker.on_key_down(InputKeyModifiersTracker::VK_LMENU);
    assert!(tracker.is_alt_pressed());
    
    // Release left alt
    tracker.on_key_up(InputKeyModifiersTracker::VK_LMENU);
    assert!(!tracker.is_alt_pressed());
    
    // Press right alt
    tracker.on_key_down(InputKeyModifiersTracker::VK_RMENU);
    assert!(tracker.is_alt_pressed());
    
    // Release right alt
    tracker.on_key_up(InputKeyModifiersTracker::VK_RMENU);
    assert!(!tracker.is_alt_pressed());
}

#[test]
fn test_multiple_keys_and_reset() {
    let mut tracker = InputKeyModifiersTracker::new();
    
    // Press multiple keys
    tracker.on_key_down(InputKeyModifiersTracker::VK_CONTROL);
    tracker.on_key_down(InputKeyModifiersTracker::VK_LSHIFT);
    tracker.on_key_down(InputKeyModifiersTracker::VK_RMENU);
    
    assert!(tracker.is_ctrl_pressed());
    assert!(tracker.is_shift_pressed());
    assert!(tracker.is_alt_pressed());
    
    // Ignore non-modifier key
    tracker.on_key_down(0x41); // 'A' key
    assert!(tracker.is_ctrl_pressed());
    assert!(tracker.is_shift_pressed());
    assert!(tracker.is_alt_pressed());
    
    // Reset state
    tracker.reset();
    assert!(!tracker.is_ctrl_pressed());
    assert!(!tracker.is_shift_pressed());
    assert!(!tracker.is_alt_pressed());
}
