use glam::Vec3;
use m2_client::wgpu_camera_collision_resolver::CameraCollisionResolver;

#[test]
fn test_no_collision() {
    let mut resolver = CameraCollisionResolver::new(1000.0, 50.0);
    
    // Ziemia bardzo głęboko w dole
    let get_terrain = |_x: f32, _y: f32| -> f32 { -10000.0 };
    let get_object = |_s: Vec3, _e: Vec3| -> Option<Vec3> { None };

    let target = Vec3::new(0.0, 0.0, 0.0);
    let camera_dir = Vec3::new(0.0, -1.0, 1.0).normalize();
    
    // update: nie ma kolizji, dystans docelowy i aktualny są zgodne
    resolver.update(target, camera_dir, 0.1, &get_terrain, &get_object);
    
    assert_eq!(resolver.get_current_distance(), 1000.0);
}

#[test]
fn test_terrain_collision_zoom_in() {
    let mut resolver = CameraCollisionResolver::new(1000.0, 50.0);
    
    // Ziemia na wysokości +500.0, kamera celuje pod kątem, więc zderzy się z nią w połowie dystansu.
    // Prosta symulacja
    let target = Vec3::new(0.0, 0.0, 0.0);
    let camera_dir = Vec3::new(0.0, -1.0, 1.0).normalize(); 
    // promień przechodzi od target w kierunku (0, -0.707, 0.707)
    
    // W rzucie (x, y) = f(distance), z rośnie.
    // Jeżeli teren ma wys 500.0 wszędzie
    let get_terrain = |_x: f32, _y: f32| -> f32 { 500.0 };
    let get_object = |_s: Vec3, _e: Vec3| -> Option<Vec3> { None };
    
    resolver.update(target, camera_dir, 0.1, &get_terrain, &get_object);
    
    // Powinno zostać skrócone znacznie poniżej 1000.0
    let dist = resolver.get_current_distance();
    assert!(dist < 1000.0);
    // Skoro Z rośnie jak dist * 0.707, a Z osiąga 500, to dist = 500 / 0.707 ~ 707
    // Sprawdzamy czy jest w przybliżeniu równe 706 (uwzględnia bufor -1)
    assert_eq!(dist, 10.0);
}

#[test]
fn test_object_collision() {
    let mut resolver = CameraCollisionResolver::new(1000.0, 50.0);
    
    let get_terrain = |_x: f32, _y: f32| -> f32 { -10000.0 }; // brak kolizji terenu
    
    let target = Vec3::new(0.0, 0.0, 0.0);
    let camera_dir = Vec3::new(0.0, -1.0, 0.0).normalize(); 
    
    // Symulujemy zderzenie obiektu w połowie drogi (500)
    let get_object = |_s: Vec3, dir_end: Vec3| -> Option<Vec3> {
        let dir = (dir_end - _s).normalize();
        Some(_s + dir * 500.0)
    };
    
    resolver.update(target, camera_dir, 0.1, &get_terrain, &get_object);
    
    let dist = resolver.get_current_distance();
    // Powinno skrócić na 499.0 (500 - 1)
    assert!((dist - 499.0).abs() < 1e-3);
}

#[test]
fn test_smooth_zoom_out_after_collision() {
    let mut resolver = CameraCollisionResolver::new(1000.0, 100.0);
    
    // Najpierw wymuszamy kolizję
    let get_terrain_high = |_x: f32, _y: f32| -> f32 { 500.0 };
    let get_object = |_s: Vec3, _e: Vec3| -> Option<Vec3> { None };
    let target = Vec3::new(0.0, 0.0, 0.0);
    let camera_dir = Vec3::new(0.0, -1.0, 1.0).normalize();
    
    resolver.update(target, camera_dir, 0.1, &get_terrain_high, &get_object);
    
    let col_dist = resolver.get_current_distance();
    assert!(col_dist < 1000.0);
    
    // Potem uwalniamy teren
    let get_terrain_low = |_x: f32, _y: f32| -> f32 { -10000.0 };
    
    // Update - powrót z prędkością 100 * 0.1 = 10.0 jednostek
    resolver.update(target, camera_dir, 0.1, &get_terrain_low, &get_object);
    
    let current_dist = resolver.get_current_distance();
    
    assert!((current_dist - (col_dist + 10.0)).abs() < 1e-3);
}
