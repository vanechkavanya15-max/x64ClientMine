
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use std::panic::{catch_unwind, AssertUnwindSafe};

#[derive(Debug, Clone, Copy)]
pub struct PickingActor {
    pub vid: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub radius: f32,
    pub height: f32,
    pub hp: i32,
    pub actor_type: u8,
}

static PICKING_ACTORS: OnceLock<RwLock<HashMap<u32, PickingActor>>> = OnceLock::new();

fn get_picking_actors() -> &'static RwLock<HashMap<u32, PickingActor>> {
    PICKING_ACTORS.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn add_picking_actor(actor: PickingActor) {
    let mut actors = get_picking_actors().write().unwrap();
    actors.insert(actor.vid, actor);
}

pub fn clear_picking_actors() {
    let mut actors = get_picking_actors().write().unwrap();
    actors.clear();
}

pub fn find_nearest_target(main_vid: u32, max_dist: f32, target_type: u8) -> Option<u32> {
    let actors = get_picking_actors().read().unwrap();
    
    let main_actor = match actors.get(&main_vid) {
        Some(a) => a,
        None => return None,
    };
    
    let mx = main_actor.x;
    let my = main_actor.y;
    
    let mut closest_vid = None;
    let mut min_dist_sq = max_dist * max_dist;
    
    for (&vid, actor) in actors.iter() {
        if vid == main_vid { continue; }
        if actor.hp <= 0 { continue; }
        if actor.actor_type != target_type { continue; }
        
        let dx = actor.x - mx;
        let dy = actor.y - my;
        let dist_sq = dx * dx + dy * dy;
        
        if dist_sq < min_dist_sq {
            min_dist_sq = dist_sq;
            closest_vid = Some(vid);
        }
    }
    
    closest_vid
}

pub fn ray_intersect_cylinder(
    ray_orig: (f32, f32, f32),
    ray_dir: (f32, f32, f32),
    cyl_pos: (f32, f32, f32),
    radius: f32,
    height: f32,
) -> Option<f32> {
    let dx = ray_dir.0;
    let dy = ray_dir.1;
    let dz = ray_dir.2;
    
    let ox = ray_orig.0 - cyl_pos.0;
    let oy = ray_orig.1 - cyl_pos.1;
    let oz = ray_orig.2 - cyl_pos.2;
    
    let a = dx * dx + dy * dy;
    let b = 2.0 * (ox * dx + oy * dy);
    let c = ox * ox + oy * oy - radius * radius;
    
    let mut t_closest = f32::MAX;
    let mut hit = false;
    
    // Check intersection with infinite cylinder Z-axis aligned
    if a.abs() > 1e-6 {
        let discriminant = b * b - 4.0 * a * c;
        if discriminant >= 0.0 {
            let sqrt_d = discriminant.sqrt();
            let t1 = (-b - sqrt_d) / (2.0 * a);
            let t2 = (-b + sqrt_d) / (2.0 * a);
            
            for t in [t1, t2].iter() {
                if *t >= 0.0 {
                    let z = oz + *t * dz;
                    if z >= 0.0 && z <= height {
                        if *t < t_closest {
                            t_closest = *t;
                            hit = true;
                        }
                    }
                }
            }
        }
    }
    
    // Check intersection with cylinder caps
    if dz.abs() > 1e-6 {
        // Bottom cap (Z = 0 relative to cyl_pos)
        let t_bottom = (0.0 - oz) / dz;
        if t_bottom >= 0.0 {
            let cx = ox + t_bottom * dx;
            let cy = oy + t_bottom * dy;
            if cx * cx + cy * cy <= radius * radius {
                if t_bottom < t_closest {
                    t_closest = t_bottom;
                    hit = true;
                }
            }
        }
        
        // Top cap (Z = height relative to cyl_pos)
        let t_top = (height - oz) / dz;
        if t_top >= 0.0 {
            let cx = ox + t_top * dx;
            let cy = oy + t_top * dy;
            if cx * cx + cy * cy <= radius * radius {
                if t_top < t_closest {
                    t_closest = t_top;
                    hit = true;
                }
            }
        }
    }
    
    if hit {
        Some(t_closest)
    } else {
        None
    }
}

