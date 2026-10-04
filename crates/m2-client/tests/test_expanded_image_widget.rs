// test_expanded_image_widget.rs
// Unit tests for ExpandedImageBox widget

use m2_client::ui_wndmgr_widget_expanded_image::{ExpandedImageBox, RenderingMode};

#[test]
fn test_expanded_image_default_initialization() {
    let image_box = ExpandedImageBox::new();
    
    assert_eq!(image_box.position, [0.0, 0.0]);
    assert_eq!(image_box.width, 0.0);
    assert_eq!(image_box.height, 0.0);
    assert_eq!(image_box.depth, 0.0);
    assert_eq!(image_box.origin, [0.0, 0.0]);
    assert_eq!(image_box.scale, [1.0, 1.0]);
    assert_eq!(image_box.rotation, 0.0);
    assert_eq!(image_box.rendering_rect, [0.0, 0.0, 0.0, 0.0]);
    assert_eq!(image_box.rendering_mode, RenderingMode::Normal);
    assert_eq!(image_box.diffuse_color, 0xFFFFFFFF);
    assert_eq!(image_box.image_rect, [0.0, 0.0, 0.0, 0.0]);
    assert_eq!(image_box.texture_width, 1.0);
    assert_eq!(image_box.texture_height, 1.0);
}

#[test]
fn test_expanded_image_scaling_transformation() {
    let mut image_box = ExpandedImageBox::new();
    image_box.set_position(10.0, 20.0);
    image_box.set_size(100.0, 50.0);
    image_box.set_scale(2.0, 0.5); // img_width = 200.0, img_height = 25.0
    image_box.set_texture_size(200.0, 100.0);
    image_box.set_image_rect(0.0, 0.0, 100.0, 50.0);

    let vertices = image_box.generate_vertices();

    // Check positions (no rendering rect, so it just adds img_width/height)
    // base position: [9.5, 19.5, 0.0]
    // vertices[0]: base
    assert_eq!(vertices[0].position, [9.5, 19.5, 0.0]);
    // vertices[1]: base + [img_width, 0] = [209.5, 19.5, 0.0]
    assert_eq!(vertices[1].position, [209.5, 19.5, 0.0]);
    // vertices[2]: base + [0, img_height] = [9.5, 44.5, 0.0]
    assert_eq!(vertices[2].position, [9.5, 44.5, 0.0]);
    // vertices[3]: base + [img_width, img_height] = [209.5, 44.5, 0.0]
    assert_eq!(vertices[3].position, [209.5, 44.5, 0.0]);
}

#[test]
fn test_expanded_image_rotation_transformation() {
    let mut image_box = ExpandedImageBox::new();
    image_box.set_position(10.0, 20.0);
    image_box.set_size(100.0, 50.0);
    image_box.set_scale(1.0, 1.0);
    image_box.set_origin(50.0, 25.0); // Center pivot
    image_box.set_rotation(90.0);
    image_box.set_texture_size(200.0, 100.0);
    image_box.set_image_rect(0.0, 0.0, 100.0, 50.0);

    let vertices = image_box.generate_vertices();

    let cos_90 = 90.0f32.to_radians().cos();
    let sin_90 = 90.0f32.to_radians().sin();
    
    // Half width = 50.0, Half height = 25.0
    // Base vertex position + origin
    let base_x = 9.5 + 50.0;
    let base_y = 19.5 + 25.0;

    // Verify v0
    let v0_x = base_x + (-50.0 * cos_90) - (-25.0 * sin_90);
    let v0_y = base_y + (-50.0 * sin_90) + (-25.0 * cos_90);
    assert!((vertices[0].position[0] - v0_x).abs() < 1e-4);
    assert!((vertices[0].position[1] - v0_y).abs() < 1e-4);

    // Verify v1
    let v1_x = base_x + (50.0 * cos_90) - (-25.0 * sin_90);
    let v1_y = base_y + (50.0 * sin_90) + (-25.0 * cos_90);
    assert!((vertices[1].position[0] - v1_x).abs() < 1e-4);
    assert!((vertices[1].position[1] - v1_y).abs() < 1e-4);

    // Verify v3
    let v3_x = base_x + (50.0 * cos_90) - (25.0 * sin_90);
    let v3_y = base_y + (50.0 * sin_90) + (25.0 * cos_90);
    assert!((vertices[3].position[0] - v3_x).abs() < 1e-4);
    assert!((vertices[3].position[1] - v3_y).abs() < 1e-4);
}

#[test]
fn test_expanded_image_uv_clipping() {
    let mut image_box = ExpandedImageBox::new();
    image_box.set_position(0.0, 0.0);
    image_box.set_size(100.0, 100.0);
    image_box.set_texture_size(200.0, 200.0); // Texture is 200x200
    image_box.set_image_rect(50.0, 50.0, 150.0, 150.0); // Image mapped to center 100x100 of texture
    
    // Apply rendering rect clipping (left, top, right, bottom relative to width/height)
    image_box.set_rendering_rect(0.1, 0.2, -0.1, -0.2); // +10px left, +20px top, -10px right, -20px bottom
    
    let vertices = image_box.generate_vertices();

    // Check su, sv, eu, ev mapped back to texture size
    // texReverse = 1/200 = 0.005
    // rendering_rect values: left = 10, top = 20, right = -10, bottom = -20
    // su = (50 + 10) / 200 = 60 / 200 = 0.3
    // sv = (50 + 20) / 200 = 70 / 200 = 0.35
    // eu = (150 + -10) / 200 = 140 / 200 = 0.7
    // ev = (150 + -20) / 200 = 130 / 200 = 0.65

    assert!((vertices[0].tex_coord[0] - 0.3).abs() < 1e-5);
    assert!((vertices[0].tex_coord[1] - 0.35).abs() < 1e-5);

    assert!((vertices[1].tex_coord[0] - 0.7).abs() < 1e-5);
    assert!((vertices[1].tex_coord[1] - 0.35).abs() < 1e-5);

    assert!((vertices[2].tex_coord[0] - 0.3).abs() < 1e-5);
    assert!((vertices[2].tex_coord[1] - 0.65).abs() < 1e-5);

    assert!((vertices[3].tex_coord[0] - 0.7).abs() < 1e-5);
    assert!((vertices[3].tex_coord[1] - 0.65).abs() < 1e-5);
}
