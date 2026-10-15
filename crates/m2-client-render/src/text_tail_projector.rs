use nalgebra::{Matrix4, Point3, Vector4};

#[repr(C)]
pub struct TextTailProjector {
    view_proj_matrix: Matrix4<f32>,
    screen_width: f32,
    screen_height: f32,
    ortho_depth: f32, // For distant projection scaling
}

#[repr(C)]
pub struct ProjectedTextTail {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Colors represented as RGBA tuples for simplicity. Can map to a Color enum or struct later.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct TextTailColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u32)]
pub enum Rank {
    Knight, // Rycerski
    Noble,  // Szlachetny
    Cruel,  // Okrutny
    None,
}

impl Rank {
    pub fn get_color(&self) -> TextTailColor {
        match self {
            // These colors correspond to PK Titles "Rycerski" (Blue), "Szlachetny" (Green), "Okrutny" (Red) 
            // - placeholder values but they match standard UI colors for these in typical setups.
            Rank::Knight => TextTailColor { r: 0.2, g: 0.2, b: 1.0, a: 1.0 },
            Rank::Noble => TextTailColor { r: 0.2, g: 1.0, b: 0.2, a: 1.0 },
            Rank::Cruel => TextTailColor { r: 1.0, g: 0.2, b: 0.2, a: 1.0 },
            Rank::None => TextTailColor { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
        }
    }
}

// Keeping this without #[repr(C)] since String is not FFI-safe anyway, but it could be replaced later
pub struct TextTailData {
    pub level: Option<u32>,
    pub guild_name: Option<String>,
    pub rank: Rank,
    pub name: String,
}

impl TextTailProjector {
    pub fn new(view_proj_matrix: Matrix4<f32>, screen_width: f32, screen_height: f32, ortho_depth: f32) -> Self {
        Self {
            view_proj_matrix,
            screen_width,
            screen_height,
            ortho_depth,
        }
    }

    /// Projects 3D head position to 2D screen coordinates
    pub fn project_position(&self, pos: &Point3<f32>, height_offset: f32, dist_from_player: f32) -> Option<ProjectedTextTail> {
        let world_pos = Point3::new(pos.x, pos.y, pos.z + height_offset);
        let world_pos_v4 = Vector4::new(world_pos.x, world_pos.y, world_pos.z, 1.0);
        let clip_pos = self.view_proj_matrix * world_pos_v4;

        if clip_pos.w <= 0.0 {
            // Behind camera
            return None;
        }

        let ndc_pos = clip_pos / clip_pos.w;

        let screen_x = (ndc_pos.x + 1.0) * 0.5 * self.screen_width;
        let screen_y = (1.0 - ndc_pos.y) * 0.5 * self.screen_height;
        
        let mut z = ndc_pos.z;
        // From C++:
        // if (pTextTail->fDistanceFromPlayer < 1300.0f) { pTextTail->z = 0.0f; }
        // else { pTextTail->z = pTextTail->z * CPythonGraphic::Instance().GetOrthoDepth() * -1.0f; pTextTail->z += 10.0f; }
        if dist_from_player < 1300.0 {
            z = 0.0;
        } else {
            z = z * self.ortho_depth * -1.0;
            z += 10.0;
        }

        Some(ProjectedTextTail {
            x: screen_x.floor(),
            y: screen_y.floor(),
            z,
        })
    }
}
