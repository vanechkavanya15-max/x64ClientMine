use m2_client_bridge::state::game_state::GLOBAL_GAME_STATE;
use m2_client_bridge::state::target::TARGET_MANAGER;
use std::ptr;

// Re-using the mathematical constants and struct definitions for OBB and DefendingSphere
use glam::{Vec3, Vec4, Mat4};

pub const HEADER_CG_TARGET: u16 = 0x0A01;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGTarget {
    pub header: u16,
    pub length: u16,
    pub dw_vid: u32,
}

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
    pub half_width: f32,
    pub half_depth: f32,
    pub height: f32,
    pub yaw: f32,
}

impl Obb {
    pub fn intersect(&self, ray: &Ray) -> bool {
        let mut tmin = 0.0f32;
        let mut tmax = f32::MAX;

        let yaw_sin = self.yaw.sin();
        let yaw_cos = self.yaw.cos();

        let axis_x = Vec3::new(yaw_cos, yaw_sin, 0.0);
        let axis_y = Vec3::new(-yaw_sin, yaw_cos, 0.0);
        let axis_z = Vec3::new(0.0, 0.0, 1.0);

        let extents = [
            self.half_width, 
            self.half_depth, 
            self.height / 2.0
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

pub fn pick_target(
    ray_origin: [f32; 3],
    ray_dir: [f32; 3],
    out_buf: &mut [u8],
) -> Result<u32, &'static str> {
    let state = GLOBAL_GAME_STATE.read().unwrap();
    let player_vid = state.get_player_vid();
    
    let ray = Ray {
        origin: ray_origin,
        dir: ray_dir,
    };
    
    let mut passing_actors = Vec::new();

    // Iterate over all actors in the game state
    for (&vid, actor) in state.actors.iter() {
        if vid == player_vid {
            continue; // Cannot target self using Pick()
        }
        
        let is_dead = actor.hp <= 0;
        
        if is_dead {
            // Approximation of OBB for testing dead bodies
            let obb = Obb {
                center: [actor.x, actor.y, actor.z],
                half_width: 50.0,
                half_depth: 50.0,
                height: 30.0,
                yaw: actor.rot * std::f32::consts::PI / 180.0, // Convert to radians
            };
            if obb.intersect(&ray) {
                passing_actors.push(vid);
            }
        } else {
            // DefendingSphere for alive bodies
            let sphere = DefendingSphere {
                center: [actor.x, actor.y, actor.z],
                radius: 100.0, // Standard approximation if we don't have dynamic bounds
            };
            if sphere.intersect(&ray) {
                passing_actors.push(vid);
            }
        }
    }
    
    // Sort actors according to CPythonCharacterManager::Pick algorithm
    passing_actors.sort_by(|&a_vid, &b_vid| {
        let a = state.actors.get(&a_vid).unwrap();
        let b = state.actors.get(&b_vid).unwrap();
        
        let a_dead = a.hp <= 0;
        let b_dead = b.hp <= 0;
        
        if a_dead != b_dead {
            return a_dead.cmp(&b_dead); // alive (false) comes before dead (true)
        }
        
        // Calculate squared distance to camera eye (approximated by ray origin)
        let dist_a = (a.x - ray.origin[0]).powi(2) + 
                     (a.y - ray.origin[1]).powi(2) + 
                     (a.z - ray.origin[2]).powi(2);
                     
        let dist_b = (b.x - ray.origin[0]).powi(2) + 
                     (b.y - ray.origin[1]).powi(2) + 
                     (b.z - ray.origin[2]).powi(2);
                     
        dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    // Fallback: Just take the first sorted one
    let picked_vid = passing_actors.first().copied();
    
    let size = std::mem::size_of::<TPacketCGTarget>();
    if out_buf.len() < size {
        return Err("Buffer is too small to serialize TPacketCGTarget");
    }

    if let Some(vid) = picked_vid {
        let actor = state.actors.get(&vid).unwrap();
        
        // Ustawienie SelectedTargetVID w TARGET_MANAGER
        TARGET_MANAGER.set_target(vid, actor.x, actor.y, actor.z, actor.hp > 0);
        
        let packet = TPacketCGTarget {
            header: HEADER_CG_TARGET,
            length: size as u16,
            dw_vid: vid,
        };
        
        unsafe {
            ptr::copy_nonoverlapping(
                &packet as *const _ as *const u8,
                out_buf.as_mut_ptr(),
                size,
            );
        }
        
        Ok(vid)
    } else {
        TARGET_MANAGER.clear_target();
        
        let packet = TPacketCGTarget {
            header: HEADER_CG_TARGET,
            length: size as u16,
            dw_vid: 0,
        };
        
        unsafe {
            ptr::copy_nonoverlapping(
                &packet as *const _ as *const u8,
                out_buf.as_mut_ptr(),
                size,
            );
        }
        
        Ok(0)
    }
}

/// FFI Export do wykorzystania przez C++ i bridge.
/// `out_buf` musi miec conajmniej 6 bajtow (wielkosc TPacketCGTarget).
/// Zwraca VID wybranego celu lub 0 w przypadku braku wyboru.
#[no_mangle]
pub unsafe extern "C" fn m2_actor_target_pick(
    ray_origin: *const [f32; 3],
    ray_dir: *const [f32; 3],
    out_buf: *mut u8,
    out_buf_len: usize,
) -> u32 {
    if ray_origin.is_null() || ray_dir.is_null() || out_buf.is_null() || out_buf_len < std::mem::size_of::<TPacketCGTarget>() {
        return 0;
    }
    
    let slice = std::slice::from_raw_parts_mut(out_buf, out_buf_len);
    match pick_target(*ray_origin, *ray_dir, slice) {
        Ok(vid) => vid,
        Err(_) => 0,
    }
}