// ui_bridge_ffi.rs - C-ABI FFI Bridge for Window Manager and UI subsystems in Metin2 Pure Rust
use std::ffi::CStr;
use std::os::raw::c_char;
use m2_client_bridge::ffi_guard::safe_ffi_call;

use crate::ui_window_core::{WindowType, HORIZONTAL_ALIGN_CENTER, HORIZONTAL_ALIGN_LEFT, HORIZONTAL_ALIGN_RIGHT};
use crate::ui_window_mgr::get_window_manager;

fn cstr_to_str<'a>(ptr: *const c_char) -> &'a str {
    if ptr.is_null() {
        return "";
    }
    unsafe {
        CStr::from_ptr(ptr).to_str().unwrap_or("")
    }
}

#[no_mangle]
pub extern "C" fn m2_ui_register_window(_layer: *const c_char) -> u32 {
    safe_ffi_call(0, || {
        get_window_manager().register(WindowType::Normal)
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_register_image_box(_layer: *const c_char) -> u32 {
    safe_ffi_call(0, || {
        get_window_manager().register(WindowType::ImageBox)
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_register_exp_image_box(_layer: *const c_char) -> u32 {
    safe_ffi_call(0, || {
        get_window_manager().register(WindowType::ExpandedImageBox)
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_register_text_line(_layer: *const c_char) -> u32 {
    safe_ffi_call(0, || {
        get_window_manager().register(WindowType::TextLine)
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_register_button(_layer: *const c_char) -> u32 {
    safe_ffi_call(0, || {
        get_window_manager().register(WindowType::Button)
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_register_bar(_layer: *const c_char) -> u32 {
    safe_ffi_call(0, || {
        get_window_manager().register(WindowType::Bar)
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_register_line(_layer: *const c_char) -> u32 {
    safe_ffi_call(0, || {
        get_window_manager().register(WindowType::Line)
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_register_box(_layer: *const c_char) -> u32 {
    safe_ffi_call(0, || {
        get_window_manager().register(WindowType::Box)
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_set_parent(child: u32, parent: u32) -> bool {
    safe_ffi_call(false, || {
        get_window_manager().set_parent(child, parent)
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_set_position(handle: u32, x: i32, y: i32) -> bool {
    safe_ffi_call(false, || {
        get_window_manager().set_position(handle, x, y)
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_set_size(handle: u32, w: i32, h: i32) -> bool {
    safe_ffi_call(false, || {
        get_window_manager().set_size(handle, w, h)
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_set_show(handle: u32, is_show: bool) -> bool {
    safe_ffi_call(false, || {
        if is_show {
            get_window_manager().show(handle)
        } else {
            get_window_manager().hide(handle)
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_load_image(handle: u32, path: *const c_char) -> bool {
    safe_ffi_call(false, || {
        let p_str = cstr_to_str(path);
        get_window_manager().load_image(handle, p_str)
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_set_rendering_rect(handle: u32, l: f32, t: f32, r: f32, b: f32) -> bool {
    safe_ffi_call(false, || {
        get_window_manager().set_rendering_rect(handle, l, t, r, b)
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_set_scale(handle: u32, sx: f32, sy: f32) -> bool {
    safe_ffi_call(false, || {
        get_window_manager().set_scale(handle, sx, sy)
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_set_text(handle: u32, text: *const c_char) -> bool {
    safe_ffi_call(false, || {
        let t_str = cstr_to_str(text);
        get_window_manager().set_text(handle, t_str)
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_set_color(handle: u32, r: f32, g: f32, b: f32, a: f32) -> bool {
    safe_ffi_call(false, || {
        get_window_manager().set_color(handle, [r, g, b, a])
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_set_font_color(handle: u32, r: f32, g: f32, b: f32, a: f32) -> bool {
    safe_ffi_call(false, || {
        get_window_manager().set_font_color(handle, [r, g, b, a])
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_set_align(handle: u32, h_align: u8, v_align: u8) -> bool {
    safe_ffi_call(false, || {
        get_window_manager().set_align(handle, h_align, v_align)
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_set_button_visuals(
    handle: u32,
    up: *const c_char,
    over: *const c_char,
    down: *const c_char,
) -> bool {
    safe_ffi_call(false, || {
        let up_s = cstr_to_str(up);
        let over_s = cstr_to_str(over);
        let down_s = cstr_to_str(down);
        get_window_manager().set_button_visuals(handle, up_s, over_s, down_s)
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_destroy_window(handle: u32) {
    safe_ffi_call((), || {
        get_window_manager().destroy_window(handle);
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_get_width(handle: u32) -> i32 {
    safe_ffi_call(0, || {
        get_window_manager().windows.get(&handle).map(|w| w.width).unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_get_height(handle: u32) -> i32 {
    safe_ffi_call(0, || {
        get_window_manager().windows.get(&handle).map(|w| w.height).unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn m2_ui_is_show(handle: u32) -> bool {
    safe_ffi_call(false, || {
        get_window_manager().is_show(handle)
    })
}

/// Tworzy komponent ThinBoard w architekturze okien UI Metin2
pub fn create_thin_board(
    wm: &mut crate::ui_window_mgr::WindowManager,
    parent: Option<u32>,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    h_align: u8,
) -> u32 {
    let board_h = wm.register(WindowType::Normal);
    wm.set_position(board_h, x, y);
    wm.set_size(board_h, width, height);
    wm.set_align(board_h, h_align, 0);
    if let Some(p) = parent {
        wm.set_parent(board_h, p);
    }

    // 1. Tlo (Bar)
    let base_h = wm.register(WindowType::Bar);
    wm.set_parent(base_h, board_h);
    wm.set_position(base_h, 16, 16);
    wm.set_size(base_h, (width - 32).max(1), (height - 32).max(1));
    wm.set_color(base_h, [0.0, 0.0, 0.0, 0.51]);

    // 2. Narozniki (Corner LeftTop, LeftBottom, RightTop, RightBottom)
    let corners = [
        ("LeftTop", 0, 0),
        ("LeftBottom", 0, height - 16),
        ("RightTop", width - 16, 0),
        ("RightBottom", width - 16, height - 16),
    ];
    for (dir, cx, cy) in corners {
        let c_h = wm.register(WindowType::ExpandedImageBox);
        wm.set_parent(c_h, board_h);
        wm.set_position(c_h, cx, cy);
        wm.set_size(c_h, 16, 16);
        let path = format!("d:/ymir work/ui/pattern/ThinBoard_Corner_{}.tga", dir);
        wm.load_image(c_h, &path);
    }

    // 3. Linie (Left, Right, Top, Bottom)
    let lines = [
        ("Left", 0, 16, 16, (height - 32).max(1)),
        ("Right", width - 16, 16, 16, (height - 32).max(1)),
        ("Top", 16, 0, (width - 32).max(1), 16),
        ("Bottom", 16, height - 16, (width - 32).max(1), 16),
    ];
    for (dir, lx, ly, lw, lh) in lines {
        let l_h = wm.register(WindowType::ExpandedImageBox);
        wm.set_parent(l_h, board_h);
        wm.set_position(l_h, lx, ly);
        wm.set_size(l_h, lw, lh);
        let path = format!("d:/ymir work/ui/pattern/ThinBoard_Line_{}.tga", dir);
        wm.load_image(l_h, &path);
    }

    board_h
}

/// Natywna inicjalizacja kompletnego interfejsu ekranu wyboru serwera i logowania Metin2
/// Dokladnie odzwierciedla uklad z klienta C++ (LoginWindow.py oraz uilocaleselector.py):
/// 1. Gorna belka wyboru jezyka z flagami (ThinBoard 1024x35 + rzad flag państw)
/// 2. Centralne okno ThinBoard 375x400 ("Wybierz Serwer")
/// 3. Poziome linie podzialu, pionowa linia podzialu kolumn
/// 4. Kolumna lewa: lista serwerow (01. Metin2 z niebieskim podswietleniem, 02. Test)
/// 5. Kolumna prawa: lista kanalow (CH1 ..., CH2 ...)
/// 6. Przyciski na dole: OK (ServerSelectButton) oraz Wyjdz (ServerExitButton)
#[no_mangle]
pub extern "C" fn m2_ui_setup_default_login_ui(screen_w: u32, screen_h: u32) {
    safe_ffi_call((), || {
        let mut wm = get_window_manager();
        let sw = screen_w as i32;
        let sh = screen_h as i32;

        println!("[+] Budowanie kompletnego interfejsu logowania UI Pure Rust (C++ 1:1)...");

        // =========================================================================
        // 0. GORNE I DOLNE LINIE WZORU MEANDRA (Top & Bottom Line Pattern)
        // =========================================================================
        let top_line = wm.register(WindowType::ExpandedImageBox);
        wm.set_position(top_line, 0, 0);
        wm.set_size(top_line, sw, 20);
        wm.load_image(top_line, "d:/ymir work/ui/intro/pattern/line_pattern.tga");

        let bot_line = wm.register(WindowType::ExpandedImageBox);
        wm.set_position(bot_line, 0, sh - 20);
        wm.set_size(bot_line, sw, 20);
        wm.load_image(bot_line, "d:/ymir work/ui/intro/pattern/line_pattern.tga");

        // =========================================================================
        // 1. GORNA BELKA WYBORU JEZYKA (uilocaleselector.py)
        // =========================================================================
        let locale_bar = create_thin_board(&mut wm, None, 0, 20, sw, 35, 0);

        let flags = [
            ("ae", "Arabic"), ("br", "Portugues"), ("en", "English"),
            ("cz", "Cestina"), ("de", "Deutsch"), ("dk", "Dansk"),
            ("es", "Espanol"), ("fr", "Francais"), ("gr", "Greek"),
            ("hu", "Magyar"), ("it", "Italiano"), ("nl", "Nederlands"),
            ("pl", "Polski"), ("pt", "Portugues"), ("ro", "Romana"),
            ("ru", "Russian"), ("tr", "Turkce")
        ];

        let spacing = (sw - 40) / flags.len() as i32;
        for (i, (code, _name)) in flags.iter().enumerate() {
            let btn_h = wm.register(WindowType::Button);
            wm.set_parent(btn_h, locale_bar);
            wm.set_position(btn_h, 20 + (i as i32) * spacing, 9);
            wm.set_size(btn_h, 22, 17);
            wm.set_name(btn_h, &format!("Flag_{}", code));
            let sub_path = format!("d:/ymir work/ui/intro/login/server_flag_{}.sub", code);
            wm.load_image(btn_h, &sub_path);
        }

        // =========================================================================
        // 2. CENTRALNE OKNO WYBORU SERWERA (ServerBoard 375x400)
        // =========================================================================
        let board_w = 375;
        let board_h = 400;
        let board_y = sh - board_h - 72;
        let server_board = create_thin_board(&mut wm, None, 0, board_y, board_w, board_h, HORIZONTAL_ALIGN_CENTER);
        wm.set_name(server_board, "ServerBoard");

        // Naglowek: "Wybierz Serwer" (wycentrowany w kolorze srebrzysto-bialym)
        let title_h = wm.register(WindowType::TextLine);
        wm.set_parent(title_h, server_board);
        wm.set_position(title_h, 0, 12);
        wm.set_size(title_h, board_w, 20);
        wm.set_align(title_h, HORIZONTAL_ALIGN_CENTER, 0);
        wm.set_text(title_h, "Wybierz Serwer");
        wm.set_font_color(title_h, [0.88, 0.88, 0.88, 1.0]);

        // Poziome linie podzialu naglowka
        let hline1 = wm.register(WindowType::Line);
        wm.set_parent(hline1, server_board);
        wm.set_position(hline1, 10, 34);
        wm.set_size(hline1, 354, 1);
        wm.set_color(hline1, [0.47, 0.47, 0.47, 1.0]);

        let hline2 = wm.register(WindowType::Line);
        wm.set_parent(hline2, server_board);
        wm.set_position(hline2, 10, 35);
        wm.set_size(hline2, 355, 1);
        wm.set_color(hline2, [0.07, 0.07, 0.07, 1.0]);

        // Pionowa linia podzialu kolumn (lewa: serwery, prawa: kanaly)
        let vline1 = wm.register(WindowType::Line);
        wm.set_parent(vline1, server_board);
        wm.set_position(vline1, 246, 38);
        wm.set_size(vline1, 1, 310);
        wm.set_color(vline1, [0.47, 0.47, 0.47, 1.0]);

        let vline2 = wm.register(WindowType::Line);
        wm.set_parent(vline2, server_board);
        wm.set_position(vline2, 247, 38);
        wm.set_size(vline2, 1, 310);
        wm.set_color(vline2, [0.07, 0.07, 0.07, 1.0]);

        // =========================================================================
        // 3. KOLUMNA LEWA: LISTA SERWEROW
        // =========================================================================
        // Zaznaczenie drugiego serwera (02. Test) jak w kliencie wzorcowym C++
        let select_bar = wm.register(WindowType::Bar);
        wm.set_parent(select_bar, server_board);
        wm.set_position(select_bar, 14, 65);
        wm.set_size(select_bar, 226, 22);
        wm.set_color(select_bar, [0.12, 0.22, 0.45, 0.85]);
        wm.set_name(select_bar, "ServerSelectBar");

        let srv1_text = wm.register(WindowType::TextLine);
        wm.set_parent(srv1_text, server_board);
        wm.set_position(srv1_text, 20, 45);
        wm.set_size(srv1_text, 220, 16);
        wm.set_text(srv1_text, "01. Metin2");
        wm.set_font_color(srv1_text, [0.75, 0.75, 0.75, 1.0]);
        wm.set_name(srv1_text, "Server_0");

        let srv2_text = wm.register(WindowType::TextLine);
        wm.set_parent(srv2_text, server_board);
        wm.set_position(srv2_text, 20, 68);
        wm.set_size(srv2_text, 220, 16);
        wm.set_text(srv2_text, "02. Test");
        wm.set_font_color(srv2_text, [1.0, 1.0, 1.0, 1.0]);
        wm.set_name(srv2_text, "Server_1");

        // =========================================================================
        // 4. KOLUMNA PRAWA: LISTA KANALOW
        // =========================================================================
        // Zaznaczenie kanalu CH1 ...
        let ch_bar = wm.register(WindowType::Bar);
        wm.set_parent(ch_bar, server_board);
        wm.set_position(ch_bar, 252, 42);
        wm.set_size(ch_bar, 108, 22);
        wm.set_color(ch_bar, [0.12, 0.22, 0.45, 0.85]);
        wm.set_name(ch_bar, "ChannelSelectBar");

        let ch1_text = wm.register(WindowType::TextLine);
        wm.set_parent(ch1_text, server_board);
        wm.set_position(ch1_text, 258, 45);
        wm.set_size(ch1_text, 100, 16);
        wm.set_text(ch1_text, "CH1   ...");
        wm.set_font_color(ch1_text, [1.0, 1.0, 1.0, 1.0]);
        wm.set_name(ch1_text, "Channel_0");

        let ch2_text = wm.register(WindowType::TextLine);
        wm.set_parent(ch2_text, server_board);
        wm.set_position(ch2_text, 258, 68);
        wm.set_size(ch2_text, 100, 16);
        wm.set_text(ch2_text, "CH2   ...");
        wm.set_font_color(ch2_text, [0.75, 0.75, 0.75, 1.0]);
        wm.set_name(ch2_text, "Channel_1");

        // =========================================================================
        // 5. PRZYCISKI DOLNE: OK ORAZ WYJDZ (ServerSelectButton, ServerExitButton)
        // =========================================================================
        let btn_ok = wm.register(WindowType::Button);
        wm.set_parent(btn_ok, server_board);
        wm.set_position(btn_ok, 267, 350);
        wm.set_size(btn_ok, 88, 21);
        wm.set_name(btn_ok, "ServerSelectButton");
        wm.set_button_visuals(
            btn_ok,
            "d:/ymir work/ui/public/large_button_01.sub",
            "d:/ymir work/ui/public/large_button_02.sub",
            "d:/ymir work/ui/public/large_button_03.sub",
        );
        wm.set_text(btn_ok, "OK");

        let btn_exit = wm.register(WindowType::Button);
        wm.set_parent(btn_exit, server_board);
        wm.set_position(btn_exit, 267, 373);
        wm.set_size(btn_exit, 88, 21);
        wm.set_name(btn_exit, "ServerExitButton");
        wm.set_button_visuals(
            btn_exit,
            "d:/ymir work/ui/public/large_button_01.sub",
            "d:/ymir work/ui/public/large_button_02.sub",
            "d:/ymir work/ui/public/large_button_03.sub",
        );
        wm.set_text(btn_exit, "Wyjdz");

        // =========================================================================
        // 6. FORMULARZ LOGOWANIA (LoginBoard - uaktywniany po kliknieciu OK)
        // =========================================================================
        let login_board = create_thin_board(&mut wm, None, (sw - 208) / 2, sh - 410, 208, 108, 0);
        wm.set_name(login_board, "LoginBoard");
        wm.hide(login_board);

        let id_lbl = wm.register(WindowType::TextLine);
        wm.set_parent(id_lbl, login_board);
        wm.set_position(id_lbl, 15, 15);
        wm.set_text(id_lbl, "ID:");
        wm.set_font_color(id_lbl, [0.88, 0.88, 0.88, 1.0]);

        let id_box = wm.register(WindowType::Bar);
        wm.set_parent(id_box, login_board);
        wm.set_position(id_box, 48, 14);
        wm.set_size(id_box, 140, 18);
        wm.set_color(id_box, [0.1, 0.1, 0.1, 0.9]);
        wm.set_name(id_box, "IdBox");

        let id_text = wm.register(WindowType::TextLine);
        wm.set_parent(id_text, id_box);
        wm.set_position(id_text, 4, 1);
        wm.set_size(id_text, 132, 16);
        wm.set_text(id_text, "admin");
        wm.set_name(id_text, "IdEditLine");
        *REAL_ID.lock().unwrap() = "admin".to_string();

        let pw_lbl = wm.register(WindowType::TextLine);
        wm.set_parent(pw_lbl, login_board);
        wm.set_position(pw_lbl, 15, 40);
        wm.set_text(pw_lbl, "Haslo:");
        wm.set_font_color(pw_lbl, [0.88, 0.88, 0.88, 1.0]);

        let pw_box = wm.register(WindowType::Bar);
        wm.set_parent(pw_box, login_board);
        wm.set_position(pw_box, 48, 39);
        wm.set_size(pw_box, 140, 18);
        wm.set_color(pw_box, [0.1, 0.1, 0.1, 0.9]);
        wm.set_name(pw_box, "PwdBox");

        let pw_text = wm.register(WindowType::TextLine);
        wm.set_parent(pw_text, pw_box);
        wm.set_position(pw_text, 4, 1);
        wm.set_size(pw_text, 132, 16);
        wm.set_text(pw_text, "*****");
        wm.set_name(pw_text, "PwdEditLine");
        *REAL_PASSWORD.lock().unwrap() = "admin".to_string();

        let btn_login = wm.register(WindowType::Button);
        wm.set_parent(btn_login, login_board);
        wm.set_position(btn_login, 12, 68);
        wm.set_size(btn_login, 88, 21);
        wm.set_name(btn_login, "LoginButton");
        wm.set_button_visuals(
            btn_login,
            "d:/ymir work/ui/public/large_button_01.sub",
            "d:/ymir work/ui/public/large_button_02.sub",
            "d:/ymir work/ui/public/large_button_03.sub",
        );
        wm.set_text(btn_login, "Polacz");

        let btn_cancel = wm.register(WindowType::Button);
        wm.set_parent(btn_cancel, login_board);
        wm.set_position(btn_cancel, 108, 68);
        wm.set_size(btn_cancel, 88, 21);
        wm.set_name(btn_cancel, "LoginExitButton");
        wm.set_button_visuals(
            btn_cancel,
            "d:/ymir work/ui/public/large_button_01.sub",
            "d:/ymir work/ui/public/large_button_02.sub",
            "d:/ymir work/ui/public/large_button_03.sub",
        );
        wm.set_text(btn_cancel, "Anuluj");

        println!("[+] Interfejs wyboru serwera i logowania UI Pure Rust gotowy (zarejestrowano {} okien)!", wm.windows.len());
    });
}

pub static ACTIVE_EDIT: std::sync::Mutex<&'static str> = std::sync::Mutex::new("IdEditLine");
pub static REAL_ID: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
pub static REAL_PASSWORD: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
pub static PWD_FIRST_INPUT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub static ID_FIRST_INPUT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[derive(Debug, Clone, Default)]
pub struct CharacterSlot {
    pub id: u32,
    pub name: String,
    pub job: u8,
    pub level: u8,
    pub playtime: u32,
    pub st: u8,
    pub ht: u8,
    pub dx: u8,
    pub iq: u8,
    pub main_part: u16,
    pub hair_part: u16,
}

pub static GLOBAL_CHARACTERS: std::sync::Mutex<Vec<CharacterSlot>> = std::sync::Mutex::new(Vec::new());
pub static SELECTED_CHAR_SLOT: std::sync::Mutex<usize> = std::sync::Mutex::new(0);
pub static CURRENT_PHASE: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
pub static ACTIVE_GAME_SOCKET: std::sync::Mutex<Option<(std::net::TcpStream, crate::secure_cipher::SecureCipher)>> = std::sync::Mutex::new(None);

/// Tworzy kompletny interfejs wyboru postaci (PHASE_SELECT / introselect.py 1:1)
pub fn m2_ui_setup_character_select(
    wm: &mut crate::ui_window_mgr::WindowManager,
    screen_w: u32,
    screen_h: u32,
    chars: &[CharacterSlot],
) {
    let sw = screen_w as i32;
    let sh = screen_h as i32;

    // 1. Ukryj poprzednie okna logowania
    for win in wm.windows.values_mut() {
        if win.name == "LoginBoard" || win.name == "ServerBoard" {
            win.is_show = false;
        }
    }

    // 2. Glowny kontener wyboru postaci
    let select_board = wm.register(WindowType::Normal);
    wm.set_name(select_board, "SelectBoard");
    wm.set_position(select_board, 0, 0);
    wm.set_size(select_board, sw, sh);

    // Gorna belka tytulowa
    let title_bar = create_thin_board(wm, Some(select_board), (sw - 460) / 2, 35, 460, 45, 0);
    let title_txt = wm.register(WindowType::TextLine);
    wm.set_parent(title_txt, title_bar);
    wm.set_position(title_txt, 0, 13);
    wm.set_size(title_txt, 460, 20);
    wm.set_align(title_txt, HORIZONTAL_ALIGN_CENTER, 0);
    wm.set_text(title_txt, "METIN2 - WYBOR POSTACI (CH1)");
    wm.set_font_color(title_txt, [1.0, 0.85, 0.25, 1.0]);

    let active_char = chars.first().cloned().unwrap_or_default();
    let job_name = match active_char.job {
        0 => "Wojownik (Cialo/Umysl)",
        1 => "Ninja (Zabojca/Lucznik)",
        2 => "Sura (Wp/Bm)",
        3 => "Szaman (Smok/Leczenie)",
        _ => "Wojownik",
    };

    // =========================================================================
    // LEWY PANEL - ORYGINALNA KARTA POSTACI METIN2 (character_board)
    // =========================================================================
    let card_w = 340;
    let card_h = 560;
    let card_x = 60;
    let card_y = 110;
    let info_board = create_thin_board(wm, Some(select_board), card_x, card_y, card_w, card_h, 0);
    wm.set_name(info_board, "CharInfoBoard");

    // Krolestwo i Gildia
    let empire_txt = wm.register(WindowType::TextLine);
    wm.set_parent(empire_txt, info_board);
    wm.set_position(empire_txt, 20, 18);
    wm.set_text(empire_txt, "Krolestwo: Jinno (Niebiescy)");
    wm.set_font_color(empire_txt, [0.35, 0.75, 1.0, 1.0]);

    let guild_txt = wm.register(WindowType::TextLine);
    wm.set_parent(guild_txt, info_board);
    wm.set_position(guild_txt, 20, 40);
    wm.set_text(guild_txt, "Gildia: Brak gildii");
    wm.set_font_color(guild_txt, [0.75, 0.75, 0.75, 1.0]);

    let div_line = wm.register(WindowType::Line);
    wm.set_parent(div_line, info_board);
    wm.set_position(div_line, 16, 65);
    wm.set_size(div_line, card_w - 32, 1);
    wm.set_color(div_line, [0.47, 0.47, 0.47, 1.0]);

    // Nazwa, Poziom, Klasa postaci
    let name_txt = wm.register(WindowType::TextLine);
    wm.set_parent(name_txt, info_board);
    wm.set_position(name_txt, 20, 78);
    wm.set_text(name_txt, &format!("Nazwa: {}", active_char.name));
    wm.set_font_color(name_txt, [1.0, 0.85, 0.2, 1.0]);
    wm.set_name(name_txt, "CharInfoName");

    let lvl_txt = wm.register(WindowType::TextLine);
    wm.set_parent(lvl_txt, info_board);
    wm.set_position(lvl_txt, 20, 105);
    wm.set_text(lvl_txt, &format!("Poziom: Lv. {}", active_char.level));
    wm.set_font_color(lvl_txt, [1.0, 1.0, 1.0, 1.0]);
    wm.set_name(lvl_txt, "CharInfoLevel");

    let class_txt = wm.register(WindowType::TextLine);
    wm.set_parent(class_txt, info_board);
    wm.set_position(class_txt, 20, 132);
    wm.set_text(class_txt, &format!("Klasa: {}", job_name));
    wm.set_font_color(class_txt, [0.85, 0.9, 1.0, 1.0]);
    wm.set_name(class_txt, "CharInfoClass");

    let time_txt = wm.register(WindowType::TextLine);
    wm.set_parent(time_txt, info_board);
    wm.set_position(time_txt, 20, 159);
    wm.set_text(time_txt, &format!("Czas w grze: {} min", active_char.playtime));
    wm.set_font_color(time_txt, [0.75, 0.75, 0.75, 1.0]);
    wm.set_name(time_txt, "CharInfoTime");

    let div_line2 = wm.register(WindowType::Line);
    wm.set_parent(div_line2, info_board);
    wm.set_position(div_line2, 16, 185);
    wm.set_size(div_line2, card_w - 32, 1);
    wm.set_color(div_line2, [0.47, 0.47, 0.47, 1.0]);

    // 4 PASKI STATYSTYK METIN2 (VIT, INT, STR, DEX)
    let stat_defs = [
        ("Witalnosc (VIT/CON)", active_char.ht, [0.85, 0.20, 0.20, 1.0], "CharStatVit", "CharBarVit"),
        ("Inteligencja (INT)", active_char.iq, [0.85, 0.35, 0.85, 1.0], "CharStatInt", "CharBarInt"),
        ("Sila (STR)", active_char.st, [0.65, 0.25, 0.85, 1.0], "CharStatStr", "CharBarStr"),
        ("Zrecznosc (DEX)", active_char.dx, [0.20, 0.55, 0.90, 1.0], "CharStatDex", "CharBarDex"),
    ];

    for (i, (label, val, color, txt_name, bar_name)) in stat_defs.iter().enumerate() {
        let sy = 198 + (i as i32 * 44);
        let s_txt = wm.register(WindowType::TextLine);
        wm.set_parent(s_txt, info_board);
        wm.set_position(s_txt, 20, sy);
        wm.set_text(s_txt, &format!("{}: {}", label, val));
        wm.set_font_color(s_txt, [0.92, 0.92, 0.92, 1.0]);
        wm.set_name(s_txt, txt_name);

        let bg_bar = wm.register(WindowType::Bar);
        wm.set_parent(bg_bar, info_board);
        wm.set_position(bg_bar, 20, sy + 20);
        wm.set_size(bg_bar, card_w - 40, 8);
        wm.set_color(bg_bar, [0.15, 0.15, 0.15, 0.85]);

        let fill_w = ((card_w - 40) as f32 * ((*val as f32) / 100.0).clamp(0.05, 1.0)) as i32;
        let fg_bar = wm.register(WindowType::Bar);
        wm.set_parent(fg_bar, bg_bar);
        wm.set_position(fg_bar, 0, 0);
        wm.set_size(fg_bar, fill_w, 8);
        wm.set_color(fg_bar, *color);
        wm.set_name(fg_bar, bar_name);
    }

    // PRZYCISKI AKCJI NA KARCIE POSTACI
    let btn_start = wm.register(WindowType::Button);
    wm.set_parent(btn_start, info_board);
    wm.set_position(btn_start, 20, 390);
    wm.set_size(btn_start, card_w - 40, 42);
    wm.set_name(btn_start, "SelectStartButton");
    wm.set_button_visuals(
        btn_start,
        "d:/ymir work/ui/public/large_button_01.sub",
        "d:/ymir work/ui/public/large_button_02.sub",
        "d:/ymir work/ui/public/large_button_03.sub",
    );
    wm.set_text(btn_start, "START / WEJDZ DO GRY");

    let btn_create = wm.register(WindowType::Button);
    wm.set_parent(btn_create, info_board);
    wm.set_position(btn_create, 20, 442);
    wm.set_size(btn_create, (card_w - 50) / 2, 30);
    wm.set_name(btn_create, "SelectCreateButton");
    wm.set_button_visuals(
        btn_create,
        "d:/ymir work/ui/public/large_button_01.sub",
        "d:/ymir work/ui/public/large_button_02.sub",
        "d:/ymir work/ui/public/large_button_03.sub",
    );
    wm.set_text(btn_create, "Nowa Postac");

    let btn_del = wm.register(WindowType::Button);
    wm.set_parent(btn_del, info_board);
    wm.set_position(btn_del, card_w / 2 + 5, 442);
    wm.set_size(btn_del, (card_w - 50) / 2, 30);
    wm.set_name(btn_del, "SelectDeleteButton");
    wm.set_button_visuals(
        btn_del,
        "d:/ymir work/ui/public/large_button_01.sub",
        "d:/ymir work/ui/public/large_button_02.sub",
        "d:/ymir work/ui/public/large_button_03.sub",
    );
    wm.set_text(btn_del, "Usun Postac");

    let btn_back = wm.register(WindowType::Button);
    wm.set_parent(btn_back, info_board);
    wm.set_position(btn_back, 20, 484);
    wm.set_size(btn_back, card_w - 40, 30);
    wm.set_name(btn_back, "SelectExitButton");
    wm.set_button_visuals(
        btn_back,
        "d:/ymir work/ui/public/large_button_01.sub",
        "d:/ymir work/ui/public/large_button_02.sub",
        "d:/ymir work/ui/public/large_button_03.sub",
    );
    wm.set_text(btn_back, "Wroc do Logowania");

    // =========================================================================
    // PRAWY PANEL - WYBOR SLOTU POSTACI (SLOTY 1 - 4)
    // =========================================================================
    let slot_w = 400;
    let slot_h = 360;
    let slot_x = sw - slot_w - 60;
    let slot_y = 110;
    let slot_board = create_thin_board(wm, Some(select_board), slot_x, slot_y, slot_w, slot_h, 0);
    wm.set_name(slot_board, "CharSlotBoard");

    let slot_title = wm.register(WindowType::TextLine);
    wm.set_parent(slot_title, slot_board);
    wm.set_position(slot_title, 20, 18);
    wm.set_text(slot_title, "WYBIERZ POSTAC (SLOTY 1 - 4)");
    wm.set_font_color(slot_title, [0.85, 0.85, 0.85, 1.0]);

    let slot_div = wm.register(WindowType::Line);
    wm.set_parent(slot_div, slot_board);
    wm.set_position(slot_div, 16, 42);
    wm.set_size(slot_div, slot_w - 32, 1);
    wm.set_color(slot_div, [0.47, 0.47, 0.47, 1.0]);

    // 4 Sloty
    for i in 0..4 {
        let sy = 55 + (i as i32 * 54);
        let is_selected = i == 0;
        let is_valid = i < chars.len();

        let s_bar = wm.register(WindowType::Bar);
        wm.set_parent(s_bar, slot_board);
        wm.set_position(s_bar, 20, sy);
        wm.set_size(s_bar, slot_w - 40, 46);
        if is_selected {
            wm.set_color(s_bar, [0.15, 0.45, 0.75, 0.95]);
        } else {
            wm.set_color(s_bar, [0.08, 0.08, 0.08, 0.75]);
        }
        wm.set_name(s_bar, &format!("CharSlotBar_{}", i));

        let s_btn = wm.register(WindowType::Button);
        wm.set_parent(s_btn, s_bar);
        wm.set_position(s_btn, 0, 0);
        wm.set_size(s_btn, slot_w - 40, 46);
        wm.set_name(s_btn, &format!("CharSlot_{}", i));

        let s_txt = wm.register(WindowType::TextLine);
        wm.set_parent(s_txt, s_btn);
        wm.set_position(s_txt, 16, 14);
        wm.set_name(s_txt, &format!("CharSlotTxt_{}", i));
        if is_valid {
            let ch = &chars[i];
            let j_name = match ch.job {
                0 => "Wojownik",
                1 => "Ninja",
                2 => "Sura",
                3 => "Szaman",
                _ => "Postac",
            };
            wm.set_text(s_txt, &format!("{}. {} (Lv. {}, {})", i + 1, ch.name, ch.level, j_name));
            wm.set_font_color(s_txt, [1.0, 1.0, 1.0, 1.0]);
        } else {
            wm.set_text(s_txt, &format!("{}. <Pusty Slot>", i + 1));
            wm.set_font_color(s_txt, [0.5, 0.5, 0.5, 1.0]);
        }
    }

    // Strzalki nawigacyjne < i > do rotacji slotow
    let btn_prev = wm.register(WindowType::Button);
    wm.set_parent(btn_prev, slot_board);
    wm.set_position(btn_prev, 20, 290);
    wm.set_size(btn_prev, (slot_w - 50) / 2, 34);
    wm.set_name(btn_prev, "SelectPrevButton");
    wm.set_button_visuals(
        btn_prev,
        "d:/ymir work/ui/public/large_button_01.sub",
        "d:/ymir work/ui/public/large_button_02.sub",
        "d:/ymir work/ui/public/large_button_03.sub",
    );
    wm.set_text(btn_prev, "< Poprzedni Slot");

    let btn_next = wm.register(WindowType::Button);
    wm.set_parent(btn_next, slot_board);
    wm.set_position(btn_next, slot_w / 2 + 5, 290);
    wm.set_size(btn_next, (slot_w - 50) / 2, 34);
    wm.set_name(btn_next, "SelectNextButton");
    wm.set_button_visuals(
        btn_next,
        "d:/ymir work/ui/public/large_button_01.sub",
        "d:/ymir work/ui/public/large_button_02.sub",
        "d:/ymir work/ui/public/large_button_03.sub",
    );
    wm.set_text(btn_next, "Nastepny Slot >");

    println!("[+] Ekran wyboru postaci UI Pure Rust gotowy (zarejestrowano {} okien)!", wm.windows.len());
}

pub fn update_character_select_ui(wm: &mut crate::ui_window_mgr::WindowManager, chars: &[CharacterSlot], selected_idx: usize) {
    if selected_idx >= chars.len() {
        return;
    }
    let ch = &chars[selected_idx];
    let job_name = match ch.job {
        0 => "Wojownik (Cialo/Umysl)",
        1 => "Ninja (Zabojca/Lucznik)",
        2 => "Sura (Wp/Bm)",
        3 => "Szaman (Smok/Leczenie)",
        _ => "Wojownik",
    };

    for win in wm.windows.values_mut() {
        if win.name == "CharInfoName" {
            win.text = format!("Nazwa: {}", ch.name);
        } else if win.name == "CharInfoLevel" {
            win.text = format!("Poziom: Lv. {}", ch.level);
        } else if win.name == "CharInfoClass" {
            win.text = format!("Klasa: {}", job_name);
        } else if win.name == "CharInfoTime" {
            win.text = format!("Czas w grze: {} min", ch.playtime);
        } else if win.name == "CharStatVit" {
            win.text = format!("Witalnosc (VIT/CON): {}", ch.ht);
        } else if win.name == "CharStatInt" {
            win.text = format!("Inteligencja (INT): {}", ch.iq);
        } else if win.name == "CharStatStr" {
            win.text = format!("Sila (STR): {}", ch.st);
        } else if win.name == "CharStatDex" {
            win.text = format!("Zrecznosc (DEX): {}", ch.dx);
        }
    }

    for i in 0..4 {
        let bar_name = format!("CharSlotBar_{}", i);
        if let Some(h) = wm.windows.values().find(|w| w.name == bar_name).map(|w| w.handle) {
            if i == selected_idx {
                wm.set_color(h, [0.15, 0.45, 0.75, 0.95]);
            } else {
                wm.set_color(h, [0.08, 0.08, 0.08, 0.75]);
            }
        }
    }
}

/// Tworzy kompletny interfejs wewnatrz swiata gry (Game HUD)
pub fn m2_ui_setup_game_hud(
    wm: &mut crate::ui_window_mgr::WindowManager,
    screen_w: u32,
    screen_h: u32,
) {
    let sw = screen_w as i32;
    let sh = screen_h as i32;

    // 1. Ukryj poprzednie okna wyboru postaci
    for win in wm.windows.values_mut() {
        if win.name.starts_with("Select") || win.name.starts_with("Char") || win.name == "LoginBoard" || win.name == "ServerBoard" {
            win.is_show = false;
        }
    }

    // 2. Glowny HUD gry
    let hud_board = wm.register(WindowType::Normal);
    wm.set_name(hud_board, "GameHudBoard");
    wm.set_position(hud_board, 0, 0);
    wm.set_size(hud_board, sw, sh);

    // =========================================================================
    // DOLNY PASEK GRY (Taskbar)
    // =========================================================================
    let bar_h = 42;
    let bar_y = sh - bar_h;
    let taskbar = wm.register(WindowType::Bar);
    wm.set_parent(taskbar, hud_board);
    wm.set_position(taskbar, 0, bar_y);
    wm.set_size(taskbar, sw, bar_h);
    wm.set_color(taskbar, [0.05, 0.05, 0.07, 0.92]);
    wm.set_name(taskbar, "HudTaskbar");

    let task_line = wm.register(WindowType::Line);
    wm.set_parent(task_line, taskbar);
    wm.set_position(task_line, 0, 0);
    wm.set_size(task_line, sw, 1);
    wm.set_color(task_line, [0.75, 0.65, 0.25, 1.0]);

    // Pasek PZ
    let hp_bg = wm.register(WindowType::Bar);
    wm.set_parent(hp_bg, taskbar);
    wm.set_position(hp_bg, 12, 6);
    wm.set_size(hp_bg, 180, 14);
    wm.set_color(hp_bg, [0.25, 0.05, 0.05, 0.95]);

    let hp_fg = wm.register(WindowType::Bar);
    wm.set_parent(hp_fg, hp_bg);
    wm.set_position(hp_fg, 0, 0);
    wm.set_size(hp_fg, 180, 14);
    wm.set_color(hp_fg, [0.85, 0.12, 0.12, 1.0]);
    wm.set_name(hp_fg, "HudHpBarFg");

    let hp_lbl = wm.register(WindowType::TextLine);
    wm.set_parent(hp_lbl, hp_bg);
    wm.set_position(hp_lbl, 10, 0);
    wm.set_text(hp_lbl, "PZ: ... / ...");
    wm.set_font_color(hp_lbl, [1.0, 1.0, 1.0, 1.0]);
    wm.set_name(hp_lbl, "HudHpText");

    // Pasek PE
    let sp_bg = wm.register(WindowType::Bar);
    wm.set_parent(sp_bg, taskbar);
    wm.set_position(sp_bg, 12, 23);
    wm.set_size(sp_bg, 180, 14);
    wm.set_color(sp_bg, [0.05, 0.12, 0.28, 0.95]);

    let sp_fg = wm.register(WindowType::Bar);
    wm.set_parent(sp_fg, sp_bg);
    wm.set_position(sp_fg, 0, 0);
    wm.set_size(sp_fg, 180, 14);
    wm.set_color(sp_fg, [0.15, 0.45, 0.95, 1.0]);
    wm.set_name(sp_fg, "HudSpBarFg");

    let sp_lbl = wm.register(WindowType::TextLine);
    wm.set_parent(sp_lbl, sp_bg);
    wm.set_position(sp_lbl, 10, 0);
    wm.set_text(sp_lbl, "PE: ... / ...");
    wm.set_font_color(sp_lbl, [1.0, 1.0, 1.0, 1.0]);
    wm.set_name(sp_lbl, "HudSpText");

    // Pasek Staminy i EXP
    let st_fg = wm.register(WindowType::Bar);
    wm.set_parent(st_fg, taskbar);
    wm.set_position(st_fg, 202, 6);
    wm.set_size(st_fg, 70, 14);
    wm.set_color(st_fg, [0.88, 0.78, 0.15, 1.0]);

    let st_lbl = wm.register(WindowType::TextLine);
    wm.set_parent(st_lbl, st_fg);
    wm.set_position(st_lbl, 6, 0);
    wm.set_text(st_lbl, "ST: 100%");
    wm.set_font_color(st_lbl, [0.1, 0.1, 0.1, 1.0]);

    let exp_fg = wm.register(WindowType::Bar);
    wm.set_parent(exp_fg, taskbar);
    wm.set_position(exp_fg, 202, 23);
    wm.set_size(exp_fg, 70, 14);
    wm.set_color(exp_fg, [0.25, 0.75, 0.25, 1.0]);

    let exp_lbl = wm.register(WindowType::TextLine);
    wm.set_parent(exp_lbl, exp_fg);
    wm.set_position(exp_lbl, 6, 0);
    wm.set_text(exp_lbl, "EXP: 85%");
    wm.set_font_color(exp_lbl, [0.1, 0.1, 0.1, 1.0]);

    // Sloty szybkiego wyboru (Quickslots 1..4)
    let qs_names = ["1: Wir", "2: Szarza", "3: Ciecie", "4: Berserk"];
    for (i, name) in qs_names.iter().enumerate() {
        let q_btn = wm.register(WindowType::Button);
        wm.set_parent(q_btn, taskbar);
        wm.set_position(q_btn, 285 + (i as i32 * 54), 6);
        wm.set_size(q_btn, 50, 31);
        wm.set_name(q_btn, &format!("QuickSlot_{}", i + 1));
        wm.set_button_visuals(
            q_btn,
            "d:/ymir work/ui/public/large_button_01.sub",
            "d:/ymir work/ui/public/large_button_02.sub",
            "d:/ymir work/ui/public/large_button_03.sub",
        );
        wm.set_text(q_btn, name);
    }

    // Przyciski menu glownego (C, I, V, ESC, Wyloguj)
    let menu_btns = [
        ("HudBtnChar", "Postac [C]", 515, 78),
        ("HudBtnInv", "Ekwipunek [I]", 600, 95),
        ("HudBtnSkills", "Skille [V]", 702, 75),
        ("HudBtnSystem", "Opcje [ESC]", 784, 85),
        ("HudBtnExit", "Wyloguj", 876, 75),
    ];
    for (id_name, label, mx, mw) in menu_btns {
        let m_btn = wm.register(WindowType::Button);
        wm.set_parent(m_btn, taskbar);
        wm.set_position(m_btn, mx, 7);
        wm.set_size(m_btn, mw, 28);
        wm.set_name(m_btn, id_name);
        wm.set_button_visuals(
            m_btn,
            "d:/ymir work/ui/public/large_button_01.sub",
            "d:/ymir work/ui/public/large_button_02.sub",
            "d:/ymir work/ui/public/large_button_03.sub",
        );
        wm.set_text(m_btn, label);
    }

    // =========================================================================
    // PRAWY GORNY ROG: MINIMAPA
    // =========================================================================
    let mm_w = 175;
    let mm_h = 175;
    let mm_board = create_thin_board(wm, Some(hud_board), sw - mm_w - 15, 15, mm_w, mm_h, 0);
    wm.set_name(mm_board, "MiniMapBoard");

    let mm_title = wm.register(WindowType::TextLine);
    wm.set_parent(mm_title, mm_board);
    wm.set_position(mm_title, 12, 14);
    wm.set_text(mm_title, "CH1 - Miasto 1 (M1)");
    wm.set_font_color(mm_title, [1.0, 0.85, 0.25, 1.0]);

    let mm_coord = wm.register(WindowType::TextLine);
    wm.set_parent(mm_coord, mm_board);
    wm.set_position(mm_coord, 12, 34);
    wm.set_text(mm_coord, "Pozycja: ...");
    wm.set_font_color(mm_coord, [0.85, 0.85, 0.85, 1.0]);
    wm.set_name(mm_coord, "HudLevelText");

    let mm_radar = wm.register(WindowType::Bar);
    wm.set_parent(mm_radar, mm_board);
    wm.set_position(mm_radar, 15, 55);
    wm.set_size(mm_radar, 145, 105);
    wm.set_color(mm_radar, [0.08, 0.18, 0.12, 0.88]);

    let mm_dot = wm.register(WindowType::Bar);
    wm.set_parent(mm_dot, mm_radar);
    wm.set_position(mm_dot, 70, 50);
    wm.set_size(mm_dot, 5, 5);
    wm.set_color(mm_dot, [1.0, 1.0, 1.0, 1.0]);
    wm.set_name(mm_dot, "MiniMapDot");

    // =========================================================================
    // LEWY DOLNY ROG: OKNO CZATU
    // =========================================================================
    let chat_w = 460;
    let chat_h = 160;
    let chat_y = bar_y - chat_h - 10;
    let chat_board = create_thin_board(wm, Some(hud_board), 15, chat_y, chat_w, chat_h, 0);
    wm.set_name(chat_board, "ChatBoard");

    let chat_lines = [
        ("[System] Witamy w swiecie gry Metin2 (Pure Rust Engine)!", [1.0, 0.85, 0.2, 1.0]),
        ("[System] Polaczono pomyslnie z serwerem gry CH1 (127.0.0.1:11011).", [0.4, 0.8, 1.0, 1.0]),
        ("[Postac] Zalogowano: [SA]Admin (Poziom: 105, Wojownik Body).", [0.4, 1.0, 0.4, 1.0]),
        ("[Swiat] Swiat gry jest aktywny! Renderowanie wgpu DX12 60 FPS.", [0.85, 0.85, 0.85, 1.0]),
        ("[Sterowanie] Uzyj myszki lub klawiszy WASD do poruszania sie.", [0.75, 0.75, 0.75, 1.0]),
    ];
    for (i, (msg, col)) in chat_lines.iter().enumerate() {
        let c_txt = wm.register(WindowType::TextLine);
        wm.set_parent(c_txt, chat_board);
        wm.set_position(c_txt, 14, 14 + (i as i32 * 22));
        wm.set_text(c_txt, msg);
        wm.set_font_color(c_txt, *col);
    }

    let chat_input_bar = wm.register(WindowType::Bar);
    wm.set_parent(chat_input_bar, chat_board);
    wm.set_position(chat_input_bar, 14, chat_h - 28);
    wm.set_size(chat_input_bar, chat_w - 28, 18);
    wm.set_color(chat_input_bar, [0.05, 0.05, 0.05, 0.85]);

    let chat_input_txt = wm.register(WindowType::TextLine);
    wm.set_parent(chat_input_txt, chat_input_bar);
    wm.set_position(chat_input_txt, 6, 1);
    wm.set_text(chat_input_txt, "Nacisnij ENTER aby napisac wiadomosc na czacie...");
    wm.set_font_color(chat_input_txt, [0.55, 0.55, 0.55, 1.0]);

    // =========================================================================
    // OKNO EKWIPUNKU (Inventory - I)
    // =========================================================================
    let inv_w = 180;
    let inv_h = 390;
    let inv_board = create_thin_board(wm, Some(hud_board), sw - inv_w - 15, bar_y - inv_h - 10, inv_w, inv_h, 0);
    wm.set_name(inv_board, "InventoryBoard");
    if let Some(w) = wm.windows.get_mut(&inv_board) {
        w.is_show = false;
    }

    let inv_title = wm.register(WindowType::TextLine);
    wm.set_parent(inv_title, inv_board);
    wm.set_position(inv_title, 14, 12);
    wm.set_text(inv_title, "Ekwipunek");
    wm.set_font_color(inv_title, [1.0, 0.85, 0.25, 1.0]);

    let inv_close = wm.register(WindowType::Button);
    wm.set_parent(inv_close, inv_board);
    wm.set_position(inv_close, inv_w - 24, 10);
    wm.set_size(inv_close, 14, 14);
    wm.set_name(inv_close, "InvCloseButton");
    wm.set_text(inv_close, "X");
    wm.set_color(inv_close, [0.8, 0.2, 0.2, 1.0]);

    for page in 0..4 {
        let page_btn = wm.register(WindowType::Button);
        wm.set_parent(page_btn, inv_board);
        wm.set_position(page_btn, 14 + (page as i32 * 38), 34);
        wm.set_size(page_btn, 36, 18);
        wm.set_name(page_btn, &format!("InvPageBtn_{}", page + 1));
        let roman = match page { 0 => "I", 1 => "II", 2 => "III", _ => "IV" };
        wm.set_text(page_btn, roman);
        if page == 0 {
            wm.set_color(page_btn, [0.3, 0.3, 0.45, 0.95]);
        } else {
            wm.set_color(page_btn, [0.15, 0.15, 0.2, 0.8]);
        }
    }

    let slot_size = 30;
    let slot_start_x = 14;
    let slot_start_y = 56;
    for row in 0..9 {
        for col in 0..5 {
            let slot_idx = row * 5 + col;
            let slot_box = wm.register(WindowType::Bar);
            wm.set_parent(slot_box, inv_board);
            wm.set_position(slot_box, slot_start_x + (col * (slot_size + 1)), slot_start_y + (row * (slot_size + 1)));
            wm.set_size(slot_box, slot_size, slot_size);
            wm.set_color(slot_box, [0.08, 0.08, 0.1, 0.9]);
            wm.set_name(slot_box, &format!("InvSlot_{}", slot_idx));

            let slot_border = wm.register(WindowType::Line);
            wm.set_parent(slot_border, slot_box);
            wm.set_position(slot_border, 0, 0);
            wm.set_size(slot_border, slot_size, 1);
            wm.set_color(slot_border, [0.35, 0.35, 0.4, 0.7]);
        }
    }

    let yang_bar = wm.register(WindowType::Bar);
    wm.set_parent(yang_bar, inv_board);
    wm.set_position(yang_bar, 14, inv_h - 32);
    wm.set_size(yang_bar, inv_w - 28, 20);
    wm.set_color(yang_bar, [0.05, 0.05, 0.08, 0.95]);

    let yang_txt = wm.register(WindowType::TextLine);
    wm.set_parent(yang_txt, yang_bar);
    wm.set_position(yang_txt, 8, 2);
    wm.set_text(yang_txt, "999,999,999 Yang");
    wm.set_font_color(yang_txt, [1.0, 0.85, 0.2, 1.0]);
    wm.set_name(yang_txt, "InvYangText");

    // =========================================================================
    // OKNO STATUSU POSTACI (Character Status - C)
    // =========================================================================
    let cs_w = 230;
    let cs_h = 370;
    let cs_board = create_thin_board(wm, Some(hud_board), 20, 50, cs_w, cs_h, 0);
    wm.set_name(cs_board, "CharacterStatusBoard");
    if let Some(w) = wm.windows.get_mut(&cs_board) {
        w.is_show = false;
    }

    let cs_title = wm.register(WindowType::TextLine);
    wm.set_parent(cs_title, cs_board);
    wm.set_position(cs_title, 14, 12);
    wm.set_text(cs_title, "Status Postaci");
    wm.set_font_color(cs_title, [1.0, 0.85, 0.25, 1.0]);

    let cs_close = wm.register(WindowType::Button);
    wm.set_parent(cs_close, cs_board);
    wm.set_position(cs_close, cs_w - 24, 10);
    wm.set_size(cs_close, 14, 14);
    wm.set_name(cs_close, "CharCloseButton");
    wm.set_text(cs_close, "X");
    wm.set_color(cs_close, [0.8, 0.2, 0.2, 1.0]);

    let stat_lines = [
        ("Postac: [SA]Admin (Wojownik)", [1.0, 1.0, 1.0, 1.0]),
        ("Poziom: 105  (EXP: 85%)", [0.4, 1.0, 0.4, 1.0]),
        ("Ranga: Rycerski (+20000)", [0.4, 0.8, 1.0, 1.0]),
        ("Gildia: Mistrzowie", [1.0, 0.85, 0.2, 1.0]),
        ("--------------------------------", [0.4, 0.4, 0.4, 1.0]),
        ("PZ: 15000 / 15000", [0.9, 0.3, 0.3, 1.0]),
        ("PE: 2000 / 2000", [0.3, 0.6, 1.0, 1.0]),
        ("WIT (Witalnosc): 90  (+)", [0.9, 0.85, 0.7, 1.0]),
        ("INT (Inteligencja): 90 (+)", [0.9, 0.85, 0.7, 1.0]),
        ("SILA (Sila): 90     (+)", [0.9, 0.85, 0.7, 1.0]),
        ("ZRE (Zrecznosc): 90  (+)", [0.9, 0.85, 0.7, 1.0]),
        ("--------------------------------", [0.4, 0.4, 0.4, 1.0]),
        ("Wartosc Ataku: 980 - 1120", [1.0, 0.7, 0.3, 1.0]),
        ("Obrona: 750", [0.7, 0.8, 1.0, 1.0]),
        ("Szybkosc Ruchu: 150", [0.8, 1.0, 0.8, 1.0]),
        ("Szybkosc Ataku: 135", [0.8, 1.0, 0.8, 1.0]),
    ];
    for (i, (txt, col)) in stat_lines.iter().enumerate() {
        let s_txt = wm.register(WindowType::TextLine);
        wm.set_parent(s_txt, cs_board);
        wm.set_position(s_txt, 14, 34 + (i as i32 * 20));
        wm.set_text(s_txt, txt);
        wm.set_font_color(s_txt, *col);
    }

    // =========================================================================
    // OKNO UMIEJETNOSCI (Skills - V)
    // =========================================================================
    let sk_w = 230;
    let sk_h = 320;
    let sk_board = create_thin_board(wm, Some(hud_board), 260, 50, sk_w, sk_h, 0);
    wm.set_name(sk_board, "SkillBoard");
    if let Some(w) = wm.windows.get_mut(&sk_board) {
        w.is_show = false;
    }

    let sk_title = wm.register(WindowType::TextLine);
    wm.set_parent(sk_title, sk_board);
    wm.set_position(sk_title, 14, 12);
    wm.set_text(sk_title, "Umiejetnosci (Body)");
    wm.set_font_color(sk_title, [1.0, 0.85, 0.25, 1.0]);

    let sk_close = wm.register(WindowType::Button);
    wm.set_parent(sk_close, sk_board);
    wm.set_position(sk_close, sk_w - 24, 10);
    wm.set_size(sk_close, 14, 14);
    wm.set_name(sk_close, "SkillCloseButton");
    wm.set_text(sk_close, "X");
    wm.set_color(sk_close, [0.8, 0.2, 0.2, 1.0]);

    let skill_list = [
        ("1. Trzystopniowe Ciecie (P)", "3-krotny cios wroga"),
        ("2. Wir Miecza (P)", "Obszarowy atak 360"),
        ("3. Szarza (P)", "Szybki doskok do celu"),
        ("4. Berserk (P)", "Zwieksza szybkosc ataku"),
        ("5. Aura Miecza (P)", "Potezny bonus do ataku"),
        ("6. Wola Zycia (P)", "Odpornosc na obrazenia"),
    ];
    for (i, (s_name, s_desc)) in skill_list.iter().enumerate() {
        let s_btn = wm.register(WindowType::Button);
        wm.set_parent(s_btn, sk_board);
        wm.set_position(s_btn, 14, 38 + (i as i32 * 44));
        wm.set_size(s_btn, sk_w - 28, 38);
        wm.set_name(s_btn, &format!("SkillBtn_{}", i + 1));
        wm.set_color(s_btn, [0.15, 0.15, 0.22, 0.9]);
        wm.set_text(s_btn, s_name);

        let d_txt = wm.register(WindowType::TextLine);
        wm.set_parent(d_txt, s_btn);
        wm.set_position(d_txt, 8, 18);
        wm.set_text(d_txt, s_desc);
        wm.set_font_color(d_txt, [0.7, 0.7, 0.7, 1.0]);
    }

    // =========================================================================
    // DIALOG SYSTEMOWY (ESC)
    // =========================================================================
    let sys_w = 200;
    let sys_h = 210;
    let sys_board = create_thin_board(wm, Some(hud_board), (sw - sys_w) / 2, (sh - sys_h) / 2, sys_w, sys_h, 0);
    wm.set_name(sys_board, "SystemDialogBoard");
    if let Some(w) = wm.windows.get_mut(&sys_board) {
        w.is_show = false;
    }

    let sys_title = wm.register(WindowType::TextLine);
    wm.set_parent(sys_title, sys_board);
    wm.set_position(sys_title, 14, 12);
    wm.set_text(sys_title, "Menu Gry [ESC]");
    wm.set_font_color(sys_title, [1.0, 0.85, 0.25, 1.0]);

    let sys_btns = [
        ("SysBtnRestart", "Restart w miescie", 38),
        ("SysBtnLogout", "Wyloguj do wyboru", 78),
        ("SysBtnExit", "Zakoncz gre", 118),
        ("SysBtnCancel", "Powrot do gry", 158),
    ];
    for (b_name, b_label, b_y) in sys_btns {
        let btn = wm.register(WindowType::Button);
        wm.set_parent(btn, sys_board);
        wm.set_position(btn, 14, b_y);
        wm.set_size(btn, sys_w - 28, 28);
        wm.set_name(btn, b_name);
        wm.set_color(btn, [0.2, 0.2, 0.28, 0.95]);
        wm.set_text(btn, b_label);
    }

    println!("[+] Game HUD UI Pure Rust gotowy (zarejestrowano {} okien)!", wm.windows.len());
}

pub fn update_game_hud_live(pos: [f32; 3], _rot: f32) {
    let (hp, max_hp, sp, max_sp, level) = if let Ok(state) = m2_client_bridge::state::game_state::GLOBAL_GAME_STATE.read() {
        (
            state.player.points[5],
            state.player.points[6].max(1),
            state.player.points[7],
            state.player.points[8].max(1),
            state.player.points[1],
        )
    } else {
        (15000, 15000, 2000, 2000, 105)
    };

    let mut wm = crate::ui_window_mgr::get_window_manager();
    if let Some(h) = wm.windows.values().find(|w| w.name == "HudHpText").map(|w| w.handle) {
        wm.set_text(h, &format!("PZ: {} / {}", hp, max_hp));
    }
    if let Some(h) = wm.windows.values().find(|w| w.name == "HudHpBarFg").map(|w| w.handle) {
        let width = ((hp as f32 / max_hp as f32) * 180.0).clamp(0.0, 180.0) as i32;
        wm.set_size(h, width, 14);
    }
    if let Some(h) = wm.windows.values().find(|w| w.name == "HudSpText").map(|w| w.handle) {
        wm.set_text(h, &format!("PE: {} / {}", sp, max_sp));
    }
    if let Some(h) = wm.windows.values().find(|w| w.name == "HudSpBarFg").map(|w| w.handle) {
        let width = ((sp as f32 / max_sp as f32) * 180.0).clamp(0.0, 180.0) as i32;
        wm.set_size(h, width, 14);
    }
    if let Some(h) = wm.windows.values().find(|w| w.name == "HudLevelText").map(|w| w.handle) {
        wm.set_text(h, &format!("Pozycja: {:.0}, {:.0} (Lv. {})", pos[0], pos[1], level));
    }
    if let Some(h) = wm.windows.values().find(|w| w.name == "MiniMapDot").map(|w| w.handle) {
        let radar_x = 70 + ((pos[0] / 80.0) as i32).clamp(-65, 65);
        let radar_y = 50 + ((-pos[1] / 80.0) as i32).clamp(-45, 45);
        wm.set_position(h, radar_x, radar_y);
    }
}

pub fn toggle_edit_focus() {
    let mut active = ACTIVE_EDIT.lock().unwrap();
    if *active == "IdEditLine" {
        *active = "PwdEditLine";
        println!("[UI] Aktywne pole edycji: Haslo (PwdEditLine)");
    } else {
        *active = "IdEditLine";
        println!("[UI] Aktywne pole edycji: ID (IdEditLine)");
    }
}

pub fn set_edit_focus(name: &'static str) {
    let mut active = ACTIVE_EDIT.lock().unwrap();
    *active = name;
    println!("[UI] Aktywne pole edycji zmienione na: {}", name);
}

pub fn handle_char_input(c: char) {
    let active = *ACTIVE_EDIT.lock().unwrap();
    let mut wm = get_window_manager();
    let handle = wm.windows.values().find(|w| w.name == active).map(|w| w.handle);
    if let Some(h) = handle {
        if let Some(win) = wm.windows.get_mut(&h) {
            if c == '\x08' { // Backspace
                if active == "PwdEditLine" {
                    let mut real_pwd = REAL_PASSWORD.lock().unwrap();
                    real_pwd.pop();
                    win.text = "*".repeat(real_pwd.len());
                } else {
                    let mut real_id = REAL_ID.lock().unwrap();
                    real_id.pop();
                    win.text = real_id.clone();
                }
            } else if c >= ' ' && c <= '~' { // Printable ASCII
                if active == "PwdEditLine" {
                    let mut real_pwd = REAL_PASSWORD.lock().unwrap();
                    if !PWD_FIRST_INPUT.swap(true, std::sync::atomic::Ordering::SeqCst) {
                        real_pwd.clear();
                    }
                    if real_pwd.len() < 16 {
                        real_pwd.push(c);
                        win.text = "*".repeat(real_pwd.len());
                    }
                } else {
                    let mut real_id = REAL_ID.lock().unwrap();
                    if !ID_FIRST_INPUT.swap(true, std::sync::atomic::Ordering::SeqCst) {
                        real_id.clear();
                    }
                    if real_id.len() < 30 {
                        real_id.push(c);
                        win.text = real_id.clone();
                    }
                }
            }
        }
    }
}

/// Obsluga interakcji uzytkownika z kontrolkami ekranu logowania (C++ 1:1)
pub fn handle_ui_click(name: &str, _handle: u32, wm: &mut crate::ui_window_mgr::WindowManager) {
    println!("[UI] Kliknieto kontrolke: {}", name);
    match name {
        "ServerExitButton" => {
            println!("[UI] Wyjscie z gry na polecenie gracza");
            crate::win32_message_dispatcher::post_quit_message(0);
        }
        "ServerSelectButton" => {
            println!("[UI] Przejscie do formularza wprowadzania danych logowania (LoginBoard)");
            let sb_handle = wm.windows.values().find(|w| w.name == "ServerBoard").map(|w| w.handle);
            let lb_handle = wm.windows.values().find(|w| w.name == "LoginBoard").map(|w| w.handle);
            if let Some(h) = sb_handle { wm.hide(h); }
            if let Some(h) = lb_handle { wm.show(h); }
        }
        "LoginExitButton" => {
            println!("[UI] Powrot do wyboru serwera (ServerBoard)");
            let lb_handle = wm.windows.values().find(|w| w.name == "LoginBoard").map(|w| w.handle);
            let sb_handle = wm.windows.values().find(|w| w.name == "ServerBoard").map(|w| w.handle);
            if let Some(h) = lb_handle { wm.hide(h); }
            if let Some(h) = sb_handle { wm.show(h); }
        }
        "IdEditLine" | "IdBox" => {
            set_edit_focus("IdEditLine");
        }
        "PwdEditLine" | "PwdBox" => {
            set_edit_focus("PwdEditLine");
        }
        "LoginButton" => {
            let id = REAL_ID.lock().unwrap().clone();
            let pwd = REAL_PASSWORD.lock().unwrap().clone();
            println!("[UI] Logowanie do serwera gry (konto='{}', dlugosc hasla={})...", id, pwd.len());
            let login_pkt = m2_client_bridge::net::cg::login::TPacketCGLogin3::new(&id, &pwd);
            let pkt_bytes = login_pkt.to_bytes();
            let header = { login_pkt.header };
            println!("[NET] Pakiet CG_LOGIN3 gotowy (rozmiar: {} bajtow, opcode=0x{:04X})", pkt_bytes.len(), header);

            let id_clone = id.clone();
            let pwd_clone = pwd.clone();
            std::thread::spawn(move || {
                use std::io::{Read, Write};
                use crate::secure_cipher::SecureCipher;

                let target_addr = "127.0.0.1:11000";
                println!("[NET] Nawiazywanie polaczenia TCP z serwerem autoryzacji: {}...", target_addr);
                match std::net::TcpStream::connect_timeout(&target_addr.parse().unwrap(), std::time::Duration::from_secs(3)) {
                    Ok(mut stream) => {
                        println!("[NET] [+] Polaczono pomyslnie z serwerem Auth ({})!", target_addr);
                        let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(4)));
                        let _ = stream.set_write_timeout(Some(std::time::Duration::from_secs(4)));

                        let mut buf = [0u8; 2048];
                        let mut total_read = 0;

                        // 1. Odbior GC::PHASE (5B) i GC::KEY_CHALLENGE (72B) - lacznie 77 bajtow
                        while total_read < 77 {
                            match stream.read(&mut buf[total_read..]) {
                                Ok(n) if n > 0 => total_read += n,
                                Ok(_) => {
                                    eprintln!("[-] Serwer zamknal polaczenie przed zakonczeniem handshake");
                                    return;
                                }
                                Err(e) => {
                                    eprintln!("[-] Blad odczytu naglowka handshake: {}", e);
                                    return;
                                }
                            }
                        }
                        println!("[NET] Odebrano naglowek handshake ({} bajtow): {:02X?}", total_read, &buf[..total_read]);

                        let phase_header = u16::from_le_bytes(buf[0..2].try_into().unwrap());
                        let phase_len = u16::from_le_bytes(buf[2..4].try_into().unwrap());
                        let phase = buf[4];
                        println!("[NET] Faza poczatkowa serwera: opcode=0x{:04X}, len={}, phase={}", phase_header, phase_len, phase);

                        let challenge_header = u16::from_le_bytes(buf[5..7].try_into().unwrap());
                        let challenge_len = u16::from_le_bytes(buf[7..9].try_into().unwrap());
                        println!("[NET] Challenge serwera: opcode=0x{:04X}, len={}", challenge_header, challenge_len);

                        let mut server_pk = [0u8; 32];
                        server_pk.copy_from_slice(&buf[9..41]);
                        let mut challenge = [0u8; 32];
                        challenge.copy_from_slice(&buf[41..73]);
                        let server_time = u32::from_le_bytes(buf[73..77].try_into().unwrap());
                        println!("[NET] Parametry krypto: server_time={}, server_pk={:02X?}...", server_time, &server_pk[..4]);

                        // 2. Inicjalizacja SecureCipher i obliczenie kluczy sesyjnych
                        let mut cipher = SecureCipher::new();
                        if !cipher.initialize() {
                            eprintln!("[-] Blad inicjalizacji SecureCipher!");
                            return;
                        }
                        if !cipher.compute_client_keys(&server_pk) {
                            eprintln!("[-] Blad obliczania kluczy sesyjnych klienta!");
                            return;
                        }
                        let challenge_resp = cipher.compute_challenge_response(&challenge);

                        // 3. Wyslanie CG::KEY_RESPONSE (68 bajtow)
                        // header (0x000A) [2B], length (68) [2B], client_pk [32B], challenge_response [32B]
                        let mut key_resp = [0u8; 68];
                        key_resp[0..2].copy_from_slice(&0x000Au16.to_le_bytes()); // CG::KEY_RESPONSE
                        key_resp[2..4].copy_from_slice(&68u16.to_le_bytes());     // len = 68
                        key_resp[4..36].copy_from_slice(&cipher.pk);
                        key_resp[36..68].copy_from_slice(&challenge_resp);

                        if let Err(e) = stream.write_all(&key_resp) {
                            eprintln!("[-] Blad wysylania CG_KEY_RESPONSE: {}", e);
                            return;
                        }
                        println!("[NET] Odeslano CG_KEY_RESPONSE (68 bajtow, opcode=0x000A)");

                        // 4. Odbior GC::KEY_COMPLETE (76 bajtow)
                        // header (0x000C) [2B], length (76) [2B], encrypted_token [48B], nonce [24B]
                        let mut complete_buf = [0u8; 76];
                        let mut complete_read = 0;
                        while complete_read < 76 {
                            match stream.read(&mut complete_buf[complete_read..]) {
                                Ok(n) if n > 0 => complete_read += n,
                                Ok(_) => {
                                    eprintln!("[-] Serwer zamknal polaczenie przed KEY_COMPLETE");
                                    return;
                                }
                                Err(e) => {
                                    eprintln!("[-] Blad odczytu KEY_COMPLETE: {}", e);
                                    return;
                                }
                            }
                        }
                        println!("[NET] Odebrano GC_KEY_COMPLETE ({} bajtow)", complete_read);

                        let complete_header = u16::from_le_bytes(complete_buf[0..2].try_into().unwrap());
                        if complete_header == 0x000C {
                            let mut token_enc = [0u8; 48];
                            token_enc.copy_from_slice(&complete_buf[4..52]);
                            let mut nonce = [0u8; 24];
                            nonce.copy_from_slice(&complete_buf[52..76]);

                            if cipher.decrypt_token(&token_enc, &nonce) {
                                println!("[NET] [+] Token sesji zostal pomyslnie odszyfrowany!");
                            } else {
                                eprintln!("[-] Ostrzezenie: Nie udalo sie odszyfrowac tokena sesji");
                            }
                        }

                        // 5. Aktywacja ciaglego szyfrowania XChaCha20 dla calej dalszej komunikacji
                        cipher.set_activated(true);
                        println!("[NET] [+] Szyfrowanie strumieniowe XChaCha20 aktywowane!");

                        // 6. Odbior potwierdzenia nowej fazy (GC::PHASE = PHASE_AUTH = 2)
                        let mut phase_buf = [0u8; 5];
                        if let Ok(n) = stream.read(&mut phase_buf) {
                            if n > 0 {
                                cipher.decrypt_in_place(&mut phase_buf[..n]);
                                let new_phase_hdr = u16::from_le_bytes(phase_buf[0..2].try_into().unwrap_or_default());
                                println!("[NET] Przejscie fazy serwera: opcode=0x{:04X}, faza={}", new_phase_hdr, phase_buf[4]);
                            }
                        }

                        // 7. Wyslanie zaszyfrowanego pakietu CG_LOGIN3 (52 bajty)
                        // header (0x0102) [2B], length (52) [2B], login [31B], passwd [17B]
                        let mut login_pkt = [0u8; 52];
                        login_pkt[0..2].copy_from_slice(&0x0102u16.to_le_bytes()); // CG::LOGIN3
                        login_pkt[2..4].copy_from_slice(&52u16.to_le_bytes());     // len = 52

                        let id_b = id_clone.as_bytes();
                        let id_len = id_b.len().min(30);
                        login_pkt[4..4 + id_len].copy_from_slice(&id_b[..id_len]);

                        let pwd_b = pwd_clone.as_bytes();
                        let pwd_len = pwd_b.len().min(16);
                        login_pkt[35..35 + pwd_len].copy_from_slice(&pwd_b[..pwd_len]);

                        cipher.encrypt_in_place(&mut login_pkt);

                        if let Err(e) = stream.write_all(&login_pkt) {
                            eprintln!("[-] Blad wysylania zaszyfrowanego pakietu logowania: {}", e);
                            return;
                        }
                        println!("[NET] [+] Zaszyfrowany pakiet CG_LOGIN3 wyslany (login: '{}', rozmiar: 52B)", id_clone);

                        // 8. Odbior i odszyfrowanie odpowiedzi serwera na logowanie
                        let mut resp_buf = [0u8; 1024];
                        match stream.read(&mut resp_buf) {
                            Ok(n) if n > 0 => {
                                cipher.decrypt_in_place(&mut resp_buf[..n]);
                                let resp_hdr = u16::from_le_bytes(resp_buf[0..2].try_into().unwrap_or_default());
                                let resp_len = u16::from_le_bytes(resp_buf[2..4].try_into().unwrap_or_default());
                                println!("[NET] Odpowiedz serwera na logowanie: opcode=0x{:04X}, dlugosc={}B", resp_hdr, resp_len);

                                // GC::AUTH_SUCCESS = 0x0108, GC::LOGIN_SUCCESS3 = 0x0104, GC::LOGIN_FAILURE = 0x0106
                                if resp_hdr == 0x0108 || resp_hdr == 0x0104 {
                                    let login_key = if n >= 8 {
                                        u32::from_le_bytes(resp_buf[4..8].try_into().unwrap_or_default())
                                    } else {
                                        0
                                    };
                                    println!("[+] ========================================================");
                                    println!("[+] [SUKCES] AUTORYZACJA POMYSLNA! KONTO ZALOGOWANE (dwLoginKey={})", login_key);
                                    println!("[+] Nawiazywanie polaczenia z Kanalem 1 gry: 127.0.0.1:11011...");
                                    println!("[+] ========================================================");

                                    // Polaczenie z Game Serverem (Channel 1)
                                    let ch_addr = "127.0.0.1:11011";
                                    match std::net::TcpStream::connect_timeout(&ch_addr.parse().unwrap(), std::time::Duration::from_secs(3)) {
                                        Ok(mut ch_stream) => {
                                            println!("[NET] [+] Polaczono pomyslnie z Game Serverem CH1 ({})!", ch_addr);
                                            let _ = ch_stream.set_read_timeout(Some(std::time::Duration::from_secs(4)));
                                            let _ = ch_stream.set_write_timeout(Some(std::time::Duration::from_secs(4)));

                                            let mut ch_buf = [0u8; 2048];
                                            let mut ch_read = 0;
                                            while ch_read < 77 {
                                                match ch_stream.read(&mut ch_buf[ch_read..]) {
                                                    Ok(bytes) if bytes > 0 => ch_read += bytes,
                                                    _ => break,
                                                }
                                            }
                                            if ch_read >= 77 {
                                                let mut ch_server_pk = [0u8; 32];
                                                ch_server_pk.copy_from_slice(&ch_buf[9..41]);
                                                let mut ch_challenge = [0u8; 32];
                                                ch_challenge.copy_from_slice(&ch_buf[41..73]);

                                                let mut ch_cipher = SecureCipher::new();
                                                ch_cipher.initialize();
                                                ch_cipher.compute_client_keys(&ch_server_pk);
                                                let ch_chal_resp = ch_cipher.compute_challenge_response(&ch_challenge);

                                                // CG::KEY_RESPONSE (68B)
                                                let mut ch_key_resp = [0u8; 68];
                                                ch_key_resp[0..2].copy_from_slice(&0x000Au16.to_le_bytes());
                                                ch_key_resp[2..4].copy_from_slice(&68u16.to_le_bytes());
                                                ch_key_resp[4..36].copy_from_slice(&ch_cipher.pk);
                                                ch_key_resp[36..68].copy_from_slice(&ch_chal_resp);
                                                let _ = ch_stream.write_all(&ch_key_resp);

                                                // GC::KEY_COMPLETE (76B)
                                                let mut ch_comp_buf = [0u8; 76];
                                                let mut comp_read = 0;
                                                while comp_read < 76 {
                                                    match ch_stream.read(&mut ch_comp_buf[comp_read..]) {
                                                        Ok(b) if b > 0 => comp_read += b,
                                                        _ => break,
                                                    }
                                                }
                                                if comp_read >= 76 {
                                                    let mut token = [0u8; 48];
                                                    token.copy_from_slice(&ch_comp_buf[4..52]);
                                                    let mut nonce = [0u8; 24];
                                                    nonce.copy_from_slice(&ch_comp_buf[52..76]);
                                                    ch_cipher.decrypt_token(&token, &nonce);
                                                }
                                                ch_cipher.set_activated(true);

                                                // GC::PHASE (5B)
                                                let mut phase5 = [0u8; 5];
                                                if let Ok(p_len) = ch_stream.read(&mut phase5) {
                                                    ch_cipher.decrypt_in_place(&mut phase5[..p_len]);
                                                }

                                                // CG::LOGIN2 (39B)
                                                let mut login2_pkt = [0u8; 39];
                                                login2_pkt[0..2].copy_from_slice(&0x0101u16.to_le_bytes()); // CG::LOGIN2
                                                login2_pkt[2..4].copy_from_slice(&39u16.to_le_bytes());
                                                let id_b = id_clone.as_bytes();
                                                let id_len = id_b.len().min(30);
                                                login2_pkt[4..4 + id_len].copy_from_slice(&id_b[..id_len]);
                                                login2_pkt[35..39].copy_from_slice(&login_key.to_le_bytes());

                                                ch_cipher.encrypt_in_place(&mut login2_pkt);
                                                let _ = ch_stream.write_all(&login2_pkt);
                                                println!("[NET] [+] Wyslano CG_LOGIN2 do CH1 (login: '{}', dwLoginKey: {})", id_clone, login_key);

                                                // Odbior odpowiedzi Game Serwera
                                                let mut game_resp = [0u8; 2048];
                                                let mut gr_len = 0;
                                                while gr_len < 502 {
                                                    match ch_stream.read(&mut game_resp[gr_len..]) {
                                                        Ok(n) if n > 0 => {
                                                            gr_len += n;
                                                            if gr_len >= 502 {
                                                                break;
                                                            }
                                                        }
                                                        _ => break,
                                                    }
                                                }

                                                if gr_len > 0 {
                                                    ch_cipher.decrypt_in_place(&mut game_resp[..gr_len]);
                                                    let g_hdr = u16::from_le_bytes(game_resp[0..2].try_into().unwrap_or_default());
                                                    println!("[NET] Odpowiedz Game Serwera CH1: opcode=0x{:04X}, rozmiar={}B", g_hdr, gr_len);
                                                    println!("[+] ========================================================");
                                                    println!("[+] [GRA] POMYSLNIE POLACZONO Z KANALEM GRY CH1!");
                                                    println!("[+] Parsowanie postaci i aktywacja PHASE_SELECT...");
                                                    println!("[+] ========================================================");

                                                    // Parsowanie postaci z bufora CH1 (od offsetu 14, gdzie zaczyna sie akSimplePlayerInformation w TPacketGCLoginSuccess4):
                                                    let mut chars = Vec::new();
                                                    let p_base = 14;
                                                    for i in 0..4 {
                                                        let off = p_base + i * 103;
                                                        if off + 103 <= gr_len {
                                                            let id = u32::from_le_bytes(game_resp[off..off+4].try_into().unwrap_or_default());
                                                            if id != 0 {
                                                                let name_raw = &game_resp[off+4..off+69];
                                                                let name_len = name_raw.iter().position(|&b| b == 0).unwrap_or(65);
                                                                let name = String::from_utf8_lossy(&name_raw[..name_len]).trim().to_string();
                                                                let job = game_resp[off+69];
                                                                let level = game_resp[off+70];
                                                                let playtime = u32::from_le_bytes(game_resp[off+71..off+75].try_into().unwrap_or_default());
                                                                let st = game_resp[off+75];
                                                                let ht = game_resp[off+76];
                                                                let dx = game_resp[off+77];
                                                                let iq = game_resp[off+78];
                                                                let main_part = u16::from_le_bytes(game_resp[off+79..off+81].try_into().unwrap_or_default());
                                                                let hair_part = u16::from_le_bytes(game_resp[off+82..off+84].try_into().unwrap_or_default());
                                                                println!("[+] [POSTAC] Slot {}: '{}', Poziom: {}, Klasa: {}, ID: {}", i + 1, name, level, job, id);
                                                                chars.push(CharacterSlot {
                                                                    id,
                                                                    name,
                                                                    job,
                                                                    level,
                                                                    playtime,
                                                                    st,
                                                                    ht,
                                                                    dx,
                                                                    iq,
                                                                    main_part,
                                                                    hair_part,
                                                                });
                                                            }
                                                        }
                                                    }

                                                    if chars.is_empty() {
                                                        println!("[!] Brak postaci w pakiecie, uzywam domyslnej postaci administratora");
                                                        chars.push(CharacterSlot {
                                                            id: 1,
                                                            name: "[SA]Admin".to_string(),
                                                            job: 0,
                                                            level: 105,
                                                            playtime: 120,
                                                            st: 90,
                                                            ht: 90,
                                                            dx: 90,
                                                            iq: 90,
                                                            main_part: 0,
                                                            hair_part: 0,
                                                        });
                                                    }

                                                    // Zapisz stan sesji do zmiennych globalnych
                                                    {
                                                        let mut g_chars = GLOBAL_CHARACTERS.lock().unwrap();
                                                        *g_chars = chars.clone();
                                                        let mut g_sel = SELECTED_CHAR_SLOT.lock().unwrap();
                                                        *g_sel = 0;
                                                        let mut g_phase = CURRENT_PHASE.lock().unwrap();
                                                        *g_phase = "SELECT".to_string();
                                                        let mut g_sock = ACTIVE_GAME_SOCKET.lock().unwrap();
                                                        *g_sock = Some((ch_stream, ch_cipher));
                                                    }

                                                    // Przelacz UI na ekran wyboru postaci
                                                    {
                                                        let mut wm = crate::ui_window_mgr::get_window_manager();
                                                        m2_ui_setup_character_select(&mut wm, 1024, 768, &chars);
                                                    }
                                                    println!("[+] Ekran wyboru postaci aktywowany pomyslnie na pulpicie!");
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("[-] Nie udalo sie polaczyc z Game Serverem CH1 {}: {}", ch_addr, e);
                                        }
                                    }
                                } else if resp_hdr == 0x0106 {
                                    let status_str = String::from_utf8_lossy(&resp_buf[4..n.min(32)]);
                                    println!("[-] [AUTH] Blad logowania: nieprawidlowy login lub haslo ({})", status_str);
                                } else {
                                    println!("[+] [AUTH] Odebrano pakiet serwera: opcode=0x{:04X}", resp_hdr);
                                }
                            }
                            Ok(_) => println!("[-] Serwer zamknal polaczenie."),
                            Err(e) => eprintln!("[-] Blad oczekiwania na odpowiedz serwera: {}", e),
                        }
                    }
                    Err(e) => {
                        println!("[-] Nie udalo sie polaczyc z serwerem Auth {}: {}", target_addr, e);
                    }
                }
            });
        }
        "SelectStartButton" => {
            println!("[UI] Kliknieto START (Wybor postaci i przejscie do fazy LOADING)!");
            let slot_idx = *SELECTED_CHAR_SLOT.lock().unwrap() as u8;
            
            // 1. Wyslij TYLKO pakiet wyboru postaci (CG_CHARACTER_SELECT)
            let send_ok = {
                let mut sock_guard = ACTIVE_GAME_SOCKET.lock().unwrap();
                if let Some((ref mut ch_stream, ref mut ch_cipher)) = *sock_guard {
                    use std::io::Write;
                    // CG::CHARACTER_SELECT (5 bajtow: header 0x0203, len 5, slot_idx)
                    let mut sel_pkt = [0u8; 5];
                    sel_pkt[0..2].copy_from_slice(&0x0203u16.to_le_bytes());
                    sel_pkt[2..4].copy_from_slice(&5u16.to_le_bytes());
                    sel_pkt[4] = slot_idx;
                    ch_cipher.encrypt_in_place(&mut sel_pkt);
                    if let Err(e) = ch_stream.write_all(&sel_pkt) {
                        eprintln!("[-] Blad wysylania CG_CHARACTER_SELECT: {}", e);
                        false
                    } else {
                        println!("[NET] [+] Wyslano CG_CHARACTER_SELECT (slot: {}, opcode=0x0203)", slot_idx);
                        true
                    }
                } else {
                    eprintln!("[-] Brak aktywnego gniazda gry CH1!");
                    false
                }
            }; // sock_guard zostaje zwolniony TUTAJ!

            if send_ok {
                // 2. Zmiana fazy na LOADING (zgodnie z protokolem C++)
                *CURRENT_PHASE.lock().unwrap() = "LOADING".to_string();
                println!("[+] ========================================================");
                println!("[+] [GRA] OCZEKIWANIE NA ZALADOWANIE POSTACI (PHASE_LOADING)...");
                println!("[+] Oczekiwanie na GC_MAIN_CHARACTER od serwera gry...");
                println!("[+] ========================================================");

                // 3. Uruchomienie watku odbioru pakietow sieciowych
                crate::game_net_receiver::start_game_recv_thread();
            }
        }
        "SelectExitButton" => {
            println!("[UI] Powrot z wyboru postaci do ekranu logowania");
            *CURRENT_PHASE.lock().unwrap() = String::new();
            let mut sock_guard = ACTIVE_GAME_SOCKET.lock().unwrap();
            *sock_guard = None;
            for win in wm.windows.values_mut() {
                if win.name.starts_with("Select") || win.name.starts_with("Char") {
                    win.is_show = false;
                } else if win.name == "ServerBoard" {
                    win.is_show = true;
                }
            }
        }
        "SelectCreateButton" => {
            println!("[UI] Kliknieto: Tworzenie nowej postaci");
        }
        "SelectDeleteButton" => {
            println!("[UI] Kliknieto: Usuwanie postaci");
        }
        "SelectPrevButton" => {
            let chars = GLOBAL_CHARACTERS.lock().unwrap().clone();
            if !chars.is_empty() {
                let mut sel = SELECTED_CHAR_SLOT.lock().unwrap();
                if *sel > 0 {
                    *sel -= 1;
                } else {
                    *sel = chars.len() - 1;
                }
                let idx = *sel;
                println!("[UI] Przelaczono na slot {}: {} (Lv. {})", idx + 1, chars[idx].name, chars[idx].level);
                update_character_select_ui(wm, &chars, idx);
            }
        }
        "SelectNextButton" => {
            let chars = GLOBAL_CHARACTERS.lock().unwrap().clone();
            if !chars.is_empty() {
                let mut sel = SELECTED_CHAR_SLOT.lock().unwrap();
                if *sel + 1 < chars.len() {
                    *sel += 1;
                } else {
                    *sel = 0;
                }
                let idx = *sel;
                println!("[UI] Przelaczono na slot {}: {} (Lv. {})", idx + 1, chars[idx].name, chars[idx].level);
                update_character_select_ui(wm, &chars, idx);
            }
        }
        "HudBtnExit" => {
            println!("[UI] HUD: Wyjscie z gry na polecenie gracza");
            crate::win32_message_dispatcher::post_quit_message(0);
        }
        "HudBtnCharacter" | "HudBtnChar" => {
            println!("[UI] HUD: Przelaczenie okna Statusu Postaci (C)");
            if let Some(h) = wm.windows.values().find(|w| w.name == "CharacterStatusBoard").map(|w| w.handle) {
                let vis = wm.windows.get(&h).map(|w| w.is_show).unwrap_or(false);
                if vis { wm.hide(h); } else { wm.show(h); }
            }
        }
        "CharCloseButton" => {
            println!("[UI] Zamkniecie okna Statusu Postaci");
            if let Some(h) = wm.windows.values().find(|w| w.name == "CharacterStatusBoard").map(|w| w.handle) {
                wm.hide(h);
            }
        }
        "HudBtnInventory" | "HudBtnInv" => {
            println!("[UI] HUD: Przelaczenie okna Ekwipunku (I)");
            if let Some(h) = wm.windows.values().find(|w| w.name == "InventoryBoard").map(|w| w.handle) {
                let vis = wm.windows.get(&h).map(|w| w.is_show).unwrap_or(false);
                if vis { wm.hide(h); } else { wm.show(h); }
            }
        }
        "InvCloseButton" => {
            println!("[UI] Zamkniecie okna Ekwipunku");
            if let Some(h) = wm.windows.values().find(|w| w.name == "InventoryBoard").map(|w| w.handle) {
                wm.hide(h);
            }
        }
        "HudBtnSkills" => {
            println!("[UI] HUD: Przelaczenie okna Umiejetnosci (V)");
            if let Some(h) = wm.windows.values().find(|w| w.name == "SkillBoard").map(|w| w.handle) {
                let vis = wm.windows.get(&h).map(|w| w.is_show).unwrap_or(false);
                if vis { wm.hide(h); } else { wm.show(h); }
            }
        }
        "SkillCloseButton" => {
            println!("[UI] Zamkniecie okna Umiejetnosci");
            if let Some(h) = wm.windows.values().find(|w| w.name == "SkillBoard").map(|w| w.handle) {
                wm.hide(h);
            }
        }
        "HudBtnSystem" => {
            println!("[UI] HUD: Menu Gry (ESC)");
            let mut closed_sub = false;
            for win_name in &["InventoryBoard", "CharacterStatusBoard", "SkillBoard"] {
                if let Some(h) = wm.windows.values().find(|w| &w.name == win_name).map(|w| w.handle) {
                    if wm.windows.get(&h).map(|w| w.is_show).unwrap_or(false) {
                        wm.hide(h);
                        closed_sub = true;
                    }
                }
            }
            if !closed_sub {
                if let Some(h) = wm.windows.values().find(|w| w.name == "SystemDialogBoard").map(|w| w.handle) {
                    let vis = wm.windows.get(&h).map(|w| w.is_show).unwrap_or(false);
                    if vis { wm.hide(h); } else { wm.show(h); }
                }
            }
        }
        "SysBtnCancel" => {
            if let Some(h) = wm.windows.values().find(|w| w.name == "SystemDialogBoard").map(|w| w.handle) {
                wm.hide(h);
            }
        }
        "SysBtnExit" => {
            println!("[UI] Wyjscie z gry z menu opcji");
            crate::win32_message_dispatcher::post_quit_message(0);
        }
        "SysBtnLogout" => {
            println!("[UI] Wylogowanie do ekranu wyboru postaci");
            if let Some(h) = wm.windows.values().find(|w| w.name == "SystemDialogBoard").map(|w| w.handle) {
                wm.hide(h);
            }
            if let Some(h) = wm.windows.values().find(|w| w.name == "GameHudBoard").map(|w| w.handle) {
                wm.hide(h);
            }
            *CURRENT_PHASE.lock().unwrap() = "SELECT".to_string();
            let chars = GLOBAL_CHARACTERS.lock().unwrap().clone();
            m2_ui_setup_character_select(wm, 1024, 768, &chars);
        }
        "SysBtnRestart" => {
            println!("[UI] Restart pozycji postaci w miescie");
            if let Some(h) = wm.windows.values().find(|w| w.name == "SystemDialogBoard").map(|w| w.handle) {
                wm.hide(h);
            }
            if let Ok(mut state) = m2_client_bridge::state::game_state::GLOBAL_GAME_STATE.write() {
                state.player.x = 0.0;
                state.player.y = 0.0;
                state.player.z = 0.0;
            }
        }
        _ if name.starts_with("QuickSlot_") => {
            let num = name.trim_start_matches("QuickSlot_");
            match num {
                "1" => println!("[QUICKSLOT] [1] Uzyto umiejetnosci: Wir Miecza!"),
                "2" => println!("[QUICKSLOT] [2] Uzyto umiejetnosci: Szarza!"),
                "3" => println!("[QUICKSLOT] [3] Uzyto umiejetnosci: Ciecie!"),
                "4" => println!("[QUICKSLOT] [4] Aktywowano buff: Berserk!"),
                "5" => println!("[QUICKSLOT] [F1] Uzyto Czerwonej Mikstury (+500 PZ)!"),
                "6" => println!("[QUICKSLOT] [F2] Uzyto Niebieskiej Mikstury (+200 PE)!"),
                "7" => println!("[QUICKSLOT] [F3] Uzyto Zielonej Mikstury (Szybkosc Ataku +30)!"),
                "8" => println!("[QUICKSLOT] [F4] Uzyto Fioletowej Mikstury (Szybkosc Ruchu +60)!"),
                _ => println!("[QUICKSLOT] Uzyto slotu {}", num),
            }
        }
        _ if name.starts_with("CharSlot") => {
            let suffix = if name.starts_with("CharSlotBar_") {
                &name[12..]
            } else if name.starts_with("CharSlotTxt_") {
                &name[12..]
            } else if name.starts_with("CharSlot_") {
                &name[9..]
            } else {
                ""
            };
            if let Ok(idx) = suffix.parse::<usize>() {
                let chars = GLOBAL_CHARACTERS.lock().unwrap().clone();
                if idx < chars.len() {
                    let mut sel = SELECTED_CHAR_SLOT.lock().unwrap();
                    *sel = idx;
                    println!("[UI] Zaznaczono postac w slocie {}: {} (Lv. {})", idx + 1, chars[idx].name, chars[idx].level);
                    update_character_select_ui(wm, &chars, idx);
                }
            }
        }
        "Server_0" => {
            println!("[UI] Zaznaczono Serwer 01: Metin2");
            if let Some(bar_h) = wm.windows.values().find(|w| w.name == "ServerSelectBar").map(|w| w.handle) {
                wm.set_position(bar_h, 14, 42);
            }
            if let Some(s0) = wm.windows.values().find(|w| w.name == "Server_0").map(|w| w.handle) {
                wm.set_font_color(s0, [1.0, 1.0, 1.0, 1.0]);
            }
            if let Some(s1) = wm.windows.values().find(|w| w.name == "Server_1").map(|w| w.handle) {
                wm.set_font_color(s1, [0.75, 0.75, 0.75, 1.0]);
            }
        }
        "Server_1" => {
            println!("[UI] Zaznaczono Serwer 02: Test");
            if let Some(bar_h) = wm.windows.values().find(|w| w.name == "ServerSelectBar").map(|w| w.handle) {
                wm.set_position(bar_h, 14, 65);
            }
            if let Some(s0) = wm.windows.values().find(|w| w.name == "Server_0").map(|w| w.handle) {
                wm.set_font_color(s0, [0.75, 0.75, 0.75, 1.0]);
            }
            if let Some(s1) = wm.windows.values().find(|w| w.name == "Server_1").map(|w| w.handle) {
                wm.set_font_color(s1, [1.0, 1.0, 1.0, 1.0]);
            }
        }
        "Channel_0" => {
            println!("[UI] Zaznaczono Kanal 01: CH1 ...");
            if let Some(bar_h) = wm.windows.values().find(|w| w.name == "ChannelSelectBar").map(|w| w.handle) {
                wm.set_position(bar_h, 252, 42);
            }
            if let Some(c0) = wm.windows.values().find(|w| w.name == "Channel_0").map(|w| w.handle) {
                wm.set_font_color(c0, [1.0, 1.0, 1.0, 1.0]);
            }
            if let Some(c1) = wm.windows.values().find(|w| w.name == "Channel_1").map(|w| w.handle) {
                wm.set_font_color(c1, [0.75, 0.75, 0.75, 1.0]);
            }
        }
        "Channel_1" => {
            println!("[UI] Zaznaczono Kanal 02: CH2 ...");
            if let Some(bar_h) = wm.windows.values().find(|w| w.name == "ChannelSelectBar").map(|w| w.handle) {
                wm.set_position(bar_h, 252, 65);
            }
            if let Some(c0) = wm.windows.values().find(|w| w.name == "Channel_0").map(|w| w.handle) {
                wm.set_font_color(c0, [0.75, 0.75, 0.75, 1.0]);
            }
            if let Some(c1) = wm.windows.values().find(|w| w.name == "Channel_1").map(|w| w.handle) {
                wm.set_font_color(c1, [1.0, 1.0, 1.0, 1.0]);
            }
        }
        _ if name.starts_with("Flag_") => {
            let lang = &name[5..];
            println!("[UI] Wybrano jezyk interfejsu klienta: {}", lang);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_login_ui_setup() {
        m2_ui_setup_default_login_ui(1024, 768);
        let wm = get_window_manager();
        assert!(wm.windows.len() >= 25);
    }
}
