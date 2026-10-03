use crate::ui_widget_rendertarget::ActorRenderPreview;

pub struct RenderTargetRotationController {
    is_dragging: bool,
    last_mouse_x: f32,
    last_mouse_y: f32,
    rotation_speed: f32,
}

impl Default for RenderTargetRotationController {
    fn default() -> Self {
        Self::new(0.5) // Default sensitivity
    }
}

impl RenderTargetRotationController {
    pub fn new(rotation_speed: f32) -> Self {
        Self {
            is_dragging: false,
            last_mouse_x: 0.0,
            last_mouse_y: 0.0,
            rotation_speed,
        }
    }

    pub fn on_mouse_down(&mut self, mouse_x: f32, mouse_y: f32) {
        self.is_dragging = true;
        self.last_mouse_x = mouse_x;
        self.last_mouse_y = mouse_y;
    }

    pub fn on_mouse_up(&mut self) {
        self.is_dragging = false;
    }

    pub fn on_mouse_move(&mut self, mouse_x: f32, _mouse_y: f32, actor_preview: &mut ActorRenderPreview) {
        if !self.is_dragging {
            return;
        }

        let delta_x = mouse_x - self.last_mouse_x;
        self.last_mouse_x = mouse_x;

        // Apply rotation
        actor_preview.rotation_y += delta_x * self.rotation_speed;

        // Normalize rotation to [0.0, 360.0)
        actor_preview.rotation_y = actor_preview.rotation_y.rem_euclid(360.0);
    }

    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }

    pub fn set_rotation_speed(&mut self, speed: f32) {
        self.rotation_speed = speed;
    }
}
