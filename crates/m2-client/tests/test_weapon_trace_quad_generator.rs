use m2_client::weapon_trace_quad_generator::{WeaponTraceQuadGenerator};
use m2_client::weapon_trace::TraceVertex;
use glam::{Vec3};

#[test]
fn test_less_than_two_pairs() {
    // A single pair (2 vertices) shouldn't produce any quads
    let input = vec![
        TraceVertex { position: Vec3::ZERO, u: 0.0, v: 0.0, alpha: 1.0 },
        TraceVertex { position: Vec3::ONE, u: 0.0, v: 1.0, alpha: 1.0 },
    ];
    let output = WeaponTraceQuadGenerator::generate_quad_strip(&input, Vec3::new(1.0, 1.0, 1.0));
    assert!(output.is_empty());
}

#[test]
fn test_two_pairs_creates_two_triangles() {
    let input = vec![
        TraceVertex { position: Vec3::new(0.0, 0.0, 0.0), u: 0.0, v: 0.0, alpha: 1.0 }, // L0
        TraceVertex { position: Vec3::new(0.0, 1.0, 0.0), u: 0.0, v: 1.0, alpha: 1.0 }, // S0
        TraceVertex { position: Vec3::new(1.0, 0.0, 0.0), u: 1.0, v: 0.0, alpha: 1.0 }, // L1
        TraceVertex { position: Vec3::new(1.0, 1.0, 0.0), u: 1.0, v: 1.0, alpha: 1.0 }, // S1
    ];
    let output = WeaponTraceQuadGenerator::generate_quad_strip(&input, Vec3::new(1.0, 0.5, 0.2));
    assert_eq!(output.len(), 6); // 2 triangles * 3 vertices

    // First triangle: L0, S0, L1
    assert_eq!(output[0].position, Vec3::new(0.0, 0.0, 0.0));
    assert_eq!(output[1].position, Vec3::new(0.0, 1.0, 0.0));
    assert_eq!(output[2].position, Vec3::new(1.0, 0.0, 0.0));

    // Second triangle: S0, S1, L1
    assert_eq!(output[3].position, Vec3::new(0.0, 1.0, 0.0));
    assert_eq!(output[4].position, Vec3::new(1.0, 1.0, 0.0));
    assert_eq!(output[5].position, Vec3::new(1.0, 0.0, 0.0));
}

#[test]
fn test_color_and_alpha_mapping() {
    let input = vec![
        TraceVertex { position: Vec3::ZERO, u: 0.0, v: 0.0, alpha: 0.5 }, // L0
        TraceVertex { position: Vec3::ZERO, u: 0.0, v: 0.0, alpha: 0.5 }, // S0
        TraceVertex { position: Vec3::ZERO, u: 0.0, v: 0.0, alpha: 0.8 }, // L1
        TraceVertex { position: Vec3::ZERO, u: 0.0, v: 0.0, alpha: 0.8 }, // S1
    ];
    let output = WeaponTraceQuadGenerator::generate_quad_strip(&input, Vec3::new(0.1, 0.2, 0.3));
    assert_eq!(output.len(), 6);

    // Check color fields (RGB from base, A from vertex)
    assert_eq!(output[0].color, [0.1, 0.2, 0.3, 0.5]); // L0
    assert_eq!(output[1].color, [0.1, 0.2, 0.3, 0.5]); // S0
    assert_eq!(output[2].color, [0.1, 0.2, 0.3, 0.8]); // L1

    assert_eq!(output[3].color, [0.1, 0.2, 0.3, 0.5]); // S0
    assert_eq!(output[4].color, [0.1, 0.2, 0.3, 0.8]); // S1
    assert_eq!(output[5].color, [0.1, 0.2, 0.3, 0.8]); // L1
}

#[test]
fn test_uv_mapping() {
    let input = vec![
        TraceVertex { position: Vec3::ZERO, u: 0.1, v: 0.0, alpha: 1.0 }, // L0
        TraceVertex { position: Vec3::ZERO, u: 0.1, v: 1.0, alpha: 1.0 }, // S0
        TraceVertex { position: Vec3::ZERO, u: 0.9, v: 0.0, alpha: 1.0 }, // L1
        TraceVertex { position: Vec3::ZERO, u: 0.9, v: 1.0, alpha: 1.0 }, // S1
    ];
    let output = WeaponTraceQuadGenerator::generate_quad_strip(&input, Vec3::ZERO);
    assert_eq!(output.len(), 6);

    // L0
    assert_eq!(output[0].tex_coord, [0.1, 0.0]);
    // S0
    assert_eq!(output[1].tex_coord, [0.1, 1.0]);
    // L1
    assert_eq!(output[2].tex_coord, [0.9, 0.0]);

    // S0
    assert_eq!(output[3].tex_coord, [0.1, 1.0]);
    // S1
    assert_eq!(output[4].tex_coord, [0.9, 1.0]);
    // L1
    assert_eq!(output[5].tex_coord, [0.9, 0.0]);
}

#[test]
fn test_alpha_clamping() {
    let input = vec![
        TraceVertex { position: Vec3::ZERO, u: 0.0, v: 0.0, alpha: -0.5 }, // L0
        TraceVertex { position: Vec3::ZERO, u: 0.0, v: 0.0, alpha: 1.5 }, // S0
        TraceVertex { position: Vec3::ZERO, u: 0.0, v: 0.0, alpha: 2.0 }, // L1
        TraceVertex { position: Vec3::ZERO, u: 0.0, v: 0.0, alpha: -10.0 }, // S1
    ];
    let output = WeaponTraceQuadGenerator::generate_quad_strip(&input, Vec3::ZERO);
    assert_eq!(output.len(), 6);

    // Output alpha should be clamped [0.0, 1.0]
    assert_eq!(output[0].color[3], 0.0); // L0 -> 0.0
    assert_eq!(output[1].color[3], 1.0); // S0 -> 1.0
    assert_eq!(output[2].color[3], 1.0); // L1 -> 1.0

    assert_eq!(output[3].color[3], 1.0); // S0 -> 1.0
    assert_eq!(output[4].color[3], 0.0); // S1 -> 0.0
    assert_eq!(output[5].color[3], 1.0); // L1 -> 1.0
}
