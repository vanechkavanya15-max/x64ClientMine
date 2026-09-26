#[path = "../src/actor_transform.rs"]
mod actor_transform;
use actor_transform::ActorTransform;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_default() {
        let default_transform = ActorTransform::default();
        assert_eq!(default_transform.x, 0.0);
        assert_eq!(default_transform.y, 0.0);
        assert_eq!(default_transform.z, 0.0);
        assert_eq!(default_transform.rot_deg, 0.0);

        let new_transform = ActorTransform::new(10.0, -20.0, 30.0, 450.0);
        assert_eq!(new_transform.x, 10.0);
        assert_eq!(new_transform.y, -20.0);
        assert_eq!(new_transform.z, 30.0);
        assert_eq!(new_transform.rot_deg, 90.0);
    }

    #[test]
    fn test_look_at() {
        let mut transform = ActorTransform::new(0.0, 0.0, 0.0, 0.0);
        
        // Target at +Y (dx=0, dy=10) -> atan2(0, 10) = 0 radians -> 0 degrees
        transform.look_at(0.0, 10.0);
        assert_eq!(transform.rot_deg, 0.0);
        
        // Target at +X (dx=10, dy=0) -> atan2(10, 0) = PI/2 radians -> 90 degrees
        transform.look_at(10.0, 0.0);
        assert_eq!(transform.rot_deg, 90.0);
        
        // Target at -Y (dx=0, dy=-10) -> atan2(0, -10) = PI radians -> 180 degrees
        transform.look_at(0.0, -10.0);
        assert_eq!(transform.rot_deg, 180.0);

        // Target at -X (dx=-10, dy=0) -> atan2(-10, 0) = -PI/2 radians -> 270 degrees
        transform.look_at(-10.0, 0.0);
        assert_eq!(transform.rot_deg, 270.0);

        // Target at -X, +Y (dx=-10, dy=10) -> atan2(-10, 10) = -PI/4 radians -> 315 degrees
        transform.look_at(-10.0, 10.0);
        assert_eq!(transform.rot_deg, 315.0);

        // Non-origin source position
        let mut transform2 = ActorTransform::new(5.0, 5.0, 0.0, 0.0);
        // Target at X=15, Y=5 -> dx=10, dy=0 -> 90 degrees
        transform2.look_at(15.0, 5.0);
        assert_eq!(transform2.rot_deg, 90.0);
    }

    #[test]
    fn test_normalize_rotation() {
        let mut t1 = ActorTransform::new(0.0, 0.0, 0.0, 0.0);
        
        t1.rot_deg = 360.0;
        t1.normalize_rotation();
        assert_eq!(t1.rot_deg, 0.0);
        
        t1.rot_deg = 450.0;
        t1.normalize_rotation();
        assert_eq!(t1.rot_deg, 90.0);
        
        t1.rot_deg = -90.0;
        t1.normalize_rotation();
        assert_eq!(t1.rot_deg, 270.0);

        t1.rot_deg = -360.0;
        t1.normalize_rotation();
        assert_eq!(t1.rot_deg, 0.0);

        t1.rot_deg = -450.0;
        t1.normalize_rotation();
        assert_eq!(t1.rot_deg, 270.0);
        
        t1.rot_deg = 720.0;
        t1.normalize_rotation();
        assert_eq!(t1.rot_deg, 0.0);
    }

    #[test]
    fn test_distance_2d() {
        let transform = ActorTransform::new(0.0, 0.0, 0.0, 0.0);
        
        // Origin to 3,4 is 5
        let dist = transform.distance_2d(3.0, 4.0);
        assert!((dist - 5.0).abs() < 1e-5);
        
        // Same position
        let dist_same = transform.distance_2d(0.0, 0.0);
        assert_eq!(dist_same, 0.0);
        
        // Negative coordinates
        let transform2 = ActorTransform::new(-5.0, -5.0, 0.0, 0.0);
        let dist2 = transform2.distance_2d(-2.0, -1.0); // dx = 3, dy = 4 => dist = 5
        assert!((dist2 - 5.0).abs() < 1e-5);
    }

    #[test]
    fn test_distance_3d() {
        let transform = ActorTransform::new(0.0, 0.0, 0.0, 0.0);
        
        // Origin to 3,4,12 is 13 (sqrt(9+16+144) = sqrt(169))
        let dist = transform.distance_3d(3.0, 4.0, 12.0);
        assert!((dist - 13.0).abs() < 1e-5);
        
        // Same position
        let dist_same = transform.distance_3d(0.0, 0.0, 0.0);
        assert_eq!(dist_same, 0.0);
        
        // Negative coordinates
        let transform2 = ActorTransform::new(-3.0, -4.0, -12.0, 0.0);
        let dist2 = transform2.distance_3d(0.0, 0.0, 0.0);
        assert!((dist2 - 13.0).abs() < 1e-5);
    }
}
