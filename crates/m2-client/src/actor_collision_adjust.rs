const BODY_RADIUS: f32 = 50.0;
const MIN_DISTANCE: f32 = 2.0 * BODY_RADIUS;

pub fn adjust_collision(actor_pos: (f32, f32), other_actor_pos: (f32, f32)) -> Option<(f32, f32)> {
    let dx = actor_pos.0 - other_actor_pos.0;
    let dy = actor_pos.1 - other_actor_pos.1;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance >= MIN_DISTANCE {
        return None;
    }

    let push_distance = MIN_DISTANCE - distance;
    
    let (nx, ny) = if distance == 0.0 {
        (1.0, 0.0) // Default push direction if exactly same position
    } else {
        (dx / distance, dy / distance)
    };

    let new_x = actor_pos.0 + nx * push_distance;
    let new_y = actor_pos.1 + ny * push_distance;

    Some((new_x, new_y))
}
