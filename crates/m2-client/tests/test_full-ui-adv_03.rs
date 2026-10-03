#[path = "../src/ui_rendertarget_rotation_controller.rs"]
mod ui_rendertarget_rotation_controller;
#[path = "../src/ui_widget_rendertarget.rs"]
mod ui_widget_rendertarget;

use ui_rendertarget_rotation_controller::RenderTargetRotationController;
use ui_widget_rendertarget::ActorRenderPreview;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let controller = RenderTargetRotationController::new(0.5);
        assert_eq!(controller.is_dragging(), false);
    }

    #[test]
    fn test_mouse_down_up() {
        let mut controller = RenderTargetRotationController::new(0.5);
        assert_eq!(controller.is_dragging(), false);
        
        controller.on_mouse_down(100.0, 50.0);
        assert_eq!(controller.is_dragging(), true);
        
        controller.on_mouse_up();
        assert_eq!(controller.is_dragging(), false);
    }

    #[test]
    fn test_mouse_move_without_down() {
        let mut controller = RenderTargetRotationController::new(0.5);
        let mut preview = ActorRenderPreview::new(0, 0.0);
        
        controller.on_mouse_move(10.0, 10.0, &mut preview);
        assert_eq!(preview.rotation_y, 0.0); // Rotation should not change if not dragging
    }

    #[test]
    fn test_mouse_move_with_down() {
        let mut controller = RenderTargetRotationController::new(1.0); // Speed 1.0 for simpler math
        let mut preview = ActorRenderPreview::new(0, 0.0);
        
        controller.on_mouse_down(100.0, 50.0);
        controller.on_mouse_move(110.0, 50.0, &mut preview);
        
        // Moved 10 pixels with speed 1.0 -> 10.0 degrees
        assert_eq!(preview.rotation_y, 10.0);
        
        // Move backwards
        controller.on_mouse_move(105.0, 50.0, &mut preview);
        // Moved -5 pixels -> 5.0 degrees
        assert_eq!(preview.rotation_y, 5.0);
    }

    #[test]
    fn test_rotation_normalization() {
        let mut controller = RenderTargetRotationController::new(1.0);
        
        // Test wrap around positive
        let mut preview1 = ActorRenderPreview::new(0, 350.0);
        controller.on_mouse_down(100.0, 50.0);
        controller.on_mouse_move(120.0, 50.0, &mut preview1);
        // 350.0 + 20.0 = 370.0 -> normalized to 10.0
        assert_eq!(preview1.rotation_y, 10.0);
        
        // Test wrap around negative
        let mut preview2 = ActorRenderPreview::new(0, 10.0);
        controller.on_mouse_down(100.0, 50.0); // reset drag position
        controller.on_mouse_move(80.0, 50.0, &mut preview2);
        // 10.0 - 20.0 = -10.0 -> normalized to 350.0
        assert_eq!(preview2.rotation_y, 350.0);

        // Test multiple wraps positive
        let mut preview3 = ActorRenderPreview::new(0, 0.0);
        controller.on_mouse_down(100.0, 50.0);
        controller.on_mouse_move(820.0, 50.0, &mut preview3);
        // 0.0 + 720.0 = 720.0 -> normalized to 0.0
        assert_eq!(preview3.rotation_y, 0.0);
        
        // Test multiple wraps negative
        let mut preview4 = ActorRenderPreview::new(0, 0.0);
        controller.on_mouse_down(100.0, 50.0);
        controller.on_mouse_move(-620.0, 50.0, &mut preview4);
        // 0.0 - 720.0 = -720.0 -> normalized to 0.0
        assert_eq!(preview4.rotation_y, 0.0);
    }
}
