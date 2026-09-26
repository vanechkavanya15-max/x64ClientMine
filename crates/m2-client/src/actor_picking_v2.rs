use glam::{Mat4, Vec3, Vec4};

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: [f32; 3],
    pub dir: [f32; 3],
}

#[derive(Debug, Clone)]
pub struct DefendingSphere {
    pub center: [f32; 3],
    pub radius: f32,
}

impl DefendingSphere {
    pub fn intersect(&self, ray: &Ray) -> bool {
        let oc = [
            ray.origin[0] - self.center[0],
            ray.origin[1] - self.center[1],
            ray.origin[2] - self.center[2],
        ];

        let a = ray.dir[0] * ray.dir[0] + ray.dir[1] * ray.dir[1] + ray.dir[2] * ray.dir[2];
        let b = 2.0 * (oc[0] * ray.dir[0] + oc[1] * ray.dir[1] + oc[2] * ray.dir[2]);
        let c = oc[0] * oc[0] + oc[1] * oc[1] + oc[2] * oc[2] - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);

        t1 >= 0.0 || t2 >= 0.0
    }
}

#[derive(Debug, Clone)]
pub struct Obb {
    pub center: [f32; 3],
    pub half_width: f32, // X extent
    pub half_depth: f32, // Y extent
    pub height: f32,     // Z extent (0 to height)
    pub yaw: f32,        // rotation in radians around Z axis
}

impl Obb {
    pub fn intersect(&self, ray: &Ray) -> bool {
        let mut tmin = 0.0f32;
        let mut tmax = f32::MAX;

        let yaw_sin = self.yaw.sin();
        let yaw_cos = self.yaw.cos();

        // Obb axes
        let axis_x = Vec3::new(yaw_cos, yaw_sin, 0.0);
        let axis_y = Vec3::new(-yaw_sin, yaw_cos, 0.0);
        let axis_z = Vec3::new(0.0, 0.0, 1.0); // Up axis

        // Obb min/max extents in local space
        let extents = [
            self.half_width, 
            self.half_depth, 
            self.height / 2.0 // shift center to middle of height
        ];

        let origin = Vec3::from_array(ray.origin);
        let dir = Vec3::from_array(ray.dir);
        let center = Vec3::new(self.center[0], self.center[1], self.center[2] + extents[2]);

        let p = center - origin;

        let axes = [axis_x, axis_y, axis_z];

        for i in 0..3 {
            let e = axes[i].dot(p);
            let f = axes[i].dot(dir);

            if f.abs() > 1e-6 {
                let mut t1 = (e + extents[i]) / f;
                let mut t2 = (e - extents[i]) / f;

                if t1 > t2 {
                    std::mem::swap(&mut t1, &mut t2);
                }

                if t1 > tmin {
                    tmin = t1;
                }
                if t2 < tmax {
                    tmax = t2;
                }
                
                if tmin > tmax {
                    return false;
                }
                if tmax < 0.0 {
                    return false;
                }
            } else if -e - extents[i] > 0.0 || -e + extents[i] < 0.0 {
                return false;
            }
        }

        tmin > 0.0 || tmax > 0.0
    }
}

pub fn project_3d_to_2d(
    pos_3d: [f32; 3],
    vp_matrix: &Mat4,
    screen_width: f32,
    screen_height: f32,
) -> Option<[f32; 2]> {
    let vec4 = Vec4::new(pos_3d[0], pos_3d[1], pos_3d[2], 1.0);
    let clip_space = *vp_matrix * vec4;

    if clip_space.z < 0.0 {
        return None; // Behind camera
    }

    let ndc_x = clip_space.x / clip_space.w;
    let ndc_y = clip_space.y / clip_space.w;

    // Convert NDC to screen space
    let screen_x = (ndc_x + 1.0) * 0.5 * screen_width;
    let screen_y = (1.0 - ndc_y) * 0.5 * screen_height;

    Some([screen_x, screen_y])
}

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

#[derive(Debug, Clone)]
pub struct PickingActorDataV2 {
    pub vid: u32,
    pub is_main: bool,
    pub is_dead: bool,
    pub can_pick: bool,
    pub center: [f32; 3], // for distance calc
    pub defending_sphere: DefendingSphere,
    pub obb: Obb,
    pub is_selected: bool,
}

static PICKING_ACTORS_V2: OnceLock<RwLock<HashMap<u32, PickingActorDataV2>>> = OnceLock::new();

