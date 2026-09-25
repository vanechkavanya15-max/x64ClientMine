use m2_client_terrain::collision_detector::{CollisionDetector, Ray, TerrainProvider, Vector3};

struct MockTerrain;

impl TerrainProvider for MockTerrain {
    fn get_height(&self, x: f32, y: f32) -> Option<f32> {
        // Teren to plaska powierzchnia na wysokosci Z = 10.0 dla x, y w przedziale [0, 100]
        if x >= 0.0 && x <= 100.0 && y >= 0.0 && y <= 100.0 {
            Some(10.0)
        } else {
            None
        }
    }

    fn is_walkable(&self, x: f32, y: f32) -> bool {
        // Mozna chodzic tylko w przedziale [10, 90]
        x >= 10.0 && x <= 90.0 && y >= 10.0 && y <= 90.0
    }
}

#[test]
fn test_integration_collision_detector() {
    let terrain = MockTerrain;
    let detector = CollisionDetector::new(terrain);
    assert!(detector.is_ready());
}

#[test]
fn test_get_height_under_player() {
    let terrain = MockTerrain;
    let detector = CollisionDetector::new(terrain);
    
    // Punkt wewnatrz terenu
    assert_eq!(detector.get_height_under_player(50.0, 50.0), Some(10.0));
    
    // Punkt poza terenem
    assert_eq!(detector.get_height_under_player(-10.0, 50.0), None);
}

#[test]
fn test_is_position_walkable() {
    let terrain = MockTerrain;
    let detector = CollisionDetector::new(terrain);
    
    // Punkt walkable
    assert!(detector.is_position_walkable(50.0, 50.0));
    
    // Punkt nie walkable
    assert!(!detector.is_position_walkable(5.0, 50.0));
}

#[test]
fn test_raycast_hit() {
    let terrain = MockTerrain;
    let detector = CollisionDetector::new(terrain);
    
    // Promien skierowany prosto w dol (z = -1) z punktu (50, 50, 20)
    let origin = Vector3::new(50.0, 50.0, 20.0);
    let direction = Vector3::new(0.0, 0.0, -1.0);
    let ray = Ray::new(origin, direction);
    
    // Przeciecie powinno wystapic dla Z=10
    let hit = detector.raycast(&ray, 50.0, 1.0);
    assert!(hit.is_some());
    let hit_point = hit.unwrap();
    assert_eq!(hit_point.x, 50.0);
    assert_eq!(hit_point.y, 50.0);
    assert_eq!(hit_point.z, 10.0);
}

#[test]
fn test_raycast_miss() {
    let terrain = MockTerrain;
    let detector = CollisionDetector::new(terrain);
    
    // Promien skierowany prosto w gore (z = 1) z punktu (50, 50, 20)
    let origin = Vector3::new(50.0, 50.0, 20.0);
    let direction = Vector3::new(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, direction);
    
    // Promien oddala sie od terenu (Z=10), wiec nie bedzie przeciecia
    let hit = detector.raycast(&ray, 50.0, 1.0);
    assert!(hit.is_none());
}

#[test]
fn test_raycast_out_of_bounds() {
    let terrain = MockTerrain;
    let detector = CollisionDetector::new(terrain);
    
    // Promien skierowany w dol, ale poza granicami terenu (X=-10)
    let origin = Vector3::new(-10.0, 50.0, 20.0);
    let direction = Vector3::new(0.0, 0.0, -1.0);
    let ray = Ray::new(origin, direction);
    
    let hit = detector.raycast(&ray, 50.0, 1.0);
    assert!(hit.is_none());
}
