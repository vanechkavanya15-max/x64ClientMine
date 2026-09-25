use m2_client_granny::gr2_skeleton::{Bone, Gr2Skeleton, multiply_matrices};

#[test]
fn test_integration_gr2_skeleton() {
    let instance = Gr2Skeleton::new();
    assert!(instance.is_ready());
}

#[test]
fn test_add_and_get_bone() {
    let mut skeleton = Gr2Skeleton::new();
    assert_eq!(skeleton.bone_count(), 0);

    let bone1 = Bone::new("Root", -1);
    let index1 = skeleton.add_bone(bone1.clone());
    assert_eq!(index1, 0);
    assert_eq!(skeleton.bone_count(), 1);

    let bone2 = Bone::new("Child", 0);
    let index2 = skeleton.add_bone(bone2.clone());
    assert_eq!(index2, 1);
    assert_eq!(skeleton.bone_count(), 2);

    let retrieved_bone1 = skeleton.get_bone(0).unwrap();
    assert_eq!(retrieved_bone1.name, "Root");
    assert_eq!(retrieved_bone1.parent_index, -1);

    let retrieved_bone2 = skeleton.get_bone(1).unwrap();
    assert_eq!(retrieved_bone2.name, "Child");
    assert_eq!(retrieved_bone2.parent_index, 0);
}

#[test]
fn test_skeleton_hierarchy_global_transform() {
    let mut skeleton = Gr2Skeleton::new();

    let mut root_bone = Bone::new("Root", -1);
    // Przesuniecie o X=1.0
    root_bone.local_transform = [
        1.0, 0.0, 0.0, 1.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    skeleton.add_bone(root_bone);

    let mut child_bone = Bone::new("Child", 0);
    // Przesuniecie o Y=2.0
    child_bone.local_transform = [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    skeleton.add_bone(child_bone);

    let mut grandchild_bone = Bone::new("Grandchild", 1);
    // Przesuniecie o Z=3.0
    grandchild_bone.local_transform = [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 3.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    skeleton.add_bone(grandchild_bone);

    // Global transform of root is just its local transform
    let root_global = skeleton.get_global_transform(0).unwrap();
    assert_eq!(root_global, [
        1.0, 0.0, 0.0, 1.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]);

    // Global transform of grandchild
    // root * child * grandchild
    let grandchild_global = skeleton.get_global_transform(2).unwrap();
    
    // Z uwagi na zaleznosc macierzy, upewnijmy sie co do oczekiwanego rezultatu.
    // To jest tylko przyklad i mozna sprawdzic konkretne pozycje.
    // root_local * child_local * grandchild_local w zaleznosci od porzadku mnozenia w hierarchii.
    // Zgodnie z nasza petla w `get_global_transform`:
    // 1. global_transform = grandchild.local * identity = grandchild.local
    // 2. global_transform = child.local * grandchild.local
    // 3. global_transform = root.local * (child.local * grandchild.local)
    let expected = multiply_matrices(&skeleton.get_bone(0).unwrap().local_transform, 
                   &multiply_matrices(&skeleton.get_bone(1).unwrap().local_transform, 
                   &skeleton.get_bone(2).unwrap().local_transform));
    assert_eq!(grandchild_global, expected);
}

#[test]
fn test_invalid_bone_index() {
    let skeleton = Gr2Skeleton::new();
    assert!(skeleton.get_bone(0).is_none());
    assert!(skeleton.get_global_transform(0).is_none());
}

#[test]
fn test_cyclic_hierarchy() {
    let mut skeleton = Gr2Skeleton::new();

    let bone0 = Bone::new("Bone0", 1); // Wskazuje na bone1
    let bone1 = Bone::new("Bone1", 0); // Wskazuje na bone0

    skeleton.add_bone(bone0);
    skeleton.add_bone(bone1);

    // To nie powinno wejsc w nieskonczona petle dzieki zabezpieczeniu HashSet
    let transform = skeleton.get_global_transform(0);
    assert!(transform.is_some()); // Upewniamy sie ze wychodzi bezpiecznie
}
