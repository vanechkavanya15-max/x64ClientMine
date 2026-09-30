use m2_client::vfs_pck_memory_mapped::{mount_archive, read_file_from_archive};
use m2_client::wgpu_engine_context::{WgpuEngineContext, Quad2D, GpuTexture, set_global_wgpu_context, set_global_login_bg};

#[test]
fn test_decode_and_render_login_assets() {
    let base_pack_dir = "E:/m2dev-client-src-main/m2dev-client-main/pack";
    let etc_path = format!("{}/ETC.pck", base_pack_dir);
    if !std::path::Path::new(&etc_path).exists() {
        println!("ETC.pck not found, skipping");
        return;
    }

    // Montowanie archiwow w py_bridge_pack
    for p in &["ETC.pck", "locale.pck", "uiscript.pck", "root.pck"] {
        let p_path = format!("{}/{}", base_pack_dir, p);
        if std::path::Path::new(&p_path).exists() {
            let _ = m2_client::py_bridge_pack::mount_pack_file(&p_path);
        }
    }

    let archive = mount_archive(&etc_path).expect("Failed to mount ETC.pck");

    // 1. Odczyt i dekodowanie login.jpg
    let login_jpg_data = read_file_from_archive(&archive, "d:/ymir work/ui/intro/login/login.jpg");
    assert!(login_jpg_data.is_some(), "login.jpg must exist in ETC.pck");
    let jpg_bytes = login_jpg_data.unwrap();
    let img = image::load_from_memory(&jpg_bytes).expect("Failed to decode login.jpg");
    let rgba = img.to_rgba8();

    // 2. Inicjalizacja kontekstu WGPU w trybie headless
    let ctx = WgpuEngineContext::new(0, 1024, 768).expect("Failed to create headless Wgpu context");
    let bg_tex = ctx.create_texture_from_rgba(&rgba, img.width(), img.height(), Some("Login Tex"))
        .expect("Failed to create GPU texture for login.jpg");

    // Rejestracja w kontekscie globalnym dla UiTextureManager
    set_global_wgpu_context(ctx);
    set_global_login_bg(bg_tex);

    // 3. Budowa okien UI
    m2_client::ui_bridge_ffi::m2_ui_setup_default_login_ui(1024, 768);

    // 4. Zbieranie partii renderowania (PRZED blokada mutexa!)
    let ui_batches = m2_client::ui_window_mgr::collect_ui_render_batches(1024, 768);
    println!("[+] Wygenerowano {} partii renderowania UI", ui_batches.len());

    let guard = m2_client::wgpu_engine_context::GLOBAL_WGPU_CONTEXT.get().unwrap().lock().unwrap();
    let ctx_ref = guard.as_ref().unwrap();

    let bg_quad = Quad2D::with_uv(
        0.0, 0.0, 1024.0, 768.0,
        0.0, 0.0, 1.0, 0.75, // 1024x768 subimage wewnatrz 1024x1024
    );

    let mut batches: Vec<(&GpuTexture, &[Quad2D])> = Vec::new();
    let bg_slice = [bg_quad];

    let bg_lock = m2_client::wgpu_engine_context::GLOBAL_LOGIN_BG.get().and_then(|m| m.lock().ok());
    if let Some(ref bg_opt) = bg_lock {
        if let Some(bg_tex) = bg_opt.as_ref() {
            batches.push((bg_tex, &bg_slice));
        }
    }

    for (tex, quads) in &ui_batches {
        batches.push((tex.as_ref(), quads.as_slice()));
    }

    let clear_color = wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    let rendered_img = ctx_ref.render_to_rgba_image(clear_color, &batches)
        .expect("Failed to render offscreen image");

    let save_path = "C:/Users/ADMIN/.gemini/antigravity/brain/56907d9d-0d1a-48a4-b2d8-374bdf3fa6e0/login_rendered_rust.png";
    rendered_img.save(save_path).expect("Failed to save rendered login screenshot");
    println!("[+] Zapisano zrzut ekranu wyrenderowanego UI do: {}", save_path);
}
