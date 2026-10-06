#[path = "../src/actor_hit_reaction_physics_v2.rs"]
mod actor_hit_reaction_physics_v2;

use glam::Vec3;
use actor_hit_reaction_physics_v2::*;

#[derive(Default)]
struct TestActor {
    is_stun: bool,
    is_dead: bool,
    is_knock_down: bool,
    is_lock: bool,
    is_using_skill: bool,
    is_stand_up_motion: bool,
    rotation: f32,
    position: Vec3,
    shake_duration: u32,
    shake_time: u32,
    
    // Test verification states
    intercepted_motions: Vec<u32>,
    pushed_once_motions: Vec<u32>,
    pushed_loop_motions: Vec<u32>,
    falling_dir_x: f32,
    falling_dir_y: f32,

    // Configurable intercepts
    mock_intercept_results: std::collections::HashMap<u32, bool>,
}

impl TestActor {
    fn new() -> Self {
        Self::default()
    }
}

impl HitReactionActor for TestActor {
    fn is_stun(&self) -> bool { self.is_stun }
    fn die(&mut self) { self.is_dead = true; }
    fn is_knock_down(&self) -> bool { self.is_knock_down }
    fn is_lock(&self) -> bool { self.is_lock }
    fn get_rotation(&self) -> f32 { self.rotation }
    
    fn intercept_once_motion(&mut self, motion_name: u32) -> bool {
        self.intercepted_motions.push(motion_name);
        *self.mock_intercept_results.get(&motion_name).unwrap_or(&true)
    }
    
    fn push_once_motion(&mut self, motion_name: u32) {
        self.pushed_once_motions.push(motion_name);
    }
    
    fn push_loop_motion(&mut self, motion_name: u32) {
        self.pushed_loop_motions.push(motion_name);
    }
    
    fn is_using_skill(&self) -> bool { self.is_using_skill }
    fn is_stand_up_motion(&self) -> bool { self.is_stand_up_motion }
    fn set_real_dead(&mut self, is_dead: bool) { self.is_dead = is_dead; }
    fn get_position(&self) -> Vec3 { self.position }
    fn set_falling_direction(&mut self, dx: f32, dy: f32) {
        self.falling_dir_x = dx;
        self.falling_dir_y = dy;
    }
    fn shake_duration(&self) -> u32 { self.shake_duration }
    fn set_shake_duration(&mut self, ms: u32) { self.shake_duration = ms; }
    fn shake_time(&self) -> u32 { self.shake_time }
    fn set_shake_time(&mut self, time: u32) { self.shake_time = time; }
}

#[test]
fn test_hit_stone_stunned() {
    let mut victim = TestActor::new();
    victim.is_stun = true;
    
    hit_stone(&mut victim);
    
    assert!(victim.is_dead);
    assert_eq!(victim.shake_duration, 0);
}

#[test]
fn test_hit_stone_not_stunned() {
    let mut victim = TestActor::new();
    victim.is_stun = false;
    
    hit_stone(&mut victim);
    
    assert!(!victim.is_dead);
    assert_eq!(victim.shake_duration, 100);
    assert!(victim.shake_time > 0);
}

#[test]
fn test_hit_good_knock_down() {
    let mut attacker = TestActor::new();
    let mut victim = TestActor::new();
    victim.is_knock_down = true;
    
    hit_good(&attacker, &mut victim);
    
    assert_eq!(victim.shake_duration, 0);
}

#[test]
fn test_hit_good_scalar_negative() {
    let mut attacker = TestActor::new();
    attacker.rotation = 90.0;
    
    let mut victim = TestActor::new();
    victim.rotation = 270.0; // Opposite facing
    
    hit_good(&attacker, &mut victim);
    
    assert_eq!(victim.shake_duration, 100);
    assert_eq!(victim.intercepted_motions.len(), 1);
    assert_eq!(victim.intercepted_motions[0], NAME_DAMAGE);
    assert_eq!(victim.pushed_loop_motions.len(), 1);
    assert_eq!(victim.pushed_loop_motions[0], NAME_WAIT);
}

