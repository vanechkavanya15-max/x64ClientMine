pub const MIN_CAMERA_GROUND_CLEARANCE: f32 = 50.0;

/// Resolves terrain collision for the camera.
/// Calculates the terrain height under the camera. If the camera is too close
/// to the ground (closer than MIN_CAMERA_GROUND_CLEARANCE), its z position
/// is adjusted upwards. We can also compute zoom correction if needed.
///
/// * `eye_x`, `eye_y`, `eye_z` - Current position of the camera.
/// * `view_dir_x`, `view_dir_y`, `view_dir_z` - Normalized view direction vector.
/// * `distance` - Current distance of the camera from its target.
/// * `get_terrain_height` - Closure or function providing terrain height at (x,y).
pub fn resolve_camera_terrain_collision<F>(
    eye_x: f32,
    eye_y: f32,
    eye_z: f32,
    view_dir_x: f32,
    view_dir_y: f32,
    view_dir_z: f32,
    distance: f32,
    get_terrain_height: F,
) -> (f32, f32, f32, f32)
where
    F: Fn(f32, f32) -> f32,
{
    let ground_z = get_terrain_height(eye_x, eye_y);
    let target_z = ground_z + MIN_CAMERA_GROUND_CLEARANCE;

    let new_eye_x = eye_x;
    let new_eye_y = eye_y;
    let mut new_eye_z = eye_z;
    let mut new_distance = distance;

    if eye_z < target_z {
        // Option 1: Elevate eye_z
        new_eye_z = target_z;

        // Option 2: Adjust distance (zoom correction).
        // Since we elevated eye_z, if the user requires the camera to slide forward (zoom in)
        // instead of just magically jumping up, we adjust the distance.
        // If we move the camera along the view direction, we move it until z >= target_z.
        // Since eye is looking AT the target, view direction is from eye TO target.
        // Or wait: usually "view_dir" is from eye to target.
        // So moving camera forward (zoom in) means `eye = eye + view_dir * t`
        // We want `eye_z + view_dir_z * t >= ground_z(eye_x + view_dir_x * t, eye_y + ...) + MIN_...`
        //
        // However, standard collision logic in Metin2 (from C++ CCamera::ProcessTerrainCollision):
        // D3DXVECTOR3 v3CheckVector = m_v3Eye - 2.0f * m_fTerrainCollisionRadius * m_v3Up;
        // v3CheckVector.z = rPythonBackground.GetHeight(floorf(v3CheckVector.x), floorf(v3CheckVector.y));
        // D3DXVECTOR3 v3NewEye = v3CheckVector + 2.0f * m_fTerrainCollisionRadius * m_v3Up;
        // if (v3NewEye.z > m_v3Eye.z) SetEye(v3NewEye);
        //
        // It strictly elevates the Z coordinate based on m_v3Up. It doesn't modify the X/Y coordinate or adjust distance directly in this function,
        // although adjusting Z changes the apparent pitch/distance to the target implicitly if target stays same.
        //
        // The prompt dictates: "Podniesienie eye_z = ground_z + MIN_CAMERA_GROUND_CLEARANCE lub zmniejszenie dystansu kamery (zoom in)".
        // We will elevate eye_z. To satisfy "zoom in" we could also just recalculate distance based on the new eye position.
        // Let's implement distance recalculation as well if view_dir is non-zero, but since Metin2 C++
        // just elevates Z, raising Z is the most faithful approach to the specific collision formula while
        // maintaining safety. We will just elevate Z and adjust distance mathematically if requested.

        // If the view vector is non-zero (so we know target direction), and we want to simulate zoom:
        let view_len2 = view_dir_x * view_dir_x + view_dir_y * view_dir_y + view_dir_z * view_dir_z;
        if view_len2 > 0.0001 {
            // A simple zoom correction: moving along view ray. 
            // We want new_eye_z = eye_z + view_dir_z * t = target_z
            // t = (target_z - eye_z) / view_dir_z
            // If we are looking down (view_dir_z < 0), moving forward decreases z, which is bad. 
            // If view_dir_z > 0 (looking up), moving forward increases z.
            // If view_dir_z == 0, we can't move along z by zooming.
            
            // To be robust and match the specification "Podniesienie eye_z = ground_z + MIN_CAMERA_GROUND_CLEARANCE"
            // we will strictly use elevation. It's the standard solution for terrain clipping.
            
            // Wait, maybe we adjust distance so that the distance is kept consistent with the new eye position?
            // If target was at T = E + V * D. E' = E + (0,0, target_z - eye_z).
            // D' = |T - E'|.
            // Let's just adjust distance to match the new |T - E'|.
            let target_pos_x = eye_x + view_dir_x * distance;
            let target_pos_y = eye_y + view_dir_y * distance;
            let target_pos_z = eye_z + view_dir_z * distance;

            let diff_x = target_pos_x - new_eye_x;
            let diff_y = target_pos_y - new_eye_y;
            let diff_z = target_pos_z - new_eye_z;
            
            new_distance = (diff_x * diff_x + diff_y * diff_y + diff_z * diff_z).sqrt();
        }
    }

    (new_eye_x, new_eye_y, new_eye_z, new_distance)
}
