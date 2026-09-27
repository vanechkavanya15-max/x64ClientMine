use glam::{Mat4, Vec3, Quat};
use m2_client::granny_matrix_palette::{BoneHierarchyNode, BoneMatrixPalette, multiply_hierarchical_transforms, update_bone_palette};

#[test]
fn test_bone_hierarchy_single_root() {
    let mut nodes = vec![
        BoneHierarchyNode {
            index: 0,
            parent_index: None,
            local_transform: Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0)),
            inv_bind_matrix: Mat4::IDENTITY,
        },
    ];

    multiply_hierarchical_transforms(&mut nodes);

    // Root node's world transform should equal its local transform
    assert_eq!(nodes[0].local_transform, Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0)));
}

#[test]
fn test_bone_hierarchy_parent_child() {
    let mut nodes = vec![
        // Root bone
        BoneHierarchyNode {
            index: 0,
            parent_index: None,
            local_transform: Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0)),
            inv_bind_matrix: Mat4::IDENTITY,
        },
        // Child bone (moves +2.0 on X relative to parent)
        BoneHierarchyNode {
            index: 1,
            parent_index: Some(0),
            local_transform: Mat4::from_translation(Vec3::new(2.0, 0.0, 0.0)),
            inv_bind_matrix: Mat4::IDENTITY,
        },
    ];

    multiply_hierarchical_transforms(&mut nodes);

    // Parent should be unchanged
    assert_eq!(nodes[0].local_transform.w_axis.x, 1.0);
    // Child should be at 1.0 + 2.0 = 3.0 on X
    assert_eq!(nodes[1].local_transform.w_axis.x, 3.0);
}

#[test]
fn test_bone_hierarchy_rotation_translation() {
    let mut nodes = vec![
        // Root bone rotated 90 degrees around Z, and moved 1 unit along X
        BoneHierarchyNode {
            index: 0,
            parent_index: None,
            local_transform: Mat4::from_rotation_translation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2), Vec3::new(1.0, 0.0, 0.0)),
            inv_bind_matrix: Mat4::IDENTITY,
        },
        // Child bone moved 1 unit along X locally
        BoneHierarchyNode {
            index: 1,
            parent_index: Some(0),
            local_transform: Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0)),
            inv_bind_matrix: Mat4::IDENTITY,
        },
    ];

    multiply_hierarchical_transforms(&mut nodes);

    // Parent translates (1,0,0) and rotates 90 deg around Z
    // Local child point (0,0,0) becomes Parent World Point (1,0,0)
    // Local child X axis (1,0,0) becomes Parent World Y axis (0,1,0) due to 90 deg rotation.
    // Child translates 1 unit along its local X. 
    // In world space, this is 1 unit along world Y, starting from (1,0,0). Result -> (1, 1, 0)
    
    let world_pos = nodes[1].local_transform.transform_point3(Vec3::ZERO);
    
    // Allow small floating point error
    assert!((world_pos.x - 1.0).abs() < 1e-6);
    assert!((world_pos.y - 1.0).abs() < 1e-6);
    assert!((world_pos.z - 0.0).abs() < 1e-6);
}

#[test]
fn test_bone_palette_update() {
    let nodes = vec![
        BoneHierarchyNode {
            index: 0,
            parent_index: None,
            local_transform: Mat4::from_translation(Vec3::new(5.0, 0.0, 0.0)), // Simulated WorldTransform
            inv_bind_matrix: Mat4::from_translation(Vec3::new(-2.0, 0.0, 0.0)),
        },
    ];

    let mut palette = BoneMatrixPalette::default();
    update_bone_palette(&nodes, &mut palette);

    assert_eq!(palette.count, 1);
    
    // FinalSkinMatrix = WorldTransform * InvBindMatrix
    // Translation: (5,0,0) + (-2,0,0) = (3,0,0)
    let final_pos = palette.matrices[0].transform_point3(Vec3::ZERO);
    assert!((final_pos.x - 3.0).abs() < 1e-6);
    assert!((final_pos.y - 0.0).abs() < 1e-6);
    assert!((final_pos.z - 0.0).abs() < 1e-6);
}

#[test]
fn test_bone_hierarchy_multiple_children() {
    let mut nodes = vec![
        BoneHierarchyNode { index: 0, parent_index: None, local_transform: Mat4::from_translation(Vec3::new(0.0, 10.0, 0.0)), inv_bind_matrix: Mat4::IDENTITY },
        BoneHierarchyNode { index: 1, parent_index: Some(0), local_transform: Mat4::from_translation(Vec3::new(5.0, 0.0, 0.0)), inv_bind_matrix: Mat4::IDENTITY },
        BoneHierarchyNode { index: 2, parent_index: Some(0), local_transform: Mat4::from_translation(Vec3::new(-5.0, 0.0, 0.0)), inv_bind_matrix: Mat4::IDENTITY },
        BoneHierarchyNode { index: 3, parent_index: Some(1), local_transform: Mat4::from_translation(Vec3::new(0.0, 5.0, 0.0)), inv_bind_matrix: Mat4::IDENTITY },
    ];

    multiply_hierarchical_transforms(&mut nodes);

    // Child 1 (Index 1) should be at (5, 10, 0)
    assert_eq!(nodes[1].local_transform.w_axis.x, 5.0);
    assert_eq!(nodes[1].local_transform.w_axis.y, 10.0);

    // Child 2 (Index 2) should be at (-5, 10, 0)
    assert_eq!(nodes[2].local_transform.w_axis.x, -5.0);
    assert_eq!(nodes[2].local_transform.w_axis.y, 10.0);

    // Grandchild (Index 3) should be at (5, 15, 0)
    assert_eq!(nodes[3].local_transform.w_axis.x, 5.0);
    assert_eq!(nodes[3].local_transform.w_axis.y, 15.0);
}

#[test]
fn test_bone_palette_limit() {
    let mut nodes = vec![];
    for i in 0..150 {
        nodes.push(BoneHierarchyNode {
            index: i,
            parent_index: None,
            local_transform: Mat4::IDENTITY,
            inv_bind_matrix: Mat4::IDENTITY,
        });
    }

    let mut palette = BoneMatrixPalette::default();
    update_bone_palette(&nodes, &mut palette);

    // The palette should cap at 128 bones
    assert_eq!(palette.count, 128);
}
