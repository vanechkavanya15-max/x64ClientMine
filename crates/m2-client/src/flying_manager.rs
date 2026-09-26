use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
pub struct FlyingData {
    pub pierce_count: u32,
}

#[derive(Debug, Clone)]
pub struct FlyingInstance {
    pub id: u32,
    pub shooter_vid: u32,
    pub target_vid: u32,
    pub current_pos: (f32, f32, f32),
    pub config: FlyingData,
    pub hit_targets: HashSet<u32>,
}

pub struct FlyingManager {
    pub instances: Vec<FlyingInstance>,
    next_id: u32,
}

impl FlyingManager {
    pub fn new() -> Self {
        Self {
            instances: Vec::new(),
            next_id: 1,
        }
    }

    pub fn spawn_projectile(
        &mut self,
        shooter_vid: u32,
        target_vid: u32,
        start_pos: (f32, f32, f32),
        config: FlyingData,
    ) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.instances.push(FlyingInstance {
            id,
            shooter_vid,
            target_vid,
            current_pos: start_pos,
            config,
            hit_targets: HashSet::new(),
        });
        id
    }

    pub fn update<F>(
        &mut self,
        dt: f32,
        actors: &HashMap<u32, (f32, f32, f32)>,
        mut on_hit: F,
    ) where
        F: FnMut(u32, u32, (f32, f32, f32)),
    {
        let speed = 1000.0;
        let collision_radius_sq = 50.0 * 50.0;

        let mut i = 0;
        while i < self.instances.len() {
            let mut is_destroyed = false;

            // Movement logic
            if let Some(&target_pos) = actors.get(&self.instances[i].target_vid) {
                let dx = target_pos.0 - self.instances[i].current_pos.0;
                let dy = target_pos.1 - self.instances[i].current_pos.1;
                let dz = target_pos.2 - self.instances[i].current_pos.2;

                let dist = (dx * dx + dy * dy + dz * dz).sqrt();
                if dist > 0.001 {
                    let move_dist = speed * dt;
                    if move_dist >= dist {
                        self.instances[i].current_pos = target_pos;
                    } else {
                        self.instances[i].current_pos.0 += (dx / dist) * move_dist;
                        self.instances[i].current_pos.1 += (dy / dist) * move_dist;
                        self.instances[i].current_pos.2 += (dz / dist) * move_dist;
                    }
                }
            }

            // Collision detection
            let mut hits_this_frame = Vec::new();
            for (&actor_vid, &actor_pos) in actors.iter() {
                if actor_vid == self.instances[i].shooter_vid {
                    continue;
                }

                if self.instances[i].hit_targets.contains(&actor_vid) {
                    continue;
                }

                let dx = actor_pos.0 - self.instances[i].current_pos.0;
                let dy = actor_pos.1 - self.instances[i].current_pos.1;
                let dz = actor_pos.2 - self.instances[i].current_pos.2;

                let dist_sq = dx * dx + dy * dy + dz * dz;
                if dist_sq <= collision_radius_sq {
                    hits_this_frame.push(actor_vid);
                }
            }

            for hit_vid in hits_this_frame {
                self.instances[i].hit_targets.insert(hit_vid);
                let hit_point = self.instances[i].current_pos;
                let shooter_vid = self.instances[i].shooter_vid;

                on_hit(shooter_vid, hit_vid, hit_point);

                if self.instances[i].config.pierce_count > 0 {
                    self.instances[i].config.pierce_count -= 1;
                } else {
                    is_destroyed = true;
                    break;
                }
            }

            if is_destroyed {
                self.instances.remove(i);
            } else {
                i += 1;
            }
        }
    }
}

impl Default for FlyingManager {
    fn default() -> Self {
        Self::new()
    }
}
