#[path = "../src/actor_picking.rs"]
mod actor_picking;

use actor_picking::{Actor, ActorPicker, Ray, intersect_defending_sphere};

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

    let mut actors: Vec<&mut dyn Actor> = vec![&mut actor1];

    // Pick where main_vid is 1 (the only actor)
    let picked = picker.pick(&ray, 1, actors.into_iter());

    assert_eq!(picked, None);
    // Since it's passed as mut ref, the original actor1 is accessible here:
    // Wait, the vec consumed it, but we can just use the fact it wasn't picked.
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
