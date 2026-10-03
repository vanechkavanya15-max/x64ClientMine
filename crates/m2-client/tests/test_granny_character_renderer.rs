use m2_client::wgpu_engine_context::WgpuEngineContext;
use m2_client::character_model_vfs_loader::{load_warrior_model, load_sword_weapon_model, calculate_mesh_bounding_box};

#[test]
fn test_load_warrior_and_sword_meshes() {
    let ctx = WgpuEngineContext::new(0, 256, 256).expect("Failed to initialize headless WGPU context");

    let warrior = load_warrior_model(&ctx.device).expect("Failed to load warrior model");
    assert!(!warrior.primary_mesh.vertices.is_empty(), "Warrior mesh vertices must not be empty");
    assert!(!warrior.primary_mesh.indices.is_empty(), "Warrior mesh indices must not be empty");
    assert_eq!(warrior.name, "warrior_novice");

    let sword = load_sword_weapon_model(&ctx.device).expect("Failed to load sword model");
    assert!(!sword.primary_mesh.vertices.is_empty(), "Sword mesh vertices must not be empty");
    assert!(!sword.primary_mesh.indices.is_empty(), "Sword mesh indices must not be empty");
    assert_eq!(sword.name, "sword_01");

    let (min_b, max_b) = calculate_mesh_bounding_box(&warrior.primary_mesh);
    assert!(max_b[2] > min_b[2], "Warrior height bounding box must be positive");
}

#[test]
fn test_render_character_to_rgba_image() {
    let ctx = WgpuEngineContext::new(0, 512, 512).expect("Failed to initialize headless WGPU context");

    let clear_color = wgpu::Color { r: 0.08, g: 0.08, b: 0.12, a: 1.0 };
    let player_pos = [0.0, 0.0, 0.0];
    let player_rot_deg = 35.0;

    let img = ctx.render_character_to_rgba_image(clear_color, player_pos, player_rot_deg)
        .expect("Failed to render character to RGBA image in headless mode");

    assert_eq!(img.width(), 512);
    assert_eq!(img.height(), 512);
    assert!(!img.as_raw().is_empty());

    let artifact_path = "C:\\Users\\ADMIN\\.gemini\\antigravity\\brain\\56907d9d-0d1a-48a4-b2d8-374bdf3fa6e0\\warrior_3d_render_live.png";
    let _ = img.save(artifact_path);
}
