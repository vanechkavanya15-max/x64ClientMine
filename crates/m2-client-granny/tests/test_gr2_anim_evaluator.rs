use m2_client_granny::gr2_anim_evaluator::{Gr2AnimEvaluator, Vec3, Quat, BoneTransform, Keyframe, Matrix4};

#[test]
fn test_integration_gr2_anim_evaluator() {
    let instance = Gr2AnimEvaluator::new();
    assert!(instance.is_ready());
}

#[test]
fn test_vec3_lerp() {
    let v1 = Vec3::new(0.0, 0.0, 0.0);
    let v2 = Vec3::new(10.0, 20.0, 30.0);
    let v_mid = v1.lerp(&v2, 0.5);
    
    assert_eq!(v_mid.x, 5.0);
    assert_eq!(v_mid.y, 10.0);
    assert_eq!(v_mid.z, 15.0);
}

#[test]
fn test_quat_normalize_and_slerp() {
    let q1 = Quat::new(0.0, 0.0, 0.0, 1.0); // Brak rotacji
    let mut q2 = Quat::new(0.0, 1.0, 0.0, 0.0); // 180 stopni Y
    q2.normalize();
    
    // SLERP blisko polowy
    let q_mid = q1.slerp(&q2, 0.5);
    
    // Dla slerp miedzy (0,0,0,1) a (0,1,0,0) oczekujemy w przyblizeniu (0, ~0.707, 0, ~0.707)
    assert!(q_mid.y > 0.7 && q_mid.y < 0.72);
    assert!(q_mid.w > 0.7 && q_mid.w < 0.72);
}

#[test]
fn test_anim_evaluator_evaluate_transform() {
    let mut evaluator = Gr2AnimEvaluator::new();
    
    evaluator.add_keyframe(Keyframe::new(0.0, BoneTransform::new(
        Vec3::new(0.0, 0.0, 0.0),
        Quat::new(0.0, 0.0, 0.0, 1.0)
    )));
    
    evaluator.add_keyframe(Keyframe::new(1.0, BoneTransform::new(
        Vec3::new(10.0, 0.0, 0.0),
        Quat::new(0.0, 1.0, 0.0, 0.0)
    )));
    
    // Test ewaluacji w polowie
    let mid = evaluator.evaluate_transform(0.5);
    assert_eq!(mid.translation.x, 5.0);
    
    // Test ewaluacji przed pierwsza klatka
    let before = evaluator.evaluate_transform(-1.0);
    assert_eq!(before.translation.x, 0.0);
    
    // Test ewaluacji po ostatniej klatce
    let after = evaluator.evaluate_transform(2.0);
    assert_eq!(after.translation.x, 10.0);
}

#[test]
fn test_anim_evaluator_evaluate_matrix() {
    let mut evaluator = Gr2AnimEvaluator::new();
    
    evaluator.add_keyframe(Keyframe::new(0.0, BoneTransform::new(
        Vec3::new(5.0, 0.0, 0.0),
        Quat::new(0.0, 0.0, 0.0, 1.0)
    )));
    
    let matrix = evaluator.evaluate_matrix(0.0);
    
    // Sprawdzanie translacji w macierzy (czwarty wiersz, kolumny 0,1,2, jesli row-major, ale dla naszego wdrozenia to tak wlasnie jest)
    assert_eq!(matrix.m[3][0], 5.0);
    assert_eq!(matrix.m[3][1], 0.0);
    assert_eq!(matrix.m[3][2], 0.0);
    assert_eq!(matrix.m[3][3], 1.0);
}

#[test]
fn test_matrix_mul() {
    let mut m1 = Matrix4::identity();
    m1.m[3][0] = 10.0; // Translacja X
    
    let mut m2 = Matrix4::identity();
    m2.m[3][1] = 20.0; // Translacja Y
    
    let m3 = m1.mul(&m2);
    
    assert_eq!(m3.m[3][0], 10.0);
    assert_eq!(m3.m[3][1], 20.0);
}
