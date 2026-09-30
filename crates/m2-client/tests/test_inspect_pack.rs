use m2_client::ui_texture_mgr::UiTextureManager;
use m2_client::sub_image_parser::SubImage;

#[test]
fn test_trace_flag_sub() {
    let base_pack_dir = "E:/m2dev-client-src-main/m2dev-client-main/pack";
    for p in &["ETC.pck", "locale.pck", "uiscript.pck", "root.pck"] {
        let p_path = format!("{}/{}", base_pack_dir, p);
        if std::path::Path::new(&p_path).exists() {
            let _ = m2_client::py_bridge_pack::mount_pack_file(&p_path);
        }
    }

    let path = "d:/ymir work/ui/intro/login/server_flag_en.sub";
    let norm_path = path.to_lowercase().replace('\\', "/");

    let sub_bytes = UiTextureManager::read_vfs_bytes(&norm_path);
    println!("Step 1 (read flag bytes): is_some={}", sub_bytes.is_some());
    if let Some(bytes) = sub_bytes {
        let text = String::from_utf8_lossy(&bytes);
        println!("Flag Sub text:\n{}", text);
        let sub = SubImage::parse(&text).unwrap();
        println!("  parsed: image={:?}", sub.image);

        let clean_img = sub.image.trim().trim_matches('"').trim();
        let cand = format!("d:/ymir work/ui/{}", clean_img);
        let cand_bytes = UiTextureManager::read_vfs_bytes(&cand.to_lowercase().replace('\\', "/"));
        println!("  Candidate {}: bytes_found={}", cand, cand_bytes.is_some());
    }
}
