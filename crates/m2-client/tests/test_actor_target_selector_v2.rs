#[path = "../src/actor_target_selector_v2.rs"]
mod actor_target_selector_v2;

use actor_target_selector_v2::{ActorTargetSelectorV2, Ray3D, TargetableInstance};
use glam::Vec3;

struct MockInstance {
    vid: u32,
    #[allow(dead_code)]
    vnum: u32,
    is_dead: bool,
    #[allow(dead_code)]
    is_pc: bool,
    can_pick: bool,
    is_movie: bool,
    center: Vec3,
    radius: f32,
    aabb_min: Vec3,
    aabb_max: Vec3,
    #[allow(dead_code)]
    selected: bool,
}

impl MockInstance {
    fn new(vid: u32, center: Vec3) -> Self {
        Self {
            vid,
            vnum: 101,
            is_dead: false,
            is_pc: true,
            can_pick: true,
            is_movie: false,
            center,
            radius: 50.0,
            aabb_min: center - Vec3::new(20.0, 20.0, 0.0),
            aabb_max: center + Vec3::new(20.0, 20.0, 150.0),
            selected: false,
        }
    }
}

impl TargetableInstance for MockInstance {
    fn get_vid(&self) -> u32 {
        self.vid
    }
    fn get_vnum(&self) -> u32 {
        self.vnum
    }
    fn is_dead(&self) -> bool {
        self.is_dead
    }
    fn is_pc(&self) -> bool {
        self.is_pc
    }
    fn can_pick_instance(&self) -> bool {
        self.can_pick
    }
    fn is_movie_mode(&self) -> bool {
        self.is_movie
    }

    fn intersect_defending_sphere(&self, ray: &Ray3D) -> bool {
        ray.intersect_sphere(self.center, self.radius)
    }

    fn intersect_bounding_box(&self, ray: &Ray3D) -> bool {
        ray.intersect_bounding_box(self.aabb_min, self.aabb_max)
    }

    fn get_distance_sq(&self, camera_eye: Vec3) -> f32 {
        self.center.distance_squared(camera_eye)
    }

    fn get_distance_to_instance(&self, _other: &dyn TargetableInstance) -> f32 {
        0.0
    }

    fn on_selected(&mut self) {
        self.selected = true;
    }
    fn on_unselected(&mut self) {
        self.selected = false;
    }
}

#[test]
fn test_ray_math_sphere_and_aabb() {
    let ray = Ray3D::new(Vec3::new(0.0, 0.0, 500.0), Vec3::new(0.0, 0.0, -1.0));

    // Sphere at 0,0,0 r=50
    assert!(ray.intersect_sphere(Vec3::ZERO, 50.0));
    assert!(!ray.intersect_sphere(Vec3::new(100.0, 0.0, 0.0), 50.0));

    // AABB at 0,0,0
    assert!(ray.intersect_bounding_box(
        Vec3::new(-10.0, -10.0, -10.0),
        Vec3::new(10.0, 10.0, 10.0)
    ));
    assert!(!ray.intersect_bounding_box(
        Vec3::new(50.0, 50.0, 50.0),
        Vec3::new(60.0, 60.0, 60.0)
    ));
}

#[test]
fn test_pick_new_closest_alive() {
    let mut inst1 = MockInstance::new(1, Vec3::new(0.0, 0.0, 0.0));
    let mut inst2 = MockInstance::new(2, Vec3::new(0.0, 0.0, -100.0));

    let ray = Ray3D::new(Vec3::new(0.0, 0.0, 500.0), Vec3::new(0.0, 0.0, -1.0));
    let camera_eye = Vec3::new(0.0, 0.0, 500.0);

    let mut instances: Vec<&mut dyn TargetableInstance> = vec![&mut inst1, &mut inst2];

    let mut picker = ActorTargetSelectorV2::new();
    let picked = picker.pick_new(&ray, camera_eye, Some(99), &mut instances);

    assert_eq!(picked, Some(1));
}

#[test]
fn test_pick_new_dead_vs_alive_priority() {
    let mut inst_dead = MockInstance::new(1, Vec3::new(0.0, 0.0, 100.0));
    inst_dead.is_dead = true; // Closer but dead
    let mut inst_alive = MockInstance::new(2, Vec3::new(0.0, 0.0, 0.0));

    let ray = Ray3D::new(Vec3::new(0.0, 0.0, 500.0), Vec3::new(0.0, 0.0, -1.0));
    let camera_eye = Vec3::new(0.0, 0.0, 500.0);

    let mut instances: Vec<&mut dyn TargetableInstance> = vec![&mut inst_dead, &mut inst_alive];
    let mut picker = ActorTargetSelectorV2::new();

    // Alive should be picked first despite being further away
    let picked = picker.pick_new(&ray, camera_eye, None, &mut instances);
    assert_eq!(picked, Some(2));
}

#[test]
fn test_pick_old() {
    let mut inst1 = MockInstance::new(1, Vec3::new(0.0, 0.0, 0.0));
    let ray = Ray3D::new(Vec3::new(0.0, 0.0, 500.0), Vec3::new(0.0, 0.0, -1.0));

    let mut instances: Vec<&mut dyn TargetableInstance> = vec![&mut inst1];
    let mut picker = ActorTargetSelectorV2::new();

    let picked = picker.pick_old(&ray, None, &mut instances);
    assert_eq!(picked, Some(1));
}

#[test]
fn test_pick_all() {
    let mut inst1 = MockInstance::new(1, Vec3::new(0.0, 0.0, 0.0));
    let ray = Ray3D::new(Vec3::new(0.0, 0.0, 500.0), Vec3::new(0.0, 0.0, -1.0));

    let instances: Vec<&mut dyn TargetableInstance> = vec![&mut inst1];
    let picked = ActorTargetSelectorV2::pick_all(&ray, &instances);
    assert_eq!(picked, Some(1));
}

#[test]
fn test_main_actor_skip() {
    let mut main_inst = MockInstance::new(1, Vec3::new(0.0, 0.0, 0.0));
    let ray = Ray3D::new(Vec3::new(0.0, 0.0, 500.0), Vec3::new(0.0, 0.0, -1.0));

    let mut instances: Vec<&mut dyn TargetableInstance> = vec![&mut main_inst];
    let mut picker = ActorTargetSelectorV2::new();

    // If we skip main_vid, it shouldn't be picked by exact/order check,
    // but in NEW_Pick it CAN be picked as a fallback if nothing else is found.
    let picked = picker.pick_new(
        &ray,
        Vec3::new(0.0, 0.0, 500.0),
        Some(1),
        &mut instances,
    );
    assert_eq!(picked, Some(1)); // Fallback picking main actor
}
