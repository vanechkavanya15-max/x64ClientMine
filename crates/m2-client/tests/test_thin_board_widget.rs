use m2_client::ui_wndmgr_widget_thin_board::{Rect, NineSliceGeometry, ThinBoard, BoardWithTitleBar, ButtonState};

#[test]
fn test_nine_slice_geometry_calculate() {
    let width = 200.0;
    let height = 100.0;
    let corner_size = 16.0;

    let geo = NineSliceGeometry::calculate(width, height, corner_size);

    // Verify corners
    assert_eq!(geo.top_left, Rect::new(0.0, 0.0, 16.0, 16.0));
    assert_eq!(geo.top_right, Rect::new(184.0, 0.0, 16.0, 16.0));
    assert_eq!(geo.bottom_left, Rect::new(0.0, 84.0, 16.0, 16.0));
    assert_eq!(geo.bottom_right, Rect::new(184.0, 84.0, 16.0, 16.0));

    // Verify edges
    assert_eq!(geo.top, Rect::new(16.0, 0.0, 168.0, 16.0)); // 200 - 32 = 168
    assert_eq!(geo.bottom, Rect::new(16.0, 84.0, 168.0, 16.0));
    assert_eq!(geo.left, Rect::new(0.0, 16.0, 16.0, 68.0)); // 100 - 32 = 68
    assert_eq!(geo.right, Rect::new(184.0, 16.0, 16.0, 68.0));

    // Verify center
    assert_eq!(geo.center, Rect::new(16.0, 16.0, 168.0, 68.0));
}

#[test]
fn test_thin_board_resize() {
    let mut board = ThinBoard::new(100.0, 100.0);
    assert_eq!(board.geometry.center.width, 68.0);
    assert_eq!(board.geometry.center.height, 68.0);

    board.resize(300.0, 200.0);
    assert_eq!(board.width, 300.0);
    assert_eq!(board.height, 200.0);
    assert_eq!(board.geometry.center.width, 268.0); // 300 - 32
    assert_eq!(board.geometry.center.height, 168.0); // 200 - 32
    assert_eq!(board.geometry.top_right.x, 284.0);
    assert_eq!(board.geometry.bottom_left.y, 184.0);
}

#[test]
fn test_board_with_title_bar_layout_and_interaction() {
    let mut board = BoardWithTitleBar::new(400.0, 300.0, "Test Window");

    // Check title bar layout
    assert_eq!(board.title_bar_rect.width, 400.0);
    assert_eq!(board.title_bar_rect.height, 24.0);

    // Check close button layout
    // Title bar height: 24, Button height: 16 -> Margin top: (24-16)/2 = 4
    // Margin right: 8, Button width: 16 -> X: 400 - 16 - 8 = 376
    assert_eq!(board.close_button_rect.x, 376.0);
    assert_eq!(board.close_button_rect.y, 4.0);
    assert_eq!(board.close_button_rect.width, 16.0);
    assert_eq!(board.close_button_rect.height, 16.0);

    // Test mouse move inside button -> Hover
    board.handle_mouse_move(380.0, 10.0);
    assert_eq!(board.close_button_state, ButtonState::Hover);

    // Test mouse move outside button -> Normal
    board.handle_mouse_move(10.0, 10.0);
    assert_eq!(board.close_button_state, ButtonState::Normal);

    // Test mouse down inside button -> Down
    let clicked = board.handle_mouse_down(380.0, 10.0);
    assert_eq!(clicked, true);
    assert_eq!(board.close_button_state, ButtonState::Down);

    // Test mouse down outside button -> False, no state change to Down
    let clicked_outside = board.handle_mouse_down(10.0, 10.0);
    assert_eq!(clicked_outside, false);
    // state remains down unless moved

    // Test mouse up inside button after down -> Hover, returns true (click registered)
    board.close_button_state = ButtonState::Down; // Ensure it's down
    let click_completed = board.handle_mouse_up(380.0, 10.0);
    assert_eq!(click_completed, true);
    assert_eq!(board.close_button_state, ButtonState::Hover);

    // Test mouse up outside button -> Normal, returns false
    let click_missed = board.handle_mouse_up(10.0, 10.0);
    assert_eq!(click_missed, false);
    assert_eq!(board.close_button_state, ButtonState::Normal);
}

#[test]
fn test_board_with_title_bar_resize() {
    let mut board = BoardWithTitleBar::new(200.0, 200.0, "Resize");
    
    // Resize to 500x400
    board.resize(500.0, 400.0);

    // Verify title bar width
    assert_eq!(board.title_bar_rect.width, 500.0);

    // Verify close button position
    // X: 500 - 16 - 8 = 476
    assert_eq!(board.close_button_rect.x, 476.0);
    
    // Verify underlying board geometry
    assert_eq!(board.board.geometry.center.width, 468.0);
    assert_eq!(board.board.geometry.center.height, 368.0);
}
