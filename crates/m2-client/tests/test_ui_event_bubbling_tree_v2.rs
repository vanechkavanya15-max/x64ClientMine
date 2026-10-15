use std::collections::HashMap;
use m2_client::ui_wndmgr_hierarchy::{WindowNode, FLAG_ATTACH, FLAG_NOT_CAPTURE, FLAG_NOT_PICK};

use m2_client::ui_event_bubbling_tree_v2::UiEventBubblingTreeV2;

#[test]
fn test_bubbling_simple_consumption() {
    let mut tree = UiEventBubblingTreeV2::new();
    let mut windows = HashMap::new();

    let win_id = 100;
    let win = WindowNode::new(win_id);
    windows.insert(win_id, win);

    let get_rect_fn = |_id| [10, 20, 110, 120];

    // Konsumuje zdarzenie (zwraca true)
    let result = tree.run_mouse_left_button_down(
        15, 25,
        Some(win_id),
        &windows,
        get_rect_fn,
        |_id| true,
    );

    assert_eq!(result, Some(win_id));
    assert_eq!(tree.left_capture_window, Some(win_id));
    assert_eq!(tree.drag_x, 15 - 10);
    assert_eq!(tree.drag_y, 25 - 20);
}

#[test]
fn test_bubbling_propagation_to_parent() {
    let mut tree = UiEventBubblingTreeV2::new();
    let mut windows = HashMap::new();

    let root_id = 1;
    let child_id = 2;

    let root = WindowNode::new(root_id);
    let mut child = WindowNode::new(child_id);
    child.parent_id = Some(root_id);

    windows.insert(root_id, root);
    windows.insert(child_id, child);

    let get_rect_fn = |id| {
        if id == root_id { [0, 0, 100, 100] } else { [10, 10, 50, 50] }
    };

    // Dziecko (id 2) zwraca false, wiec propaduje do rodzica (id 1)
    let result = tree.run_mouse_left_button_down(
        15, 15,
        Some(child_id),
        &windows,
        get_rect_fn,
        |id| id == root_id, // Tylko root konsumuje zdarzenie
    );

    assert_eq!(result, Some(root_id));
    assert_eq!(tree.left_capture_window, Some(root_id));
    assert_eq!(tree.drag_x, 15 - 0);
    assert_eq!(tree.drag_y, 15 - 0);
}

#[test]
fn test_bubbling_flag_not_pick() {
    let mut tree = UiEventBubblingTreeV2::new();
    let mut windows = HashMap::new();

    let root_id = 1;
    let child_id = 2;

    let root = WindowNode::new(root_id);
    let mut child = WindowNode::new(child_id);
    child.parent_id = Some(root_id);
    child.flags = FLAG_NOT_PICK; // Ma byc pominiety calkowicie

    windows.insert(root_id, root);
    windows.insert(child_id, child);

    let get_rect_fn = |_| [0, 0, 100, 100];

    // Konsumuje zdarzenie tylko root. Dziecko jest ignorowane z powodu flagi FLAG_NOT_PICK.
    let result = tree.run_mouse_left_button_down(
        50, 50,
        Some(child_id),
        &windows,
        get_rect_fn,
        |_id| true,
    );

    assert_eq!(result, Some(root_id));
    assert_eq!(tree.left_capture_window, Some(root_id));
}

#[test]
fn test_bubbling_flag_attach() {
    let mut tree = UiEventBubblingTreeV2::new();
    let mut windows = HashMap::new();

    let root_id = 1;
    let middle_id = 2;
    let child_id = 3;

    let root = WindowNode::new(root_id);
    
    let mut middle = WindowNode::new(middle_id);
    middle.parent_id = Some(root_id);

    let mut child = WindowNode::new(child_id);
    child.parent_id = Some(middle_id);
    child.flags = FLAG_ATTACH; // Powinno przejsc od razu do roota.

    windows.insert(root_id, root);
    windows.insert(middle_id, middle);
    windows.insert(child_id, child);

    let get_rect_fn = |_| [0, 0, 100, 100];

    // Zwraca true, kiedy event dociera do roota (poniewaz dziecko ma FLAG_ATTACH)
    let result = tree.run_mouse_left_button_down(
        50, 50,
        Some(child_id),
        &windows,
        get_rect_fn,
        |_id| true,
    );

    assert_eq!(result, Some(root_id));
    assert_eq!(tree.left_capture_window, Some(root_id));
}

#[test]
fn test_bubbling_flag_not_capture() {
    let mut tree = UiEventBubblingTreeV2::new();
    let mut windows = HashMap::new();

    let win_id = 1;
    let mut win = WindowNode::new(win_id);
    win.flags = FLAG_NOT_CAPTURE;

    windows.insert(win_id, win);

    let get_rect_fn = |_| [10, 10, 100, 100];

    let result = tree.run_mouse_left_button_down(
        20, 20,
        Some(win_id),
        &windows,
        get_rect_fn,
        |_id| true,
    );

    assert_eq!(result, Some(win_id));
    assert_eq!(tree.left_capture_window, None); // Capture jest wylaczone
    assert_eq!(tree.drag_x, 10);
    assert_eq!(tree.drag_y, 10);
}
