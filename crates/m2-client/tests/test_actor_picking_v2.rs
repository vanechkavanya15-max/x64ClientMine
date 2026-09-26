use glam::Mat4;
use std::sync::Mutex;
use m2_client::actor_picking_v2::*;

static TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_defending_sphere_intersect() {
    let sphere = DefendingSphere {
        center: [0.0, 0.0, 0.0],
        radius: 2.0,
    };

    let ray_hit = Ray { origin: [5.0, 0.0, 0.0], dir: [-1.0, 0.0, 0.0] };
    assert!(sphere.intersect(&ray_hit), "Ray pointing at sphere should hit");

    let ray_miss = Ray { origin: [5.0, 5.0, 0.0], dir: [-1.0, 0.0, 0.0] };
    assert!(!sphere.intersect(&ray_miss), "Ray missing sphere should not hit");

    let ray_away = Ray { origin: [5.0, 0.0, 0.0], dir: [1.0, 0.0, 0.0] };
    assert!(!sphere.intersect(&ray_away), "Ray pointing away should not hit");

    let ray_inside = Ray { origin: [0.0, 0.0, 0.0], dir: [1.0, 0.0, 0.0] };
    assert!(sphere.intersect(&ray_inside), "Ray inside sphere should hit");
}

#[test]
fn test_obb_intersect() {
    let obb = Obb {
        center: [0.0, 0.0, 0.0],
        half_width: 1.0,
        half_depth: 2.0,
        height: 10.0, // Extends from 0 to 10 on Z
        yaw: std::f32::consts::FRAC_PI_2, // 90 deg rotation, swaps X and Y conceptually
    };

    let ray_hit_front = Ray { origin: [5.0, 0.0, 5.0], dir: [-1.0, 0.0, 0.0] };
    assert!(obb.intersect(&ray_hit_front), "Ray hitting rotated front should hit");

    let ray_miss_high = Ray { origin: [5.0, 0.0, 15.0], dir: [-1.0, 0.0, 0.0] };
    assert!(!obb.intersect(&ray_miss_high), "Ray above OBB should not hit");
}

#[test]
fn test_project_3d_to_2d() {
    let vp_matrix = Mat4::IDENTITY;
    let width = 800.0;
    let height = 600.0;

    let pos = [0.0, 0.0, 0.5]; // visible in NDC space for default identity 
    let proj = project_3d_to_2d(pos, &vp_matrix, width, height);
    assert!(proj.is_some());
    let proj = proj.unwrap();
    assert_eq!(proj[0], 400.0);
    assert_eq!(proj[1], 300.0);

    let pos_behind = [0.0, 0.0, -1.0];
    let proj_behind = project_3d_to_2d(pos_behind, &vp_matrix, width, height);
    assert!(proj_behind.is_none());
}