pub fn get_picking_actors_v2() -> &'static RwLock<HashMap<u32, PickingActorDataV2>> {
    PICKING_ACTORS_V2.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn add_picking_actor_v2(actor: PickingActorDataV2) {
    let mut actors = get_picking_actors_v2().write().unwrap();
    actors.insert(actor.vid, actor);
}

pub fn remove_picking_actor_v2(vid: u32) {
    let mut actors = get_picking_actors_v2().write().unwrap();
    actors.remove(&vid);
}

pub fn clear_picking_actors_v2() {
    let mut actors = get_picking_actors_v2().write().unwrap();
    actors.clear();
}

pub struct ActorPickerV2 {
    pub picked_vid: Option<u32>,
}

impl ActorPickerV2 {
    pub fn new() -> Self {
        Self { picked_vid: None }
    }

    pub fn pick(&mut self, ray: &Ray, eye_pos: [f32; 3]) -> Option<u32> {
        let mut actors = get_picking_actors_v2().write().unwrap();
        
        let mut passing_actors: Vec<u32> = Vec::new();
        
        for (&vid, actor) in actors.iter() {
            if !actor.can_pick {
                continue;
            }
            
            if actor.is_dead {
                if actor.obb.intersect(ray) {
                    passing_actors.push(vid);
                }
            } else {
                if actor.defending_sphere.intersect(ray) {
                    passing_actors.push(vid);
                }
            }
        }
        
        passing_actors.sort_by(|&a_vid, &b_vid| {
            let a = actors.get(&a_vid).unwrap();
            let b = actors.get(&b_vid).unwrap();
            
            if a.is_dead != b.is_dead {
                return a.is_dead.cmp(&b.is_dead); // false (alive) comes before true (dead)
            }
            
            let dist_a = (a.center[0] - eye_pos[0]).powi(2) + 
                         (a.center[1] - eye_pos[1]).powi(2) + 
                         (a.center[2] - eye_pos[2]).powi(2);
                         
            let dist_b = (b.center[0] - eye_pos[0]).powi(2) + 
                         (b.center[1] - eye_pos[1]).powi(2) + 
                         (b.center[2] - eye_pos[2]).powi(2);
                         
            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        let mut new_picked_vid = None;
        
        // Precise pass
        for &vid in passing_actors.iter() {
            let actor = actors.get(&vid).unwrap();
            if !actor.is_main && actor.obb.intersect(ray) {
                new_picked_vid = Some(vid);
                break;
            }
        }
        
        // Fallback pass (if precision fails, pick in order)
        if new_picked_vid.is_none() {
            for &vid in passing_actors.iter() {
                let actor = actors.get(&vid).unwrap();
                if !actor.is_main {
                    new_picked_vid = Some(vid);
                    break;
                }
            }
        }
        
        // Main actor fallback if nothing else picked
        if new_picked_vid.is_none() {
            if let Some((&main_vid, _)) = actors.iter().find(|(_, a)| a.is_main) {
                if passing_actors.contains(&main_vid) {
                    new_picked_vid = Some(main_vid);
                }
            }
        }
        
        if self.picked_vid != new_picked_vid {
            if let Some(old_vid) = self.picked_vid {
                if let Some(old_actor) = actors.get_mut(&old_vid) {
                    old_actor.is_selected = false;
                }
            }
            
            if let Some(new_vid) = new_picked_vid {
                if let Some(new_actor) = actors.get_mut(&new_vid) {
                    new_actor.is_selected = true;
                }
            }
            
            self.picked_vid = new_picked_vid;
        }
        
        new_picked_vid
    }
}

// Global picker instance for FFI
static GLOBAL_PICKER_V2: OnceLock<RwLock<ActorPickerV2>> = OnceLock::new();

pub fn get_global_picker_v2() -> &'static RwLock<ActorPickerV2> {
    GLOBAL_PICKER_V2.get_or_init(|| RwLock::new(ActorPickerV2::new()))
}

#[no_mangle]
pub extern "C" fn m2_actor_pick_obb(ray_orig: *const [f32; 3], ray_dir: *const [f32; 3]) -> u32 {
    if ray_orig.is_null() || ray_dir.is_null() { return 0; }
    let mut picker = get_global_picker_v2().write().unwrap();
    let ray = Ray { origin: unsafe { *ray_orig }, dir: unsafe { *ray_dir } };
    
    // eye_pos is approximately ray_orig for the sorting logic
    picker.pick(&ray, unsafe { *ray_orig }).unwrap_or(0)
}
