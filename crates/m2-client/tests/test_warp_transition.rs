use m2_client::warp_transition::WarpTransition;

#[test]
fn test_warp_resets_motion_vectors() {
    let mut transition = WarpTransition::new();
    transition.set_velocity(10.0, 20.0, 30.0);
    transition.add_interpolation_point(1.0, 2.0, 3.0);

    assert_eq!(transition.velocity, [10.0, 20.0, 30.0]);
    assert_eq!(transition.interpolation_queue.len(), 1);

    transition.warp(1000, 2000, 500, 500, 10.5);

    assert_eq!(transition.velocity, [0.0, 0.0, 0.0]);
    assert_eq!(transition.interpolation_queue.len(), 0);
}

#[test]
fn test_warp_sets_correct_base_and_local_pos() {
    let mut transition = WarpTransition::new();

    transition.warp(1000, 2000, 500, 500, 10.5);

    assert_eq!(transition.map_base_x, 500);
    assert_eq!(transition.map_base_y, 500);

    assert_eq!(transition.local_pos, [500.0, 1500.0, 10.5]);
}

#[test]
fn test_warp_locks_physics_while_loading_map() {
    let mut transition = WarpTransition::new();

    transition.warp(1000, 2000, 500, 500, 10.5);
    assert!(transition.is_physics_locked());

    // Proba ustawienia predkosci - powinna byc zablokowana
    transition.set_velocity(10.0, 20.0, 30.0);
    assert_eq!(transition.velocity, [0.0, 0.0, 0.0]);

    // Proba dodania punktu interpolacji - powinna byc zablokowana
    transition.add_interpolation_point(1.0, 2.0, 3.0);
    assert_eq!(transition.interpolation_queue.len(), 0);
}

#[test]
fn test_physics_unlocked_after_terrain_loaded() {
    let mut transition = WarpTransition::new();

    transition.warp(1000, 2000, 500, 500, 10.5);
    assert!(transition.is_physics_locked());

    transition.on_terrain_loaded();
    assert!(!transition.is_physics_locked());

    // Ustawienie predkosci - powinno dzialac
    transition.set_velocity(10.0, 20.0, 30.0);
    assert_eq!(transition.velocity, [10.0, 20.0, 30.0]);

    // Dodanie punktu interpolacji - powinno dzialac
    transition.add_interpolation_point(1.0, 2.0, 3.0);
    assert_eq!(transition.interpolation_queue.len(), 1);
}

#[test]
fn test_negative_coordinates() {
    let mut transition = WarpTransition::new();

    // Negatywne koordynaty, np. global jest ujemny a bazy sa ujemne, lub odwrotnie
    transition.warp(-500, 300, -1000, 0, -5.0);

    assert_eq!(transition.map_base_x, -1000);
    assert_eq!(transition.map_base_y, 0);

    assert_eq!(transition.local_pos, [500.0, 300.0, -5.0]);
}

#[test]
fn test_default_implementation() {
    let transition = WarpTransition::default();
    
    assert_eq!(transition.velocity, [0.0, 0.0, 0.0]);
    assert_eq!(transition.interpolation_queue.len(), 0);
    assert_eq!(transition.map_base_x, 0);
    assert_eq!(transition.map_base_y, 0);
    assert_eq!(transition.local_pos, [0.0, 0.0, 0.0]);
    assert!(!transition.is_physics_locked());
}
