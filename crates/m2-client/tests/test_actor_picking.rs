#[path = "../src/actor_picking.rs"]
mod actor_picking;

use actor_picking::{
    Actor, ActorPicker, Ray, intersect_defending_sphere,
    find_nearest_target, ray_intersect_cylinder, PickingActor,
    add_picking_actor, clear_picking_actors,
    m2_actor_ray_pick, m2_chr_mgr_find_nearest_target
};

struct TestActor {
    vid: u32,
    center: [f32; 3],
    radius: f32,
    is_selected: bool,
}

impl TestActor {
    fn new(vid: u32, center: [f32; 3], radius: f32) -> Self {
        Self {
            vid,
            center,
            radius,
            is_selected: false,
        }
    }
}

impl Actor for TestActor {
    fn vid(&self) -> u32 {
        self.vid
    }

    fn sphere_center(&self) -> [f32; 3] {
        self.center
    }

    fn radius(&self) -> f32 {
        self.radius
    }

    fn on_selected(&mut self) {
        self.is_selected = true;
    }

    fn on_unselected(&mut self) {
        self.is_selected = false;
    }
}

#[test]
fn test_intersect_defending_sphere_hit() {
    let ray = Ray {
        origin: [0.0, 0.0, 0.0],
        dir: [1.0, 0.0, 0.0],
    };
    let center = [5.0, 0.0, 0.0];
    let radius = 2.0;

    assert!(intersect_defending_sphere(&ray, center, radius));
}

#[test]
fn test_intersect_defending_sphere_miss() {
    let ray = Ray {
        origin: [0.0, 0.0, 0.0],
        dir: [1.0, 0.0, 0.0],
    };
    let center = [0.0, 5.0, 0.0];
    let radius = 2.0;

    assert!(!intersect_defending_sphere(&ray, center, radius));
}

#[test]
fn test_intersect_defending_sphere_behind() {
    let ray = Ray {
        origin: [0.0, 0.0, 0.0],
        dir: [1.0, 0.0, 0.0],
    };
    let center = [-5.0, 0.0, 0.0];
    let radius = 2.0;

    assert!(!intersect_defending_sphere(&ray, center, radius));
}

#[test]
fn test_picker_ignores_main_actor() {
    let ray = Ray {
        origin: [0.0, 0.0, 0.0],
        dir: [1.0, 0.0, 0.0],
    };
    let mut picker = ActorPicker::new();
    let mut actor1 = TestActor::new(1, [5.0, 0.0, 0.0], 2.0); // Directly in front

    let actors: Vec<&mut dyn Actor> = vec![&mut actor1];

    // Pick where main_vid is 1 (the only actor)
    let picked = picker.pick(&ray, 1, actors.into_iter());

    assert_eq!(picked, None);
}

#[test]
fn test_picker_selects_closest_actor() {
    let ray = Ray {
        origin: [0.0, 0.0, 0.0],
        dir: [1.0, 0.0, 0.0],
    };
    let mut picker = ActorPicker::new();
    let mut actor1 = TestActor::new(1, [10.0, 0.0, 0.0], 2.0); // Further
    let mut actor2 = TestActor::new(2, [5.0, 0.0, 0.0], 2.0);  // Closer

    let actors: Vec<&mut dyn Actor> = vec![&mut actor1, &mut actor2];

    let picked = picker.pick(&ray, 99, actors.into_iter());

    assert_eq!(picked, Some(2));
    assert!(!actor1.is_selected);
    assert!(actor2.is_selected);
}

#[test]
fn test_picker_updates_selection() {
    let mut picker = ActorPicker::new();
    let ray1 = Ray { origin: [0.0, 0.0, 0.0], dir: [1.0, 0.0, 0.0] };
    let ray2 = Ray { origin: [0.0, 0.0, 0.0], dir: [0.0, 1.0, 0.0] };

    let mut actor1 = TestActor::new(1, [5.0, 0.0, 0.0], 2.0);
    let mut actor2 = TestActor::new(2, [0.0, 5.0, 0.0], 2.0);

    // Pick first actor
    {
        let actors: Vec<&mut dyn Actor> = vec![&mut actor1, &mut actor2];
        let picked = picker.pick(&ray1, 99, actors.into_iter());
        assert_eq!(picked, Some(1));
    }
    
    assert!(actor1.is_selected);
    assert!(!actor2.is_selected);

    // Pick second actor
    {
        let actors: Vec<&mut dyn Actor> = vec![&mut actor1, &mut actor2];
        let picked = picker.pick(&ray2, 99, actors.into_iter());
        assert_eq!(picked, Some(2));
    }

    assert!(!actor1.is_selected); // Should be unselected
    assert!(actor2.is_selected);  // Should be selected
    
    // Pick none (miss)
    let ray3 = Ray { origin: [0.0, 0.0, 0.0], dir: [0.0, 0.0, 1.0] };
    {
        let actors: Vec<&mut dyn Actor> = vec![&mut actor1, &mut actor2];
        let picked = picker.pick(&ray3, 99, actors.into_iter());
        assert_eq!(picked, None);
    }
    
    assert!(!actor1.is_selected);
    assert!(!actor2.is_selected); // Should be unselected
}

