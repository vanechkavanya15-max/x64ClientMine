use m2_client::camera_terrain_clamp_v2::*;
use glam::Vec3;

#[test]
fn test_no_collision_above_terrain() {
    let eye = Vec3::new(0.0, 0.0, 1000.0);
    let target = Vec3::new(0.0, 100.0, 900.0);
    
    // Zwracamy bardzo niski teren (0.0) - zaden promien nie powinien go przeciac.
    let get_height = |_, _| 0.0;
    
    let result = process_terrain_collision(eye, target, &get_height);
    
    assert_eq!(result.x, 0.0);
    assert_eq!(result.y, 0.0);
    assert_eq!(result.z, 1000.0);
}

#[test]
fn test_target_to_camera_bottom_collision() {
    // Kamera ustawiona tak by promien przecial gore.
    let eye = Vec3::new(0.0, 0.0, 100.0);
    let target = Vec3::new(0.0, 100.0, 100.0);
    
    // Gdy sprawdzany jest teren w poblizu oczka (0,0), niech zwraca gore = 200.0
    // To podbije nam oczko w gore. 
    let get_height = |x: f32, y: f32| {
        if x.abs() < 50.0 && y.abs() < 50.0 {
            200.0
        } else {
            0.0
        }
    };
    
    let result = process_terrain_collision(eye, target, &get_height);
    
    // Oczko bylo na (0,0,100). Poniewaz teren w (0,0) ma 200, 
    // ray uderzy w gore (lub sama funkcja podniesie na podstawie check_vector).
    // Nowe oczko to z reguly wysokosc terenu (200) + 2*Radius (2*50 = 100) -> z = 300
    // Sprawdzmy sam fakt podbicia.
    assert!(result.z > 100.0, "Oczko powinno zostac podbite przez pierwsza fole kolizji");
}

#[test]
fn test_camera_bottom_to_terrain_collision() {
    let eye = Vec3::new(100.0, 100.0, 50.0);
    let target = Vec3::new(200.0, 200.0, 50.0);
    
    // Oczko jest blisko terenu. m_kCameraBottomToTerrainRay idzie w dol od oka o 2 * TCR (100).
    // Czyli od Z=50 w dol do Z=-50. Jezeli teren ma Z=25, to zglosi hit.
    let get_height = |x: f32, y: f32| {
        if (x - 100.0).abs() < 10.0 && (y - 100.0).abs() < 10.0 {
            25.0
        } else {
            -1000.0
        }
    };
    
    let result = process_terrain_collision(eye, target, &get_height);
    
    // Jesli uderzy, wynik to hit_point + up * 2 * radius (up_z = 1.0 dla poziomego view)
    // hit Z = 25.0. 25 + 100 = 125.
    assert!(result.z >= 125.0, "Oczko zostalo podbite do {}", result.z);
}

#[test]
fn test_epsilon_zero_distance() {
    let eye = Vec3::new(0.0, 0.0, 0.0);
    let target = Vec3::new(0.0, 0.0, 0.0);
    
    let get_height = |_, _| -100.0;
    
    // Nie powinno sypac zerami ani panicami. 
    let result = process_terrain_collision(eye, target, &get_height);
    
    assert_eq!(result, eye);
}

#[test]
fn test_steep_view_vector() {
    // Kamera patrzy mocno w dol (niemal pionowo)
    let eye = Vec3::new(0.0, 0.1, 1000.0);
    let target = Vec3::new(0.0, 0.0, 0.0);
    
    let get_height = |_, _| -100.0;
    
    let result = process_terrain_collision(eye, target, &get_height);
    
    assert_eq!(result.x, 0.0);
    assert_eq!(result.y, 0.1);
    assert_eq!(result.z, 1000.0);
}
