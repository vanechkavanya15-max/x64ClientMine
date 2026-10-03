use m2_client::ui_scissor_clip_rect::{clip_polygon, clip_triangles, ScissorRect, ScissorVertex};

#[test]
fn test_trivial_accept() {
    let rect = ScissorRect {
        left: 0.0,
        top: 0.0,
        right: 100.0,
        bottom: 100.0,
    };
    let vertices = vec![
        ScissorVertex::new(10.0, 10.0, 0.0, 1.0, 0xFFFFFFFF, 0.0, 0.0),
        ScissorVertex::new(90.0, 10.0, 0.0, 1.0, 0xFFFFFFFF, 1.0, 0.0),
        ScissorVertex::new(50.0, 90.0, 0.0, 1.0, 0xFFFFFFFF, 0.5, 1.0),
    ];

    let clipped = clip_polygon(vertices.clone(), &rect);
    assert_eq!(clipped.len(), 3);
    assert_eq!(clipped, vertices);
}

#[test]
fn test_trivial_reject() {
    let rect = ScissorRect {
        left: 0.0,
        top: 0.0,
        right: 100.0,
        bottom: 100.0,
    };
    let vertices = vec![
        ScissorVertex::new(110.0, 110.0, 0.0, 1.0, 0xFFFFFFFF, 0.0, 0.0),
        ScissorVertex::new(190.0, 110.0, 0.0, 1.0, 0xFFFFFFFF, 1.0, 0.0),
        ScissorVertex::new(150.0, 190.0, 0.0, 1.0, 0xFFFFFFFF, 0.5, 1.0),
    ];

    let clipped = clip_polygon(vertices.clone(), &rect);
    assert!(clipped.is_empty());
}

#[test]
fn test_partial_clipping_single_edge() {
    let rect = ScissorRect {
        left: 0.0,
        top: 0.0,
        right: 100.0,
        bottom: 100.0,
    };
    
    let vertices = vec![
        ScissorVertex::new(-50.0, 50.0, 0.0, 1.0, 0xFFFF0000, 0.0, 0.5), // Outside (Left)
        ScissorVertex::new(50.0, 10.0, 0.0, 1.0, 0xFF00FF00, 1.0, 0.1), // Inside
        ScissorVertex::new(50.0, 90.0, 0.0, 1.0, 0xFF0000FF, 1.0, 0.9), // Inside
    ];

    let clipped = clip_polygon(vertices, &rect);
    
    // Cutting a triangle with one line results in a quadrilateral (4 vertices)
    assert_eq!(clipped.len(), 4);

    for v in &clipped {
        assert!(v.x >= rect.left);
        assert!(v.x <= rect.right);
        assert!(v.y >= rect.top);
        assert!(v.y <= rect.bottom);
    }
}

#[test]
fn test_multi_edge_clipping_corner() {
    let rect = ScissorRect {
        left: 20.0,
        top: 20.0,
        right: 80.0,
        bottom: 80.0,
    };

    let vertices = vec![
        ScissorVertex::new(0.0, 0.0, 0.0, 1.0, 0xFFFFFFFF, 0.0, 0.0), // Outside top-left
        ScissorVertex::new(100.0, 50.0, 0.0, 1.0, 0xFFFFFFFF, 1.0, 0.5), // Outside right
        ScissorVertex::new(50.0, 100.0, 0.0, 1.0, 0xFFFFFFFF, 0.5, 1.0), // Outside bottom
    ];

    let clipped = clip_polygon(vertices, &rect);

    for v in &clipped {
        assert!(v.x >= rect.left - 1e-4);
        assert!(v.x <= rect.right + 1e-4);
        assert!(v.y >= rect.top - 1e-4);
        assert!(v.y <= rect.bottom + 1e-4);
    }
}

#[test]
fn test_color_and_uv_interpolation() {
    let rect = ScissorRect {
        left: 50.0,
        top: 0.0,
        right: 100.0,
        bottom: 100.0,
    };

    let v1 = ScissorVertex::new(0.0, 50.0, 0.0, 1.0, 0xFF000000, 0.0, 0.5); // Black
    let v2 = ScissorVertex::new(100.0, 50.0, 0.0, 1.0, 0xFFFFFFFF, 1.0, 0.5); // White
    
    let clipped_v = v1.lerp(&v2, 0.5);
    
    assert_eq!(clipped_v.x, 50.0);
    assert_eq!(clipped_v.u, 0.5);
    // Interpolated color between Black (0x00) and White (0xFF) -> 0x80 = 128
    // Note: 255/2 rounded is 128
    assert_eq!(clipped_v.color, 0xFF808080);
}

#[test]
fn test_clip_triangles_batching() {
    let rect = ScissorRect {
        left: 0.0,
        top: 0.0,
        right: 100.0,
        bottom: 100.0,
    };
    
    let vertices = vec![
        // Tri 1 (Inside)
        ScissorVertex::new(10.0, 10.0, 0.0, 1.0, 0xFFFFFFFF, 0.0, 0.0),
        ScissorVertex::new(90.0, 10.0, 0.0, 1.0, 0xFFFFFFFF, 1.0, 0.0),
        ScissorVertex::new(50.0, 90.0, 0.0, 1.0, 0xFFFFFFFF, 0.5, 1.0),
        
        // Tri 2 (Outside)
        ScissorVertex::new(110.0, 110.0, 0.0, 1.0, 0xFFFFFFFF, 0.0, 0.0),
        ScissorVertex::new(190.0, 110.0, 0.0, 1.0, 0xFFFFFFFF, 1.0, 0.0),
        ScissorVertex::new(150.0, 190.0, 0.0, 1.0, 0xFFFFFFFF, 0.5, 1.0),
    ];
    
    let clipped = clip_triangles(&vertices, &rect);
    
    // Only the first triangle should remain
    assert_eq!(clipped.len(), 3);
}
