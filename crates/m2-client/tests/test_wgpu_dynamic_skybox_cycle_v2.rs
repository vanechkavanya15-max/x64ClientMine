// test_wgpu_dynamic_skybox_cycle_v2.rs
//
// 100% C++ 1:1 Parity Metin2 SkyBox Implementation in Pure Rust Test File

#[path = "../src/wgpu_dynamic_skybox_cycle_v2.rs"]
mod wgpu_dynamic_skybox_cycle_v2;

use wgpu_dynamic_skybox_cycle_v2::*;

#[test]
fn test_color_transition_helper() {
    let mut helper = ColorTransitionHelper::new();
    
    // Test Initial State
    assert_eq!(helper.get_cur_color(), 0);
    assert_eq!(helper.is_transition_started(), false);

    // Set Src and Dst
    helper.set_src_color(0.0, 0.0, 0.0, 0.0);
    helper.set_transition(1.0, 0.5, 0.25, 1.0, 1000); // 1000ms duration

    helper.start_transition(0); // Start at time 0
    assert_eq!(helper.is_transition_started(), true);

    // Update at time 500ms (50%)
    let mut still_running = helper.update(500);
    assert_eq!(still_running, true);
    assert_eq!(helper.is_transition_started(), true);

    let cur_color = helper.get_cur_color();
    // Expected at 50%: R: 0.5, G: 0.25, B: 0.125, A: 0.5
    // ARGB:
    // A: 0.5 * 255 = 127
    // R: 0.5 * 255 = 127
    // G: 0.25 * 255 = 63
    // B: 0.125 * 255 = 31
    let a = ((0.5 * 255.0) as u32) & 0xff;
    let r = ((0.5 * 255.0) as u32) & 0xff;
    let g = ((0.25 * 255.0) as u32) & 0xff;
    let b = ((0.125 * 255.0) as u32) & 0xff;
    let expected_color = (a << 24) | (r << 16) | (g << 8) | b;
    assert_eq!(cur_color, expected_color);

    // Update at time 1000ms (100%)
    still_running = helper.update(1000);
    assert_eq!(still_running, false);
    assert_eq!(helper.is_transition_started(), false);

    let final_color = helper.get_cur_color();
    let a_f = ((1.0 * 255.0) as u32) & 0xff;
    let r_f = ((1.0 * 255.0) as u32) & 0xff;
    let g_f = ((0.5 * 255.0) as u32) & 0xff;
    let b_f = ((0.25 * 255.0) as u32) & 0xff;
    let expected_final_color = (a_f << 24) | (r_f << 16) | (g_f << 8) | b_f;
    assert_eq!(final_color, expected_final_color);
}

#[test]
fn test_sky_object_quad() {
    let mut quad = SkyObjectQuad::new();
    quad.clear(0, 1.0, 0.0, 0.0, 1.0);
    quad.set_src_color(1, 0.0, 1.0, 0.0, 1.0);
    quad.set_transition(1, 1.0, 1.0, 1.0, 1.0, 2000);

    let v = TPDTVertex {
        position: TPosition::new(1.0, 2.0, 3.0),
        diffuse: 0,
        tex_coord: TTextureCoordinate::new(0.5, 0.5),
    };

    quad.set_vertex(0, &v);

    let index_0_mapped = quad.m_indices[0] as usize;
    assert_eq!(quad.m_vertex[index_0_mapped].position.x, 1.0);
    assert_eq!(quad.m_vertex[index_0_mapped].position.y, 2.0);
    assert_eq!(quad.m_vertex[index_0_mapped].position.z, 3.0);
}