#[test]
fn test_hit_good_scalar_positive() {
    let mut attacker = TestActor::new();
    attacker.rotation = 90.0;
    
    let mut victim = TestActor::new();
    victim.rotation = 90.0; // Same facing
    
    hit_good(&attacker, &mut victim);
    
    assert_eq!(victim.shake_duration, 100);
    assert_eq!(victim.intercepted_motions.len(), 1);
    assert_eq!(victim.intercepted_motions[0], NAME_DAMAGE_BACK);
    assert_eq!(victim.pushed_loop_motions.len(), 1);
    assert_eq!(victim.pushed_loop_motions[0], NAME_WAIT);
}

#[test]
fn test_hit_greate_scalar_negative() {
    let mut attacker = TestActor::new();
    attacker.rotation = 90.0;
    
    let mut victim = TestActor::new();
    victim.rotation = 270.0; // Opposite facing
    
    hit_greate(&attacker, &mut victim);
    
    assert_eq!(victim.shake_duration, 100);
    assert_eq!(victim.intercepted_motions.len(), 1);
    assert_eq!(victim.intercepted_motions[0], NAME_DAMAGE_FLYING);
    assert_eq!(victim.pushed_once_motions.len(), 1);
    assert_eq!(victim.pushed_once_motions[0], NAME_STAND_UP);
    assert_eq!(victim.pushed_loop_motions.len(), 1);
    assert_eq!(victim.pushed_loop_motions[0], NAME_WAIT);
}

#[test]
fn test_push_circle() {
    let mut victim = TestActor::new();
    victim.position = Vec3::new(10.0, 10.0, 0.0);
    
    // Attacker pos passed to push_circle
    let attacker_pos = Vec3::new(0.0, -10.0, 0.0); // Wait, C++ uses -y for src_pos: Vec3(atk.x, -atk.y, atk.z)
    // So src_pos will be (0.0, 10.0, 0.0) inside push_circle
    // dst_pos is (10.0, 10.0, 0.0)
    // Direction x = 10, y = 0
    // Normalized = 1.0, 0.0
    
    push_circle(attacker_pos, &mut victim);
    
    assert!((victim.falling_dir_x - 1.0).abs() < f32::EPSILON);
    assert!(victim.falling_dir_y.abs() < f32::EPSILON);
}

#[test]
fn test_shake_process() {
    let mut victim = TestActor::new();
    victim.set_shake_time(100);
    
    let mut pos = Vec3::new(0.0, 0.0, 0.0);
    
    // Test shake active (use time that produces non-zero offset)
    shake_process(&mut victim, 51, &mut pos);
    assert!(pos.x != 0.0 || pos.y != 0.0 || pos.z != 0.0);
    
    // Test shake expire
    victim.set_shake_time(100);
    shake_process(&mut victim, 150, &mut pos);
    assert_eq!(victim.shake_time(), 0);
}

#[test]
fn test_push_direct() {
    let mut victim = TestActor::new();
    push_direct(90.0, &mut victim);
    
    // 90 + 270 = 360 -> cos(360) = 1, sin(360) = 0
    assert!((victim.falling_dir_x - 1.0).abs() < 1e-5);
    assert!(victim.falling_dir_y.abs() < 1e-5);
}

#[test]
fn test_is_invisible() {
    assert!(is_invisible(true, 100, 50));
    assert!(is_invisible(false, 100, 50));
    assert!(!is_invisible(false, 100, 150));
}

#[test]
fn test_invisible_time_adjust() {
    let adj1 = get_invisible_time_adjust(3, 0, 2.6, 2.0); // Shaman W, No skill, double speed -> scale 1.0 -> 1.0
    assert!((adj1 - 1.0).abs() < f32::EPSILON);
    
    let adj2 = get_invisible_time_adjust(0, 0, 2.6, 2.0); // Warrior M -> 0
    assert!(adj2.abs() < f32::EPSILON);
}
