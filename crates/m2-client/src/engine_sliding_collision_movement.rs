use glam::Vec3;

pub const GC_F_ACTOR_SLIDE_MOVE_SPEED: f32 = 5.0;

/// Projects velocity vector onto the wall plane when a collision occurs and clamps to GC_F_ACTOR_SLIDE_MOVE_SPEED.
/// Returns a tuple of (new_position, new_velocity).
pub fn calculate_sliding_movement(
    cur_pos: Vec3,
    mut velocity: Vec3,
    collision_normal: Vec3,
) -> (Vec3, Vec3) {
    if velocity.x == 0.0 && velocity.y == 0.0 && velocity.z == 0.0 {
        return (cur_pos, velocity);
    }

    let mut move_length = velocity.length();
    if move_length > GC_F_ACTOR_SLIDE_MOVE_SPEED {
        velocity *= GC_F_ACTOR_SLIDE_MOVE_SPEED / move_length;
    }

    // Project velocity onto the collision plane (tangent)
    // The vector tangent to the plane is: V - (V dot N) * N
    let dot = velocity.dot(collision_normal);
    let slide_delta = velocity - collision_normal * dot;

    velocity = slide_delta;
    let mut new_pos = cur_pos + slide_delta;

    if slide_delta.x != 0.0 || slide_delta.y != 0.0 || slide_delta.z != 0.0 {
        move_length = velocity.length();
        if move_length > GC_F_ACTOR_SLIDE_MOVE_SPEED {
            velocity *= GC_F_ACTOR_SLIDE_MOVE_SPEED / move_length;
            new_pos = cur_pos + velocity;
        }
    }

    (new_pos, velocity)
}

/// Calculates the normal vector for a cylinder-based collision (e.g., trees).
/// Assumes collision occurs primarily on the XY plane.
pub fn calculate_cylinder_collision_normal(player_pos: Vec3, obstacle_center: Vec3) -> Vec3 {
    let mut diff = player_pos - obstacle_center;
    diff.z = 0.0;
    if diff.length_squared() > 0.0 {
        diff.normalize()
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

/// Calculates the normal vector for an AABB-based collision (e.g., building walls).
/// Assumes collision occurs primarily on the XY plane.
pub fn calculate_aabb_collision_normal(player_pos: Vec3, aabb_min: Vec3, aabb_max: Vec3) -> Vec3 {
    let center = (aabb_min + aabb_max) * 0.5;
    let extents = (aabb_max - aabb_min) * 0.5;

    let mut local_pos = player_pos - center;
    local_pos.z = 0.0;

    let mut normal = Vec3::ZERO;

    let x_ratio = if extents.x > 0.0 { (local_pos.x.abs() / extents.x) } else { 0.0 };
    let y_ratio = if extents.y > 0.0 { (local_pos.y.abs() / extents.y) } else { 0.0 };

    if x_ratio >= y_ratio {
        normal.x = if local_pos.x >= 0.0 { 1.0 } else { -1.0 };
    } else {
        normal.y = if local_pos.y >= 0.0 { 1.0 } else { -1.0 };
    }

    normal
}

/// Calculates the normal vector for a blocked terrain tile collision.
pub fn calculate_terrain_tile_collision_normal(player_pos: Vec3, tile_center: Vec3, tile_size: f32) -> Vec3 {
    let half_size = tile_size * 0.5;
    let aabb_min = tile_center - Vec3::new(half_size, half_size, 0.0);
    let aabb_max = tile_center + Vec3::new(half_size, half_size, 0.0);
    calculate_aabb_collision_normal(player_pos, aabb_min, aabb_max)
}
