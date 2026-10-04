use glam::{Mat4, Vec3};
use m2_client::wgpu_texttail_world_renderer::{Color, RenderElement, TextTailData, TextTailWorldRenderer};

#[test]
fn test_project_position() {
    let view_proj = Mat4::IDENTITY;
    let world_pos = Vec3::new(0.0, 0.0, 50.0);
    let height_offset = 10.0;

    let viewport_width = 800.0;
    let viewport_height = 600.0;

    let result = TextTailWorldRenderer::project_position(
        world_pos,
        height_offset,
        &view_proj,
        viewport_width,
        viewport_height,
    );

    assert!(result.is_some());
    let (sx, sy, sz) = result.unwrap();
    // With IDENTITY view_proj:
    // pos_3d = (0, 0, 60, 1)
    // clip_coords = (0, 0, 60, 1)
    // w = 1
    // ndc_x = 0
    // ndc_y = 0
    // ndc_z = 60
    // screen_x = (0 + 1) * 0.5 * 800 = 400
    // screen_y = (1 - 0) * 0.5 * 600 = 300
    assert_eq!(sx, 400.0);
    assert_eq!(sy, 300.0);
    assert_eq!(sz, 60.0);
}

#[test]
fn test_project_position_behind_camera() {
    let view_proj = Mat4::from_cols_array_2d(&[
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, -1.0], // w = -z
        [0.0, 0.0, 0.0, 0.0],
    ]);
    let world_pos = Vec3::new(0.0, 0.0, 50.0); // positive z => negative w
    let result =
        TextTailWorldRenderer::project_position(world_pos, 0.0, &view_proj, 800.0, 600.0);
    assert!(result.is_none());
}

#[test]
fn test_generate_render_elements() {
    let mut renderer = TextTailWorldRenderer::new();

    renderer.add_text_tail(TextTailData {
        id: 1,
        position: Vec3::ZERO,
        height_offset: 10.0,
        name: "TestPlayer".to_string(),
        guild_name: Some("TestGuild".to_string()),
        kingdom_color: Some(Color::new(1.0, 0.0, 0.0, 1.0)), // Red for Shinsoo
        level: Some(75),
        hp_percent: Some(50),
        color: Color::new(1.0, 1.0, 1.0, 1.0),
    });

    let view_proj = Mat4::IDENTITY;
    let elements = renderer.generate_render_elements(&view_proj, 800.0, 600.0);

    // Expected:
    // 1. Text: Guild Name ("TestGuild")
    // 2. Text: Name ("[Lv. 75] TestPlayer")
    // 3. Rect: HP Background
    // 4. Rect: HP Fill
    assert_eq!(elements.len(), 4);

    if let RenderElement::Text { text, color, .. } = &elements[0] {
        assert_eq!(text, "TestGuild");
        assert_eq!(color.r, 1.0);
        assert_eq!(color.g, 0.0);
    } else {
        panic!("Expected Text element for guild");
    }

    if let RenderElement::Text { text, .. } = &elements[1] {
        assert_eq!(text, "[Lv. 75] TestPlayer");
    } else {
        panic!("Expected Text element for name");
    }

    if let RenderElement::Rect { width, color, .. } = &elements[2] {
        assert_eq!(*width, 50.0);
        assert_eq!(color.a, 1.0); // Black bg
    } else {
        panic!("Expected Rect element for HP bg");
    }

    if let RenderElement::Rect { width, color, .. } = &elements[3] {
        assert_eq!(*width, 25.0); // 50% of 50.0
        assert_eq!(color.r, 1.0); // Red fill
    } else {
        panic!("Expected Rect element for HP fill");
    }
}

#[test]
fn test_z_order_sorting() {
    let mut renderer = TextTailWorldRenderer::new();

    // Closer object
    renderer.add_text_tail(TextTailData {
        id: 1,
        position: Vec3::new(0.0, 0.0, 10.0),
        height_offset: 0.0,
        name: "Closer".to_string(),
        guild_name: None,
        kingdom_color: None,
        level: None,
        hp_percent: None,
        color: Color::new(1.0, 1.0, 1.0, 1.0),
    });

    // Further object
    renderer.add_text_tail(TextTailData {
        id: 2,
        position: Vec3::new(0.0, 0.0, 50.0),
        height_offset: 0.0,
        name: "Further".to_string(),
        guild_name: None,
        kingdom_color: None,
        level: None,
        hp_percent: None,
        color: Color::new(1.0, 1.0, 1.0, 1.0),
    });

    let view_proj = Mat4::IDENTITY;
    let elements = renderer.generate_render_elements(&view_proj, 800.0, 600.0);

    assert_eq!(elements.len(), 2);

    // Z-sorting should be descending (back-to-front), so higher Z comes first.
    // With IDENTITY view_proj, z is mapped directly to ndc_z.
    // Further object has z=50, Closer has z=10.
    // So Further should be elements[0], Closer should be elements[1].

    if let RenderElement::Text { text, z, .. } = &elements[0] {
        assert_eq!(text, "Further");
        assert_eq!(*z, 50.0);
    } else {
        panic!("Expected Further text first");
    }

    if let RenderElement::Text { text, z, .. } = &elements[1] {
        assert_eq!(text, "Closer");
        assert_eq!(*z, 10.0);
    } else {
        panic!("Expected Closer text second");
    }
}
