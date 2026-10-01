use glam::{Mat4, Vec3};
use m2_client::wgpu_texttail_renderer::*;

#[test]
fn test_transform_to_screen_in_front() {
    let view_proj = Mat4::IDENTITY;
    let world_pos = Vec3::new(0.0, 0.0, 50.0);
    let height_offset = 10.0;

    // With IDENTITY, w will be 1.0.
    // z = 60.0
    // In NDC: x = 0, y = 0, z = 60
    // Screen mappings: x = 0.5 * width, y = 0.5 * height
    let viewport_width = 800.0;
    let viewport_height = 600.0;

    let result = TextTailRenderer::transform_to_screen(
        world_pos,
        height_offset,
        &view_proj,
        viewport_width,
        viewport_height,
    );
    assert!(result.is_some());
    let (sx, sy, sz) = result.unwrap();
    assert_eq!(sx, 400.0);
    assert_eq!(sy, 300.0);
    assert_eq!(sz, 60.0);
}

#[test]
fn test_transform_to_screen_behind_camera() {
    // A matrix that simulates a point behind the camera (w <= 0.0)
    let view_proj = Mat4::from_cols_array_2d(&[
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, -1.0], // w = -z
        [0.0, 0.0, 0.0, 0.0],
    ]);
    let world_pos = Vec3::new(0.0, 0.0, 50.0); // positive z => negative w
    let result = TextTailRenderer::transform_to_screen(world_pos, 0.0, &view_proj, 800.0, 600.0);
    assert!(result.is_none());
}

#[test]
fn test_alignment_grade_logic() {
    let chivalric = AlignmentGrade::from_alignment(12000);
    assert_eq!(chivalric, AlignmentGrade::Chivalric);
    assert_eq!(chivalric.to_title_string(), Some("Rycerski"));
    assert_eq!(chivalric.color(), Color::new(0.0, 0.0, 1.0, 1.0));

    let neutral = AlignmentGrade::from_alignment(0);
    assert_eq!(neutral, AlignmentGrade::Neutral);
    assert_eq!(neutral.to_title_string(), None);
    assert_eq!(neutral.color(), Color::new(1.0, 1.0, 0.0, 1.0));

    let cruel = AlignmentGrade::from_alignment(-20000);
    assert_eq!(cruel, AlignmentGrade::Cruel);
    assert_eq!(cruel.to_title_string(), Some("Okrutny"));
    assert_eq!(cruel.color(), Color::new(1.0, 0.0, 0.0, 1.0));
}

#[test]
fn test_render_payload_generation() {
    let mut renderer = TextTailRenderer::new();

    // Add a text tail with guild, level, and 50% HP
    renderer.register_text_tail(TextTail {
        virtual_id: 1,
        name: "TestPlayer".to_string(),
        level: Some(75),
        guild_name: Some("TestGuild".to_string()),
        alignment: 15000,
        position: Vec3::new(0.0, 0.0, 0.0),
        height: 10.0,
        distance_from_player: 100.0,
        hp_percent: Some(50),
    });

    let view_proj = Mat4::IDENTITY;
    let payloads = renderer
        .generate_render_payloads(&view_proj, 800.0, 600.0)
        .unwrap();

    // Expected payloads:
    // 1. Text: Guild Name ("TestGuild")
    // 2. Text: Title + Level + Name ("Rycerski [Lv. 75] TestPlayer")
    // 3. Rect: HP Background
    // 4. Rect: HP Fill
    assert_eq!(payloads.len(), 4);

    if let RenderPayload::Text { text, color, .. } = &payloads[0] {
        assert_eq!(text, "TestGuild");
        assert_eq!(color.r, 1.0); // White
    } else {
        panic!("Expected Text payload for guild");
    }

    if let RenderPayload::Text { text, color, .. } = &payloads[1] {
        assert_eq!(text, "Rycerski [Lv. 75] TestPlayer");
        assert_eq!(color.b, 1.0); // Blue (Chivalric)
    } else {
        panic!("Expected Text payload for full name");
    }

    if let RenderPayload::Rect { width, color, .. } = &payloads[2] {
        assert_eq!(*width, 50.0);
        assert_eq!(color.a, 1.0); // Black bg
    } else {
        panic!("Expected Rect payload for HP bg");
    }

    if let RenderPayload::Rect { width, color, .. } = &payloads[3] {
        assert_eq!(*width, 25.0); // 50% of 50.0
        assert_eq!(color.r, 1.0); // Red fill
    } else {
        panic!("Expected Rect payload for HP fill");
    }
}

#[test]
fn test_damage_numbers_generation() {
    let mut renderer = TextTailRenderer::new();

    renderer.add_damage_number(DamageNumber {
        virtual_id: 1,
        damage: 1520,
        flag: DamageFlag::Normal,
        position: Vec3::new(0.0, 0.0, 0.0),
        start_time: 0.0,
        living_time: 0.0,
    });

    renderer.add_damage_number(DamageNumber {
        virtual_id: 2,
        damage: 0,
        flag: DamageFlag::Miss,
        position: Vec3::new(10.0, 0.0, 0.0),
        start_time: 0.0,
        living_time: 0.0,
    });

    let view_proj = Mat4::IDENTITY;
    let payloads = renderer
        .generate_render_payloads(&view_proj, 800.0, 600.0)
        .unwrap();

    assert_eq!(payloads.len(), 2);

    if let RenderPayload::Text { text, color, .. } = &payloads[0] {
        assert_eq!(text, "-1520");
        assert_eq!(color.r, 1.0);
    } else {
        panic!("Expected Text payload for damage");
    }

    if let RenderPayload::Text { text, color, .. } = &payloads[1] {
        assert_eq!(text, "MISS");
        assert_eq!(color.r, 1.0);
    } else {
        panic!("Expected Text payload for miss");
    }
}

#[test]
fn test_damage_number_floating_and_culling() {
    let mut renderer = TextTailRenderer::new();

    renderer.add_damage_number(DamageNumber {
        virtual_id: 1,
        damage: 500,
        flag: DamageFlag::Normal,
        position: Vec3::new(0.0, 0.0, 0.0),
        start_time: 0.0,
        living_time: 1.0,
    });

    // Update with 0.6 delta, this pushes living_time to 1.6
    renderer.update_damage_numbers(0.6);

    // Should be culled (>= 1.5s)
    assert_eq!(renderer.damage_numbers.len(), 0);
}