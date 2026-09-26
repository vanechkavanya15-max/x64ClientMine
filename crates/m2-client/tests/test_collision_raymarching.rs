#[path = "../src/collision_raymarching.rs"]
mod collision_raymarching;

use collision_raymarching::{BoxCollider, Ray};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_hits_building_wall() {
        let box_collider = BoxCollider {
            min: (-5.0, 0.0, -5.0),
            max: (5.0, 10.0, 5.0),
        };

        // Ray looking directly at front wall (z = -5)
        let ray = Ray {
            origin: (0.0, 5.0, -10.0),
            direction: (0.0, 0.0, 1.0),
        };

        let hit = box_collider.intersect(&ray);
        assert!(hit.is_some(), "Ray should hit the building wall.");

        let hit_data = hit.unwrap();
        assert_eq!(hit_data.distance, 5.0);
        assert_eq!(hit_data.point, (0.0, 5.0, -5.0));
        assert_eq!(hit_data.normal, (0.0, 0.0, -1.0));
    }

    #[test]
    fn test_ray_misses_building() {
        let box_collider = BoxCollider {
            min: (-5.0, 0.0, -5.0),
            max: (5.0, 10.0, 5.0),
        };

        // Ray parallel to the building, missing it
        let ray = Ray {
            origin: (10.0, 5.0, -10.0),
            direction: (0.0, 0.0, 1.0),
        };

        let hit = box_collider.intersect(&ray);
        assert!(hit.is_none(), "Ray should miss the building entirely.");
    }

    #[test]
    fn test_ray_from_inside_building() {
        let box_collider = BoxCollider {
            min: (-5.0, 0.0, -5.0),
            max: (5.0, 10.0, 5.0),
        };

        // Ray starts inside, pointing outwards along X axis
        let ray = Ray {
            origin: (0.0, 5.0, 0.0),
            direction: (1.0, 0.0, 0.0),
        };

        let hit = box_collider.intersect(&ray);
        assert!(hit.is_some(), "Ray from inside should hit the wall when exiting.");

        let hit_data = hit.unwrap();
        assert_eq!(hit_data.distance, 5.0);
        assert_eq!(hit_data.point, (5.0, 5.0, 0.0));
        assert_eq!(hit_data.normal, (1.0, 0.0, 0.0));
    }

    #[test]
    fn test_ray_hits_corner() {
        let box_collider = BoxCollider {
            min: (0.0, 0.0, 0.0),
            max: (2.0, 2.0, 2.0),
        };

        let ray = Ray {
            origin: (-2.0, -2.0, -2.0),
            direction: (1.0, 1.0, 1.0),
        };

        let hit = box_collider.intersect(&ray);
        assert!(hit.is_some(), "Ray should hit the box corner.");
        
        let hit_data = hit.unwrap();
        assert_eq!(hit_data.distance, 2.0);
        assert_eq!(hit_data.point, (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_ray_parallel_but_outside() {
        let box_collider = BoxCollider {
            min: (-1.0, -1.0, -1.0),
            max: (1.0, 1.0, 1.0),
        };

        let ray = Ray {
            origin: (2.0, 0.0, -5.0),
            direction: (0.0, 0.0, 1.0), // parallel to z axis, but x=2 is outside [-1, 1]
        };

        let hit = box_collider.intersect(&ray);
        assert!(hit.is_none());
    }

    #[test]
    fn test_ray_pointing_away() {
        let box_collider = BoxCollider {
            min: (-1.0, -1.0, -1.0),
            max: (1.0, 1.0, 1.0),
        };

        let ray = Ray {
            origin: (0.0, 0.0, 2.0),
            direction: (0.0, 0.0, 1.0), // moving away from the box
        };

        let hit = box_collider.intersect(&ray);
        assert!(hit.is_none());
    }
}
