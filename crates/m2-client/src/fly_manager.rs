#[derive(Debug, Clone, PartialEq)]
pub struct FlyProjectile {
    pub id: u32,
    pub shooter_vid: u32,
    pub target_vid: u32,
    pub cur_pos: (f32, f32, f32),
    pub target_pos: (f32, f32, f32),
    pub speed: f32,
    pub has_hit: bool,
}

#[derive(Debug, Default)]
pub struct FlyManager {
    projectiles: Vec<FlyProjectile>,
}

impl FlyManager {
    pub fn new() -> Self {
        Self {
            projectiles: Vec::new(),
        }
    }

    pub fn add_projectile(&mut self, projectile: FlyProjectile) {
        self.projectiles.push(projectile);
    }

    pub fn update_target(&mut self, id: u32, new_target_pos: (f32, f32, f32)) {
        if let Some(proj) = self.projectiles.iter_mut().find(|p| p.id == id) {
            proj.target_pos = new_target_pos;
        }
    }

    pub fn update<F>(&mut self, delta_time: f32, mut hit_callback: F)
    where
        F: FnMut(u32, u32, u32),
    {
        for proj in &mut self.projectiles {
            if proj.has_hit {
                continue;
            }

            let dx = proj.target_pos.0 - proj.cur_pos.0;
            let dy = proj.target_pos.1 - proj.cur_pos.1;
            let dz = proj.target_pos.2 - proj.cur_pos.2;

            let distance_sq = dx * dx + dy * dy + dz * dz;
            let distance = distance_sq.sqrt();

            if distance < 20.0 {
                proj.has_hit = true;
                hit_callback(proj.id, proj.shooter_vid, proj.target_vid);
                continue;
            }

            let move_dist = proj.speed * delta_time;

            if move_dist >= distance || distance - move_dist < 20.0 {
                proj.cur_pos = proj.target_pos;
                proj.has_hit = true;
                hit_callback(proj.id, proj.shooter_vid, proj.target_vid);
            } else {
                let dir_x = dx / distance;
                let dir_y = dy / distance;
                let dir_z = dz / distance;

                proj.cur_pos.0 += dir_x * move_dist;
                proj.cur_pos.1 += dir_y * move_dist;
                proj.cur_pos.2 += dir_z * move_dist;
            }
        }

        self.projectiles.retain(|p| !p.has_hit);
    }

    pub fn get_projectile(&self, id: u32) -> Option<&FlyProjectile> {
        self.projectiles.iter().find(|p| p.id == id)
    }

    pub fn projectile_count(&self) -> usize {
        self.projectiles.len()
    }
}
