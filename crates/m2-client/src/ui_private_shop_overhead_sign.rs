use glam::{Mat4, Vec3, Vec4};
use std::collections::HashMap;

/// Represents an action triggered by interacting with a private shop sign.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShopSignAction {
    None,
    OpenShop(u32), // Contains the VID (Virtual ID) of the shop owner
}

/// A struct representing a single private shop sign in the 3D world.
#[derive(Debug, Clone)]
pub struct PrivateShopSign {
    pub vid: u32,
    pub title: String,
    pub position: Vec3,
    pub height_offset: f32,
}

/// Render state for a sign, including its 2D projected screen coordinates and depth.
#[derive(Debug, Clone, Copy)]
pub struct SignRenderState {
    pub vid: u32,
    pub screen_x: f32,
    pub screen_y: f32,
    pub screen_z: f32,
    pub width: f32,
    pub height: f32,
}

/// Manager responsible for tracking, projecting, and hit-testing private shop signs.
pub struct PrivateShopSignManager {
    signs: HashMap<u32, PrivateShopSign>,
    render_states: Vec<SignRenderState>,
}

impl Default for PrivateShopSignManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PrivateShopSignManager {
    /// Creates a new PrivateShopSignManager.
    pub fn new() -> Self {
        Self {
            signs: HashMap::new(),
            render_states: Vec::new(),
        }
    }

    /// Adds or updates a private shop sign for a specific character/NPC.
    pub fn add_sign(&mut self, vid: u32, title: String, position: Vec3, height_offset: f32) {
        self.signs.insert(
            vid,
            PrivateShopSign {
                vid,
                title,
                position,
                height_offset,
            },
        );
    }

    /// Removes a private shop sign based on VID.
    pub fn remove_sign(&mut self, vid: u32) {
        self.signs.remove(&vid);
    }

    /// Updates the 3D position of an existing sign.
    pub fn update_sign_position(&mut self, vid: u32, position: Vec3) -> Result<(), &'static str> {
        if let Some(sign) = self.signs.get_mut(&vid) {
            sign.position = position;
            Ok(())
        } else {
            Err("Sign not found")
        }
    }

    /// Retrieves an immutable reference to a sign.
    pub fn get_sign(&self, vid: u32) -> Option<&PrivateShopSign> {
        self.signs.get(&vid)
    }

    /// Clears all signs and render states.
    pub fn clear(&mut self) {
        self.signs.clear();
        self.render_states.clear();
    }

    /// Returns the current list of render states (for drawing the signs).
    pub fn get_render_states(&self) -> &[SignRenderState] {
        &self.render_states
    }

    /// Updates the render states by projecting the 3D positions into 2D screen space.
    pub fn update_render_states(
        &mut self,
        view_proj: &Mat4,
        viewport_width: f32,
        viewport_height: f32,
        font_char_width: f32,
        font_char_height: f32,
        padding_x: f32,
        padding_y: f32,
    ) {
        self.render_states.clear();

        for sign in self.signs.values() {
            let world_pos = Vec4::new(
                sign.position.x,
                sign.position.y,
                sign.position.z + sign.height_offset,
                1.0,
            );

            let clip_coords = *view_proj * world_pos;

            // Only project and render if the point is in front of the camera (w > 0)
            if clip_coords.w > 0.0 {
                let ndc_x = clip_coords.x / clip_coords.w;
                let ndc_y = clip_coords.y / clip_coords.w;
                let ndc_z = clip_coords.z / clip_coords.w;

                // Screen projection assumes (0,0) is top-left, matching typical 2D APIs.
                let screen_x = (ndc_x + 1.0) * 0.5 * viewport_width;
                let screen_y = (1.0 - ndc_y) * 0.5 * viewport_height;
                
                // Calculate bounding box dimensions for the sign based on title text length.
                let text_len = sign.title.chars().count() as f32;
                let width = text_len * font_char_width + padding_x * 2.0;
                let height = font_char_height + padding_y * 2.0;

                self.render_states.push(SignRenderState {
                    vid: sign.vid,
                    screen_x,
                    screen_y,
                    screen_z: ndc_z,
                    width,
                    height,
                });
            }
        }

        // Depth sorting: smaller screen_z means closer to the camera.
        // We sort the vector so that closer signs are first, enabling correct hit testing.
        self.render_states.sort_by(|a, b| {
            a.screen_z.partial_cmp(&b.screen_z).unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Performs a 2D hit test on the rendered signs and returns the VID of the clicked sign, if any.
    pub fn hit_test(&self, mouse_x: f32, mouse_y: f32) -> Option<u32> {
        // Because render_states is sorted closest-to-furthest, the first hit is our target.
        for state in &self.render_states {
            let half_width = state.width * 0.5;
            
            // Assuming the text is centered at screen_x horizontally, 
            // and bottom-aligned at screen_y vertically.
            let left = state.screen_x - half_width;
            let right = state.screen_x + half_width;
            let top = state.screen_y - state.height;
            let bottom = state.screen_y;

            if mouse_x >= left && mouse_x <= right && mouse_y >= top && mouse_y <= bottom {
                return Some(state.vid);
            }
        }
        None
    }

    /// Determines the action to execute given a mouse click coordinate.
    pub fn handle_mouse_click(&self, mouse_x: f32, mouse_y: f32) -> ShopSignAction {
        if let Some(vid) = self.hit_test(mouse_x, mouse_y) {
            ShopSignAction::OpenShop(vid)
        } else {
            ShopSignAction::None
        }
    }
}