#[test]
fn test_find_nearest_target() {
    clear_picking_actors();
    
    // Main actor at (0, 0)
    add_picking_actor(PickingActor { vid: 1, x: 0.0, y: 0.0, z: 0.0, radius: 1.0, height: 1.0, hp: 100, actor_type: 1 });
    
    // Target 2: Valid, dist = 5.0
    add_picking_actor(PickingActor { vid: 2, x: 5.0, y: 0.0, z: 0.0, radius: 1.0, height: 1.0, hp: 100, actor_type: 2 });
    
    // Target 3: Valid, dist = 3.0 (closest)
    add_picking_actor(PickingActor { vid: 3, x: 0.0, y: 3.0, z: 0.0, radius: 1.0, height: 1.0, hp: 100, actor_type: 2 });
    
    // Target 4: Dead, dist = 2.0
    add_picking_actor(PickingActor { vid: 4, x: 0.0, y: 2.0, z: 0.0, radius: 1.0, height: 1.0, hp: 0, actor_type: 2 });
    
    // Target 5: Wrong type, dist = 1.0
    add_picking_actor(PickingActor { vid: 5, x: 1.0, y: 0.0, z: 0.0, radius: 1.0, height: 1.0, hp: 100, actor_type: 3 });
    
    // Target 6: Out of range, dist = 20.0
    add_picking_actor(PickingActor { vid: 6, x: 20.0, y: 0.0, z: 0.0, radius: 1.0, height: 1.0, hp: 100, actor_type: 2 });
    
    // Target 7-11: random other targets
    add_picking_actor(PickingActor { vid: 7, x: 10.0, y: 10.0, z: 0.0, radius: 1.0, height: 1.0, hp: 100, actor_type: 2 });
    add_picking_actor(PickingActor { vid: 8, x: -10.0, y: 0.0, z: 0.0, radius: 1.0, height: 1.0, hp: 100, actor_type: 2 });
    add_picking_actor(PickingActor { vid: 9, x: 0.0, y: -8.0, z: 0.0, radius: 1.0, height: 1.0, hp: 100, actor_type: 2 });
    add_picking_actor(PickingActor { vid: 10, x: -5.0, y: -5.0, z: 0.0, radius: 1.0, height: 1.0, hp: 100, actor_type: 2 });
    add_picking_actor(PickingActor { vid: 11, x: 6.0, y: 6.0, z: 0.0, radius: 1.0, height: 1.0, hp: 100, actor_type: 2 });

    let closest = find_nearest_target(1, 10.0, 2);
    assert_eq!(closest, Some(3));
    
    let closest_ffi = m2_chr_mgr_find_nearest_target(1, 10.0, 2);
    assert_eq!(closest_ffi, 3);
}

#[test]
fn test_ray_intersect_cylinder_hit_side() {
    let t = ray_intersect_cylinder(
        (0.0, 0.0, 5.0),
        (1.0, 0.0, 0.0),
        (5.0, 0.0, 0.0),
        2.0,
        10.0
    );
    assert!(t.is_some());
    assert!((t.unwrap() - 3.0).abs() < 1e-4);
}

#[test]
fn test_ray_intersect_cylinder_hit_top() {
    let t = ray_intersect_cylinder(
        (5.0, 0.0, 15.0),
        (0.0, 0.0, -1.0),
        (5.0, 0.0, 0.0),
        2.0,
        10.0
    );
    assert!(t.is_some());
    assert!((t.unwrap() - 5.0).abs() < 1e-4);
}

#[test]
fn test_ray_intersect_cylinder_miss() {
    let t = ray_intersect_cylinder(
        (0.0, 0.0, 5.0),
        (1.0, 0.0, 0.0),
        (5.0, 5.0, 0.0),
        2.0,
        10.0
    );
    assert!(t.is_none());
}

#[test]
fn test_m2_actor_ray_pick_ffi() {
    clear_picking_actors();
    
    add_picking_actor(PickingActor { vid: 1, x: 10.0, y: 0.0, z: 0.0, radius: 2.0, height: 10.0, hp: 100, actor_type: 1 });
    add_picking_actor(PickingActor { vid: 2, x: 5.0, y: 0.0, z: 0.0, radius: 2.0, height: 10.0, hp: 100, actor_type: 1 }); // Closer
    
    let hit_vid = m2_actor_ray_pick(0.0, 0.0, 5.0, 1.0, 0.0, 0.0);
    assert_eq!(hit_vid, 2);
}
