 //! Wzorzec referencyjny C++: `source/GameLib/FlyingObjectManager.cpp oraz source/GameLib/FlyingInstance.cpp`
 //! Opis: Odwzorowanie CFlyingInstance: trajektoria lotu strzaly i ognistych kul od strzelca do ruchomego celu (target_vid), detekcja momentu trafienia.
 
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub mod nalgebra {
    use glam::{Quat, Vec3};
    use std::ops::{Add, AddAssign, Mul, Sub};

    #[derive(Debug, Clone, Copy, PartialEq, Default)]
    pub struct Vector3<T = f32> {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        _marker: std::marker::PhantomData<T>,
    }

    impl Vector3<f32> {
        pub fn new(x: f32, y: f32, z: f32) -> Self {
            Self { x, y, z, _marker: std::marker::PhantomData }
        }
        pub fn zeros() -> Self {
            Self::new(0.0, 0.0, 0.0)
        }
        pub fn to_glam(self) -> Vec3 {
            Vec3::new(self.x, self.y, self.z)
        }
        pub fn from_glam(v: Vec3) -> Self {
            Self::new(v.x, v.y, v.z)
        }
        pub fn norm_squared(&self) -> f32 {
            self.to_glam().length_squared()
        }
        pub fn norm(&self) -> f32 {
            self.to_glam().length()
        }
        pub fn normalize(&self) -> Self {
            Self::from_glam(self.to_glam().normalize_or_zero())
        }
        pub fn dot(&self, other: &Self) -> f32 {
            self.to_glam().dot(other.to_glam())
        }
        pub fn into_owned(self) -> Self {
            self
        }
    }

    impl Add for Vector3<f32> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Self::from_glam(self.to_glam() + rhs.to_glam())
        }
    }
    impl AddAssign for Vector3<f32> {
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
            self.z += rhs.z;
        }
    }
    impl Sub for Vector3<f32> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
            Self::from_glam(self.to_glam() - rhs.to_glam())
        }
    }
    impl Mul<f32> for Vector3<f32> {
        type Output = Self;
        fn mul(self, rhs: f32) -> Self {
            Self::from_glam(self.to_glam() * rhs)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Quaternion<T = f32> {
        pub q: Quat,
        _marker: std::marker::PhantomData<T>,
    }

    impl Quaternion<f32> {
        pub fn from_parts(w: f32, v: Vector3<f32>) -> Self {
            Self { q: Quat::from_xyzw(v.x, v.y, v.z, w), _marker: std::marker::PhantomData }
        }
        pub fn scalar(&self) -> f32 {
            self.q.w
        }
        pub fn vector(&self) -> Vector3<f32> {
            Vector3::new(self.q.x, self.q.y, self.q.z)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct UnitQuaternion<T = f32> {
        pub q: Quat,
        _marker: std::marker::PhantomData<T>,
    }

    impl UnitQuaternion<f32> {
        pub fn identity() -> Self {
            Self { q: Quat::IDENTITY, _marker: std::marker::PhantomData }
        }
        pub fn rotation_between(a: &Vector3<f32>, b: &Vector3<f32>) -> Option<Self> {
            let na = a.to_glam().normalize_or_zero();
            let nb = b.to_glam().normalize_or_zero();
            if na.length_squared() < 1e-6 || nb.length_squared() < 1e-6 {
                return None;
            }
            Some(Self { q: Quat::from_rotation_arc(na, nb), _marker: std::marker::PhantomData })
        }
        pub fn quaternion(&self) -> Quaternion<f32> {
            Quaternion { q: self.q, _marker: std::marker::PhantomData }
        }
        pub fn from_quaternion(q: Quaternion<f32>) -> Self {
            Self { q: q.q.normalize(), _marker: std::marker::PhantomData }
        }
    }

    impl Mul for UnitQuaternion<f32> {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self {
            Self { q: (self.q * rhs.q).normalize(), _marker: std::marker::PhantomData }
        }
    }
    impl Mul<Vector3<f32>> for UnitQuaternion<f32> {
        type Output = Vector3<f32>;
        fn mul(self, rhs: Vector3<f32>) -> Vector3<f32> {
            Vector3::from_glam(self.q * rhs.to_glam())
        }
    }
}
pub use nalgebra::{Quaternion, UnitQuaternion, Vector3};


/// Reprezentuje cel pocisku
#[derive(Debug, Clone, PartialEq)]
pub enum FlyTarget {
    None,
    Object(u32), // Przechowuje VID obiektu
    Position(Vector3<f32>),
}

impl Default for FlyTarget {
    fn default() -> Self {
        FlyTarget::None
    }
}

/// Dane wzorcowe przelotu pocisku (odpowiednik CFlyingData)
#[derive(Debug, Clone)]
pub struct FlyData {
    pub init_vel: f32,
    pub gravity: f32,
    pub range: f32,
    pub bomb_range: f32,
    pub is_homing: bool,
    pub homing_max_angle: f32,
    pub homing_start_time: f32,
    pub accel: Vector3<f32>,
    pub maintain_parallel: bool,
    pub hit_on_background: bool,
    pub hit_on_another_monster: bool,
    pub pierce_count: i32,
    pub collision_sphere_radius: f32,
    pub cone_angle: f32,
    pub roll_angle: f32,
    pub spreading: bool,
}

impl Default for FlyData {
    fn default() -> Self {
        Self {
            init_vel: 200.0,
            gravity: 0.0,
            range: 5000.0, // Zmodyfikowano, by default mial duzy zasieg dla bezpieczenstwa
            bomb_range: 10.0,
            is_homing: false,
            homing_max_angle: 3.0,
            homing_start_time: 0.0,
            accel: Vector3::zeros(),
            maintain_parallel: false,
            hit_on_background: false,
            hit_on_another_monster: false,
            pierce_count: 0,
            collision_sphere_radius: 0.0,
            cone_angle: 0.0,
            roll_angle: 0.0,
            spreading: false,
        }
    }
}

pub trait FlyEventHandler {
    fn on_exploding_at_target(&mut self, skill_index: u32, target_vid: u32);
    fn on_exploding_at_another_target(&mut self, skill_index: u32, target_vid: u32);
    fn on_exploding_out_of_range(&mut self);
    fn on_exploding_at_background(&mut self);
}

/// Instancja lecacego pocisku (odpowiednik CFlyingInstance)
#[derive(Debug, Clone)]
pub struct FlyInstance {
    pub id: u32,
    pub data: Arc<FlyData>,
    pub is_alive: bool,
    pub can_attack: bool,
    
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub accel: Vector3<f32>,
    pub rotation: UnitQuaternion<f32>,
    
    pub remain_range: f32,
    pub start_time: f32,
    pub fly_target: FlyTarget,
    pub pierce_count: i32,
    pub skill_index: u32,
    
    pub owner_vid: u32,
    pub target_hitted: bool,
    pub hitted_object_set: HashSet<u32>,
}

impl FlyInstance {
    pub fn new(id: u32, data: Arc<FlyData>, start_pos: Vector3<f32>, target: FlyTarget, owner_vid: u32, can_attack: bool, skill_index: u32, current_time: f32) -> Self {
        let mut inst = Self {
            id,
            data: data.clone(),
            is_alive: true,
            can_attack,
            
            position: start_pos,
            velocity: Vector3::new(0.0, -data.init_vel, 0.0),
            accel: data.accel,
            rotation: UnitQuaternion::identity(),
            
            remain_range: data.range,
            start_time: current_time,
            fly_target: target.clone(),
            pierce_count: data.pierce_count,
            skill_index,
            
            owner_vid,
            target_hitted: false,
            hitted_object_set: HashSet::new(),
        };

        match target {
            FlyTarget::Position(pos) => inst.set_target_direction(Some(pos)),
            _ => {}
        }
        
        inst
    }

    pub fn set_target_direction(&mut self, target_pos_opt: Option<Vector3<f32>>) {
        let target_pos = match target_pos_opt {
            Some(pos) => pos,
            None => return,
        };

        let mut t_pos = target_pos;
        if self.data.maintain_parallel {
            t_pos.z += 50.0;
        }

        let mut target_dir = t_pos - self.position;
        if target_dir.norm_squared() > 0.0001 {
            target_dir = target_dir.normalize();
        }

        self.set_target_normalized_direction(target_dir);
    }

    fn set_target_normalized_direction(&mut self, dir: Vector3<f32>) {
        let base_dir = Vector3::new(0.0, -1.0, 0.0);
        let q = UnitQuaternion::rotation_between(&base_dir, &dir).unwrap_or(UnitQuaternion::identity());
        
        self.rotation = q;
        // W C++ velocity startuje jako (0, -v, 0) po czym jest obracane kwaternionem rotacji
        let local_vel = Vector3::new(0.0, -self.data.init_vel, 0.0);
        self.velocity = self.rotation * local_vel;
        self.accel = self.rotation * self.data.accel;
    }
    
    pub fn adjust_direction_for_homing(&mut self, target_pos: Vector3<f32>) {
        let mut target_dir = target_pos - self.position;
        if target_dir.norm_squared() < 0.0001 {
            return;
        }
        target_dir = target_dir.normalize();
        
        let mut vel_dir = self.velocity;
        if vel_dir.norm_squared() < 0.0001 {
            return;
        }
        vel_dir = vel_dir.normalize();
        
        if (vel_dir - target_dir).norm_squared() < 0.001 {
            return;
        }
        
        let q = UnitQuaternion::rotation_between(&vel_dir, &target_dir).unwrap_or(UnitQuaternion::identity());
        
        if self.data.homing_max_angle > 180.0 {
            self.velocity = q * self.velocity;
            self.accel = q * self.accel;
            self.rotation = q * self.rotation;
            return;
        }
        
        // Ograniczenie kata homing tak jak w C++
        let max_angle_rad = self.data.homing_max_angle.to_radians();
        let c = max_angle_rad.cos();
        let s = max_angle_rad.sin();
        
        let mut q_w = q.quaternion().scalar();
        let mut q_v = q.quaternion().vector().into_owned();
        
        if q_w <= -1.0 + 0.0001 {
            q_v = Vector3::new(0.0, 0.0, s);
            q_w = c;
        } else if q_w <= c && q_w <= 1.0 - 0.0001 {
            let factor = s / (1.0 - q_w * q_w).sqrt();
            q_v = q_v * factor;
            q_w = c;
        }
        
        let adjusted_q = UnitQuaternion::from_quaternion(Quaternion::from_parts(q_w, q_v));
        
        self.velocity = adjusted_q * self.velocity;
        self.accel = adjusted_q * self.accel;
        self.rotation = adjusted_q * self.rotation;
    }

    pub fn update(&mut self, dt: f32, current_time: f32, target_pos_opt: Option<Vector3<f32>>, handler: &mut Option<&mut dyn FlyEventHandler>) -> bool {
        if !self.is_alive {
            return false;
        }

        if self.data.is_homing && (self.data.homing_start_time + self.start_time < current_time) {
            if let Some(t_pos) = target_pos_opt {
                self.adjust_direction_for_homing(t_pos);
            }
        }

        let last_pos = self.position;

        self.velocity += self.accel * dt;
        self.velocity.z += self.data.gravity * dt;
        
        let movement = self.velocity * dt;
        let move_distance = movement.norm();
        let _collision_sphere_radius = move_distance.max(self.data.collision_sphere_radius);
        
        self.remain_range -= move_distance;
        self.position += movement;

        if self.remain_range < 0.0 {
            if let Some(h) = handler {
                h.on_exploding_out_of_range();
            }
            self.is_alive = false;
            return false;
        }

        // Sprawdzanie trafienia
        if let Some(t_pos) = target_pos_opt {
            if !self.target_hitted {
                let dist_sq = square_distance_between_linesegment_and_point(self.position, last_pos, t_pos);
                if dist_sq < self.data.bomb_range * self.data.bomb_range {
                    if let Some(h) = handler {
                        match self.fly_target {
                            FlyTarget::Object(vid) => h.on_exploding_at_target(self.skill_index, vid),
                            _ => h.on_exploding_at_target(self.skill_index, 0),
                        }
                    }
                    
                    if self.pierce_count > 0 {
                        self.pierce_count -= 1;
                    } else {
                        self.target_hitted = true;
                        self.is_alive = false;
                        return false;
                    }
                    return true; // Dalej leci
                }
            }
        } else if let FlyTarget::Position(t_pos) = self.fly_target {
            let dist_sq = square_distance_between_linesegment_and_point(self.position, last_pos, t_pos);
            if dist_sq < self.data.bomb_range * self.data.bomb_range {
                if let Some(h) = handler {
                    h.on_exploding_at_target(self.skill_index, 0);
                }
                self.is_alive = false;
                return false;
            }
        }

        true
    }
}

// Pomocnicza funkcja geometryczna (port z GrpMath)
fn square_distance_between_linesegment_and_point(p1: Vector3<f32>, p2: Vector3<f32>, pt: Vector3<f32>) -> f32 {
    let d = p2 - p1;
    let length_sq = d.norm_squared();
    if length_sq == 0.0 {
        return (pt - p1).norm_squared();
    }
    
    let t = ((pt - p1).dot(&d) / length_sq).clamp(0.0, 1.0);
    let projection = p1 + d * t;
    (pt - projection).norm_squared()
}

#[derive(Default)]
 pub struct FlyManager {
     pub initialized: bool,
    fly_data_map: HashMap<u32, Arc<FlyData>>,
    fly_instances: Vec<FlyInstance>,
    id_counter: u32,
    current_time: f32,
 }
 
 impl FlyManager {
     pub fn new() -> Self {
        Self {
            initialized: true,
            fly_data_map: HashMap::new(),
            fly_instances: Vec::new(),
            id_counter: 1,
            current_time: 0.0,
        }
     }
 
     pub fn is_ready(&self) -> bool {
         self.initialized
     }
    
    pub fn update_time(&mut self, dt: f32) {
        self.current_time += dt;
    }

    pub fn register_fly_data(&mut self, id: u32, data: FlyData) {
        self.fly_data_map.insert(id, Arc::new(data));
    }
 
    pub fn create_fly_instance(
        &mut self,
        data_id: u32,
        start_pos: Vector3<f32>,
        target: FlyTarget,
        owner_vid: u32,
        can_attack: bool,
        skill_index: u32,
    ) -> Option<u32> {
        let data = self.fly_data_map.get(&data_id)?.clone();
        
        let id = self.id_counter;
        self.id_counter += 1;
        
        let mut instance = FlyInstance::new(id, data, start_pos, target.clone(), owner_vid, can_attack, skill_index, self.current_time);
        
        // Poprawka celu, jesli to pozycja, juz na starcie
        if let FlyTarget::Position(pos) = target {
            instance.set_target_direction(Some(pos));
        }
        
        self.fly_instances.push(instance);
        Some(id)
    }

    /// Aktualizuje wszystkie istniejace pociski
    /// Zwraca zdarzenia eksplozji lub obrazen
    pub fn update(&mut self, dt: f32, target_positions: &HashMap<u32, Vector3<f32>>, mut handler: Option<&mut dyn FlyEventHandler>) {
        let mut i = 0;
        while i < self.fly_instances.len() {
            let mut inst = self.fly_instances[i].clone();
            
            let mut t_pos_opt = None;
            match inst.fly_target {
                FlyTarget::Object(vid) => {
                    if let Some(pos) = target_positions.get(&vid) {
                        t_pos_opt = Some(*pos);
                    }
                }
                FlyTarget::Position(pos) => {
                    t_pos_opt = Some(pos);
                }
                _ => {}
            }
            
            let alive = inst.update(dt, self.current_time, t_pos_opt, &mut handler);
            
            if alive {
                self.fly_instances[i] = inst;
                i += 1;
            } else {
                self.fly_instances.remove(i);
            }
        }
    }
    
    pub fn get_instance_count(&self) -> usize {
        self.fly_instances.len()
    }
    
    pub fn get_instances(&self) -> &Vec<FlyInstance> {
        &self.fly_instances
    }
 
    pub fn get_instances_mut(&mut self) -> &mut Vec<FlyInstance> {
        &mut self.fly_instances
     }
 }
