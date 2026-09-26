#[path = "../src/splash_attack.rs"]
mod splash_attack;

use glam::Vec3;
use splash_attack::{HitGeometry, TargetInfo, process_splash_attack};

#[test]
fn test_splash_attack_dagger_limit() {
    let geometry = HitGeometry {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(1.0, 0.0, 0.0), // Patrzy wzdluz osi X
        range: 10.0,
        cone_angle_cos: 0.0, // Stozek 180 stopni z przodu
    };

    let targets = vec![
        TargetInfo { vid: 1, position: Vec3::new(2.0, 0.0, 0.0) },
        TargetInfo { vid: 2, position: Vec3::new(3.0, 0.0, 0.0) },
        TargetInfo { vid: 3, position: Vec3::new(4.0, 0.0, 0.0) },
    ];

    // Sztylet uderza 1 cel
    let hits = process_splash_attack(&geometry, 1, &targets);

    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].vid, 1); // Powinien trafic tylko najblizszy
}

#[test]
fn test_splash_attack_sword_limit_and_sorting() {
    let geometry = HitGeometry {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(1.0, 0.0, 0.0),
        range: 10.0,
        cone_angle_cos: 0.5, // 120 stozek, cos(60) = 0.5
    };

    // Kolejnosc nieposortowana po dystansie
    let targets = vec![
        TargetInfo { vid: 4, position: Vec3::new(5.0, 0.0, 0.0) },
        TargetInfo { vid: 5, position: Vec3::new(2.0, 0.0, 0.0) },
        TargetInfo { vid: 6, position: Vec3::new(8.0, 0.0, 0.0) },
        TargetInfo { vid: 7, position: Vec3::new(3.0, 0.0, 0.0) },
    ];

    // Miecz moze uderzyc 3 cele
    let hits = process_splash_attack(&geometry, 3, &targets);

    assert_eq!(hits.len(), 3);
    assert_eq!(hits[0].vid, 5); // 2.0 dist
    assert_eq!(hits[1].vid, 7); // 3.0 dist
    assert_eq!(hits[2].vid, 4); // 5.0 dist
}

#[test]
fn test_splash_attack_cone_filtering() {
    let geometry = HitGeometry {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(1.0, 0.0, 0.0), // Wzdluz X
        range: 10.0,
        cone_angle_cos: 0.707, // ~90 stopni (cos 45 = 0.707)
    };

    let targets = vec![
        TargetInfo { vid: 1, position: Vec3::new(5.0, 0.0, 0.0) }, // Na wprost (wewnatrz)
        TargetInfo { vid: 2, position: Vec3::new(0.0, 5.0, 0.0) }, // Obok (kat 90 stopni, a polowa kata to 45, wiec odpada)
        TargetInfo { vid: 3, position: Vec3::new(-5.0, 0.0, 0.0) }, // Z tylu (odpada)
    ];

    let hits = process_splash_attack(&geometry, 3, &targets);

    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].vid, 1);
}

#[test]
fn test_splash_attack_range_filtering() {
    let geometry = HitGeometry {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(1.0, 0.0, 0.0),
        range: 5.0,
        cone_angle_cos: 0.0,
    };

    let targets = vec![
        TargetInfo { vid: 1, position: Vec3::new(3.0, 0.0, 0.0) }, // W zasiegu
        TargetInfo { vid: 2, position: Vec3::new(5.1, 0.0, 0.0) }, // Poza zasiegiem
    ];

    let hits = process_splash_attack(&geometry, 3, &targets);

    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].vid, 1);
}

#[test]
fn test_splash_attack_exact_overlap() {
    let geometry = HitGeometry {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(1.0, 0.0, 0.0),
        range: 5.0,
        cone_angle_cos: 0.5,
    };

    let targets = vec![
        TargetInfo { vid: 1, position: Vec3::new(0.0, 0.0, 0.0) }, // Idealnie na srodku uderzenia
    ];

    let hits = process_splash_attack(&geometry, 3, &targets);

    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].vid, 1);
}

#[test]
fn test_splash_attack_zero_limit() {
    let geometry = HitGeometry {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(1.0, 0.0, 0.0),
        range: 5.0,
        cone_angle_cos: 0.5,
    };

    let targets = vec![
        TargetInfo { vid: 1, position: Vec3::new(3.0, 0.0, 0.0) },
    ];

    let hits = process_splash_attack(&geometry, 0, &targets);

    assert_eq!(hits.len(), 0);
}
