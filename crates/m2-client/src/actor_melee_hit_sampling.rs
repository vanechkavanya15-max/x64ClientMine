use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DynamicSphereInstance {
    pub v3_position: Vec3,
    pub v3_last_position: Vec3,
    pub f_radius: f32,
}

impl DynamicSphereInstance {
    pub fn new(v3_last_position: Vec3, v3_position: Vec3, f_radius: f32) -> Self {
        Self {
            v3_position,
            v3_last_position,
            f_radius,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HitDataId(pub u32);

pub struct HitData {
    pub id: HitDataId,
    pub f_attack_start_time: f32,
    pub f_attack_end_time: f32,
    pub map_hit_position: BTreeMap<i32, DynamicSphereInstance>, // key is time * 1000 to use integer keys
}

pub struct AttackData {
    pub f_invisible_time: f32,
    pub i_hit_limit_count: usize,
    pub i_motion_type: MotionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionType {
    Normal,
    Combo,
    Skill,
}

pub type HittedInstanceMap = HashMap<u32, f32>; // Target ID -> Time when invincibility expires
pub type HitDataMap = HashMap<HitDataId, HittedInstanceMap>;

// Line segment intersection logic adapted from GameUtil.cpp (IntersectLineSegments and DetectCollisionDynamicZCylinderVSDynamicZCylinder)
pub fn intersect_line_segments_2d(a1: Vec3, a2: Vec3, b1: Vec3, b2: Vec3) -> (Vec3, Vec3) {
    let epsilon = 1e-6;
    let la = Vec3::new(a2.x - a1.x, a2.y - a1.y, 0.0);
    let lb = Vec3::new(b2.x - b1.x, b2.y - b1.y, 0.0);
    
    let l11 = la.x * la.x + la.y * la.y;
    let l22 = lb.x * lb.x + lb.y * lb.y;
    
    let mut s = 0.0;
    let mut t = 0.0;
    
    if l11 < epsilon && l22 < epsilon {
        // Both are points
    } else if l11 < epsilon {
        // Line A is degenerate
        let dot = (a1.x - b1.x) * lb.x + (a1.y - b1.y) * lb.y;
        t = (dot / l22).clamp(0.0, 1.0);
    } else if l22 < epsilon {
        // Line B is degenerate
        let dot = (b1.x - a1.x) * la.x + (b1.y - a1.y) * la.y;
        s = (dot / l11).clamp(0.0, 1.0);
    } else {
        // General case
        let l12 = la.x * lb.x + la.y * lb.y;
        let det = l11 * l22 - l12 * l12;
        let ra = a1.x - b1.x;
        let rb = a1.y - b1.y;
        let r1 = ra * la.x + rb * la.y;
        let r2 = ra * lb.x + rb * lb.y;
        
        if det.abs() > epsilon {
            s = (r2 * l12 - r1 * l22) / det;
            t = (r2 * l11 - r1 * l12) / det;
            
            s = s.clamp(0.0, 1.0);
            t = t.clamp(0.0, 1.0);
            
            // Refine after clamp
            if s == 0.0 || s == 1.0 {
                let dot = (a1.x + s * la.x - b1.x) * lb.x + (a1.y + s * la.y - b1.y) * lb.y;
                t = (dot / l22).clamp(0.0, 1.0);
            }
            if t == 0.0 || t == 1.0 {
                let dot = (b1.x + t * lb.x - a1.x) * la.x + (b1.y + t * lb.y - a1.y) * la.y;
                s = (dot / l11).clamp(0.0, 1.0);
            }
        } else {
            // Parallel lines
            let dot = (b1.x - a1.x) * la.x + (b1.y - a1.y) * la.y;
            s = (dot / l11).clamp(0.0, 1.0);
            let dot2 = (a1.x + s * la.x - b1.x) * lb.x + (a1.y + s * la.y - b1.y) * lb.y;
            t = (dot2 / l22).clamp(0.0, 1.0);
        }
    }
    
    let out_a = Vec3::new(a1.x + s * la.x, a1.y + s * la.y, 0.0);
    let out_b = Vec3::new(b1.x + t * lb.x, b1.y + t * lb.y, 0.0);
    
    (out_a, out_b)
}

pub fn detect_collision_dynamic_z_cylinder_vs_dynamic_z_cylinder(c_r_sphere1: &DynamicSphereInstance, c_r_sphere2: &DynamicSphereInstance) -> bool {
    let mut c_r_cylinder1 = *c_r_sphere1;
    let mut c_r_cylinder2 = *c_r_sphere2;
    
    c_r_cylinder1.v3_position.z = 0.0;
    c_r_cylinder1.v3_last_position.z = 0.0;
    c_r_cylinder2.v3_position.z = 0.0;
    c_r_cylinder2.v3_last_position.z = 0.0;

    let r = c_r_cylinder1.f_radius + c_r_cylinder2.f_radius;
    let rsq = r * r;

    let mut mi1 = c_r_cylinder1.v3_last_position;
    let mut mi2 = c_r_cylinder1.v3_position;
    let mut mi3 = c_r_cylinder2.v3_last_position;
    let mut mi4 = c_r_cylinder2.v3_position;

    if mi1.x > mi2.x { std::mem::swap(&mut mi1.x, &mut mi2.x); }
    if mi1.y > mi2.y { std::mem::swap(&mut mi1.y, &mut mi2.y); }
    if mi3.x > mi4.x { std::mem::swap(&mut mi3.x, &mut mi4.x); }
    if mi3.y > mi4.y { std::mem::swap(&mut mi3.y, &mut mi4.y); }

    mi1.x -= c_r_cylinder1.f_radius; mi1.y -= c_r_cylinder1.f_radius;
    mi2.x += c_r_cylinder1.f_radius; mi2.y += c_r_cylinder1.f_radius;
    mi3.x -= c_r_cylinder2.f_radius; mi3.y -= c_r_cylinder2.f_radius;
    mi4.x += c_r_cylinder2.f_radius; mi4.y += c_r_cylinder2.f_radius;

    if mi4.x < mi1.x || mi2.x < mi3.x { return false; }
    if mi4.y < mi1.y || mi2.y < mi3.y { return false; }

    let (v_a, v_b) = intersect_line_segments_2d(
        c_r_cylinder1.v3_last_position,
        c_r_cylinder1.v3_position,
        c_r_cylinder2.v3_last_position,
        c_r_cylinder2.v3_position
    );

    let dx = v_a.x - v_b.x;
    let dy = v_a.y - v_b.y;
    let dist_sq = dx * dx + dy * dy;

    dist_sq <= rsq
}

pub struct CollisionResult {
    pub victim_id: u32,
    pub hit_position: Vec3,
}

#[allow(clippy::too_many_arguments)]
pub fn process_melee_hit_sampling(
    attacker_pos: Vec3,
    attacker_rot_deg: f32,
    attacker_reach_scale: f32,
    attack_radius: f32,
    motion_time: f32,
    elapsed_second: f32,
    local_time: f32,
    attack_data: &AttackData,
    hit_data_list: &[HitData],
    hit_data_map: &mut HitDataMap,
    victims: &[(u32, Vec3, Vec<DynamicSphereInstance>)],
) -> Vec<CollisionResult> {
    let mut results = Vec::new();
    let c = attacker_rot_deg.to_radians().cos();
    let s = attacker_rot_deg.to_radians().sin();

    for hit_data in hit_data_list {
        let t_min = ((motion_time - elapsed_second) * 1000.0) as i32;
        let t_max = (motion_time * 1000.0) as i32;

        let range = hit_data.map_hit_position.range(t_min..=t_max);

        for (_, dsi_src) in range {
            let mut dsi = *dsi_src;
            dsi.f_radius = attack_radius;

            // Transform based on reach scale and rotation
            let mut v3_src_dir_x = dsi_src.v3_position.x - dsi_src.v3_last_position.x;
            let mut v3_src_dir_y = dsi_src.v3_position.y - dsi_src.v3_last_position.y;
            
            v3_src_dir_x *= attacker_reach_scale;
            v3_src_dir_y *= attacker_reach_scale;

            let v3_src_pos_x = dsi_src.v3_last_position.x + v3_src_dir_x;
            let v3_src_pos_y = dsi_src.v3_last_position.y + v3_src_dir_y;

            dsi.v3_position.x = v3_src_pos_x * c - v3_src_pos_y * s + attacker_pos.x;
            dsi.v3_position.y = v3_src_pos_x * s + v3_src_pos_y * c + attacker_pos.y;
            dsi.v3_position.z = dsi_src.v3_position.z + attacker_pos.z;

            let v3_src_last_x = dsi_src.v3_last_position.x;
            let v3_src_last_y = dsi_src.v3_last_position.y;

            dsi.v3_last_position.x = v3_src_last_x * c - v3_src_last_y * s + attacker_pos.x;
            dsi.v3_last_position.y = v3_src_last_x * s + v3_src_last_y * c + attacker_pos.y;
            dsi.v3_last_position.z = dsi_src.v3_last_position.z + attacker_pos.z;

            for (victim_id, victim_pos, victim_spheres) in victims {
                let hitted_map = hit_data_map.entry(hit_data.id.clone()).or_insert_with(HashMap::new);

                if let Some(exp_time) = hitted_map.get(victim_id) {
                    if attack_data.i_motion_type == MotionType::Combo || *exp_time > local_time {
                        continue;
                    }
                }

                for sub in victim_spheres {
                    if detect_collision_dynamic_z_cylinder_vs_dynamic_z_cylinder(&dsi, sub) {
                        let i_current_hit_count = hitted_map.len();
                        
                        let max_hit_count = if attack_data.i_motion_type == MotionType::Normal || attack_data.i_motion_type == MotionType::Combo {
                            16
                        } else {
                            attack_data.i_hit_limit_count
                        };

                        if i_current_hit_count >= max_hit_count {
                            // Max hit limit reached, skip
                            continue;
                        }

                        hitted_map.insert(*victim_id, local_time + attack_data.f_invisible_time);

                        let hit_position = Vec3::new(
                            (attacker_pos.x + victim_pos.x) * 0.5,
                            (attacker_pos.y + victim_pos.y) * 0.5,
                            (attacker_pos.z + victim_pos.z) * 0.5,
                        );

                        results.push(CollisionResult {
                            victim_id: *victim_id,
                            hit_position,
                        });
                        
                        // Once victim is hit in this segment, move to next victim to avoid multi-hitting same victim per swing
                        break;
                    }
                }
            }
        }
    }

    results
}
