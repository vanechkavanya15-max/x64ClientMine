use m2_client_granny::gr2_bone_bindings::{Gr2BoneBindings, VertexBoneData};

#[test]
fn test_integration_gr2_bone_bindings() {
    let instance = Gr2BoneBindings::new();
    assert!(instance.is_ready());
    assert_eq!(instance.len(), 0);
    assert!(instance.is_empty());
}

#[test]
fn test_gr2_bone_bindings_with_capacity() {
    let instance = Gr2BoneBindings::with_capacity(100);
    assert!(instance.is_ready());
    assert!(instance.is_empty());
    assert_eq!(instance.vertices.capacity(), 100);
}

#[test]
fn test_add_vertex_and_normalize_all() {
    let mut bindings = Gr2BoneBindings::new();
    
    let mut v1 = VertexBoneData::new();
    v1.add_bone(1, 0.5).unwrap();
    v1.add_bone(2, 0.5).unwrap();
    
    let mut v2 = VertexBoneData::new();
    v2.add_bone(3, 10.0).unwrap();
    v2.add_bone(4, 30.0).unwrap();
    
    bindings.add_vertex_binding(v1);
    bindings.add_vertex_binding(v2);
    
    assert_eq!(bindings.len(), 2);
    assert!(!bindings.is_empty());
    
    bindings.normalize_all();
    
    assert_eq!(bindings.vertices[0].bone_weights[0], 0.5);
    assert_eq!(bindings.vertices[0].bone_weights[1], 0.5);
    
    assert_eq!(bindings.vertices[1].bone_weights[0], 0.25);
    assert_eq!(bindings.vertices[1].bone_weights[1], 0.75);
}
