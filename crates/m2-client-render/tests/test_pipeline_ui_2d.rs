use m2_client_render::pipeline_ui_2d::PipelineUi2d;

#[test]
fn test_integration_pipeline_ui_2d() {
    let mut instance = PipelineUi2d::new();
    assert!(instance.is_ready());

    instance.set_screen_size(1024.0, 768.0);
    assert_eq!(instance.screen_width, 1024.0);
    assert_eq!(instance.screen_height, 768.0);

    instance.begin();
    assert_eq!(instance.vertices().len(), 0);
    assert_eq!(instance.indices().len(), 0);
    assert_eq!(instance.batches().len(), 0);
    
    // Draw without texture
    instance.draw_rect(10.0, 10.0, 100.0, 50.0, [1.0, 1.0, 1.0, 1.0]);
    
    // Check vertices and indices count
    assert_eq!(instance.vertices().len(), 4);
    assert_eq!(instance.indices().len(), 6);
    assert_eq!(instance.batches().len(), 0); // Not flushed yet

    // Change texture, which should flush
    instance.set_texture(Some(1234));
    
    assert_eq!(instance.batches().len(), 1);
    assert_eq!(instance.batches()[0].texture_id, None);
    assert_eq!(instance.batches()[0].index_start, 0);
    assert_eq!(instance.batches()[0].index_count, 6);

    // Draw with texture
    instance.draw_textured_rect(20.0, 20.0, 200.0, 100.0, [0.0, 0.0, 0.5, 0.5], [1.0, 0.0, 0.0, 1.0]);

    // Draw another with same texture (should batch together)
    instance.draw_textured_rect(30.0, 30.0, 50.0, 50.0, [0.5, 0.5, 1.0, 1.0], [0.0, 1.0, 0.0, 1.0]);

    instance.end();
    
    assert_eq!(instance.vertices().len(), 12);
    assert_eq!(instance.indices().len(), 18);
    assert_eq!(instance.batches().len(), 2);

    assert_eq!(instance.batches()[1].texture_id, Some(1234));
    assert_eq!(instance.batches()[1].index_start, 6);
    assert_eq!(instance.batches()[1].index_count, 12);
    
    // Verify first vertex of the second rect
    let v = &instance.vertices()[4];
    assert_eq!(v.position, [20.0, 20.0]);
    assert_eq!(v.uv, [0.0, 0.0]);
    assert_eq!(v.color, [1.0, 0.0, 0.0, 1.0]);
    
    // Check that begin clears everything
    instance.begin();
    assert_eq!(instance.vertices().len(), 0);
    assert_eq!(instance.indices().len(), 0);
    assert_eq!(instance.batches().len(), 0);
}
