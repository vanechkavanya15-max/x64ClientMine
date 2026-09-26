#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MovementResult {
    pub is_moving: bool,
    pub dir_x: f32,
    pub dir_y: f32,
    pub rotation_deg: f32,
}

pub fn calculate_keyboard_movement(
    key_w: bool,
    key_s: bool,
    key_a: bool,
    key_d: bool,
    yaw_deg: f32,
) -> MovementResult {
    let forward: f32 = (if key_w { 1.0 } else { 0.0 }) - (if key_s { 1.0 } else { 0.0 });
    let strafe: f32 = (if key_d { 1.0 } else { 0.0 }) - (if key_a { 1.0 } else { 0.0 });

    let length = (forward * forward + strafe * strafe).sqrt();

    if length > 0.0 {
        let norm_f = forward / length;
        let norm_s = strafe / length;

        let yaw_rad = yaw_deg.to_radians();
        let cos_y = yaw_rad.cos();
        let sin_y = yaw_rad.sin();

        // Rotate local direction (norm_s, norm_f) by camera yaw
        // To be consistent with Metin2's GetDegreeFromDirection:
        // W=0, S=180, A=90, D=-90
        // This corresponds to local angle = atan2(-strafe, forward)
        let dir_x = norm_s * cos_y - norm_f * sin_y;
        let dir_y = norm_s * sin_y + norm_f * cos_y;

        let local_rot_rad = f32::atan2(-norm_s, norm_f);
        let mut rotation_deg = yaw_deg + local_rot_rad.to_degrees();

        // Normalize rotation to 0..360
        rotation_deg = rotation_deg % 360.0;
        if rotation_deg < 0.0 {
            rotation_deg += 360.0;
        }

        MovementResult {
            is_moving: true,
            dir_x,
            dir_y,
            rotation_deg,
        }
    } else {
        MovementResult {
            is_moving: false,
            dir_x: 0.0,
            dir_y: 0.0,
            rotation_deg: 0.0,
        }
    }
}
