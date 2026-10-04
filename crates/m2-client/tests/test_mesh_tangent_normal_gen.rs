use m2_client::wgpu_mesh_tangent_normal_generator::*;
use glam::Vec3;

#[test]
fn test_tangent_generation_flat_quad() {
    let positions = vec![
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
    ];
    let normals = vec![
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ];
    let uvs = vec![
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
    ];
    let indices = vec![0, 1, 2, 0, 2, 3];

    let result = MeshTangentGenerator::generate_tangents(&indices, &positions, &normals, &uvs);

    assert_eq!(result.len(), 4);
    for v in result.iter() {
        let t = Vec3::from(v.tangent);
        let b = Vec3::from(v.bitangent);
        let n = Vec3::from(v.normal);

        // Tangent and bitangent should be orthogonal to normal
        assert!(t.dot(n).abs() < 1e-5, "Tangent not orthogonal to normal");
        assert!(b.dot(n).abs() < 1e-5, "Bitangent not orthogonal to normal");
        
        // Tangent and bitangent should be orthogonal to each other
        assert!(t.dot(b).abs() < 1e-5, "Tangent not orthogonal to bitangent");

        // They should be normalized
        assert!((t.length() - 1.0).abs() < 1e-5, "Tangent not normalized");
        assert!((b.length() - 1.0).abs() < 1e-5, "Bitangent not normalized");
    }
}

#[test]
fn test_tangent_mismatched_lengths() {
    let positions = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
    let normals = vec![[0.0, 0.0, 1.0]]; // Mismatched length
    let uvs = vec![[0.0, 0.0], [1.0, 0.0]];
    let indices = vec![0, 1, 0];

    let result = MeshTangentGenerator::generate_tangents(&indices, &positions, &normals, &uvs);
    assert!(result.is_empty(), "Should return empty on mismatched lengths");
}

#[test]
fn test_degenerate_triangle() {
    // Triangle with zero area
    let positions = vec![
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
    ];
    let normals = vec![
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ];
    let uvs = vec![
        [0.0, 0.0],
        [0.0, 0.0],
        [0.0, 0.0],
    ];
    let indices = vec![0, 1, 2];

    let result = MeshTangentGenerator::generate_tangents(&indices, &positions, &normals, &uvs);

    assert_eq!(result.len(), 3);
    for v in result.iter() {
        let t = Vec3::from(v.tangent);
        let n = Vec3::from(v.normal);
        
        assert!(t.dot(n).abs() < 1e-5, "Fallback tangent not orthogonal to normal");
    }
}

#[test]
fn test_orthogonalization_with_unnormalized_inputs() {
    let positions = vec![
        [0.0, 0.0, 0.0],
        [10.0, 0.0, 0.0], // Big triangle
        [0.0, 10.0, 0.0],
    ];
    let normals = vec![
        [0.0, 0.0, 2.0], // Unnormalized normal
        [0.0, 0.0, 2.0],
        [0.0, 0.0, 2.0],
    ];
    let uvs = vec![
        [0.0, 0.0],
        [2.0, 0.0],
        [0.0, 2.0],
    ];
    let indices = vec![0, 1, 2];

    let result = MeshTangentGenerator::generate_tangents(&indices, &positions, &normals, &uvs);

    assert_eq!(result.len(), 3);
    for v in result.iter() {
        let t = Vec3::from(v.tangent);
        let b = Vec3::from(v.bitangent);
        let n = Vec3::from(v.normal);

        // The tangent generator should still orthogonalize against the provided normal direction
        // Since input normal is [0,0,2], normalize it for our check
        let n_norm = n.normalize_or_zero();
        
        assert!(t.dot(n_norm).abs() < 1e-5, "Tangent not orthogonal to normal direction");
        assert!(b.dot(n_norm).abs() < 1e-5, "Bitangent not orthogonal to normal direction");
        
        // Output tangent/bitangent should be normalized
        assert!((t.length() - 1.0).abs() < 1e-5, "Tangent not normalized");
        assert!((b.length() - 1.0).abs() < 1e-5, "Bitangent not normalized");
    }
}