#[no_mangle]
pub extern "C" fn m2_chr_mgr_find_nearest_target(main_vid: u32, max_dist: f32, target_type: u8) -> u32 {
    catch_unwind(AssertUnwindSafe(|| {
        find_nearest_target(main_vid, max_dist, target_type).unwrap_or(0)
    })).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn m2_actor_ray_pick(ray_x: f32, ray_y: f32, ray_z: f32, dir_x: f32, dir_y: f32, dir_z: f32) -> u32 {
    catch_unwind(AssertUnwindSafe(|| {
        let actors = get_picking_actors().read().unwrap();
        let ray_orig = (ray_x, ray_y, ray_z);
        let ray_dir = (dir_x, dir_y, dir_z);
        
        let mut closest_vid = 0;
        let mut min_t = f32::MAX;
        
        for (&vid, actor) in actors.iter() {
            if let Some(t) = ray_intersect_cylinder(
                ray_orig,
                ray_dir,
                (actor.x, actor.y, actor.z),
                actor.radius,
                actor.height
            ) {
                if t < min_t {
                    min_t = t;
                    closest_vid = vid;
                }
            }
        }
        
        closest_vid
    })).unwrap_or(0)
}

pub struct Ray {
    pub origin: [f32; 3],
    pub dir: [f32; 3],
}

pub trait Actor {
    fn vid(&self) -> u32;
    fn sphere_center(&self) -> [f32; 3];
    fn radius(&self) -> f32;
    fn on_selected(&mut self);
    fn on_unselected(&mut self);
}

pub fn intersect_defending_sphere(ray: &Ray, sphere_center: [f32; 3], radius: f32) -> bool {
    let oc = [
        ray.origin[0] - sphere_center[0],
        ray.origin[1] - sphere_center[1],
        ray.origin[2] - sphere_center[2],
    ];

    let a = ray.dir[0] * ray.dir[0] + ray.dir[1] * ray.dir[1] + ray.dir[2] * ray.dir[2];
    let b = 2.0 * (oc[0] * ray.dir[0] + oc[1] * ray.dir[1] + oc[2] * ray.dir[2]);
    let c = oc[0] * oc[0] + oc[1] * oc[1] + oc[2] * oc[2] - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return false;
    }

    let sqrt_discriminant = discriminant.sqrt();
    let t1 = (-b - sqrt_discriminant) / (2.0 * a);
    let t2 = (-b + sqrt_discriminant) / (2.0 * a);

    t1 >= 0.0 || t2 >= 0.0
}

fn calculate_distance_sq(p1: [f32; 3], p2: [f32; 3]) -> f32 {
    let dx = p1[0] - p2[0];
    let dy = p1[1] - p2[1];
    let dz = p1[2] - p2[2];
    dx * dx + dy * dy + dz * dz
}

pub struct ActorPicker {
    picked_vid: Option<u32>,
}

impl ActorPicker {
    pub fn new() -> Self {
        Self { picked_vid: None }
    }

    pub fn pick<'a, I>(
        &mut self,
        ray: &Ray,
        main_vid: u32,
        actors: I,
    ) -> Option<u32>
    where
        I: IntoIterator<Item = &'a mut dyn Actor>,
    {
        let mut closest_vid: Option<u32> = None;
        let mut min_distance_sq = f32::MAX;

        // Instead of doing multiple passes over the iterator, we can collect the actors as mutable references in a Vec, 
        // find the best one, and then iterate through the Vec again to call on_selected/on_unselected.
        let mut actors_vec: Vec<&mut dyn Actor> = actors.into_iter().collect();

        for actor in &actors_vec {
            if actor.vid() == main_vid {
                continue;
            }

            if intersect_defending_sphere(ray, actor.sphere_center(), actor.radius()) {
                let dist_sq = calculate_distance_sq(ray.origin, actor.sphere_center());
                if dist_sq < min_distance_sq {
                    min_distance_sq = dist_sq;
                    closest_vid = Some(actor.vid());
                }
            }
        }

        if self.picked_vid == closest_vid {
            return closest_vid;
        }

        for actor in &mut actors_vec {
            if Some(actor.vid()) == self.picked_vid {
                actor.on_unselected();
            }
            if Some(actor.vid()) == closest_vid {
                actor.on_selected();
            }
        }

        self.picked_vid = closest_vid;
        closest_vid
    }
}
