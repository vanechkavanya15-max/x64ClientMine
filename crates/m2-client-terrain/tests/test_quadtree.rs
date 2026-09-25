use m2_client_terrain::quadtree::{Aabb, Frustum, Plane, Quadtree, Vec3};

#[test]
fn test_aabb_frustum_intersection() {
    let aabb = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 10.0, 10.0));
    
    // Frustum enclosing the AABB
    let frustum = Frustum::new([
        Plane::new(Vec3::new(1.0, 0.0, 0.0), 10.0),   // Left
        Plane::new(Vec3::new(-1.0, 0.0, 0.0), 20.0), // Right
        Plane::new(Vec3::new(0.0, 1.0, 0.0), 10.0),   // Bottom
        Plane::new(Vec3::new(0.0, -1.0, 0.0), 20.0), // Top
        Plane::new(Vec3::new(0.0, 0.0, 1.0), 10.0),   // Near
        Plane::new(Vec3::new(0.0, 0.0, -1.0), 20.0), // Far
    ]);
    
    assert!(frustum.intersects_aabb(&aabb));
    
    // Frustum far away
    let frustum_far = Frustum::new([
        Plane::new(Vec3::new(1.0, 0.0, 0.0), -20.0),
        Plane::new(Vec3::new(-1.0, 0.0, 0.0), 10.0),
        Plane::new(Vec3::new(0.0, 1.0, 0.0), 10.0),
        Plane::new(Vec3::new(0.0, -1.0, 0.0), 20.0),
        Plane::new(Vec3::new(0.0, 0.0, 1.0), 10.0),
        Plane::new(Vec3::new(0.0, 0.0, -1.0), 20.0),
    ]);
    
    assert!(!frustum_far.intersects_aabb(&aabb));
}

#[test]
fn test_quadtree_insert() {
    let mut tree = Quadtree::new(Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(100.0, 100.0, 100.0)));
    assert!(tree.is_ready());
    
    let aabb1 = Aabb::new(Vec3::new(10.0, 0.0, 10.0), Vec3::new(20.0, 10.0, 20.0));
    let inserted = tree.insert(1, aabb1);
    assert!(inserted);
    
    // Try to insert outside the quadtree bounds
    let aabb2 = Aabb::new(Vec3::new(200.0, 0.0, 200.0), Vec3::new(210.0, 10.0, 210.0));
    let inserted2 = tree.insert(2, aabb2);
    assert!(!inserted2);
}

#[test]
fn test_quadtree_culling_query() {
    let mut tree = Quadtree::new(Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(100.0, 100.0, 100.0)));
    
    // Inserting elements
    tree.insert(1, Aabb::new(Vec3::new(10.0, 0.0, 10.0), Vec3::new(20.0, 10.0, 20.0)));
    tree.insert(2, Aabb::new(Vec3::new(80.0, 0.0, 80.0), Vec3::new(90.0, 10.0, 90.0)));
    
    // Frustum catching the first element
    let frustum1 = Frustum::new([
        Plane::new(Vec3::new(1.0, 0.0, 0.0), 0.0),
        Plane::new(Vec3::new(-1.0, 0.0, 0.0), 50.0),
        Plane::new(Vec3::new(0.0, 1.0, 0.0), 0.0),
        Plane::new(Vec3::new(0.0, -1.0, 0.0), 50.0),
        Plane::new(Vec3::new(0.0, 0.0, 1.0), 0.0),
        Plane::new(Vec3::new(0.0, 0.0, -1.0), 50.0),
    ]);
    
    let result1 = tree.query_culling(&frustum1);
    assert!(result1.contains(&1));
    assert!(!result1.contains(&2));
    
    // Frustum catching both
    let frustum2 = Frustum::new([
        Plane::new(Vec3::new(1.0, 0.0, 0.0), 0.0),
        Plane::new(Vec3::new(-1.0, 0.0, 0.0), 100.0),
        Plane::new(Vec3::new(0.0, 1.0, 0.0), 0.0),
        Plane::new(Vec3::new(0.0, -1.0, 0.0), 100.0),
        Plane::new(Vec3::new(0.0, 0.0, 1.0), 0.0),
        Plane::new(Vec3::new(0.0, 0.0, -1.0), 100.0),
    ]);
    
    let result2 = tree.query_culling(&frustum2);
    assert!(result2.contains(&1));
    assert!(result2.contains(&2));
}
