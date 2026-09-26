use glam::Vec3;
use m2_client::skybox_lighting_cycle_v2::*;

#[test]
fn test_fog_params_calculation() {
    let mut env_data = SEnvironmentData::default();
    env_data.fog_enable = TRUE;
    env_data.fog_near_distance = 1000.0;
    env_data.fog_far_distance = 5000.0;
    env_data.fog_color = 0xFFAABBCC;

    // Python fog level 0 (Light): scale = 0.75
    let params = calculate_fog_params(&env_data, 0);
    assert_eq!(params.near_distance, 750.0);
    assert_eq!(params.far_distance, 3750.0);
    assert_eq!(params.color, 0xFFAABBCC);

    // Python fog level 1 (Middle): scale = 1.0
    let params = calculate_fog_params(&env_data, 1);
    assert_eq!(params.near_distance, 1000.0);
    assert_eq!(params.far_distance, 5000.0);
    assert_eq!(params.color, 0xFFAABBCC);

    // Python fog level 2 (Dense): scale = 1.25
    let params = calculate_fog_params(&env_data, 2);
    assert_eq!(params.near_distance, 1250.0);
    assert_eq!(params.far_distance, 6250.0);
    assert_eq!(params.color, 0xFFAABBCC);
}

#[test]
fn test_fog_disabled() {
    let mut env_data = SEnvironmentData::default();
    env_data.fog_enable = FALSE;
    env_data.fog_near_distance = 1000.0;
    env_data.fog_far_distance = 5000.0;
    env_data.fog_color = 0xFFAABBCC;

    let params = calculate_fog_params(&env_data, 1);
    assert_eq!(params.near_distance, 0.0);
    assert_eq!(params.far_distance, 0.0);
    assert_eq!(params.color, 0xFFAABBCC);
}

#[test]
fn test_ffi_fog_params() {
    let mut env_data = SEnvironmentData::default();
    env_data.fog_enable = TRUE;
    env_data.fog_near_distance = 1000.0;
    env_data.fog_far_distance = 5000.0;
    env_data.fog_color = 0xFFAABBCC;

    let env_ptr: *const SEnvironmentData = &env_data;
    let params = m2_skybox_get_fog_params(env_ptr, 1);
    assert_eq!(params.near_distance, 1000.0);
    assert_eq!(params.far_distance, 5000.0);
    assert_eq!(params.color, 0xFFAABBCC);
}

#[test]
fn test_ffi_fog_params_null_ptr() {
    let env_ptr: *const SEnvironmentData = std::ptr::null();
    let params = m2_skybox_get_fog_params(env_ptr, 1);
    assert_eq!(params.near_distance, 0.0);
    assert_eq!(params.far_distance, 0.0);
    assert_eq!(params.color, 0);
}

#[test]
fn test_color_transition_helper_zero_duration() {
    let mut helper = ColorTransitionHelper::new();
    helper.clear(0.0, 0.0, 0.0, 0.0);
    helper.set_transition(1.0, 1.0, 1.0, 1.0, 0);
    helper.start_transition();

    // With 0 duration, the percent calculation sets to 1.0 immediately
    let is_running = helper.update_with_elapsed(0);
    assert_eq!(is_running, false);
    assert_eq!(helper.transition_started, false);

    // (A * 255) << 24 | (R * 255) << 16 | (G * 255) << 8 | (B * 255)
    // 255 = 0xFF, so result should be 0xFFFFFFFF
    assert_eq!(helper.get_cur_color(), 0xFFFFFFFF);
}

#[test]
fn test_color_transition_helper_lerp() {
    let mut helper = ColorTransitionHelper::new();
    helper.set_src_color(0.0, 0.0, 0.0, 0.0);
    helper.set_transition(1.0, 0.5, 0.25, 0.1, 1000);
    helper.start_transition();

    // At 500ms (halfway)
    let is_running = helper.update_with_elapsed(500);
    assert_eq!(is_running, true);

    // R = 0.5, G = 0.25, B = 0.125, A = 0.05
    let a = (0.05 * 255.0) as u32; // ~12
    let r = (0.5 * 255.0) as u32; // ~127
    let g = (0.25 * 255.0) as u32; // ~63
    let b = (0.125 * 255.0) as u32; // ~31

    let expected_color = (a << 24) | (r << 16) | (g << 8) | b;
    assert_eq!(helper.get_cur_color(), expected_color);

    // At 1000ms (end)
    let is_running_end = helper.update_with_elapsed(1000);
    assert_eq!(is_running_end, false);

    let a_end = (0.1 * 255.0) as u32; // ~25
    let r_end = (1.0 * 255.0) as u32; // ~255
    let g_end = (0.5 * 255.0) as u32; // ~127
    let b_end = (0.25 * 255.0) as u32; // ~63

    let expected_color_end = (a_end << 24) | (r_end << 16) | (g_end << 8) | b_end;
    assert_eq!(helper.get_cur_color(), expected_color_end);
}

#[test]
fn test_skybox_transition() {
    let mut skybox = SkyBox::new();

    let c1 = TGradientColor {
        first_color: TColor {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        },
        second_color: TColor {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        },
    };
    let c2 = TGradientColor {
        first_color: TColor {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
        second_color: TColor {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
    };

    skybox.set_cloud_color(&c1, &c2, 1000);

    let sky_colors_1 = vec![c1];
    let sky_colors_2 = vec![c2];
    skybox.set_sky_color(sky_colors_1.as_ptr(), sky_colors_1.len(), sky_colors_2.as_ptr(), sky_colors_2.len(), 1000);

    skybox.start_transition();

    assert_eq!(skybox.cloud_helpers.len(), 1);
    assert_eq!(skybox.sky_helpers.len(), 1);

    assert_eq!(skybox.cloud_helpers[0].transition_started, true);
    assert_eq!(skybox.sky_helpers[0].transition_started, true);
}

#[test]
fn test_d3d_light_default_values() {
    let light = D3dLight9::default();
    assert_eq!(light.type_, 0);
    assert_eq!(light.diffuse.r, 0.0);
    assert_eq!(light.position, Vec3::ZERO);
    assert_eq!(light.range, 0.0);
}
