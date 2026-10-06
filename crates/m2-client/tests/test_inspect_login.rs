#[test]
fn test_inspect_login_scripts() {
    let pck_paths = [
        "E:/m2dev-client-src-main/m2dev-client-main/pack/uiscript.pck",
        "E:/m2dev-client-src-main/m2dev-client-main/pack/root.pck",
    ];

    for p in &pck_paths {
        if let Ok(arch) = m2_client::vfs_pck_memory_mapped::mount_archive(p) {
            println!("=== Otwarto archiwum: {} (plikow: {}) ===", p, arch.header.as_ref().map(|h| h.entry_num).unwrap_or(0));
            let files_to_find = [
                "uiscript/loginwindow.py",
                "UIScript/LoginWindow.py",
                "introLogin.py",
                "intrologin.py",
                "ui.py",
            ];
            if let Some(bytes) = m2_client::vfs_pck_memory_mapped::read_file_from_archive(&arch, "ui.py") {
                let text = String::from_utf8_lossy(&bytes);
                if let Some(idx) = text.find("def SetText(self, text, height = 4):") {
                    println!("=== def SetText W BUTTON ===");
                    let sub = &text[idx..];
                    let lines: Vec<&str> = sub.lines().take(25).collect();
                    println!("{}", lines.join("\n"));
                }
            }
        } else {
            println!("Nie udalo sie otworzyc {}", p);
        }
    }
}
