use m2_client::game_combat_sphere_collision_system::{
    CollisionSphere, CombatTargetActor, GameCombatSphereCollisionSystem, Vec3,
};

#[test]
fn test_collision_sphere_intersection() {
    let s1 = CollisionSphere::new(Vec3::new(0.0, 0.0, 0.0), 50.0);
    let s2 = CollisionSphere::new(Vec3::new(60.0, 0.0, 0.0), 20.0);
    let s3 = CollisionSphere::new(Vec3::new(100.0, 0.0, 0.0), 20.0);

    // Odleglosc s1 do s2 = 60, r1 + r2 = 70 -> przecinaja sie
    assert!(s1.intersects(&s2));
    assert!(s2.intersects(&s1));

    // Odleglosc s1 do s3 = 100, r1 + r3 = 70 -> brak przeciecia
    assert!(!s1.intersects(&s3));
}

#[test]
fn test_combat_collision_hit_and_prevent_double_hit() {
    let mut system = GameCombatSphereCollisionSystem::new();

    let mut enemy = CombatTargetActor::new(1001, false);
    enemy.add_defending_sphere(Vec3::new(100.0, 0.0, 0.0), 40.0);

    let mut ally = CombatTargetActor::new(1002, true);
    ally.add_defending_sphere(Vec3::new(100.0, 0.0, 0.0), 40.0);

    let weapon_spheres = vec![
        CollisionSphere::new(Vec3::new(90.0, 0.0, 0.0), 30.0),
    ];

    let targets = vec![enemy, ally];

    // Pierwsza klatka machniecia
    let hits1 = system.test_attack_collision(&weapon_spheres, &targets);
    assert_eq!(hits1.len(), 1);
    assert_eq!(hits1[0].target_vid, 1001); // Trafilismy wroga, sojusznik pominiety

    // Druga klatka tego samego machniecia: wrog juz nie powinien byc ponownie uderzony
    let hits2 = system.test_attack_collision(&weapon_spheres, &targets);
    assert_eq!(hits2.len(), 0);

    // Po resecie zamachu (nowy cios): mozemy znow trafic wroga
    system.reset_swing();
    let hits3 = system.test_attack_collision(&weapon_spheres, &targets);
    assert_eq!(hits3.len(), 1);
    assert_eq!(hits3[0].target_vid, 1001);
}
