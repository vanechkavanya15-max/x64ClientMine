use glam::Vec3;
use std::cmp::Ordering;

// A standalone robust implementation for Raycasting and Target Selection in pure Rust
// matching exactly CPythonCharacterManager::__NEW_Pick, __OLD_Pick, PickAll, etc.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray3D {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray3D {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn intersect_sphere(&self, center: Vec3, radius: f32) -> bool {
        let oc = self.origin - center;
        let b = oc.dot(self.direction);
        let c = oc.dot(oc) - radius * radius;
        let discriminant = b * b - c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_disc = discriminant.sqrt();
        let t1 = -b - sqrt_disc;
        let t2 = -b + sqrt_disc;

        t1 >= 0.0 || t2 >= 0.0
    }

    pub fn intersect_bounding_box(&self, min: Vec3, max: Vec3) -> bool {
        let inv_dir = Vec3::new(1.0 / self.direction.x, 1.0 / self.direction.y, 1.0 / self.direction.z);
        let t0 = (min - self.origin) * inv_dir;
        let t1 = (max - self.origin) * inv_dir;

        let tmin = t0.min(t1);
        let tmax = t0.max(t1);

        let t_enter = tmin.max_element();
        let t_exit = tmax.min_element();

        t_enter <= t_exit && t_exit >= 0.0
    }
}

pub trait TargetableInstance {
    fn get_vid(&self) -> u32;
    fn get_vnum(&self) -> u32;
    fn is_dead(&self) -> bool;
    
    #[allow(dead_code)]
    fn is_pc(&self) -> bool;
    
    fn can_pick_instance(&self) -> bool;
    fn is_movie_mode(&self) -> bool;
    
    // Bounds checking
    fn intersect_defending_sphere(&self, ray: &Ray3D) -> bool;
    fn intersect_bounding_box(&self, ray: &Ray3D) -> bool;
    
    // Positions
    fn get_distance_sq(&self, camera_eye: Vec3) -> f32;
    fn get_distance_to_instance(&self, other: &dyn TargetableInstance) -> f32;
    
    // UI Events
    fn on_selected(&mut self);
    fn on_unselected(&mut self);
}

pub struct ActorTargetSelectorV2 {
    picked_vid: Option<u32>,
}

impl Default for ActorTargetSelectorV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ActorTargetSelectorV2 {
    pub fn new() -> Self {
        Self { picked_vid: None }
    }

    #[allow(dead_code)]
    pub fn get_picked_vid(&self) -> Option<u32> {
        self.picked_vid
    }

    pub fn pick_new(
        &mut self,
        ray: &Ray3D,
        camera_eye: Vec3,
        main_vid: Option<u32>,
        alive_instances: &mut [&mut dyn TargetableInstance],
    ) -> Option<u32> {
        let mut picked_indices = Vec::new();

        for (i, inst) in alive_instances.iter().enumerate() {
            if inst.can_pick_instance() {
                if inst.is_dead() {
                    if inst.intersect_bounding_box(ray) {
                        picked_indices.push(i);
                    }
                } else {
                    if inst.intersect_defending_sphere(ray) {
                        picked_indices.push(i);
                    }
                }
            }
        }

        // Sort by CInstanceBase_SLessCameraDistance
        picked_indices.sort_by(|&a, &b| {
            let inst_a = &alive_instances[a];
            let inst_b = &alive_instances[b];

            let a_dead = inst_a.is_dead();
            let b_dead = inst_b.is_dead();

            if a_dead != b_dead {
                return if !a_dead && b_dead { Ordering::Less } else { Ordering::Greater };
            }

            let dist_a = inst_a.get_distance_sq(camera_eye);
            let dist_b = inst_b.get_distance_sq(camera_eye);
            
            dist_a.partial_cmp(&dist_b).unwrap_or(Ordering::Equal)
        });

        if let Some(m_vid) = main_vid {
            if let Some(main_idx) = alive_instances.iter().position(|inst| inst.get_vid() == m_vid) {
                if alive_instances[main_idx].is_movie_mode() {
                    if let Some(p_vid) = self.picked_vid {
                        if let Some(p_idx) = alive_instances.iter().position(|inst| inst.get_vid() == p_vid) {
                            alive_instances[p_idx].on_unselected();
                        }
                    }
                    self.picked_vid = None;
                    return None;
                }
            }
        }

        // Exact Check (IntersectBoundingBox)
        for &idx in &picked_indices {
            let inst = &mut alive_instances[idx];
            if Some(inst.get_vid()) != main_vid && inst.intersect_bounding_box(ray) {
                let target_vid = inst.get_vid();
                self.update_selection(target_vid, alive_instances);
                return Some(target_vid);
            }
        }

        // Order Check
        for &idx in &picked_indices {
            let inst = &mut alive_instances[idx];
            if Some(inst.get_vid()) != main_vid {
                let target_vid = inst.get_vid();
                self.update_selection(target_vid, alive_instances);
                return Some(target_vid);
            }
        }

        // Main Actor
        if let Some(m_vid) = main_vid {
            if let Some(main_idx) = alive_instances.iter().position(|inst| inst.get_vid() == m_vid) {
                if alive_instances[main_idx].can_pick_instance() && picked_indices.contains(&main_idx) {
                    self.update_selection(m_vid, alive_instances);
                    return Some(m_vid);
                }
            }
        }

        if let Some(p_vid) = self.picked_vid {
            if let Some(p_idx) = alive_instances.iter().position(|inst| inst.get_vid() == p_vid) {
                alive_instances[p_idx].on_unselected();
            }
            self.picked_vid = None;
        }

        None
    }

