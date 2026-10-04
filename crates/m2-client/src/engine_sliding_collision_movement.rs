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
