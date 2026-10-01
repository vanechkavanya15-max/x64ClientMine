use m2_client::wgpu_camera_3d_pipeline::{CameraUniforms, TpsCamera};
use glam::{Vec3, Vec4, Mat4};
use std::mem;

#[test]
fn test_camera_uniforms_layout() {
    let size = mem::size_of::<CameraUniforms>();
    // 16 floats (view) + 16 floats (proj) + 4 floats (pos) = 36 * 4 = 144 bytes
    assert_eq!(size, 144);
    
    let align = mem::align_of::<CameraUniforms>();
    assert_eq!(align, 4); // f32 alignment
}

#[test]
fn test_camera_clamping() {
    let mut camera = TpsCamera::new(Vec3::ZERO, -50.0, 400.0, 100.0);
    
    // Pitch should clamp to [-10.0, 80.0]
    assert_eq!(camera.pitch, TpsCamera::PITCH_MIN);
    
    // Zoom should clamp to [400.0, 2500.0]
    assert_eq!(camera.zoom, TpsCamera::ZOOM_MIN);
    
    // Yaw should wrap to [0.0, 360.0)
    // 400 % 360 = 40.0
    assert!((camera.yaw - 40.0).abs() < 0.0001);

    camera.set_pitch(90.0);
    assert_eq!(camera.pitch, TpsCamera::PITCH_MAX);
    
    camera.set_zoom(3000.0);
    assert_eq!(camera.zoom, TpsCamera::ZOOM_MAX);
    
    camera.set_yaw(-45.0);
    assert!((camera.yaw - 315.0).abs() < 0.0001);
}

#[test]
fn test_camera_pos_and_view_matrix() {
    let target = Vec3::new(10.0, 20.0, 30.0);
    // Pitch 0, Yaw 0 -> Looking along +Y, so eye is at -Y from target
    let zoom = 1000.0;
    let camera = TpsCamera::new(target, 0.0, 0.0, zoom);
    
    let camera_pos = camera.get_camera_pos();
    // With Pitch 0, Yaw 0:
    // dx = zoom * cos(0) * sin(0) = 0
    // dy = zoom * cos(0) * cos(0) = zoom
    // dz = zoom * sin(0) = 0
    // From implementation: eye = target - forward * zoom
    // forward: x=0, y=1, z=0
    // eye = target - (0, 1000, 0)
    
    assert!((camera_pos.x - 10.0).abs() < 0.001);
    assert!((camera_pos.y - (20.0 - zoom)).abs() < 0.001);
    assert!((camera_pos.z - 30.0).abs() < 0.001);

    let view = camera.get_view_matrix();
    
    // Transform the target point with the view matrix
    // The target should end up at the origin (0, 0) in X,Y and -zoom in Z
    let target_view_space = view * Vec4::new(target.x, target.y, target.z, 1.0);
    
    assert!(target_view_space.x.abs() < 0.001);
    assert!(target_view_space.y.abs() < 0.001);
    assert!((target_view_space.z - (-zoom)).abs() < 0.001);
}

#[test]
fn test_projection_matrix() {
    let aspect_ratio = 16.0 / 9.0;
    let proj = TpsCamera::get_projection_matrix(aspect_ratio);
    
    // Verify typical projection properties
    // FOV is 60 degrees. 
    let point_view_space = Vec4::new(0.0, 0.0, -100.0, 1.0);
    let point_clip_space = proj * point_view_space;
    
    assert!(point_clip_space.w > 0.0);
    assert!(point_clip_space.z > 0.0 && point_clip_space.z < point_clip_space.w);
    
    let point_ndc = point_clip_space / point_clip_space.w;
    
    assert_eq!(point_ndc.x, 0.0);
    assert_eq!(point_ndc.y, 0.0);
    assert!(point_ndc.z > 0.0 && point_ndc.z < 1.0);
}

#[test]
fn test_get_uniforms() {
    let target = Vec3::new(10.0, 20.0, 30.0);
    let camera = TpsCamera::new(target, 45.0, 90.0, 1500.0);
    let aspect_ratio = 800.0 / 600.0;
    
    let uniforms = camera.get_uniforms(aspect_ratio);
    
    let view_mat = Mat4::from_cols_array_2d(&uniforms.view_matrix);
    let proj_mat = Mat4::from_cols_array_2d(&uniforms.proj_matrix);
    let expected_view = camera.get_view_matrix();
    let expected_proj = TpsCamera::get_projection_matrix(aspect_ratio);
    
    // Ensure the output is bit-for-bit identical to Mat4 to_cols_array_2d output
    assert_eq!(view_mat, expected_view);
    assert_eq!(proj_mat, expected_proj);
    
    // Check camera pos length
    let expected_pos = camera.get_camera_pos();
    assert!((uniforms.camera_pos[0] - expected_pos.x).abs() < 0.001);
    assert!((uniforms.camera_pos[1] - expected_pos.y).abs() < 0.001);
    assert!((uniforms.camera_pos[2] - expected_pos.z).abs() < 0.001);
    assert_eq!(uniforms.camera_pos[3], 1.0);
}

#[test]
fn test_world_to_clip_space_transform() {
    let target = Vec3::new(500.0, -200.0, 150.0);
    let camera = TpsCamera::new(target, 30.0, 45.0, 800.0);
    let aspect_ratio = 16.0 / 9.0;
    
    let view = camera.get_view_matrix();
    let proj = TpsCamera::get_projection_matrix(aspect_ratio);
    
    let view_proj = proj * view;
    
    let target_world = Vec4::new(target.x, target.y, target.z, 1.0);
    let target_clip = view_proj * target_world;
    let target_ndc = target_clip / target_clip.w;
    
    // Since the camera is looking directly at the target, the target should be at the center of the screen
    assert!(target_ndc.x.abs() < 0.001);
    assert!(target_ndc.y.abs() < 0.001);
    
    // And it should be in front of the camera (z > 0 and z < 1 in WGPU clip space)
    assert!(target_ndc.z > 0.0 && target_ndc.z < 1.0);
}
