use m2_client_granny::gr2_transform::{Vector3, Quaternion, Matrix4x4, Gr2Transform};

fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-5
}

#[test]
fn test_vector3_add_sub_mul() {
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(4.0, 5.0, 6.0);

    let sum = v1 + v2;
    assert!(approx_eq(sum.x, 5.0));
    assert!(approx_eq(sum.y, 7.0));
    assert!(approx_eq(sum.z, 9.0));

    let diff = v2 - v1;
    assert!(approx_eq(diff.x, 3.0));
    assert!(approx_eq(diff.y, 3.0));
    assert!(approx_eq(diff.z, 3.0));

    let scaled = v1 * 2.0;
    assert!(approx_eq(scaled.x, 2.0));
    assert!(approx_eq(scaled.y, 4.0));
    assert!(approx_eq(scaled.z, 6.0));
}

#[test]
fn test_vector3_dot_cross_normalize() {
    let v1 = Vector3::new(1.0, 0.0, 0.0);
    let v2 = Vector3::new(0.0, 1.0, 0.0);

    let dot = v1.dot(&v2);
    assert!(approx_eq(dot, 0.0));

    let cross = v1.cross(&v2);
    assert!(approx_eq(cross.x, 0.0));
    assert!(approx_eq(cross.y, 0.0));
    assert!(approx_eq(cross.z, 1.0));

    let v3 = Vector3::new(3.0, 4.0, 0.0);
    let norm = v3.normalize();
    assert!(approx_eq(norm.x, 0.6));
    assert!(approx_eq(norm.y, 0.8));
    assert!(approx_eq(norm.z, 0.0));
    assert!(approx_eq(norm.length(), 1.0));
}

#[test]
fn test_quaternion_multiply_inverse() {
    let q1 = Quaternion::new(1.0, 0.0, 0.0, 0.0);
    let q2 = Quaternion::new(0.0, 1.0, 0.0, 0.0);
    let q3 = q1.multiply(&q2);

    assert!(approx_eq(q3.x, 0.0));
    assert!(approx_eq(q3.y, 0.0));
    assert!(approx_eq(q3.z, 1.0));
    assert!(approx_eq(q3.w, 0.0));

    let q4 = Quaternion::new(1.0, 2.0, 3.0, 4.0).normalize();
    let inv = q4.inverse();
    let id = q4.multiply(&inv);
    
    assert!(approx_eq(id.x, 0.0));
    assert!(approx_eq(id.y, 0.0));
    assert!(approx_eq(id.z, 0.0));
    assert!(approx_eq(id.w, 1.0));
}

#[test]
fn test_quaternion_to_matrix() {
    let q = Quaternion::new(0.0, 0.0, 0.0, 1.0);
    let m = q.to_matrix();
    let id = Matrix4x4::identity();

    for i in 0..4 {
        for j in 0..4 {
            assert!(approx_eq(m.m[i][j], id.m[i][j]));
        }
    }
}

#[test]
fn test_matrix4x4_multiply_inverse() {
    let mut m1 = Matrix4x4::identity();
    m1.m[0][1] = 2.0;
    m1.m[1][2] = 3.0;

    let mut m2 = Matrix4x4::identity();
    m2.m[0][1] = -2.0;
    m2.m[1][2] = -3.0;
    m2.m[0][2] = 6.0;

    let id = m1.multiply(&m2);
    let expected_id = Matrix4x4::identity();

    for i in 0..4 {
        for j in 0..4 {
            assert!(approx_eq(id.m[i][j], expected_id.m[i][j]));
        }
    }

    let inv = m1.inverse().unwrap();
    let id2 = m1.multiply(&inv);

    for i in 0..4 {
        for j in 0..4 {
            assert!(approx_eq(id2.m[i][j], expected_id.m[i][j]));
        }
    }
}

#[test]
fn test_gr2_transform_to_matrix() {
    let mut transform = Gr2Transform::new();
    transform.translation = Vector3::new(1.0, 2.0, 3.0);
    transform.scale = Vector3::new(2.0, 2.0, 2.0);

    let m = transform.to_matrix();
    
    assert!(approx_eq(m.m[0][0], 2.0));
    assert!(approx_eq(m.m[1][1], 2.0));
    assert!(approx_eq(m.m[2][2], 2.0));
    assert!(approx_eq(m.m[3][0], 1.0));
    assert!(approx_eq(m.m[3][1], 2.0));
    assert!(approx_eq(m.m[3][2], 3.0));
}
