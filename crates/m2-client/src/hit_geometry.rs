use std::f32::consts::PI;

pub fn test_actor_clash(
    attacker_x: f32,
    attacker_y: f32,
    victim_x: f32,
    victim_y: f32,
    rot_deg: f32,
    hit_angle_limit_deg: f32,
    weapon_reach_base: f32,
    reach_scale: f32,
    victim_is_huge: bool,
    victim_collision_radius: f32,
) -> bool {
    let dx = victim_x - attacker_x;
    let dy = victim_y - attacker_y;
    
    let dist_2d = (dx * dx + dy * dy).sqrt();
    
    let mut allowed_reach = weapon_reach_base * reach_scale;
    if victim_is_huge {
        allowed_reach += victim_collision_radius;
    }

    if dist_2d > allowed_reach {
        return false;
    }

    if dist_2d > 0.0 {
        let dir_v_x = dx / dist_2d;
        let dir_v_y = dy / dist_2d;

        let rot_rad = rot_deg * PI / 180.0;
        let look_x = rot_rad.sin();
        let look_y = rot_rad.cos();

        let dot = look_x * dir_v_x + look_y * dir_v_y;

        let limit_rad = hit_angle_limit_deg * PI / 180.0;
        if dot < limit_rad.cos() {
            return false;
        }
    }

    true
}
