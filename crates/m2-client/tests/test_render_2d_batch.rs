use m2_client::render_2d_batch::Batch2D;

#[test]
fn test_single_quad_generation() {
    let mut batch = Batch2D::new();
    batch.add_quad(10.0, 20.0, 100.0, 50.0, (0.0, 0.0, 1.0, 1.0), [1.0, 1.0, 1.0, 1.0]);

    assert_eq!(batch.vertices.len(), 4);
    assert_eq!(batch.indices.len(), 6);

    assert_eq!(batch.vertices[0].pos, (10.0, 20.0));
    assert_eq!(batch.vertices[1].pos, (110.0, 20.0));
    assert_eq!(batch.vertices[2].pos, (10.0, 70.0));
    assert_eq!(batch.vertices[3].pos, (110.0, 70.0));

    assert_eq!(batch.vertices[0].uv, (0.0, 0.0));
    assert_eq!(batch.vertices[3].uv, (1.0, 1.0));

    assert_eq!(batch.indices, vec![0, 1, 2, 2, 1, 3]);
}

#[test]
fn test_quad_batching() {
    let mut batch = Batch2D::new();
    batch.add_quad(0.0, 0.0, 10.0, 10.0, (0.0, 0.0, 1.0, 1.0), [1.0, 1.0, 1.0, 1.0]);
    batch.add_quad(20.0, 20.0, 10.0, 10.0, (0.0, 0.0, 1.0, 1.0), [1.0, 1.0, 1.0, 1.0]);

    assert_eq!(batch.vertices.len(), 8);
    assert_eq!(batch.indices.len(), 12);

    assert_eq!(batch.indices, vec![0, 1, 2, 2, 1, 3, 4, 5, 6, 6, 5, 7]);
}

#[test]
fn test_scissor_clip_reject_outside() {
    let mut batch = Batch2D::new();
    batch.set_clip_rect(50.0, 50.0, 100.0, 100.0);

    // Left
    batch.add_quad(0.0, 60.0, 40.0, 20.0, (0.0, 0.0, 1.0, 1.0), [1.0, 1.0, 1.0, 1.0]);
    // Right
    batch.add_quad(110.0, 60.0, 40.0, 20.0, (0.0, 0.0, 1.0, 1.0), [1.0, 1.0, 1.0, 1.0]);
    // Top
    batch.add_quad(60.0, 0.0, 20.0, 40.0, (0.0, 0.0, 1.0, 1.0), [1.0, 1.0, 1.0, 1.0]);
    // Bottom
    batch.add_quad(60.0, 110.0, 20.0, 40.0, (0.0, 0.0, 1.0, 1.0), [1.0, 1.0, 1.0, 1.0]);

    assert_eq!(batch.vertices.len(), 0);
    assert_eq!(batch.indices.len(), 0);
}

#[test]
fn test_scissor_clip_partial() {
    let mut batch = Batch2D::new();
    batch.set_clip_rect(50.0, 50.0, 100.0, 100.0);

    batch.add_quad(40.0, 40.0, 40.0, 40.0, (0.0, 0.0, 1.0, 1.0), [1.0, 1.0, 1.0, 1.0]);

    assert_eq!(batch.vertices.len(), 4);
    assert_eq!(batch.indices.len(), 6);

    let v = &batch.vertices;
    assert_eq!(v[0].pos, (50.0, 50.0));
    assert_eq!(v[1].pos, (80.0, 50.0));
    assert_eq!(v[2].pos, (50.0, 80.0));
    assert_eq!(v[3].pos, (80.0, 80.0));

    assert_eq!(v[0].uv, (0.25, 0.25));
    assert_eq!(v[3].uv, (1.0, 1.0));
}

#[test]
fn test_scissor_clip_clear() {
    let mut batch = Batch2D::new();
    batch.set_clip_rect(50.0, 50.0, 100.0, 100.0);
    batch.clear_clip_rect();
    batch.add_quad(0.0, 0.0, 10.0, 10.0, (0.0, 0.0, 1.0, 1.0), [1.0, 1.0, 1.0, 1.0]);

    assert_eq!(batch.vertices.len(), 4);
    assert_eq!(batch.indices.len(), 6);
}

