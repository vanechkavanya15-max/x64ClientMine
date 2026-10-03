use crate::win32_message_dispatcher::{WinMsg, WM_MOUSEMOVE, WM_RBUTTONDOWN, WM_RBUTTONUP};
use crate::target_selection::TargetManager;

#[derive(Debug, Default)]
pub struct RButtonDispatcher {
    pub is_rotating_camera: bool,
    pub last_mouse_x: i32,
    pub last_mouse_y: i32,
    pub camera_yaw: f32,
    pub camera_pitch: f32,
}

impl RButtonDispatcher {
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles right mouse button and movement messages.
    /// Returns `Some(())` if the message was handled, `None` otherwise.
    pub fn handle_message(
        &mut self,
        msg: &WinMsg,
        hovered_target_vid: Option<u32>,
        target_manager: &mut TargetManager,
    ) -> Option<()> {
        match msg.message {
            WM_RBUTTONDOWN => {
                // If hovering over a target, select it instead of rotating the camera
                if let Some(vid) = hovered_target_vid {
                    target_manager.select_target(vid, 100, 100); // Dummy HP for target selection
                } else {
                    self.is_rotating_camera = true;
                    self.last_mouse_x = msg.pt_x;
                    self.last_mouse_y = msg.pt_y;
                }
                Some(())
            }
            WM_RBUTTONUP => {
                if self.is_rotating_camera {
                    self.is_rotating_camera = false;
                    Some(())
                } else {
                    None
                }
            }
            WM_MOUSEMOVE => {
                if self.is_rotating_camera {
                    let dx = msg.pt_x - self.last_mouse_x;
                    let dy = msg.pt_y - self.last_mouse_y;

                    // Update camera rotation based on mouse delta
                    self.camera_yaw += dx as f32;
                    self.camera_pitch -= dy as f32; // Invert Y-axis for standard camera behavior

                    // Clamp pitch to avoid flipping
                    self.camera_pitch = self.camera_pitch.clamp(-89.0, 89.0);

                    // Wrap yaw around 360 degrees
                    while self.camera_yaw < 0.0 {
                        self.camera_yaw += 360.0;
                    }
                    while self.camera_yaw >= 360.0 {
                        self.camera_yaw -= 360.0;
                    }

                    self.last_mouse_x = msg.pt_x;
                    self.last_mouse_y = msg.pt_y;
                    Some(())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
