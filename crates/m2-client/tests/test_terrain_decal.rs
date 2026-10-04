use m2_client::wgpu_terrain_decal_projector::{Plane, TerrainDecalProjector, DECAL_EPSILON};
use glam::Vec3;

#[test]
fn test_plane_distance() {
    let normal = Vec3::new(0.0, 1.0, 0.0);
    let plane = Plane::new(normal, 5.0);

    let p_on = Vec3::new(10.0, 5.0, -10.0);
    assert!(plane.distance(p_on).abs() < 1e-5);

    let p_front = Vec3::new(0.0, 10.0, 0.0);
    assert!(plane.distance(p_front) > 0.0);
    assert_eq!(plane.distance(p_front), 5.0);

    let p_back = Vec3::new(0.0, -2.0, 0.0);
    assert!(plane.distance(p_back) < 0.0);
    assert_eq!(plane.distance(p_back), -7.0);
}

#[test]
fn test_clip_polygon_fully_visible() {
    let normal = Vec3::new(0.0, 1.0, 0.0);
    let plane = Plane::new(normal, 0.0);

    let poly = vec![
        Vec3::new(-5.0, 5.0, 0.0),
        Vec3::new(5.0, 5.0, 0.0),
        Vec3::new(0.0, 10.0, 0.0),
    ];

    let clipped = TerrainDecalProjector::clip_polygon_against_plane(&plane, &poly);
    assert_eq!(clipped.len(), 3);
    assert_eq!(clipped, poly);
}

#[test]
fn test_clip_polygon_fully_culled() {
    let normal = Vec3::new(0.0, 1.0, 0.0);
    let plane = Plane::new(normal, 0.0);

    let poly = vec![
        Vec3::new(-5.0, -5.0, 0.0),
        Vec3::new(5.0, -5.0, 0.0),
        Vec3::new(0.0, -10.0, 0.0),
    ];

    let clipped = TerrainDecalProjector::clip_polygon_against_plane(&plane, &poly);
    assert_eq!(clipped.len(), 0);
}

#[test]
fn test_clip_polygon_partial() {
    let normal = Vec3::new(1.0, 0.0, 0.0);
    let plane = Plane::new(normal, 0.0); // Plane x = 0

    let poly = vec![
        Vec3::new(-10.0, -10.0, 0.0),
        Vec3::new(10.0, -10.0, 0.0),
        Vec3::new(10.0, 10.0, 0.0),
        Vec3::new(-10.0, 10.0, 0.0),
    ];

    let clipped = TerrainDecalProjector::clip_polygon_against_plane(&plane, &poly);
    // Should be a rectangle from x=0 to x=10
    assert_eq!(clipped.len(), 4);
    
    // Intersection on bottom edge: x goes from -10 to 10. Intersects at x=0
    assert_eq!(clipped[0], Vec3::new(0.0, -10.0, 0.0));
    assert_eq!(clipped[1], Vec3::new(10.0, -10.0, 0.0));
    assert_eq!(clipped[2], Vec3::new(10.0, 10.0, 0.0));
    // Intersection on top edge: x goes from 10 to -10. Intersects at x=0
    assert_eq!(clipped[3], Vec3::new(0.0, 10.0, 0.0));
}

#[test]
fn test_decal_generation() {
    let center = Vec3::new(100.0, 100.0, 50.0);
    let normal = Vec3::new(0.0, 0.0, 1.0);
    let tangent = Vec3::new(1.0, 0.0, 0.0);
    
    let projector = TerrainDecalProjector::new(center, normal, tangent, 20.0, 20.0, 10.0);

    // Mock terrain where everything is flat at z = 50
    let get_height = |_, _| 50.0;

    let cell_scale = 200.0; // Same as Metin2 CTerrainImpl::CELLSCALE
    
    let geometry = projector.generate_decal_geometry(cell_scale, get_height);
    
    // Geometry should be non-empty and triangulated correctly
    assert!(!geometry.is_empty());
    
    // Check depth bias application and UVs
    for v in geometry {
        // Since height is always 50, output should have Z = 50 + EPSILON
        assert!((v.position[2] - (50.0 + DECAL_EPSILON)).abs() < 1e-4);
        
        // UVs should be in [0, 1] range
        assert!(v.tex_coord[0] >= 0.0 && v.tex_coord[0] <= 1.0);
        assert!(v.tex_coord[1] >= 0.0 && v.tex_coord[1] <= 1.0);
    }
}
