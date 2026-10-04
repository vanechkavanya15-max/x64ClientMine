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

/// Zgodne 1:1 z CActorInstance::__AdjustCollisionMovement z Metin2 C++.
/// Koryguje wektor ruchu gracza przy kolizji z innym aktorem lub przeszkoda cylindryczna,
/// realizujac plynne slizganie wzdluz obwodu kolizji.
pub fn adjust_actor_collision_movement(
    old_pos: Vec3,
    cur_pos: Vec3,
    obstacle_pos: Vec3,
    combined_radius: f32,
) -> Option<Vec3> {
    let diff = cur_pos - obstacle_pos;
    let diff_2d = Vec3::new(diff.x, diff.y, 0.0);
    let dist_sq = diff_2d.length_squared();

    if dist_sq >= combined_radius * combined_radius {
        return None; // Brak kolizji
    }

    let dist = dist_sq.sqrt();
    let normal = if dist > 1e-4 {
        diff_2d / dist
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };

    // Wektor ruchu miedzy klatkami
    let move_delta = cur_pos - old_pos;
    let dot = move_delta.x * normal.x + move_delta.y * normal.y;

    // Slizganie po stycznej: V_slide = V - (V dot N) * N
    let slide_x = move_delta.x - dot * normal.x;
    let slide_y = move_delta.y - dot * normal.y;

    // Przesuniecie gracza na bezpieczny promien
    let safe_x = obstacle_pos.x + normal.x * combined_radius + slide_x;
    let safe_y = obstacle_pos.y + normal.y * combined_radius + slide_y;

    Some(Vec3::new(safe_x, safe_y, cur_pos.z))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjust_actor_collision_movement_sliding() {
        let old_pos = Vec3::new(0.0, 0.0, 0.0);
        let cur_pos = Vec3::new(50.0, 50.0, 0.0);
        let obstacle = Vec3::new(60.0, 50.0, 0.0);
        let combined_radius = 50.0;

        let adjusted = adjust_actor_collision_movement(old_pos, cur_pos, obstacle, combined_radius);
        assert!(adjusted.is_some(), "Kolizja z przeszkoda musi zwrocic skorygowana pozycje");
        let new_pos = adjusted.unwrap();

        // Odleglosc od przeszkody nie moze byc mniejsza niz combined_radius
        let dist = (Vec3::new(new_pos.x, new_pos.y, 0.0) - Vec3::new(obstacle.x, obstacle.y, 0.0)).length();
        assert!(dist >= combined_radius - 0.01, "Pozycja po slizgu musi byc poza promien kolizji");
    }

    #[test]
    fn test_adjust_actor_collision_no_collision() {
        let old_pos = Vec3::new(0.0, 0.0, 0.0);
        let cur_pos = Vec3::new(10.0, 10.0, 0.0);
        let obstacle = Vec3::new(100.0, 100.0, 0.0);
        let combined_radius = 50.0;

        let adjusted = adjust_actor_collision_movement(old_pos, cur_pos, obstacle, combined_radius);
        assert!(adjusted.is_none(), "Brak kolizji powinien zwrocic None");
    }
}

