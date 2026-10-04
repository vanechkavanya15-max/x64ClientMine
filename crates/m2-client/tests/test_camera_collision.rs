use glam::Vec3;
use m2_client::wgpu_camera_collision_resolver::CameraCollisionResolver;

#[test]
fn test_no_collision() {
    let mut resolver = CameraCollisionResolver::new(1000.0, 50.0);
    
    // Ziemia głęboko pod celem
    let get_terrain = |_x: f32, _y: f32| -> f32 { -10000.0 };
    let get_object = |_s: Vec3, _e: Vec3| -> Option<Vec3> { None };

    let target = Vec3::new(0.0, 0.0, 0.0);
    let camera_dir = Vec3::new(0.0, -1.0, 1.0).normalize();
    
    // Brak kolizji - dystans powinien pozostać zgodny z zadaniem
    resolver.update(target, camera_dir, 0.1, &get_terrain, &get_object);
    
    assert_eq!(resolver.get_current_distance(), 1000.0);
}

#[test]
fn test_terrain_collision_zoom_in() {
    let mut resolver = CameraCollisionResolver::new(1000.0, 50.0);
    
    // Ustawiamy głowę gracza na jakiejś normalnej wysokości
    let target = Vec3::new(0.0, 0.0, 100.0);
    
    // Kamera patrzy na cel z góry pod kątem (wektor Od celu Do kamery)
    // Niech wektor idzie poziomo wzdłuż osi Y, by łatwo policzyć odległość.
    // Oś Y będzie malała.
    let camera_dir = Vec3::new(0.0, -1.0, 0.0).normalize(); 
    
    // Ziemia to "ściana", która rośnie gwałtownie do z = 500.0, 
    // gdy Y osiągnie próg -200.0.
    // Ponieważ cel ma Z=100.0, a teren skoczy na 500.0, nastąpi kolizja, 
    // gdy promień osiągnie Y <= -200.0.
    // Promień ma direction (0.0, -1.0, 0.0), więc punkt w odległości 200.0 
    // ma pozycję (0.0, -200.0, 100.0).
    // Kiedy f_pos osiągnie 200.0, wykryjemy teren na 500.0 > 100.0.
    let get_terrain = |_x: f32, y: f32| -> f32 { 
        if y <= -200.0 { 500.0 } else { 0.0 }
    };
    let get_object = |_s: Vec3, _e: Vec3| -> Option<Vec3> { None };
    
    resolver.update(target, camera_dir, 0.1, &get_terrain, &get_object);
    
    let dist = resolver.get_current_distance();
    
    // closest_distance zostanie wyliczone jako dystans od target (0,0,100) 
    // do punktu zderzenia (0,-200,100). Czyli 200.0.
    // Następnie safe_distance odlicza bufor 1.0, więc current_distance
    // powinno wynieść dokładnie 199.0.
    // Używamy znikomej tolerancji dla błędów float'a, 
    // ale dzięki skokom step=5/10 powinno być bliskie równe.
    assert!((dist - 199.0).abs() < 5.0, "Dystans zderzenia z terenem jest błędny: {}", dist);
}

#[test]
fn test_object_collision() {
    let mut resolver = CameraCollisionResolver::new(1000.0, 50.0);
    
    let get_terrain = |_x: f32, _y: f32| -> f32 { -10000.0 }; // brak kolizji terenu
    
    let target = Vec3::new(0.0, 0.0, 0.0);
    let camera_dir = Vec3::new(0.0, -1.0, 0.0).normalize(); 
    
    // Symulujemy zderzenie z obiektem w połowie drogi (na dystansie 500.0)
    let get_object = |s: Vec3, dir_end: Vec3| -> Option<Vec3> {
        let dir = (dir_end - s).normalize();
        Some(s + dir * 500.0)
    };
    
    resolver.update(target, camera_dir, 0.1, &get_terrain, &get_object);
    
    let dist = resolver.get_current_distance();
    // closest_distance to 500.0, safe_distance z buforem 1.0 to 499.0.
    assert!((dist - 499.0).abs() < 1e-3, "Dystans zderzenia z obiektem jest błędny: {}", dist);
}

#[test]
fn test_smooth_zoom_out_after_collision() {
    let mut resolver = CameraCollisionResolver::new(1000.0, 100.0);
    
    // Najpierw wymuszamy kolizję z wysoką ziemią, np tuż za celem
    let get_terrain_high = |_x: f32, _y: f32| -> f32 { 500.0 };
    let get_object = |_s: Vec3, _e: Vec3| -> Option<Vec3> { None };
    let target = Vec3::new(0.0, 0.0, 100.0);
    let camera_dir = Vec3::new(0.0, -1.0, 0.0).normalize();
    
    resolver.update(target, camera_dir, 0.1, &get_terrain_high, &get_object);
    
    let col_dist = resolver.get_current_distance();
    // Ponieważ teren ma 500.0 zaraz w punkcie startowym i wszędzie, kolizja jest niemal natychmiast, 
    // safe_distance może wynosić minimum 10.0 (clamp)
    assert!(col_dist <= 10.0);
    
    // Potem uwalniamy teren (kamera wyszła z podziemnego zablokowania)
    let get_terrain_low = |_x: f32, _y: f32| -> f32 { -10000.0 };
    
    // Update - powrót z prędkością 100 * 0.1 = 10.0 jednostek na klatkę
    resolver.update(target, camera_dir, 0.1, &get_terrain_low, &get_object);
    
    let current_dist = resolver.get_current_distance();
    
    assert!((current_dist - (col_dist + 10.0)).abs() < 1e-3, "Płynny zoom po uwolnieniu nie działa: {}", current_dist);
}
