use glam::{Mat4, Vec3, Vec4};
use m2_client::wgpu_skybox_lighting::{
    calculate_day_night_cycle, interpolate_color, SkyboxLightingSystem,
};

#[test]
fn test_update_camera_strips_translation() {
    let mut system = SkyboxLightingSystem::new();

    let view = Mat4::from_translation(Vec3::new(100.0, 200.0, 300.0))
        * Mat4::from_rotation_y(std::f32::consts::PI / 4.0);
    let proj = Mat4::perspective_rh(std::f32::consts::PI / 4.0, 16.0 / 9.0, 0.1, 1000.0);

    system.update_camera(view, proj);

    let view_proj_cols = system.uniforms.skybox_view_proj;
    let view_proj = Mat4::from_cols_array_2d(&view_proj_cols);

    // Aby zweryfikowac, ze translacja zostala usunieta z macierzy widoku:
    // tworzymy macierz widoku BEZ translacji i sprawdzamy czy view_proj na tej bazie da to samo
    let mut view_no_trans = view;
    view_no_trans.w_axis = Vec4::new(0.0, 0.0, 0.0, 1.0);
    let expected_view_proj = proj * view_no_trans;

    // Poniewaz operacje zmiennoprzecinkowe moga dawac minimalne roznice, porownujemy kazdy element
    for (col_idx, col) in expected_view_proj.to_cols_array_2d().iter().enumerate() {
        for (row_idx, &val) in col.iter().enumerate() {
            let actual_val = view_proj_cols[col_idx][row_idx];
            assert!(
                (actual_val - val).abs() < 1e-5,
                "Mismatch at [{}][{}]: expected {}, got {}",
                col_idx,
                row_idx,
                val,
                actual_val
            );
        }
    }
}

#[test]
fn test_update_fog() {
    let mut system = SkyboxLightingSystem::new();
    system.update_fog(200.0, 800.0, [0.1, 0.2, 0.3, 1.0]);

    assert_eq!(system.uniforms.fog_params[0], 200.0);
    assert_eq!(system.uniforms.fog_params[1], 800.0);
    assert_eq!(system.uniforms.fog_color, [0.1, 0.2, 0.3, 1.0]);
}

#[test]
fn test_update_lighting() {
    let mut system = SkyboxLightingSystem::new();
    let sun_dir = Vec3::new(0.5, 0.5, -0.5).normalize();
    system.update_lighting(sun_dir, [0.1, 0.1, 0.1, 1.0], [0.8, 0.8, 0.8, 1.0]);

    assert_eq!(
        system.uniforms.sun_direction,
        [sun_dir.x, sun_dir.y, sun_dir.z, 0.0]
    );
    assert_eq!(system.uniforms.ambient_color, [0.1, 0.1, 0.1, 1.0]);
    assert_eq!(system.uniforms.diffuse_color, [0.8, 0.8, 0.8, 1.0]);
}

#[test]
fn test_interpolate_color() {
    let start = [0.0, 0.0, 0.0, 0.0];
    let end = [1.0, 1.0, 1.0, 1.0];

    let mid = interpolate_color(start, end, 0.5);
    assert_eq!(mid, [0.5, 0.5, 0.5, 0.5]);

    let start_clamped = interpolate_color(start, end, -0.5);
    assert_eq!(start_clamped, start);

    let end_clamped = interpolate_color(start, end, 1.5);
    assert_eq!(end_clamped, end);
}

#[test]
fn test_calculate_day_night_cycle() {
    let day_ambient = [0.5, 0.5, 0.5, 1.0];
    let night_ambient = [0.1, 0.1, 0.1, 1.0];
    let day_diffuse = [0.9, 0.9, 0.9, 1.0];
    let night_diffuse = [0.2, 0.2, 0.2, 1.0];

    // Srodek dnia (time = 0.5)
    let (ambient, diffuse, sun_dir) = calculate_day_night_cycle(
        0.5,
        day_ambient,
        night_ambient,
        day_diffuse,
        night_diffuse,
    );

    assert_eq!(ambient, day_ambient);
    assert_eq!(diffuse, day_diffuse);
    // slonce gdzies w punkcie kulminacyjnym, cos(pi)=-1, sin(pi)=0, ale my sprawdzamy normalizacje
    assert!((sun_dir.length() - 1.0).abs() < 1e-5);
    assert_eq!(sun_dir.z, 0.5 / ((1.0_f32).powi(2) + 0.5_f32.powi(2)).sqrt()); // zalezy od dlugosci

    // Srodek nocy (time = 0.0 lub 1.0)
    let (ambient, diffuse, _sun_dir) = calculate_day_night_cycle(
        0.0,
        day_ambient,
        night_ambient,
        day_diffuse,
        night_diffuse,
    );

    assert_eq!(ambient, night_ambient);
    assert_eq!(diffuse, night_diffuse);
}

#[test]
fn test_default_environment_uniforms() {
    let system = SkyboxLightingSystem::new();
    assert_eq!(
        system.uniforms.skybox_view_proj,
        Mat4::IDENTITY.to_cols_array_2d()
    );
    assert_eq!(system.uniforms.sun_direction, [0.0, -1.0, 0.0, 0.0]);
}
