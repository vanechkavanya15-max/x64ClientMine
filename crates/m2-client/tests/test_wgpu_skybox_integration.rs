use m2_client::wgpu_skybox_integration::{parse_environment_data, EnvironmentData};

#[test]
fn test_parse_environment_data_empty() {
    let text = "";
    let env = parse_environment_data(text).expect("Failed to parse");
    assert_eq!(env.b_sky_box_texture_render_mode, false);
    assert_eq!(env.f_cloud_height, 1.0);
}

#[test]
fn test_parse_environment_data_basic() {
    let text = r#"
        skybox
        {
            btexturerendermode 1
            scale 1.5 2.0 2.5
            gradientlevelupper 5
            gradientlevellower 10
        }
    "#;
    let env = parse_environment_data(text).expect("Failed to parse");
    assert_eq!(env.b_sky_box_texture_render_mode, true);
    assert_eq!(env.v3_sky_box_scale, [1.5, 2.0, 2.5]);
    assert_eq!(env.by_sky_box_gradient_level_upper, 5);
    assert_eq!(env.by_sky_box_gradient_level_lower, 10);
}

#[test]
fn test_parse_environment_data_filenames() {
    let text = r#"
        skybox
        {
            frontfacefilename ymir_work/environment/front.tga
            backfacefilename ymir_work/environment/back.tga
            leftfacefilename ymir_work/environment/left.tga
            rightfacefilename ymir_work/environment/right.tga
            topfacefilename ymir_work/environment/top.tga
            bottomfacefilename ymir_work/environment/bottom.tga
        }
    "#;
    let env = parse_environment_data(text).expect("Failed to parse");
    assert_eq!(env.str_sky_box_face_file_name[0], "ymir_work/environment/front.tga");
    assert_eq!(env.str_sky_box_face_file_name[1], "ymir_work/environment/back.tga");
    assert_eq!(env.str_sky_box_face_file_name[2], "ymir_work/environment/left.tga");
    assert_eq!(env.str_sky_box_face_file_name[3], "ymir_work/environment/right.tga");
    assert_eq!(env.str_sky_box_face_file_name[4], "ymir_work/environment/top.tga");
    assert_eq!(env.str_sky_box_face_file_name[5], "ymir_work/environment/bottom.tga");
}

#[test]
fn test_parse_environment_data_clouds() {
    let text = r#"
        skybox
        {
            cloudscale 2.0 3.0
            cloudheight 50.0
            cloudtexturescale 0.5 0.5
            cloudspeed 0.1 0.2
            cloudtexturefilename ymir_work/environment/cloud.tga
        }
    "#;
    let env = parse_environment_data(text).expect("Failed to parse");
    assert_eq!(env.v2_cloud_scale, [2.0, 3.0]);
    assert_eq!(env.f_cloud_height, 50.0);
    assert_eq!(env.v2_cloud_texture_scale, [0.5, 0.5]);
    assert_eq!(env.v2_cloud_speed, [0.1, 0.2]);
    assert_eq!(env.str_cloud_texture_file_name, "ymir_work/environment/cloud.tga");
}

#[test]
fn test_parse_environment_data_ignore_other_blocks() {
    let text = r#"
        fog
        {
            color 0.5 0.5 0.5
        }
        skybox
        {
            btexturerendermode 1
        }
        material
        {
            ambient 1.0 1.0 1.0
        }
    "#;
    let env = parse_environment_data(text).expect("Failed to parse");
    assert_eq!(env.b_sky_box_texture_render_mode, true);
}
