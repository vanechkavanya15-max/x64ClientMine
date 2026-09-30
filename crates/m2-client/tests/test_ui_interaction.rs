// tests/test_ui_interaction.rs - Test interakcji myszy, klawiatury i przelaczania UI Pure Rust
use m2_client::ui_window_mgr::get_window_manager;
use m2_client::ui_bridge_ffi::{
    m2_ui_setup_default_login_ui, handle_ui_click, handle_char_input, toggle_edit_focus, set_edit_focus,
};
use m2_client::win32_message_dispatcher::{WinMsg, process_single_message, WM_MOUSEMOVE, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_KEYDOWN, WM_CHAR};

#[test]
fn test_ui_mouse_and_keyboard_interaction() {
    // 1. Inicjalizacja interfejsu logowania
    m2_ui_setup_default_login_ui(1024, 768);

    // Sprawdz stan poczatkowy: ServerBoard widoczny, LoginBoard schowany
    {
        let wm = get_window_manager();
        let sb = wm.windows.values().find(|w| w.name == "ServerBoard").expect("ServerBoard must exist");
        let lb = wm.windows.values().find(|w| w.name == "LoginBoard").expect("LoginBoard must exist");
        assert!(sb.is_show, "ServerBoard must be visible initially");
        assert!(!lb.is_show, "LoginBoard must be hidden initially");
    }

    // 2. Symulacja klikniecia na ServerSelectButton (OK)
    // ServerBoard ma w=375, h=400, wycentrowany poziomo: gx = (1024 - 375)/2 = 324, gy = 768 - 400 - 72 = 296.
    // ServerSelectButton ma local_x = 267, local_y = 350, w = 88, h = 21.
    // Zatem ServerSelectButton lezy pod gx = 324 + 267 = 591..679, gy = 296 + 350 = 646..667.
    let click_x = 600;
    let click_y = 655;

    // Ruch myszy
    let msg_move = WinMsg {
        hwnd: 0,
        message: WM_MOUSEMOVE,
        wparam: 0,
        lparam: (click_x & 0xFFFF) as isize | (((click_y & 0xFFFF) as isize) << 16),
        time: 0,
        pt_x: click_x,
        pt_y: click_y,
    };
    process_single_message(&msg_move);

    // Wcisniecie lewego przycisku myszy
    let msg_down = WinMsg {
        hwnd: 0,
        message: WM_LBUTTONDOWN,
        wparam: 1,
        lparam: (click_x & 0xFFFF) as isize | (((click_y & 0xFFFF) as isize) << 16),
        time: 0,
        pt_x: click_x,
        pt_y: click_y,
    };
    process_single_message(&msg_down);

    // Puszczenie lewego przycisku myszy (wyzwolenie klikniecia)
    let msg_up = WinMsg {
        hwnd: 0,
        message: WM_LBUTTONUP,
        wparam: 0,
        lparam: (click_x & 0xFFFF) as isize | (((click_y & 0xFFFF) as isize) << 16),
        time: 0,
        pt_x: click_x,
        pt_y: click_y,
    };
    process_single_message(&msg_up);

    // Po kliknieciu OK, ServerBoard powinien byc ukryty, a LoginBoard pokazany!
    {
        let wm = get_window_manager();
        let sb = wm.windows.values().find(|w| w.name == "ServerBoard").unwrap();
        let lb = wm.windows.values().find(|w| w.name == "LoginBoard").unwrap();
        assert!(!sb.is_show, "ServerBoard should now be hidden");
        assert!(lb.is_show, "LoginBoard should now be visible");
    }

    // 3. Test wprowadzania tekstu z klawiatury (WM_CHAR) do ID
    set_edit_focus("IdEditLine");
    // Wpisz '1', '2', '3'
    for c in ['1', '2', '3'] {
        let msg_char = WinMsg {
            hwnd: 0,
            message: WM_CHAR,
            wparam: c as usize,
            lparam: 0,
            time: 0,
            pt_x: 0,
            pt_y: 0,
        };
        process_single_message(&msg_char);
    }

    {
        let wm = get_window_manager();
        let id_edit = wm.windows.values().find(|w| w.name == "IdEditLine").unwrap();
        assert!(id_edit.text.ends_with("123"), "ID text should have received typed chars");
    }

    // 4. Test przelaczenia TAB na haslo i wpisania znakow maskowanych gwiazdkami
    let msg_tab = WinMsg {
        hwnd: 0,
        message: WM_KEYDOWN,
        wparam: 0x09, // VK_TAB
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };
    process_single_message(&msg_tab);

    for c in ['a', 'b', 'c'] {
        let msg_char = WinMsg {
            hwnd: 0,
            message: WM_CHAR,
            wparam: c as usize,
            lparam: 0,
            time: 0,
            pt_x: 0,
            pt_y: 0,
        };
        process_single_message(&msg_char);
    }

    {
        let wm = get_window_manager();
        let pwd_edit = wm.windows.values().find(|w| w.name == "PwdEditLine").unwrap();
        assert!(pwd_edit.text.chars().all(|ch| ch == '*'), "Password characters must be masked with '*'");
    }

    // 5. Test powrotu do ServerBoard przez ESC
    let msg_esc = WinMsg {
        hwnd: 0,
        message: WM_KEYDOWN,
        wparam: 0x1B, // VK_ESCAPE
        lparam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };
    process_single_message(&msg_esc);

    {
        let wm = get_window_manager();
        let sb = wm.windows.values().find(|w| w.name == "ServerBoard").unwrap();
        let lb = wm.windows.values().find(|w| w.name == "LoginBoard").unwrap();
        assert!(sb.is_show, "ServerBoard should be visible again after ESC");
        assert!(!lb.is_show, "LoginBoard should be hidden after ESC");
    }

    println!("[+] Pelny test interakcji myszy i klawiatury UI Pure Rust: 100% PASS!");
}
