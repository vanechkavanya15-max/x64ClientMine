use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    x: u16,
    y: u16,
    f_score: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.f_score.cmp(&self.f_score)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct AStarPathfinder;

impl AStarPathfinder {
    pub fn new() -> Self {
        Self
    }

    fn heuristic(x1: u16, y1: u16, x2: u16, y2: u16) -> u32 {
        let dx = (x1 as i32 - x2 as i32).abs() as u32;
        let dy = (y1 as i32 - y2 as i32).abs() as u32;
        // Octile distance
        let min_d = dx.min(dy);
        10 * (dx + dy) - 6 * min_d // equivalent to 10*(dx+dy) + (14-2*10)*min(dx,dy)
    }

    fn is_raycast_clear(
        start: (u16, u16),
        end: (u16, u16),
        is_blocked: &impl Fn(u16, u16) -> bool,
    ) -> bool {
        let (mut x, mut y) = (start.0 as i32, start.1 as i32);
        let x2 = end.0 as i32;
        let y2 = end.1 as i32;

        let dx = (x2 - x).abs();
        let dy = (y2 - y).abs();
        let sx = if x < x2 { 1 } else { -1 };
        let sy = if y < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        while x != x2 || y != y2 {
            if is_blocked(x as u16, y as u16) {
                return false;
            }

            let e2 = 2 * err;
            let mut step_x = false;
            let mut step_y = false;
            
            if e2 > -dy {
                err -= dy;
                x += sx;
                step_x = true;
            }
            if e2 < dx {
                err += dx;
                y += sy;
                step_y = true;
            }

            // Check corner cutting for diagonal raycast step
            if step_x && step_y {
                let px = (x - sx) as u16;
                let py = (y - sy) as u16;
                let nx = x as u16;
                let ny = y as u16;
                
                if is_blocked(px, ny) || is_blocked(nx, py) {
                    return false;
                }
            }
        }
        
        !is_blocked(x2 as u16, y2 as u16)
    }

    pub fn smooth_path(
        &self,
        path: Vec<(u16, u16)>,
        is_blocked: impl Fn(u16, u16) -> bool,
    ) -> Vec<(u16, u16)> {
        if path.len() <= 2 {
            return path;
        }

        let mut smoothed_path = Vec::new();
        smoothed_path.push(path[0]);
        
        let mut current_idx = 0;
        
        while current_idx < path.len() - 1 {
            let mut furthest_visible_idx = current_idx + 1;
            
            for check_idx in (current_idx + 2)..path.len() {
                if Self::is_raycast_clear(path[current_idx], path[check_idx], &is_blocked) {
                    furthest_visible_idx = check_idx;
                }
            }
            
            smoothed_path.push(path[furthest_visible_idx]);
            current_idx = furthest_visible_idx;
        }

        smoothed_path
    }

    pub fn find_path(
        &self,
        start: (u16, u16),
        goal: (u16, u16),
        is_blocked: impl Fn(u16, u16) -> bool,
        apply_smoothing: bool,
    ) -> Option<Vec<(u16, u16)>> {
        if is_blocked(start.0, start.1) || is_blocked(goal.0, goal.1) {
            return None;
        }

        if start == goal {
            return Some(vec![start]);
        }

        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<(u16, u16), (u16, u16)> = HashMap::new();
        let mut g_score: HashMap<(u16, u16), u32> = HashMap::new();

        g_score.insert(start, 0);
        open_set.push(Node {
            x: start.0,
            y: start.1,
            f_score: Self::heuristic(start.0, start.1, goal.0, goal.1),
        });

        // 8 directions: 4 cardinal, 4 diagonal
        let dirs: [(i32, i32, u32); 8] = [
            (0, -1, 10), (0, 1, 10), (-1, 0, 10), (1, 0, 10), // Cardinal
            (-1, -1, 14), (1, -1, 14), (-1, 1, 14), (1, 1, 14) // Diagonal
        ];

        while let Some(Node { x, y, f_score: _ }) = open_set.pop() {
            if (x, y) == goal {
                // Reconstruct path
                let mut path = vec![goal];
                let mut curr = goal;
                while let Some(&prev) = came_from.get(&curr) {
                    path.push(prev);
                    curr = prev;
                }
                path.reverse();
                
                if apply_smoothing {
                    return Some(self.smooth_path(path, is_blocked));
                } else {
                    return Some(path);
                }
            }

            let current_g = *g_score.get(&(x, y)).unwrap_or(&u32::MAX);

            for (dx, dy, cost) in dirs.iter() {
                let nx_i32 = x as i32 + dx;
                let ny_i32 = y as i32 + dy;

                if nx_i32 < 0 || ny_i32 < 0 || nx_i32 > u16::MAX as i32 || ny_i32 > u16::MAX as i32 {
                    continue;
                }

                let nx = nx_i32 as u16;
                let ny = ny_i32 as u16;

                if is_blocked(nx, ny) {
                    continue;
                }

                // Corner cutting prevention for diagonal moves
                if *cost == 14 {
                    let mut cut_corner = false;
                    // Check horizontal neighbor
                    if is_blocked(nx, y) {
                        cut_corner = true;
                    }
                    // Check vertical neighbor
                    if is_blocked(x, ny) {
                        cut_corner = true;
                    }
                    if cut_corner {
                        continue;
                    }
                }

                let tentative_g = current_g.saturating_add(*cost);
                let neighbor_g = *g_score.get(&(nx, ny)).unwrap_or(&u32::MAX);

                if tentative_g < neighbor_g {
                    came_from.insert((nx, ny), (x, y));
                    g_score.insert((nx, ny), tentative_g);
                    let f_score = tentative_g.saturating_add(Self::heuristic(nx, ny, goal.0, goal.1));
                    open_set.push(Node { x: nx, y: ny, f_score });
                }
            }
        }

        None
    }
}
