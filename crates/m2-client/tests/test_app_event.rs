use m2_client::app_event::{AppEvent, Camera, Point, Rect};

#[test]
fn test_on_camera_update_ready() {
    let mut app = AppEvent::new();
    app.map_ready = true;
    app.camera = Some(Camera::default());

    app.on_camera_update();

    assert!(app.camera.unwrap().update_called);
}

#[test]
fn test_on_camera_update_not_ready() {
    let mut app = AppEvent::new();
    app.map_ready = false;
    app.camera = Some(Camera::default());

    app.on_camera_update();

    assert!(!app.camera.unwrap().update_called);
}

#[test]
fn test_on_ui_update() {
    let mut app = AppEvent::new();
    
    app.on_ui_update();
    
    assert!(app.window_manager.update_called);
}

#[test]
fn test_on_ui_render() {
    let mut app = AppEvent::new();
    
    app.on_ui_render();
    
    assert!(app.window_manager.render_called);
}

#[test]
fn test_on_mouse_move_drag() {
    let mut app = AppEvent::new();
    app.map_ready = true;
    app.client_rect = Rect { left: 0, top: 0, right: 800, bottom: 600 };
    
    let mut camera = Camera::default();
    camera.drag_result = true;
    camera.drag_point = Point { x: 100, y: 150 };
    app.camera = Some(camera);

    app.on_mouse_move(50, 50);

    // check window manager state
    assert_eq!(app.window_manager.resolution, (800, 600));
    assert_eq!(app.window_manager.mouse_pos, (100, 150));

    // check mock set cursor
    assert_eq!(app.cursor_pos_set, Some(Point { x: 110, y: 160 }));
}

#[test]
fn test_on_mouse_move_no_drag() {
    let mut app = AppEvent::new();
    app.map_ready = true; // or false, drag_result is false anyway
    app.client_rect = Rect { left: 10, top: 20, right: 1034, bottom: 788 };
    
    let mut camera = Camera::default();
    camera.drag_result = false;
    app.camera = Some(camera);

    app.on_mouse_move(50, 60);

    // check window manager state
    assert_eq!(app.window_manager.resolution, (1024, 768));
    assert_eq!(app.window_manager.mouse_pos, (50, 60));

    // mock set cursor should be None
    assert_eq!(app.cursor_pos_set, None);
}

#[test]
fn test_on_mouse_left_button_down() {
    let mut app = AppEvent::new();

    app.on_mouse_left_button_down(300, 400);

    assert!(app.window_manager.left_button_down);
    assert_eq!(app.window_manager.mouse_pos, (300, 400));
}

#[test]
fn test_on_mouse_right_button_down() {
    let mut app = AppEvent::new();

    app.on_mouse_right_button_down(500, 600);

    assert!(app.window_manager.right_button_down);
    assert_eq!(app.window_manager.mouse_pos, (500, 600));
}
