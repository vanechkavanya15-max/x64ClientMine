use m2_client_render::render_scene::RenderScene;

#[test]
fn test_integration_render_scene() {
    let instance = RenderScene::new();
    assert!(instance.is_ready());
}

#[test]
fn test_render_scene_add_and_stats() {
    let mut scene = RenderScene::new();
    
    // Dodajemy obiekty nieprzezroczyste
    scene.add_object(1, 10.0, false, 100);
    scene.add_object(2, 5.0, false, 200);
    
    // Dodajemy obiekty przezroczyste
    scene.add_object(3, 15.0, true, 50);
    scene.add_object(4, 20.0, true, 150);

    let stats = scene.get_stats();
    assert_eq!(stats.draw_calls, 4);
    assert_eq!(stats.rendered_vertices, 500);
    assert_eq!(stats.opaque_objects_count, 2);
    assert_eq!(stats.transparent_objects_count, 2);
}

#[test]
fn test_render_scene_sorting() {
    let mut scene = RenderScene::new();
    
    // Opaque objects
    scene.add_object(1, 10.0, false, 100);
    scene.add_object(2, 5.0, false, 100);
    scene.add_object(3, 15.0, false, 100);

    // Transparent objects
    scene.add_object(4, 10.0, true, 100);
    scene.add_object(5, 5.0, true, 100);
    scene.add_object(6, 15.0, true, 100);

    scene.sort_queues();

    // Opaque: przod-tyl (rosnaco)
    assert_eq!(scene.opaque_queue[0].id, 2); // 5.0
    assert_eq!(scene.opaque_queue[1].id, 1); // 10.0
    assert_eq!(scene.opaque_queue[2].id, 3); // 15.0

    // Transparent: tyl-przod (malejaco)
    assert_eq!(scene.transparent_queue[0].id, 6); // 15.0
    assert_eq!(scene.transparent_queue[1].id, 4); // 10.0
    assert_eq!(scene.transparent_queue[2].id, 5); // 5.0
}

#[test]
fn test_render_scene_clear() {
    let mut scene = RenderScene::new();
    
    scene.add_object(1, 10.0, false, 100);
    scene.add_object(2, 5.0, true, 200);
    
    scene.clear();
    
    let stats = scene.get_stats();
    assert_eq!(stats.draw_calls, 0);
    assert_eq!(stats.rendered_vertices, 0);
    assert_eq!(stats.opaque_objects_count, 0);
    assert_eq!(stats.transparent_objects_count, 0);
    assert!(scene.opaque_queue.is_empty());
    assert!(scene.transparent_queue.is_empty());
}
