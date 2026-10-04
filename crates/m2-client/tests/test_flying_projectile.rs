#[path = "../src/wgpu_flying_projectile_homing.rs"]
mod wgpu_flying_projectile_homing;

use wgpu_flying_projectile_homing::{
    calculate_bezier_trajectory, FlyTarget, FlyingProjectileData, FlyingProjectileHoming, FlyingStatus,
};

#[test]
fn test_simple_parabolic_flight() {
    let start_pos = (0.0, 0.0, 0.0);
    let start_vel = (100.0, 0.0, 0.0);
    
    let mut data = FlyingProjectileData::default();
    data.gravity = 9.8;
    data.accel = (0.0, 0.0, 0.0);

    let mut homing = FlyingProjectileHoming::new(start_pos, start_vel, FlyTarget::None, data);

    // Krok 1 sekundy (dt = 1.0)
    let move_dist = homing.calculate_step(1.0);
    
    // Prędkość Z zmaleje o 9.8 => velocity.2 = -9.8
    assert_eq!(homing.velocity.2, -9.8);
    // Pozycja X wzrośnie o 100 => pos.0 = 100
    assert_eq!(homing.pos.0, 100.0);
    // Pozycja Z zmieni się o (velocity.2 * dt) = -9.8
    assert_eq!(homing.pos.2, -9.8);
    // Dystans przebyty to sqrt(100^2 + 0^2 + (-9.8)^2)
    let expected_dist = (100.0f32.powi(2) + (-9.8f32).powi(2)).sqrt();
    assert!((move_dist - expected_dist).abs() < 0.001);
}

#[test]
fn test_homing_direction_adjustment() {
    let start_pos = (0.0, 0.0, 0.0);
    // Lot w kierunku osi Y
    let start_vel = (0.0, 100.0, 0.0);
    
    let mut data = FlyingProjectileData::default();
    data.is_homing = true;
    data.homing_max_angle = 90.0; // Możliwy obrót do 90 stopni
    data.homing_start_time = 0.0; // Naprowadzanie od samego początku

    let mut homing = FlyingProjectileHoming::new(start_pos, start_vel, FlyTarget::Position(100.0, 100.0, 0.0), data);
    
    homing.set_time(1.0);
    let status = homing.update(1.0, |_| None);
    
    assert_eq!(status, FlyingStatus::Flying);

    // Prędkość powinna się lekko wychylić z osi Y na oś X, 
    // ponieważ cel leży pod kątem 45 stopni od pierwotnego wektora prędkości.
    assert!(homing.velocity.0 > 0.0, "Velocity X should increase due to homing");
    assert!(homing.velocity.1 > 0.0, "Velocity Y should remain positive");
}

#[test]
fn test_hit_detection() {
    let start_pos = (0.0, 0.0, 0.0);
    let start_vel = (10.0, 0.0, 0.0);
    
    let mut data = FlyingProjectileData::default();
    data.bomb_range = 5.0; // Zasięg detekcji trafienia
    
    // Cel leży wzdłuż przewidywanej trajektorii i znajduje się na linii (między (0,0,0) a (10,0,0))
    let mut homing = FlyingProjectileHoming::new(start_pos, start_vel, FlyTarget::Position(5.0, 1.0, 0.0), data);

    // Po 1 kroku pocisk przebędzie odcinek od (0,0,0) do (10,0,0)
    let status = homing.update(1.0, |_| None);
    
    // Punkt (5,1,0) jest odległy od odcinka [ (0,0,0), (10,0,0) ] o 1 jednostkę.
    // 1 < 5 (bomb_range), więc powinno dojść do zderzenia
    assert_eq!(status, FlyingStatus::Hit);
}

#[test]
fn test_hit_detection_out_of_range() {
    let start_pos = (0.0, 0.0, 0.0);
    let start_vel = (10.0, 0.0, 0.0);
    
    let mut data = FlyingProjectileData::default();
    data.bomb_range = 1.0; 
    
    // Cel znajduje się poza odcinkiem na osi Y (odległość = 5)
    let mut homing = FlyingProjectileHoming::new(start_pos, start_vel, FlyTarget::Position(5.0, 5.0, 0.0), data);

    let status = homing.update(1.0, |_| None);
    
    assert_eq!(status, FlyingStatus::Flying);
}

#[test]
fn test_bezier_trajectory() {
    let p0 = (0.0, 0.0, 0.0);
    let p1 = (5.0, 10.0, 5.0);
    let p2 = (10.0, 0.0, 0.0);

    let pt_start = calculate_bezier_trajectory(p0, p1, p2, 0.0);
    assert_eq!(pt_start, p0);

    let pt_end = calculate_bezier_trajectory(p0, p1, p2, 1.0);
    assert_eq!(pt_end, p2);

    let pt_mid = calculate_bezier_trajectory(p0, p1, p2, 0.5);
    // Dla t=0.5: x = 0.25*0 + 2*0.5*0.5*5 + 0.25*10 = 0 + 2.5 + 2.5 = 5.0
    // y = 0.25*0 + 2*0.5*0.5*10 + 0.25*0 = 5.0
    // z = 0.25*0 + 2*0.5*0.5*5 + 0.25*0 = 2.5
    assert!((pt_mid.0 - 5.0).abs() < 0.001);
    assert!((pt_mid.1 - 5.0).abs() < 0.001);
    assert!((pt_mid.2 - 2.5).abs() < 0.001);
}

#[test]
fn test_homing_vid_target() {
    let start_pos = (0.0, 0.0, 0.0);
    let start_vel = (50.0, 0.0, 0.0);
    
    let mut data = FlyingProjectileData::default();
    data.bomb_range = 5.0; 
    
    let target_vid = 1234;
    let mut homing = FlyingProjectileHoming::new(start_pos, start_vel, FlyTarget::Object(target_vid), data);

    // Dynamiczna funkcja podająca pozycję obiektu VID
    let get_target_pos = |vid: u32| -> Option<(f32, f32, f32)> {
        if vid == 1234 {
            Some((25.0, 0.0, 0.0)) // Cel po drodze lotu
        } else {
            None
        }
    };

    let status = homing.update(1.0, get_target_pos);
    assert_eq!(status, FlyingStatus::Hit);
}
