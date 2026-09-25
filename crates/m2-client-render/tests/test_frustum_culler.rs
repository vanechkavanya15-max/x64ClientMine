use m2_client_render::frustum_culler::FrustumCuller;

#[test]
fn test_integration_frustum_culler() {
    let instance = FrustumCuller::new();
    assert!(instance.is_ready());
}

#[test]
fn test_frustum_update_and_culling() {
    let mut culler = FrustumCuller::new();
    
    // Macierz tozsamosciowa - frustum to szescian [-1, 1]
    let identity = [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    
    culler.update(&identity);
    
    // Testowanie punktow
    assert!(culler.test_point(&[0.0, 0.0, 0.0]));
    assert!(culler.test_point(&[0.9, 0.9, 0.9]));
    assert!(!culler.test_point(&[1.1, 0.0, 0.0]));
    assert!(!culler.test_point(&[0.0, -1.1, 0.0]));
    assert!(!culler.test_point(&[0.0, 0.0, 2.0]));

    // Testowanie kuli
    assert!(culler.test_sphere(&[0.0, 0.0, 0.0], 0.5));
    assert!(culler.test_sphere(&[1.0, 0.0, 0.0], 0.5)); // Srodek na krawedzi, promien sie przecina
    assert!(!culler.test_sphere(&[2.0, 0.0, 0.0], 0.5)); // Calkowicie na zewnatrz
    assert!(culler.test_sphere(&[-1.1, 0.0, 0.0], 0.2)); // Srodek poza, ale promien siega
    assert!(!culler.test_sphere(&[-1.5, 0.0, 0.0], 0.2));

    // Testowanie bryly AABB (min, max)
    assert!(culler.test_aabb(&[-0.5, -0.5, -0.5], &[0.5, 0.5, 0.5])); // Wewnatrz
    assert!(culler.test_aabb(&[0.5, 0.5, 0.5], &[1.5, 1.5, 1.5])); // Przecina
    assert!(!culler.test_aabb(&[1.1, 1.1, 1.1], &[2.0, 2.0, 2.0])); // Na zewnatrz
    assert!(!culler.test_aabb(&[-2.0, -2.0, -2.0], &[-1.1, -1.1, -1.1])); // Na zewnatrz
}
