#[derive(Debug, Clone, PartialEq)]
pub struct RaycastResult {
    pub hit: bool,
    pub hit_point: (u16, u16),
    pub distance: f32,
    pub traversed_cells: Vec<(u16, u16)>,
}

pub fn terrain_raycast_2d(
    x0: u16,
    y0: u16,
    x1: u16,
    y1: u16,
    is_blocked: &impl Fn(u16, u16) -> bool,
) -> RaycastResult {
    let mut traversed_cells = Vec::new();

    let dx = x1 as f32 - x0 as f32;
    let dy = y1 as f32 - y0 as f32;

    if dx == 0.0 && dy == 0.0 {
        traversed_cells.push((x0, y0));
        let hit = is_blocked(x0, y0);
        return RaycastResult {
            hit,
            hit_point: if hit { (x0, y0) } else { (0, 0) },
            distance: 0.0,
            traversed_cells,
        };
    }

    let step_x = if dx > 0.0 { 1 } else if dx < 0.0 { -1 } else { 0 };
    let step_y = if dy > 0.0 { 1 } else if dy < 0.0 { -1 } else { 0 };

    let delta_x = if dx == 0.0 { f32::INFINITY } else { (1.0 / dx).abs() };
    let delta_y = if dy == 0.0 { f32::INFINITY } else { (1.0 / dy).abs() };

    let mut t_max_x = if dx == 0.0 { f32::INFINITY } else { 0.5 * delta_x };
    let mut t_max_y = if dy == 0.0 { f32::INFINITY } else { 0.5 * delta_y };

    let mut current_x = x0 as i32;
    let mut current_y = y0 as i32;

    let end_x = x1 as i32;
    let end_y = y1 as i32;

    loop {
        if current_x < 0 || current_x > u16::MAX as i32 || current_y < 0 || current_y > u16::MAX as i32 {
            break;
        }
        
        let cx = current_x as u16;
        let cy = current_y as u16;
        traversed_cells.push((cx, cy));

        if is_blocked(cx, cy) {
            let dist = ((current_x - x0 as i32) as f32).hypot((current_y - y0 as i32) as f32);
            return RaycastResult {
                hit: true,
                hit_point: (cx, cy),
                distance: dist,
                traversed_cells,
            };
        }

        if current_x == end_x && current_y == end_y {
            break;
        }

        if t_max_x < t_max_y {
            t_max_x += delta_x;
            current_x += step_x;
        } else {
            t_max_y += delta_y;
            current_y += step_y;
        }
    }

    let total_dist = dx.hypot(dy);
    RaycastResult {
        hit: false,
        hit_point: (0, 0),
        distance: total_dist,
        traversed_cells,
    }
}
