#[path = "../src/hit_geometry.rs"]
mod hit_geometry;

use hit_geometry::test_actor_clash;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_success_target_in_front() {
        let result = test_actor_clash(
            0.0, 0.0,
            0.0, 100.0,
            0.0, // facing positive y
            90.0,
            200.0,
            1.0,
            false,
            0.0,
        );
        assert!(result);
    }

    #[test]
    fn test_hit_rejection_target_behind() {
        let result = test_actor_clash(
            0.0, 0.0,
            0.0, -100.0,
            0.0, // facing positive y
            90.0,
            200.0,
            1.0,
            false,
            0.0,
        );
        assert!(!result);
    }

    #[test]
    fn test_hit_rejection_target_out_of_reach() {
        let result = test_actor_clash(
            0.0, 0.0,
            0.0, 300.0,
            0.0, // facing positive y
            90.0,
            200.0,
            1.0,
            false,
            0.0,
        );
        assert!(!result);
    }

    #[test]
    fn test_hit_success_huge_target_out_of_base_reach_but_in_collision_reach() {
        let result = test_actor_clash(
            0.0, 0.0,
            0.0, 300.0,
            0.0, // facing positive y
            90.0,
            200.0,
            1.0,
            true,
            150.0,
        );
        assert!(result);
    }

    #[test]
    fn test_hit_rejection_huge_target_still_out_of_reach() {
        let result = test_actor_clash(
            0.0, 0.0,
            0.0, 400.0,
            0.0, // facing positive y
            90.0,
            200.0,
            1.0,
            true,
            150.0,
        );
        assert!(!result);
    }

    #[test]
    fn test_hit_success_zero_distance() {
        let result = test_actor_clash(
            0.0, 0.0,
            0.0, 0.0,
            0.0,
            90.0,
            200.0,
            1.0,
            false,
            0.0,
        );
        assert!(result);
    }

    #[test]
    fn test_hit_success_target_within_angle() {
        let result = test_actor_clash(
            0.0, 0.0,
            100.0, 100.0, // angle is 45 deg
            0.0, // facing positive y
            60.0,
            200.0,
            1.0,
            false,
            0.0,
        );
        assert!(result);
    }

    #[test]
    fn test_hit_rejection_target_outside_angle() {
        let result = test_actor_clash(
            0.0, 0.0,
            100.0, 100.0, // angle is 45 deg
            0.0, // facing positive y
            30.0,
            200.0,
            1.0,
            false,
            0.0,
        );
        assert!(!result);
    }

    #[test]
    fn test_hit_success_target_rotated_look() {
        let result = test_actor_clash(
            0.0, 0.0,
            100.0, 0.0, // Target is on x-axis
            90.0, // Facing positive x
            30.0,
            200.0,
            1.0,
            false,
            0.0,
        );
        assert!(result);
    }

    #[test]
    fn test_reach_scale_applies() {
        let result = test_actor_clash(
            0.0, 0.0,
            0.0, 250.0,
            0.0, // facing positive y
            90.0,
            200.0,
            1.5, // Total reach is 300
            false,
            0.0,
        );
        assert!(result);

        let result = test_actor_clash(
            0.0, 0.0,
            0.0, 250.0,
            0.0, // facing positive y
            90.0,
            200.0,
            1.0, // Total reach is 200
            false,
            0.0,
        );
        assert!(!result);
    }
}
