use m2_client::horse_rider_sync_v2::m2_mount_get_rider_matrix;
use std::ptr;

#[test]
fn test_m2_mount_get_rider_matrix_null_horse() {
    let mut out_rider_mat: [f32; 16] = [0.0; 16];
    let result = unsafe { m2_mount_get_rider_matrix(ptr::null(), out_rider_mat.as_mut_ptr()) };
    assert!(!result);
}

#[test]
fn test_m2_mount_get_rider_matrix_null_rider() {
    let horse_mat: [f32; 16] = [0.0; 16];
    let result = unsafe { m2_mount_get_rider_matrix(horse_mat.as_ptr(), ptr::null_mut()) };
    assert!(!result);
}

#[test]
fn test_m2_mount_get_rider_matrix_both_null() {
    let result = unsafe { m2_mount_get_rider_matrix(ptr::null(), ptr::null_mut()) };
    assert!(!result);
}

#[test]
fn test_m2_mount_get_rider_matrix_identity() {
    let horse_mat: [f32; 16] = [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    let mut out_rider_mat: [f32; 16] = [0.0; 16];
    let result = unsafe { m2_mount_get_rider_matrix(horse_mat.as_ptr(), out_rider_mat.as_mut_ptr()) };
    assert!(result);
    assert_eq!(horse_mat, out_rider_mat);
}

#[test]
fn test_m2_mount_get_rider_matrix_translation() {
    let horse_mat: [f32; 16] = [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        15.5, 20.0, 10.1, 1.0,
    ];
    let mut out_rider_mat: [f32; 16] = [0.0; 16];
    let result = unsafe { m2_mount_get_rider_matrix(horse_mat.as_ptr(), out_rider_mat.as_mut_ptr()) };
    assert!(result);
    assert_eq!(horse_mat, out_rider_mat);
}

#[test]
fn test_m2_mount_get_rider_matrix_rotation_scale() {
    let horse_mat: [f32; 16] = [
        2.0, 0.5, 0.0, 0.0,
        -0.5, 2.0, 0.0, 0.0,
        0.0, 0.0, 3.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    let mut out_rider_mat: [f32; 16] = [0.0; 16];
    let result = unsafe { m2_mount_get_rider_matrix(horse_mat.as_ptr(), out_rider_mat.as_mut_ptr()) };
    assert!(result);
    assert_eq!(horse_mat, out_rider_mat);
}