#[test]
fn test_actor_picking_logic() {
    let _lock = TEST_LOCK.lock().unwrap();
    clear_picking_actors_v2();

    add_picking_actor_v2(PickingActorDataV2 {
        vid: 1,
        is_main: true,
        is_dead: false,
        can_pick: true,
        center: [0.0, 0.0, 0.0],
        defending_sphere: DefendingSphere { center: [0.0, 0.0, 0.0], radius: 1.0 },
        obb: Obb { center: [0.0, 0.0, 0.0], half_width: 1.0, half_depth: 1.0, height: 2.0, yaw: 0.0 },
        is_selected: false,
    });

    add_picking_actor_v2(PickingActorDataV2 {
        vid: 2,
        is_main: false,
        is_dead: false,
        can_pick: true,
        center: [10.0, 0.0, 0.0],
        defending_sphere: DefendingSphere { center: [10.0, 0.0, 0.0], radius: 2.0 },
        obb: Obb { center: [10.0, 0.0, 0.0], half_width: 1.0, half_depth: 1.0, height: 2.0, yaw: 0.0 },
        is_selected: false,
    });

    add_picking_actor_v2(PickingActorDataV2 {
        vid: 3,
        is_main: false,
        is_dead: true, // Dead actor
        can_pick: true,
        center: [5.0, 0.0, 0.0], // Closer than 2
        defending_sphere: DefendingSphere { center: [5.0, 0.0, 0.0], radius: 2.0 },
        obb: Obb { center: [5.0, 0.0, 0.0], half_width: 1.0, half_depth: 1.0, height: 2.0, yaw: 0.0 },
        is_selected: false,
    });

    let mut picker = ActorPickerV2::new();
    
    // Ray pointing at both 2 and 3
    let ray = Ray { origin: [-5.0, 0.0, 1.0], dir: [1.0, 0.0, 0.0] };

    // Eye pos is ray origin
    let picked = picker.pick(&ray, [-5.0, 0.0, 1.0]);
    
    // Alive actor (2) should be sorted before Dead actor (3) even though 3 is closer.
    // However, 2 only passes DefendingSphere (it misses OBB if we aim right at edge? Let's check).
    // Actually both pass their respective precise checks. 2 passes DefendingSphere (it is alive, but picker does precise OBB check to select).
    // Wait, picker first pass (Precise) checks OBB.
    // Ray origin is Z=1.0. Obb height is 2.0. So it goes through center Z. Both OBBs will be hit.
    // Since 2 is sorted before 3 (alive > dead), 2 should be picked.
    assert_eq!(picked, Some(2));
    
    {
        let actors = get_picking_actors_v2().read().unwrap();
        assert!(actors.get(&2).unwrap().is_selected);
        assert!(!actors.get(&3).unwrap().is_selected);
    }
}

#[test]
fn test_actor_picking_fallback() {
    let _lock = TEST_LOCK.lock().unwrap();
    clear_picking_actors_v2();

    add_picking_actor_v2(PickingActorDataV2 {
        vid: 4,
        is_main: false,
        is_dead: false,
        can_pick: true,
        center: [10.0, 0.0, 0.0],
        defending_sphere: DefendingSphere { center: [10.0, 0.0, 0.0], radius: 2.0 },
        obb: Obb { center: [10.0, 0.0, 0.0], half_width: 0.1, half_depth: 0.1, height: 0.2, yaw: 0.0 }, // Very small OBB
        is_selected: false,
    });

    let mut picker = ActorPickerV2::new();
    
    // Ray misses OBB but hits DefendingSphere
    let ray = Ray { origin: [10.0, 1.0, 1.0], dir: [0.0, -1.0, 0.0] }; 
    // Wait, if origin is [10.0, 1.0, 1.0] and dir is [0.0, -1.0, 0.0], it passes through (10.0, 0.0, 1.0).
    // OBB at [10, 0, 0] with Z=0..0.2. Ray at Z=1.0 misses OBB!
    // Defending sphere is at [10, 0, 0] radius 2.0. Ray at [10, Y, 1] distance is 1.0 from center in Z. So it hits sphere!

    let picked = picker.pick(&ray, [10.0, 1.0, 1.0]);
    assert_eq!(picked, Some(4), "Should pick via fallback when OBB misses but Sphere hits");
}

#[test]
fn test_ffi_pick() {
    let _lock = TEST_LOCK.lock().unwrap();
    clear_picking_actors_v2();

    add_picking_actor_v2(PickingActorDataV2 {
        vid: 5,
        is_main: false,
        is_dead: false,
        can_pick: true,
        center: [10.0, 0.0, 0.0],
        defending_sphere: DefendingSphere { center: [10.0, 0.0, 0.0], radius: 2.0 },
        obb: Obb { center: [10.0, 0.0, 0.0], half_width: 1.0, half_depth: 1.0, height: 2.0, yaw: 0.0 },
        is_selected: false,
    });

    let orig = [5.0, 0.0, 1.0];
    let dir = [1.0, 0.0, 0.0];
    let picked = m2_actor_pick_obb(&orig, &dir);
    assert_eq!(picked, 5);
}
