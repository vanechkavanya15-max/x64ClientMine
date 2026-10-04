use m2_client::ui_wndmgr_widget_scroll_bar::{ScrollBarWidget, ScrollBarOrientation};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos_ratio_calculations() {
        let mut scrollbar = ScrollBarWidget::new(ScrollBarOrientation::Vertical);
        
        // Default pos is 0.0
        assert_eq!(scrollbar.get_pos(), 0.0);
        
        // Test clamping upper bound
        scrollbar.set_pos(1.5);
        assert_eq!(scrollbar.get_pos(), 1.0);
        
        // Test clamping lower bound
        scrollbar.set_pos(-0.5);
        assert_eq!(scrollbar.get_pos(), 0.0);
        
        // Test normal pos
        scrollbar.set_pos(0.75);
        assert_eq!(scrollbar.get_pos(), 0.75);
    }

    #[test]
    fn test_stepping_logic() {
        let mut scrollbar = ScrollBarWidget::new(ScrollBarOrientation::Vertical);
        scrollbar.step = 0.1;
        
        // Scroll down
        scrollbar.scroll_down();
        assert_eq!(scrollbar.get_pos(), 0.1);
        
        scrollbar.scroll_down();
        assert_eq!(scrollbar.get_pos(), 0.2);
        
        // Scroll up
        scrollbar.scroll_up();
        assert_eq!(scrollbar.get_pos(), 0.1);
        
        // Scroll up below 0
        scrollbar.scroll_up();
        scrollbar.scroll_up();
        assert_eq!(scrollbar.get_pos(), 0.0);
        
        // Scroll down past 1
        scrollbar.set_pos(0.95);
        scrollbar.scroll_down();
        scrollbar.scroll_down();
        assert_eq!(scrollbar.get_pos(), 1.0);
    }

    #[test]
    fn test_mouse_wheel_behavior() {
        let mut scrollbar = ScrollBarWidget::new(ScrollBarOrientation::Vertical);
        scrollbar.step = 0.2;
        scrollbar.set_pos(0.5);
        
        // Wheel up (positive delta) -> scroll up
        scrollbar.on_mouse_wheel(120);
        assert_eq!(scrollbar.get_pos(), 0.3); // 0.5 - 0.2 (Float imprecision might happen, let's check)
        assert!((scrollbar.get_pos() - 0.3).abs() < f32::EPSILON);
        
        // Wheel down (negative delta) -> scroll down
        scrollbar.on_mouse_wheel(-120);
        assert!((scrollbar.get_pos() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_track_clicking() {
        let mut scrollbar = ScrollBarWidget::new(ScrollBarOrientation::Vertical);
        scrollbar.track_length = 100.0;
        scrollbar.thumb_length = 20.0;
        scrollbar.page_step = 0.2;
        
        // Thumb is at 0.0 * 80.0 = 0.0, to 20.0
        scrollbar.set_pos(0.0);
        
        // Click below thumb (e.g., at 50.0) -> scroll down
        scrollbar.on_mouse_down_track(50.0);
        assert_eq!(scrollbar.get_pos(), 0.2);
        
        // Thumb is now at 0.2 * 80.0 = 16.0, to 36.0
        // Click above thumb (e.g., at 10.0) -> scroll up
        scrollbar.on_mouse_down_track(10.0);
        assert_eq!(scrollbar.get_pos(), 0.0);
    }

    #[test]
    fn test_drag_scrolling_logic() {
        let mut scrollbar = ScrollBarWidget::new(ScrollBarOrientation::Vertical);
        scrollbar.track_length = 100.0;
        scrollbar.thumb_length = 20.0; // Movable track is 80.0
        scrollbar.set_pos(0.5); // Starts at 0.5 (thumb at 40.0)
        
        // Begin drag
        scrollbar.begin_drag(50.0); // Mouse starts at 50.0
        assert_eq!(scrollbar.is_dragging, true);
        
        // Update drag (move mouse down by 40 pixels)
        // 40 pixels on 80 movable track = 0.5 ratio change
        scrollbar.update_drag(90.0);
        assert_eq!(scrollbar.get_pos(), 1.0); // 0.5 + 0.5
        
        // Update drag (move mouse up by 80 pixels from start)
        // -80 pixels on 80 movable track = -1.0 ratio change
        scrollbar.update_drag(-30.0); 
        assert_eq!(scrollbar.get_pos(), 0.0); // clamped
        
        // End drag
        scrollbar.end_drag();
        assert_eq!(scrollbar.is_dragging, false);
        
        // Update drag after ending should not change pos
        scrollbar.update_drag(50.0);
        assert_eq!(scrollbar.get_pos(), 0.0);
    }
}