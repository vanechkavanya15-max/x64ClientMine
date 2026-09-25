use m2_client_render::pipeline_effect::{Billboard, BlendMode, PipelineEffect};

#[test]
fn test_integration_pipeline_effect() {
    let instance = PipelineEffect::new();
    assert!(instance.is_ready());
}

#[test]
fn test_billboard_default_values() {
    let bb = Billboard::default();
    assert_eq!(bb.position, [0.0, 0.0, 0.0]);
    assert_eq!(bb.size, [1.0, 1.0]);
    assert_eq!(bb.color, [1.0, 1.0, 1.0, 1.0]);
    assert_eq!(bb.blend_mode, BlendMode::AlphaBlend);
}

#[test]
fn test_billboard_compute_vertices() {
    let mut bb = Billboard::default();
    bb.position = [10.0, 20.0, 30.0];
    bb.size = [2.0, 4.0];
    
    // Uproszczone wektory kamery skierowanej wzdloz osi -Z
    let camera_right = [1.0, 0.0, 0.0];
    let camera_up = [0.0, 1.0, 0.0];

    let verts = bb.compute_vertices(&camera_right, &camera_up);
    
    assert_eq!(verts.len(), 4);
    
    // Sprawdzenie pozycji wierzcholkow
    // Lewy-dolny
    assert_eq!(verts[0].position, [9.0, 18.0, 30.0]);
    assert_eq!(verts[0].uv, [0.0, 1.0]);
    
    // Prawy-dolny
    assert_eq!(verts[1].position, [11.0, 18.0, 30.0]);
    assert_eq!(verts[1].uv, [1.0, 1.0]);
    
    // Prawy-gorny
    assert_eq!(verts[2].position, [11.0, 22.0, 30.0]);
    assert_eq!(verts[2].uv, [1.0, 0.0]);
    
    // Lewy-gorny
    assert_eq!(verts[3].position, [9.0, 22.0, 30.0]);
    assert_eq!(verts[3].uv, [0.0, 0.0]);
}

#[test]
fn test_pipeline_effect_add_clear() {
    let mut effect = PipelineEffect::new();
    assert_eq!(effect.billboards.len(), 0);
    
    effect.add_billboard(Billboard::default());
    effect.add_billboard(Billboard::default());
    assert_eq!(effect.billboards.len(), 2);
    
    effect.clear();
    assert_eq!(effect.billboards.len(), 0);
}

#[test]
fn test_pipeline_effect_sort_billboards() {
    let mut effect = PipelineEffect::new();
    
    let mut bb1 = Billboard::default();
    bb1.position = [0.0, 0.0, 10.0]; // Najblizej kamery
    
    let mut bb2 = Billboard::default();
    bb2.position = [0.0, 0.0, 30.0]; // Najdalej kamery
    
    let mut bb3 = Billboard::default();
    bb3.position = [0.0, 0.0, 20.0]; // Srodek
    
    effect.add_billboard(bb1);
    effect.add_billboard(bb2);
    effect.add_billboard(bb3);
    
    let camera_pos = [0.0, 0.0, 0.0];
    effect.sort_billboards(&camera_pos);
    
    // Sprawdzanie czy posortowano od najdalszych (z=30) do najblizszych (z=10)
    assert_eq!(effect.billboards[0].position, [0.0, 0.0, 30.0]);
    assert_eq!(effect.billboards[1].position, [0.0, 0.0, 20.0]);
    assert_eq!(effect.billboards[2].position, [0.0, 0.0, 10.0]);
}

#[test]
fn test_pipeline_effect_process_for_rendering() {
    let mut effect = PipelineEffect::new();
    
    let mut bb1 = Billboard::default();
    bb1.position = [0.0, 0.0, 10.0];
    
    let mut bb2 = Billboard::default();
    bb2.position = [0.0, 0.0, 20.0];
    
    effect.add_billboard(bb1);
    effect.add_billboard(bb2);
    
    let camera_right = [1.0, 0.0, 0.0];
    let camera_up = [0.0, 1.0, 0.0];
    
    let vertices = effect.process_for_rendering(&camera_right, &camera_up);
    
    // Dwa billboardy po 4 wierzcholki = 8 wierzcholkow
    assert_eq!(vertices.len(), 8);
}
