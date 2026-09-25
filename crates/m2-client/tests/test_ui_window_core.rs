use m2_client::ui_window_core::{UiWindowCore, HorizontalAlign, VerticalAlign, window_flags};
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_integration_ui_window_core() {
    let instance = UiWindowCore::new();
    assert!(instance.is_ready());
}

#[test]
fn test_window_flags() {
    let mut win = UiWindowCore::new();
    assert_eq!(win.flags, 0);
    
    win.add_flag(window_flags::MOVABLE | window_flags::LIMIT);
    assert!(win.is_flag(window_flags::MOVABLE));
    assert!(win.is_flag(window_flags::LIMIT));
    assert!(!win.is_flag(window_flags::SNAP));
    
    win.remove_flag(window_flags::MOVABLE);
    assert!(!win.is_flag(window_flags::MOVABLE));
    assert!(win.is_flag(window_flags::LIMIT));
}

#[test]
fn test_window_visibility() {
    let mut win = UiWindowCore::new();
    assert!(!win.is_show());
    
    win.show();
    assert!(win.is_show());
    
    win.hide();
    assert!(!win.is_show());
}

#[test]
fn test_window_alignments() {
    let mut win = UiWindowCore::new();
    assert_eq!(win.horizontal_align, HorizontalAlign::Left);
    assert_eq!(win.vertical_align, VerticalAlign::Top);
    
    win.set_horizontal_align(HorizontalAlign::Right);
    win.set_vertical_align(VerticalAlign::Bottom);
    
    assert_eq!(win.horizontal_align, HorizontalAlign::Right);
    assert_eq!(win.vertical_align, VerticalAlign::Bottom);
}

#[test]
fn test_window_size_position() {
    let mut win = UiWindowCore::new();
    win.set_size(100, 200);
    win.set_position(10, 20);
    
    assert_eq!(win.get_width(), 100);
    assert_eq!(win.get_height(), 200);
    assert_eq!(win.get_position(), (10, 20));
}


#[test]
fn test_window_is_in() {
    let mut win = UiWindowCore::new();
    win.set_size(100, 100);
    win.set_position(50, 50);
    win.update_rect(); // prostokat to left: 50, top: 50, right: 150, bottom: 150
    
    assert!(win.is_in(50, 50));
    assert!(win.is_in(100, 100));
    assert!(win.is_in(150, 150));
    
    assert!(!win.is_in(49, 50));
    assert!(!win.is_in(50, 49));
    assert!(!win.is_in(151, 150));
    assert!(!win.is_in(150, 151));
}

#[test]
fn test_update_rect_with_parent() {
    let parent = Rc::new(RefCell::new(UiWindowCore::new()));
    parent.borrow_mut().set_size(800, 600);
    parent.borrow_mut().set_position(100, 100);
    parent.borrow_mut().update_rect(); // prostokat rodzica: left: 100, top: 100, right: 900, bottom: 700

    let child = Rc::new(RefCell::new(UiWindowCore::new()));
    child.borrow_mut().set_size(100, 50);
    child.borrow_mut().set_position(10, 20);
    
    parent.borrow_mut().add_child(Rc::clone(&child), Rc::downgrade(&parent));
    
    // Domyslne (Lewy Gorny)
    child.borrow_mut().update_rect();
    let rect = child.borrow().get_rect();
    assert_eq!(rect.right, 210);
    assert_eq!(rect.bottom, 170);
    
    // Prawy Dolny
    child.borrow_mut().set_horizontal_align(HorizontalAlign::Right);
    child.borrow_mut().set_vertical_align(VerticalAlign::Bottom);
    child.borrow_mut().update_rect();
    let rect = child.borrow().get_rect();
     // parent.width (800) - parent.left(10) ??? Wait.
    assert_eq!(rect.left, 890); // 800 - 10 + 100
    
    assert_eq!(rect.top, 680);
    
    // Srodek
    child.borrow_mut().set_horizontal_align(HorizontalAlign::Center);
    child.borrow_mut().set_vertical_align(VerticalAlign::Center);
    child.borrow_mut().update_rect();
    let rect = child.borrow().get_rect();
    assert_eq!(rect.left, 460);
    assert_eq!(rect.top, 395);
}


#[test]
fn test_rtl_alignment() {
    let parent = Rc::new(RefCell::new(UiWindowCore::new()));
    parent.borrow_mut().set_size(1000, 1000);
    parent.borrow_mut().add_flag(window_flags::RTL);
    parent.borrow_mut().update_rect();

    let child = Rc::new(RefCell::new(UiWindowCore::new()));
    child.borrow_mut().set_size(200, 200);
    child.borrow_mut().set_position(50, 50);
    parent.borrow_mut().add_child(Rc::clone(&child), Rc::downgrade(&parent));

    // RTL domyslne
    child.borrow_mut().update_rect();
    let rect = child.borrow().get_rect();
    assert_eq!(rect.left, 750);
    
    // RTL Srodek
    child.borrow_mut().set_horizontal_align(HorizontalAlign::Center);
    child.borrow_mut().update_rect();
    let rect = child.borrow().get_rect();
    assert_eq!(rect.left, 250);
    
    // RTL Prawa Strona
    child.borrow_mut().set_horizontal_align(HorizontalAlign::Right);
    child.borrow_mut().update_rect();
    let rect = child.borrow().get_rect();
    assert_eq!(rect.left, -50);
}
