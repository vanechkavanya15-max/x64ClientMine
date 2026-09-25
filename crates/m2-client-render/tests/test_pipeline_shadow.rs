use m2_client_render::pipeline_shadow::PipelineShadow;

#[test]
fn test_integration_pipeline_shadow() {
    let mut instance = PipelineShadow::new(1024);
    
    // Testowanie wstepnej inicjalizacji
    assert!(instance.is_ready());
    assert_eq!(instance.resolution, 1024);
    
    // Testowanie ustawiania parametrow slonca
    instance.set_sun_parameters([50.0, 100.0, 50.0], [10.0, 0.0, 10.0]);
    assert_eq!(instance.sun_position, [50.0, 100.0, 50.0]);
    assert_eq!(instance.sun_target, [10.0, 0.0, 10.0]);
    
    // Testowanie aktualizacji macierzy
    instance.update_matrices();
    let proj = instance.get_light_view_proj();
    assert_eq!(proj[0][0], 2.0);
    assert_eq!(proj[3][0], 50.0);
    assert_eq!(proj[3][1], 100.0);
    assert_eq!(proj[3][2], 50.0);
    
    // Testowanie generowania mapy glebi
    assert!(instance.generate_depth_map());
}