    pub fn pick_old(
        &mut self,
        ray: &Ray3D,
        main_vid: Option<u32>,
        alive_instances: &mut [&mut dyn TargetableInstance],
    ) -> Option<u32> {
        let mut found_vid = None;
        for i in 0..alive_instances.len() {
            if Some(alive_instances[i].get_vid()) == main_vid {
                continue;
            }
            if alive_instances[i].intersect_defending_sphere(ray) {
                found_vid = Some(alive_instances[i].get_vid());
                break;
            }
        }

        if let Some(v) = found_vid {
            self.update_selection(v, alive_instances);
            return Some(v);
        }

        if let Some(p_vid) = self.picked_vid {
            if let Some(p_idx) = alive_instances.iter().position(|inst| inst.get_vid() == p_vid) {
                alive_instances[p_idx].on_unselected();
            }
            self.picked_vid = None;
        }

        None
    }

    pub fn pick_all(
        ray: &Ray3D,
        alive_instances: &[&mut dyn TargetableInstance],
    ) -> Option<u32> {
        for inst in alive_instances {
            if inst.intersect_defending_sphere(ray) {
                return Some(inst.get_vid());
            }
        }
        None
    }

    #[allow(dead_code)]
    pub fn get_close_instance<'a>(
        target_vid: u32,
        alive_instances: &'a [&'a mut dyn TargetableInstance],
    ) -> Option<u32> {
        let mut min_distance = 10000.0f32;
        let mut close_vid = None;
        
        let target_idx = alive_instances.iter().position(|inst| inst.get_vid() == target_vid)?;
        
        for (i, inst) in alive_instances.iter().enumerate() {
            if i == target_idx {
                continue;
            }
            
            // CPythonNonPlayer::ON_CLICK_EVENT_BATTLE check mockup (assumes vnum > 0)
            if inst.get_vnum() == 0 {
                continue; // Not battle event type
            }
            
            let dist = alive_instances[target_idx].get_distance_to_instance(*inst);
            if dist < min_distance {
                min_distance = dist;
                close_vid = Some(inst.get_vid());
            }
        }
        
        close_vid
    }

    fn update_selection(&mut self, new_vid: u32, alive_instances: &mut [&mut dyn TargetableInstance]) {
        if self.picked_vid != Some(new_vid) {
            if let Some(p_vid) = self.picked_vid {
                if let Some(p_idx) = alive_instances.iter().position(|inst| inst.get_vid() == p_vid) {
                    alive_instances[p_idx].on_unselected();
                }
            }
            
            if let Some(n_idx) = alive_instances.iter().position(|inst| inst.get_vid() == new_vid) {
                alive_instances[n_idx].on_selected();
            }
            self.picked_vid = Some(new_vid);
        }
    }
}