#[test]
fn test_skybox_set_gradient_level_and_refresh() {
    let mut skybox = SkyBox::new(0);
    skybox.m_object.set_render_mode(SKY_RENDER_MODE_DEFAULT);
    
    // Set 2 upper layers, 1 lower layer -> total 3 layers per vertical face
    skybox.set_gradient_level(2, 1);
    
    skybox.refresh();

    // Front face (index 0) should be populated with 3 quads
    assert_eq!(skybox.m_faces[0].m_strfacename, "front");
    assert_eq!(skybox.m_faces[0].m_sky_object_quad_vector.len(), 3);

    let quads = &skybox.m_faces[0].m_sky_object_quad_vector;
    
    // First upper layer (ucY = 0)
    // 1.0f - (float)(ucY + 1)/ (float)(m_ucVirticalGradientLevelUpper)
    // 1.0 - 1.0 / 2.0 = 0.5
    let v0_z_l0 = 0.5;
    let v0_v_l0 = (1.0 / 2.0) * 0.5; // 0.25

    let v1_z_l0 = 1.0; // 1.0 - 0.0/2.0
    let v1_v_l0 = 0.0; // 0.0 * 0.5

    assert_eq!(quads[0].m_vertex[quads[0].m_indices[0] as usize].position.z, v0_z_l0);
    assert_eq!(quads[0].m_vertex[quads[0].m_indices[0] as usize].tex_coord.y, v0_v_l0);
    assert_eq!(quads[0].m_vertex[quads[0].m_indices[1] as usize].position.z, v1_z_l0);
    assert_eq!(quads[0].m_vertex[quads[0].m_indices[1] as usize].tex_coord.y, v1_v_l0);

    // Check Lower layer (ucY = 0) -> this is at index 2
    let lower_quad = &quads[2];
    
    // -(float)(ucY + 1)/ (float)(m_ucVirticalGradientLevelLower) -> -1.0/1.0 = -1.0
    let v0_z_lower = -1.0;
    // The C++ Bug: 0.5f + (float)(ucY + 1)/ (float)(m_ucVirticalGradientLevelUpper) * 0.5f -> 0.5 + (1.0/2.0)*0.5 = 0.75
    let v0_v_lower = 0.75;
    assert_eq!(lower_quad.m_vertex[lower_quad.m_indices[0] as usize].position.z, v0_z_lower);
    assert_eq!(lower_quad.m_vertex[lower_quad.m_indices[0] as usize].tex_coord.y, v0_v_lower);

    // Horizon faces (top/bottom) should have 1 quad
    assert_eq!(skybox.m_faces[4].m_sky_object_quad_vector.len(), 1);
    assert_eq!(skybox.m_faces[5].m_sky_object_quad_vector.len(), 1);

    // Cloud should have 1 quad
    assert_eq!(skybox.m_object.m_face_cloud.m_sky_object_quad_vector.len(), 1);
}

#[test]
fn test_skybox_set_sky_color() {
    let mut skybox = SkyBox::new(0);
    skybox.m_object.set_render_mode(SKY_RENDER_MODE_DEFAULT);
    skybox.set_gradient_level(1, 1);
    skybox.refresh();

    let c1 = TGradientColor {
        m_first_color: TColor::new(1.0, 0.0, 0.0, 1.0),
        m_second_color: TColor::new(0.5, 0.0, 0.0, 1.0),
    };
    let c2 = TGradientColor {
        m_first_color: TColor::new(0.0, 1.0, 0.0, 1.0),
        m_second_color: TColor::new(0.0, 0.5, 0.0, 1.0),
    };

    let colors = vec![c1.clone(), c2.clone()];
    
    let n1 = TGradientColor {
        m_first_color: TColor::new(0.0, 0.0, 1.0, 1.0),
        m_second_color: TColor::new(0.0, 0.0, 0.5, 1.0),
    };
    let n2 = TGradientColor {
        m_first_color: TColor::new(1.0, 1.0, 0.0, 1.0),
        m_second_color: TColor::new(0.5, 0.5, 0.0, 1.0),
    };
    let next_colors = vec![n1.clone(), n2.clone()];

    skybox.set_sky_color(&colors, &next_colors, 1000);

    // Check bottom face (Face 5) which is populated with the last color gradient.
    // Length is 2, so index 1 is used.
    let quad = &skybox.m_faces[5].m_sky_object_quad_vector[0];
    
    // aSkyObjectQuad.SetSrcColor(0, c_rColorVector[ulVectorGradientColornum].m_SecondColor...)
    // Index 1 second color is c2.m_second_color
    let expected_src_red = c2.m_second_color.r;
    assert_eq!(quad.m_helper[0].m_f_src_red, expected_src_red);

    // aSkyObjectQuad.SetTransition(0, c_rNextColorVector[ulVectorGradientColornum].m_SecondColor...)
    let expected_dst_red = n2.m_second_color.r;
    assert_eq!(quad.m_helper[0].m_f_dst_red, expected_dst_red);
}

#[test]
fn test_skybox_render_mode_texture() {
    let mut skybox = SkyBox::new(0);
    skybox.m_object.set_render_mode(SKY_RENDER_MODE_TEXTURE);
    skybox.refresh();

    // In TEXTURE mode, every face uses horizon quad projection (1 quad per face)
    for i in 0..6 {
        assert_eq!(skybox.m_faces[i].m_sky_object_quad_vector.len(), 1);
    }
}
