#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinType {
    Player,
    Monster,
    Metin,
    Portal,
    PartyMember,
}

impl PinType {
    /// Returns the ARGB color associated with the pin type
    pub fn get_color(&self) -> u32 {
        match self {
            // Player is often the center mark, or party member blue. 
            // We use default colors based on requirements:
            PinType::Player => 0xFFFFFFFF,       // default white or main player mark
            PinType::Monster => 0xFFFF0000,      // czerwony (red)
            PinType::Metin => 0xFFFFFF00,        // zolty (yellow)
            PinType::Portal => 0xFF00FF00,       // zielony (green)
            PinType::PartyMember => 0xFF0000FF,  // niebieski (blue)
        }
    }
}

pub struct MinimapCompassMapperV2;

impl MinimapCompassMapperV2 {
    /// Calculate compass needle rotation based on camera rotation (roll).
    pub fn calculate_compass_rotation(camera_rotation: f32) -> f32 {
        // In C++ it directly uses pkCmrCur->GetRoll(). We will just return it.
        // Or if it's based on player rotation:
        // fRotation = (540.0f - pkInst->GetRotation());
        // while(fRotation > 360.0f) fRotation -= 360.0f;
        // while(fRotation < 0.0f) fRotation += 360.0f;
        // We assume camera_rotation is the raw roll if we are rotating the compass itself.
        // If we were rotating the player arrow, we would map the 540-rotation.
        // The task asks for "Obrot igly kompasu wzgledem rotacji kamery" (Compass needle rotation relative to camera rotation).
        // Since `m_MiniMapCameraraphicImageInstance.SetRotation(pkCmrCur->GetRoll())` is used:
        camera_rotation
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MinimapScreenPos {
    pub x: f32,
    pub y: f32,
}

/// Computes the minimap screen offset coordinates for a target relative to the player.
///
/// `player_x`, `player_y`: the center (player) world coordinates.
/// `target_x`, `target_y`: the target world coordinates.
/// `radius`: the minimap window dynamic radius (e.g., width / 2.0 - 9.0).
///
/// Returns `MinimapScreenPos` representing the X and Y screen offsets from the center of the minimap.
#[no_mangle]
pub extern "C" fn m2_minimap_world_to_screen(
    player_x: f32,
    player_y: f32,
    target_x: f32,
    target_y: f32,
    radius: f32,
) -> MinimapScreenPos {
    let foo_cell_scale = 1.0 / 200.0; // CTerrainImpl::CELLSCALE is 200

    let mut distance_from_center_x = (target_x - player_x) * foo_cell_scale;
    let mut distance_from_center_y = (target_y - player_y) * foo_cell_scale;

    let distance_from_center = (distance_from_center_x * distance_from_center_x
        + distance_from_center_y * distance_from_center_y)
        .sqrt();

    if distance_from_center >= radius {
        let radian = distance_from_center_y.atan2(distance_from_center_x);
        distance_from_center_x = radius * radian.cos();
        distance_from_center_y = radius * radian.sin();
    }

    MinimapScreenPos {
        x: distance_from_center_x,
        y: distance_from_center_y,
    }
}
