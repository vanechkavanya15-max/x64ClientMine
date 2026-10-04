use glam::Vec3;

pub fn adjust_collision_movement(
    cur_pos: Vec3,
    delta_move: &mut Vec3,
    normal_collision_plane: Vec3,
    slide_speed: f32,
) -> Vec3 {
    let mut v3_movement = *delta_move;
    if v3_movement.x == 0.0 && v3_movement.y == 0.0 && v3_movement.z == 0.0 {
        return cur_pos;
    }

    let mut move_length = v3_movement.length();
    if move_length > slide_speed {
        v3_movement *= slide_speed / move_length;
    }

    // Instead of zeroing velocity, project it onto the collision plane (tangent)
    // The vector tangent to the plane is: V - (V dot N) * N
    let dot = v3_movement.dot(normal_collision_plane);
    let slide_delta = v3_movement - normal_collision_plane * dot;

    v3_movement = slide_delta;
    let mut new_pos = cur_pos + slide_delta;

    if slide_delta.x != 0.0 || slide_delta.y != 0.0 || slide_delta.z != 0.0 {
        move_length = v3_movement.length();
        if move_length > slide_speed {
            v3_movement *= slide_speed / move_length;
            new_pos = cur_pos + v3_movement;
        }
    }

    *delta_move = v3_movement;

    new_pos
}
