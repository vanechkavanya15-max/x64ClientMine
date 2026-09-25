//! Modul `app_event` — Obsluga zdarzen cyklu klatki i przekazywanie wejscia
//! Wzorzec referencyjny C++: `source/UserInterface/PythonApplicationEvent.cpp`
//! Opis: Odwzorowanie metod OnCameraUpdate, OnUIUpdate, OnUIRender, OnMouseMove, OnMouseLeftButtonDown, OnMouseRightButtonDown.

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Default)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Debug, Clone, Default)]
pub struct Camera {
    pub update_called: bool,
    pub drag_result: bool,
    pub drag_point: Point,
}

impl Camera {
    pub fn update(&mut self) {
        self.update_called = true;
    }

    pub fn drag(&self, _x: i32, _y: i32, point: &mut Point) -> bool {
        if self.drag_result {
            point.x = self.drag_point.x;
            point.y = self.drag_point.y;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct WindowManager {
    pub update_called: bool,
    pub render_called: bool,
    pub resolution: (i32, i32),
    pub mouse_pos: (i32, i32),
    pub left_button_down: bool,
    pub right_button_down: bool,
}

impl WindowManager {
    pub fn update(&mut self) {
        self.update_called = true;
    }

    pub fn render(&mut self) {
        self.render_called = true;
    }

    pub fn set_resolution(&mut self, width: i32, height: i32) {
        self.resolution = (width, height);
    }

    pub fn run_mouse_move(&mut self, x: i32, y: i32) {
        self.mouse_pos = (x, y);
    }

    pub fn run_mouse_left_button_down(&mut self, x: i32, y: i32) {
        self.left_button_down = true;
        self.mouse_pos = (x, y);
    }

    pub fn run_mouse_right_button_down(&mut self, x: i32, y: i32) {
        self.right_button_down = true;
        self.mouse_pos = (x, y);
    }
}

#[derive(Debug, Clone, Default)]
pub struct AppEvent {
    pub initialized: bool,
    pub map_ready: bool,
    pub camera: Option<Camera>,
    pub window_manager: WindowManager,
    pub client_rect: Rect,
    pub cursor_pos_set: Option<Point>,
}

impl AppEvent {
    pub fn new() -> Self {
        Self {
            initialized: true,
            map_ready: false,
            camera: None,
            window_manager: WindowManager::default(),
            client_rect: Rect::default(),
            cursor_pos_set: None,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn on_camera_update(&mut self) {
        if self.map_ready {
            if let Some(ref mut camera) = self.camera {
                camera.update();
            }
        }
    }

    pub fn on_ui_update(&mut self) {
        self.window_manager.update();
    }

    pub fn on_ui_render(&mut self) {
        self.window_manager.render();
    }

    pub fn on_mouse_move(&mut self, mut x: i32, mut y: i32) {
        let mut point = Point::default();

        if let Some(ref camera) = self.camera {
            if self.map_ready && camera.drag(x, y, &mut point) {
                x = point.x;
                y = point.y;
                
                // ClientToScreen mock - zakladamy offset
                point.x += 10;
                point.y += 10;
                
                // SetCursorPos mock
                self.cursor_pos_set = Some(Point { x: point.x, y: point.y });
            }
        }

        let width = self.client_rect.right - self.client_rect.left;
        let height = self.client_rect.bottom - self.client_rect.top;
        
        self.window_manager.set_resolution(width, height);
        self.window_manager.run_mouse_move(x, y);
    }

    pub fn on_mouse_left_button_down(&mut self, x: i32, y: i32) {
        self.window_manager.run_mouse_left_button_down(x, y);
    }

    pub fn on_mouse_right_button_down(&mut self, x: i32, y: i32) {
        self.window_manager.run_mouse_right_button_down(x, y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_app_event() {
        let instance = AppEvent::new();
        assert!(instance.is_ready());
    }
}
