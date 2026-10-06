#[path = "../src/ui_modal_dialog_queue_v2.rs"]
mod ui_modal_dialog_queue_v2;

use std::ptr;
use ui_modal_dialog_queue_v2::{DialogType, UiModalDialogQueueV2};

#[test]
fn test_queue_initialization() {
    let queue = UiModalDialogQueueV2::new(ptr::null_mut());
    assert!(!queue.has_active_dialog());
    assert_eq!(queue.get_mouse_capture_ref(), 0);
}

#[test]
fn test_push_and_pop_single_dialog() {
    let mut queue = UiModalDialogQueueV2::new(ptr::null_mut());

    let id = queue.push_dialog(DialogType::ConfirmExitGame);
    assert!(queue.has_active_dialog());
    assert_eq!(queue.get_mouse_capture_ref(), 1);

    let active = queue.get_active_dialog().unwrap();
    assert_eq!(active.id, id);
    assert_eq!(active.dialog_type, DialogType::ConfirmExitGame);

    let popped = queue.pop_dialog().unwrap();
    assert_eq!(popped.id, id);
    assert!(!queue.has_active_dialog());
    assert_eq!(queue.get_mouse_capture_ref(), 0);
}

#[test]
fn test_push_multiple_dialogs_queue_behavior() {
    let mut queue = UiModalDialogQueueV2::new(ptr::null_mut());

    let id1 = queue.push_dialog(DialogType::ConfirmDestroyItem {
        item_vnum: 1234,
        slot_index: 0,
    });
    let id2 = queue.push_dialog(DialogType::InfoPopup {
        message: "Inventory Full".to_string(),
    });
    let id3 = queue.push_dialog(DialogType::Question {
        question: "Are you sure?".to_string(),
    });

    // Active dialog should be the first one pushed
    assert!(queue.has_active_dialog());
    assert_eq!(queue.get_active_dialog().unwrap().id, id1);
    
    // Mouse capture should be exactly 1, because only one dialog is active at a time
    assert_eq!(queue.get_mouse_capture_ref(), 1);

    // Pop the first dialog
    let popped1 = queue.pop_dialog().unwrap();
    assert_eq!(popped1.id, id1);
    
    // Now the active dialog should be the second one pushed
    assert!(queue.has_active_dialog());
    assert_eq!(queue.get_active_dialog().unwrap().id, id2);
    
    // Mouse capture should STILL be exactly 1, since we popped one and immediately activated another
    assert_eq!(queue.get_mouse_capture_ref(), 1);

    // Pop the second dialog
    let popped2 = queue.pop_dialog().unwrap();
    assert_eq!(popped2.id, id2);

    // Active dialog should be the third one pushed
    assert!(queue.has_active_dialog());
    assert_eq!(queue.get_active_dialog().unwrap().id, id3);

    // Pop the third dialog
    let popped3 = queue.pop_dialog().unwrap();
    assert_eq!(popped3.id, id3);

    // Queue should now be empty
    assert!(!queue.has_active_dialog());
    assert_eq!(queue.get_mouse_capture_ref(), 0);
}

#[test]
fn test_clear_queue() {
    let mut queue = UiModalDialogQueueV2::new(ptr::null_mut());

    queue.push_dialog(DialogType::ConfirmExitGame);
    queue.push_dialog(DialogType::InfoPopup {
        message: "Test".to_string(),
    });

    assert!(queue.has_active_dialog());
    assert_eq!(queue.get_mouse_capture_ref(), 1);

    queue.clear();

    assert!(!queue.has_active_dialog());
    assert_eq!(queue.get_mouse_capture_ref(), 0);
}
