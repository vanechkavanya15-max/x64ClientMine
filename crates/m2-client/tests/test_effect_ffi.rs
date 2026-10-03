use std::sync::{Mutex, OnceLock};
use std::ptr;

use m2_client::actor_ffi;
use m2_client::effect_ffi::{m2_effect_create, m2_effect_update, m2_effect_destroy, m2_flying_spawn, m2_flying_update};

static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn run_test<F: FnOnce()>(test_fn: F) {
    let _guard = TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    // Clean up actor state before test
    actor_ffi::test_clear_actors();
    test_fn();
}

#[test]
fn test_invalid_effect_instance_id() {
    run_test(|| {
        assert_eq!(m2_effect_update(0, 1.0), false);
        assert_eq!(m2_effect_destroy(0), false);
        assert_eq!(m2_effect_update(9999, 1.0), false);
        assert_eq!(m2_effect_destroy(9999), false);
    });
}

#[test]
fn test_effect_create_update_destroy() {
    run_test(|| {
        let effect_id = m2_effect_create(101, 10.0, 20.0, 30.0);
        assert!(effect_id > 0);

        // Update successful
        assert_eq!(m2_effect_update(effect_id, 0.5), true);
        assert_eq!(m2_effect_update(effect_id, 0.5), true);

        // Destroy successful
        assert_eq!(m2_effect_destroy(effect_id), true);

        // Update/Destroy fail after destruction
        assert_eq!(m2_effect_update(effect_id, 0.5), false);
        assert_eq!(m2_effect_destroy(effect_id), false);
    });
}

#[test]
fn test_flying_spawn_and_update() {
    run_test(|| {
        // Spawn a target actor at a distance
        assert_eq!(actor_ffi::m2_chr_create(2, 0, 1000.0, 1000.0, 0.0, 0.0), true);

        let sx = 0.0;
        let sy = 0.0;
        let sz = 0.0;
        let speed = 100.0;
        let flying_id = m2_flying_spawn(1, 2, sx, sy, sz, speed);
        assert!(flying_id > 0);

        let mut out_x = 0.0;
        let mut out_y = 0.0;
        let mut out_z = 0.0;

        // Null pointer handling should work and not crash
        assert_eq!(m2_flying_update(flying_id, 0.1, ptr::null_mut(), ptr::null_mut(), ptr::null_mut()), true);

        // Actual movement update
        assert_eq!(m2_flying_update(flying_id, 1.0, &mut out_x, &mut out_y, &mut out_z), true);

        // The projectile should move towards the target. 
        // With speed 100.0 and dt 1.0, distance moved is ~100.0
        // Target is at (1000.0, 1000.0, 0.0), so both X and Y should increase.
        assert!(out_x > 0.0);
        assert!(out_y > 0.0);
        assert_eq!(out_z, 0.0);
    });
}

#[test]
fn test_flying_hit_and_destroy() {
    run_test(|| {
        // Spawn a target actor at a specific distance
        assert_eq!(actor_ffi::m2_chr_create(2, 0, 1000.0, 0.0, 0.0, 0.0), true);

        let sx = 0.0;
        let sy = 0.0;
        let sz = 0.0;
        let speed = 2000.0; // Bardzo szybki pocisk (distance is 1000, so it hits in 0.5s)
        let flying_id = m2_flying_spawn(1, 2, sx, sy, sz, speed);
        assert!(flying_id > 0);

        let mut out_x = 0.0;
        let mut out_y = 0.0;
        let mut out_z = 0.0;

        // The projectile hits the target in 1.0s (> 0.5s required)
        assert_eq!(m2_flying_update(flying_id, 1.0, &mut out_x, &mut out_y, &mut out_z), false); 
        
        // It is destroyed, so subsequent updates fail
        assert_eq!(m2_flying_update(flying_id, 0.1, &mut out_x, &mut out_y, &mut out_z), false);
    });
}
