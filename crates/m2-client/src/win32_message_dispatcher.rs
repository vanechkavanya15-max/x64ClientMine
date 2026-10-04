use std::sync::Mutex;
use std::collections::VecDeque;

#[path = "input_mouse_lbutton_dispatcher.rs"]
pub mod input_mouse_lbutton_dispatcher;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WinMsg {
    pub hwnd: usize,
    pub message: u32,
    pub wparam: usize,
    pub lparam: isize,
    pub time: u32,
    pub pt_x: i32,
    pub pt_y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgQueueStatus {
    Continue,
    Quit(i32),
}

// Win32 constants
pub const WM_SETFOCUS: u32 = 0x0007;
pub const WM_KILLFOCUS: u32 = 0x0008;
pub const WM_PAINT: u32 = 0x000F;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_QUIT: u32 = 0x0012;
pub const WM_ACTIVATE: u32 = 0x0006;
pub const WM_KEYDOWN: u32 = 0x0100;
pub const WM_KEYUP: u32 = 0x0101;
pub const WM_CHAR: u32 = 0x0102;
pub const WM_MOUSEMOVE: u32 = 0x0200;
pub const WM_LBUTTONDOWN: u32 = 0x0201;
pub const WM_LBUTTONUP: u32 = 0x0202;
pub const WM_RBUTTONDOWN: u32 = 0x0204;
pub const WM_RBUTTONUP: u32 = 0x0205;
pub const WM_MOUSEWHEEL: u32 = 0x020A;

static IS_RMB_DOWN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static LAST_RMB_X: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static LAST_RMB_Y: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

pub const WA_INACTIVE: usize = 0;
pub const WA_ACTIVE: usize = 1;
pub const WA_CLICKACTIVE: usize = 2;

struct DispatcherState {
    message_queue: VecDeque<WinMsg>,
    is_active: bool,
    is_minimized: bool,
    has_focus: bool,
    quit_requested: Option<i32>,
}

impl DispatcherState {
    const fn new() -> Self {
        Self {
            message_queue: VecDeque::new(),
            is_active: true,
            is_minimized: false,
            has_focus: true,
            quit_requested: None,
        }
    }
}

static DISPATCHER_STATE: Mutex<DispatcherState> = Mutex::new(DispatcherState::new());

pub fn poll_window_messages() -> MsgQueueStatus {
    #[cfg(windows)]
    {
        #[repr(C)]
        struct MSG {
            hwnd: *mut std::ffi::c_void,
            message: u32,
            wparam: usize,
            lparam: isize,
            time: u32,
            pt_x: i32,
            pt_y: i32,
        }

        #[link(name = "user32")]
        extern "system" {
            fn PeekMessageA(
                lpMsg: *mut MSG,
                hWnd: *mut std::ffi::c_void,
                wMsgFilterMin: u32,
                wMsgFilterMax: u32,
                wRemoveMsg: u32,
            ) -> i32;
            fn TranslateMessage(lpMsg: *const MSG) -> i32;
            fn DispatchMessageA(lpMsg: *const MSG) -> isize;
        }

        const PM_REMOVE: u32 = 0x0001;

        let mut msg = MSG {
            hwnd: std::ptr::null_mut(),
            message: 0,
            wparam: 0,
            lparam: 0,
            time: 0,
            pt_x: 0,
            pt_y: 0,
        };

        while unsafe { PeekMessageA(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) } != 0 {
            if msg.message == WM_QUIT {
                post_quit_message(msg.wparam as i32);
                return MsgQueueStatus::Quit(msg.wparam as i32);
            }

            let win_msg = WinMsg {
                hwnd: msg.hwnd as usize,
                message: msg.message,
                wparam: msg.wparam,
                lparam: msg.lparam,
                time: msg.time,
                pt_x: msg.pt_x,
                pt_y: msg.pt_y,
            };

            if let Ok(mut state) = DISPATCHER_STATE.lock() {
                process_single_message_internal(&mut state, &win_msg);
            }

            unsafe {
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
        }
    }

    let mut state = DISPATCHER_STATE.lock().unwrap();

    if let Some(exit_code) = state.quit_requested {
        return MsgQueueStatus::Quit(exit_code);
    }

    // Processing queued messages
    while let Some(msg) = state.message_queue.pop_front() {
        process_single_message_internal(&mut state, &msg);
        if let Some(exit_code) = state.quit_requested {
            return MsgQueueStatus::Quit(exit_code);
        }
    }

    MsgQueueStatus::Continue
}

pub fn process_single_message(msg: &WinMsg) -> bool {
    let mut state = DISPATCHER_STATE.lock().unwrap();
    process_single_message_internal(&mut state, msg)
}

fn process_single_message_internal(state: &mut DispatcherState, msg: &WinMsg) -> bool {
    match msg.message {
        WM_CLOSE => {
            state.quit_requested = Some(0);
            true
        }
        WM_QUIT => {
            state.quit_requested = Some(msg.wparam as i32);
            true
        }
        WM_ACTIVATE => {
            let state_flag = msg.wparam & 0xFFFF;
            let minimized = (msg.wparam >> 16) != 0;
            
            state.is_minimized = minimized;
            state.is_active = state_flag != WA_INACTIVE && !minimized;
            true
        }
        WM_PAINT => {
            // Paint logic would go here. Usually involves BeginPaint/EndPaint
            true
        }
        WM_SETFOCUS => {
            state.has_focus = true;
            true
        }
        WM_KILLFOCUS => {
            state.has_focus = false;
            // Clean up keyboard state on focus lost is handled externally via input_focus_lost_cleanup::process_focus_lost_cleanup
            true
        }
        WM_MOUSEMOVE => {
            let mx = (msg.lparam & 0xFFFF) as i16 as i32;
            let my = ((msg.lparam >> 16) & 0xFFFF) as i16 as i32;
            if IS_RMB_DOWN.load(std::sync::atomic::Ordering::SeqCst) {
                let prev_x = LAST_RMB_X.swap(mx, std::sync::atomic::Ordering::SeqCst);
                let prev_y = LAST_RMB_Y.swap(my, std::sync::atomic::Ordering::SeqCst);
                let dx = mx - prev_x;
                let dy = my - prev_y;
                crate::py_bridge_app::m2_app_rotate_camera_delta(dx as f32 * 0.4, -(dy as f32) * 0.25);
            }
            let mut wm = crate::ui_window_mgr::get_window_manager();
            wm.handle_mouse_move(mx, my, 1024, 768);
            true
        }
        WM_LBUTTONDOWN => {
            let mx = (msg.lparam & 0xFFFF) as i16 as i32;
            let my = ((msg.lparam >> 16) & 0xFFFF) as i16 as i32;
            let hit = input_mouse_lbutton_dispatcher::dispatch_wm_lbutton_down(mx, my, 1024, 768);
            if hit.is_none() {
                handle_game_world_click(mx, my);
            }
            true
        }
        WM_LBUTTONUP => {
            let mx = (msg.lparam & 0xFFFF) as i16 as i32;
            let my = ((msg.lparam >> 16) & 0xFFFF) as i16 as i32;
            let (hit, name) = input_mouse_lbutton_dispatcher::dispatch_wm_lbutton_up(mx, my, 1024, 768);
            if let Some(h) = hit {
                let mut wm = crate::ui_window_mgr::get_window_manager();
                crate::ui_bridge_ffi::handle_ui_click(&name, h, &mut wm);
            }
            true
        }
        WM_KEYDOWN => {
            let vk = msg.wparam as u32;
            {
                let mut wm = crate::ui_window_mgr::get_window_manager();
                wm.handle_key_down(vk);
            }
            let phase = crate::ui_bridge_ffi::CURRENT_PHASE.lock().unwrap().clone();
            match vk {
                0x1B => { // VK_ESCAPE
                    let mut wm = crate::ui_window_mgr::get_window_manager();
                    wm.handle_escape();
                    if phase == "GAME" {
                        crate::ui_bridge_ffi::handle_ui_click("HudBtnSystem", 0, &mut wm);
                    } else if phase == "SELECT" {
                        crate::ui_bridge_ffi::handle_ui_click("SelectExitButton", 0, &mut wm);
                    } else {
                        let is_lb_vis = wm.windows.values().find(|w| w.name == "LoginBoard").map(|w| w.is_show).unwrap_or(false);
                        if is_lb_vis {
                            crate::ui_bridge_ffi::handle_ui_click("LoginExitButton", 0, &mut wm);
                        } else {
                            crate::ui_bridge_ffi::handle_ui_click("ServerExitButton", 0, &mut wm);
                        }
                    }
                }
                0x0D => { // VK_RETURN
                    let mut wm = crate::ui_window_mgr::get_window_manager();
                    wm.handle_return();
                    if phase == "SELECT" {
                        crate::ui_bridge_ffi::handle_ui_click("SelectStartButton", 0, &mut wm);
                    } else if phase == "GAME" {
                        println!("[CHAT] Otwieranie pola wprowadzania wiadomosci na czacie...");
                    } else {
                        let is_lb_vis = wm.windows.values().find(|w| w.name == "LoginBoard").map(|w| w.is_show).unwrap_or(false);
                        if is_lb_vis {
                            crate::ui_bridge_ffi::handle_ui_click("LoginButton", 0, &mut wm);
                        } else {
                            crate::ui_bridge_ffi::handle_ui_click("ServerSelectButton", 0, &mut wm);
                        }
                    }
                }
                0x09 => { // VK_TAB
                    let mut wm = crate::ui_window_mgr::get_window_manager();
                    wm.handle_tab();
                    crate::ui_bridge_ffi::toggle_edit_focus();
                }
                0x49 => { // 'I' - Ekwipunek
                    if phase == "GAME" {
                        let mut wm = crate::ui_window_mgr::get_window_manager();
                        crate::ui_bridge_ffi::handle_ui_click("HudBtnInventory", 0, &mut wm);
                    }
                }
                0x43 => { // 'C' - Status postaci
                    if phase == "GAME" {
                        let mut wm = crate::ui_window_mgr::get_window_manager();
                        crate::ui_bridge_ffi::handle_ui_click("HudBtnCharacter", 0, &mut wm);
                    }
                }
                0x56 => { // 'V' - Skille
                    if phase == "GAME" {
                        let mut wm = crate::ui_window_mgr::get_window_manager();
                        crate::ui_bridge_ffi::handle_ui_click("HudBtnSkills", 0, &mut wm);
                    }
                }
                0x20 => { // VK_SPACE - Atak wrecz mieczem
                    if phase == "GAME" {
                        let (px, py, pz, p_vid) = if let Ok(state) = m2_client_bridge::state::game_state::GLOBAL_GAME_STATE.read() {
                            (state.player.x, state.player.y, state.player.z, state.player.vid)
                        } else {
                            (0.0, 0.0, 0.0, 0)
                        };
                        let player_pos_vec = glam::Vec3::new(px, py, pz);

                        // Znajdz najblizszego wroga w promieniu 350 jednostek
                        let mut target_vid = 0u32;
                        let mut min_dist = 350.0f32;
                        if let Ok(state) = m2_client_bridge::state::game_state::GLOBAL_GAME_STATE.read() {
                            for (&vid, actor) in &state.actors {
                                if vid == p_vid { continue; }
                                let act_pos = glam::Vec3::new(actor.x, actor.y, actor.z);
                                let d = (act_pos - player_pos_vec).length();
                                if d < min_dist {
                                    min_dist = d;
                                    target_vid = vid;
                                }
                            }
                        }

                        // Wyslij pakiet HEADER_CG_ATTACK (0x02, 6 bajtow)
                        let mut atk_pkt = [0u8; 6];
                        atk_pkt[0] = 0x02; // HEADER_CG_ATTACK
                        atk_pkt[1] = 0;    // attack_type
                        atk_pkt[2..6].copy_from_slice(&target_vid.to_le_bytes());

                        if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
                            if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
                                let mut encrypted = atk_pkt.to_vec();
                                cipher.encrypt_in_place(&mut encrypted);
                                if std::io::Write::write_all(stream, &encrypted).is_ok() {
                                    println!("[COMBAT] [SPACJA] Atak mieczem na VID={} (dist={:.1})", target_vid, min_dist);
                                }
                            }
                        }
                    }
                }
                0x57 | 0x26 => { // 'W' lub Strzalka w gore - bieg w przod
                    if phase == "GAME" {
                        handle_player_keyboard_step(150.0, 0.0);
                    }
                }
                0x53 | 0x28 => { // 'S' lub Strzalka w dol - krok w tyl
                    if phase == "GAME" {
                        handle_player_keyboard_step(-100.0, 0.0);
                    }
                }
                0x41 | 0x25 => { // 'A' lub Strzalka w lewo - obrot postaci w lewo
                    if phase == "GAME" {
                        handle_player_keyboard_rotate(-15.0);
                    }
                }
                0x44 | 0x27 => { // 'D' lub Strzalka w prawo - obrot postaci w prawo
                    if phase == "GAME" {
                        handle_player_keyboard_rotate(15.0);
                    }
                }
                0x51 => { // 'Q' - obrot kamery w lewo
                    if phase == "GAME" {
                        crate::py_bridge_app::m2_app_rotate_camera_delta(-10.0, 0.0);
                    }
                }
                0x45 => { // 'E' - obrot kamery w prawo
                    if phase == "GAME" {
                        crate::py_bridge_app::m2_app_rotate_camera_delta(10.0, 0.0);
                    }
                }
                0x31..=0x34 => { // '1' - '4' - Pasek QuickSlot (potki HP/SP/speed)
                    if phase == "GAME" {
                        let slot = vk - 0x31;
                        let mut wm = crate::ui_window_mgr::get_window_manager();
                        let slot_name = format!("QuickSlot_{}", slot + 1);
                        crate::ui_bridge_ffi::handle_ui_click(&slot_name, 0, &mut wm);
                    }
                }
                0x70..=0x73 => { // F1 - F4 - Pasek QuickSlot skilli
                    if phase == "GAME" {
                        let slot = vk - 0x70 + 4;
                        let mut wm = crate::ui_window_mgr::get_window_manager();
                        let slot_name = format!("QuickSlot_{}", slot + 1);
                        crate::ui_bridge_ffi::handle_ui_click(&slot_name, 0, &mut wm);
                    }
                }
                0x4E => { // 'N' - Zadania / Quests
                    if phase == "GAME" {
                        let mut wm = crate::ui_window_mgr::get_window_manager();
                        crate::ui_bridge_ffi::handle_ui_click("HudBtnQuests", 0, &mut wm);
                    }
                }
                _ => {}
            }
            true
        }
        WM_KEYUP => {
            let vk = msg.wparam as u32;
            let mut wm = crate::ui_window_mgr::get_window_manager();
            wm.handle_key_up(vk);
            true
        }
        WM_RBUTTONDOWN => {
            let mx = (msg.lparam & 0xFFFF) as i16 as i32;
            let my = ((msg.lparam >> 16) & 0xFFFF) as i16 as i32;
            IS_RMB_DOWN.store(true, std::sync::atomic::Ordering::SeqCst);
            LAST_RMB_X.store(mx, std::sync::atomic::Ordering::SeqCst);
            LAST_RMB_Y.store(my, std::sync::atomic::Ordering::SeqCst);
            let mut wm = crate::ui_window_mgr::get_window_manager();
            wm.handle_mouse_right_down(mx, my, 1024, 768);
            true
        }
        WM_RBUTTONUP => {
            let mx = (msg.lparam & 0xFFFF) as i16 as i32;
            let my = ((msg.lparam >> 16) & 0xFFFF) as i16 as i32;
            IS_RMB_DOWN.store(false, std::sync::atomic::Ordering::SeqCst);
            let mut wm = crate::ui_window_mgr::get_window_manager();
            wm.handle_mouse_right_up(mx, my, 1024, 768);
            true
        }
        WM_MOUSEWHEEL => {
            let delta = (msg.wparam >> 16) as i16 as f32;
            crate::py_bridge_app::m2_app_zoom_camera(-delta * 0.5);
            true
        }
        WM_CHAR => {
            let ch = (msg.wparam & 0xFFFF) as u32;
            if let Some(c) = char::from_u32(ch) {
                let mut wm = crate::ui_window_mgr::get_window_manager();
                wm.handle_char(c);
                crate::ui_bridge_ffi::handle_char_input(c);
            }
            true
        }
        _ => false,
    }
}

/// Obsluga klikniecia w swiat gry (wyznaczanie punktu na plaszczyznie terenu i wyslanie ruchu do serwera)
pub fn handle_game_world_click(mx: i32, my: i32) {
    let current_phase = crate::ui_bridge_ffi::CURRENT_PHASE.lock().map(|p| p.clone()).unwrap_or_default();
    if current_phase != "GAME" {
        return;
    }

    // 1. Pobieramy aktualne wspolrzedne gracza
    let (player_pos, player_rot) = {
        if let Ok(state) = m2_client_bridge::state::game_state::GLOBAL_GAME_STATE.read() {
            ([state.player.x, state.player.y, state.player.z], state.player.rot)
        } else {
            ([0.0, 0.0, 0.0], 0.0)
        }
    };

    // 2. Pobieramy dynamiczne parametry kamery TPP
    let (cam_dist, cam_pitch, cam_rot, target_height) = if let Ok(app) = crate::py_bridge_app::app_lock() {
        (app.camera.distance, app.camera.pitch, app.camera.rotation, app.camera.target_height)
    } else {
        (650.0, 30.0, 0.0, 100.0)
    };

    // 3. Obliczamy macierze widoku i projekcji
    let aspect = 1024.0 / 768.0;
    let proj = glam::Mat4::perspective_rh(50.0f32.to_radians(), aspect, 10.0, 15000.0);

    let pitch_rad = cam_pitch.to_radians();
    let rot_rad = (player_rot + cam_rot).to_radians();
    let eye = glam::Vec3::new(
        player_pos[0] - cam_dist * pitch_rad.cos() * rot_rad.sin(),
        player_pos[1] - cam_dist * pitch_rad.cos() * rot_rad.cos(),
        player_pos[2] + target_height + cam_dist * pitch_rad.sin(),
    );
    let target = glam::Vec3::new(player_pos[0], player_pos[1], player_pos[2] + target_height);
    let up = glam::Vec3::new(0.0, 0.0, 1.0);
    let view = glam::Mat4::look_at_rh(eye, target, up);

    let inv_vp = (proj * view).inverse();
    let ndc_x = (2.0 * mx as f32 / 1024.0) - 1.0;
    let ndc_y = 1.0 - (2.0 * my as f32 / 768.0);

    let p_near = inv_vp.project_point3(glam::Vec3::new(ndc_x, ndc_y, 0.0));
    let p_far = inv_vp.project_point3(glam::Vec3::new(ndc_x, ndc_y, 1.0));
    let dir = (p_far - p_near).normalize();

    // 3.5. Sprawdzenie czy klikniecie trafilo w aktora (potwor, gracz, NPC)
    let mut hit_actor: Option<(u32, bool)> = None;
    let mut closest_hit_t = f32::MAX;

    if let Ok(state) = m2_client_bridge::state::game_state::GLOBAL_GAME_STATE.read() {
        for (&vid, actor) in &state.actors {
            if vid == state.player.vid { continue; }
            let actor_center = glam::Vec3::new(actor.x, actor.y, actor.z + 90.0);
            let v = actor_center - p_near;
            let t = v.dot(dir);
            if t > 0.0 && t < closest_hit_t {
                let d_sq = v.length_squared() - t * t;
                let radius = 100.0f32;
                if d_sq <= radius * radius {
                    closest_hit_t = t;
                    let is_enemy = actor.b_type == 0 || actor.b_type == 1 || actor.race >= 101;
                    hit_actor = Some((vid, is_enemy));
                }
            }
        }
    }

    if let Some((target_vid, is_enemy)) = hit_actor {
        println!("[TARGET] [+] Kliknieto aktora VID={} (wrog={})", target_vid, is_enemy);
        if is_enemy {
            let mut atk_pkt = [0u8; 6];
            atk_pkt[0] = 0x02; // HEADER_CG_ATTACK
            atk_pkt[1] = 0;    // attack_type
            atk_pkt[2..6].copy_from_slice(&target_vid.to_le_bytes());

            if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
                if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
                    let mut encrypted = atk_pkt.to_vec();
                    cipher.encrypt_in_place(&mut encrypted);
                    let _ = std::io::Write::write_all(stream, &encrypted);
                }
            }
        }
        return;
    }

    // 4. Przeciecie promienia z plaszczyzna terenu Z = player_pos[2]
    let (target_x, target_y) = if dir.z.abs() > 1e-4 {
        let t = (player_pos[2] - p_near.z) / dir.z;
        if t > 0.0 {
            (p_near.x + t * dir.x, p_near.y + t * dir.y)
        } else {
            (player_pos[0], player_pos[1])
        }
    } else {
        (player_pos[0], player_pos[1])
    };

    let dx = target_x - player_pos[0];
    let dy = target_y - player_pos[1];
    let rot_deg = (dy.atan2(dx).to_degrees() + 90.0).rem_euclid(360.0);

    // 5. Konstrukcja i wyslanie 16-bajtowego pakietu HEADER_CG_CHARACTER_MOVE (0x07)
    let mut move_pkt = [0u8; 16];
    move_pkt[0] = 0x07; // HEADER_CG_CHARACTER_MOVE
    move_pkt[1] = 0;    // bFunc = FUNC_MOVE
    move_pkt[2] = 0;    // bArg
    move_pkt[3] = (rot_deg / 5.0) as u8; // bRot
    move_pkt[4..8].copy_from_slice(&(target_x as i32).to_le_bytes());
    move_pkt[8..12].copy_from_slice(&(target_y as i32).to_le_bytes());
    let dw_time = crate::app_timing_clock::m2_get_time() as u32;
    move_pkt[12..16].copy_from_slice(&dw_time.to_le_bytes());

    if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
        if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
            let mut encrypted = move_pkt.to_vec();
            cipher.encrypt_in_place(&mut encrypted);
            if std::io::Write::write_all(stream, &encrypted).is_ok() {
                println!("[MOVE] [+] Wyslano HEADER_CG_CHARACTER_MOVE do ({:.0}, {:.0}) rot={:.1} deg", target_x, target_y, rot_deg);
            }
        }
    }

    // 6. Aktualizacja stanu gracza
    if let Ok(mut state) = m2_client_bridge::state::game_state::GLOBAL_GAME_STATE.write() {
        state.player.rot = rot_deg;
    }
}

/// Obsluga plynnego przesuniecia postaci klawiatura (WASD / strzalki)
pub fn handle_player_keyboard_step(forward_step: f32, side_step: f32) {
    let (mut px, mut py, pz, rot) = {
        if let Ok(state) = m2_client_bridge::state::game_state::GLOBAL_GAME_STATE.read() {
            (state.player.x, state.player.y, state.player.z, state.player.rot)
        } else {
            return;
        }
    };

    let rad = rot.to_radians();
    px += rad.sin() * forward_step + rad.cos() * side_step;
    py += rad.cos() * forward_step - rad.sin() * side_step;

    if let Ok(mut state) = m2_client_bridge::state::game_state::GLOBAL_GAME_STATE.write() {
        state.player.x = px;
        state.player.y = py;
    }

    // Wyslij pakiet HEADER_CG_CHARACTER_MOVE (0x03, 16 bajtow) do serwera
    let mut move_pkt = [0u8; 16];
    move_pkt[0] = 0x03; // HEADER_CG_CHARACTER_MOVE
    move_pkt[1] = 0;    // bFunc = FUNC_MOVE
    move_pkt[2] = 0;    // bArg
    move_pkt[3] = (rot / 5.0) as u8; // bRot
    move_pkt[4..8].copy_from_slice(&(px as i32).to_le_bytes());
    move_pkt[8..12].copy_from_slice(&(py as i32).to_le_bytes());
    let dw_time = crate::app_timing_clock::m2_get_time() as u32;
    move_pkt[12..16].copy_from_slice(&dw_time.to_le_bytes());

    if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
        if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
            let mut encrypted = move_pkt.to_vec();
            cipher.encrypt_in_place(&mut encrypted);
            let _ = std::io::Write::write_all(stream, &encrypted);
        }
    }
    println!("[MOVE] [KLAWIATURA] Gracz: X={:.1}, Y={:.1}, Z={:.1}, Rot={:.1}°", px, py, pz, rot);
}

/// Obsluga plynnego obrotu postaci klawiatura (A / D / strzalki lewo / prawo)
pub fn handle_player_keyboard_rotate(delta_deg: f32) {
    if let Ok(mut state) = m2_client_bridge::state::game_state::GLOBAL_GAME_STATE.write() {
        state.player.rot = (state.player.rot + delta_deg).rem_euclid(360.0);
        println!("[MOVE] [OBROT] Rot={:.1}°", state.player.rot);
    }
}

pub fn post_quit_message(exit_code: i32) {
    let mut state = DISPATCHER_STATE.lock().unwrap();
    state.quit_requested = Some(exit_code);
}

// Function to push messages to the queue without blocking the main game thread
pub fn enqueue_message(msg: WinMsg) {
    if let Ok(mut state) = DISPATCHER_STATE.lock() {
        state.message_queue.push_back(msg);
    }
}

pub fn is_window_active() -> bool {
    if let Ok(state) = DISPATCHER_STATE.lock() {
        state.is_active
    } else {
        false
    }
}

pub fn is_window_minimized() -> bool {
    if let Ok(state) = DISPATCHER_STATE.lock() {
        state.is_minimized
    } else {
        false
    }
}

pub fn has_window_focus() -> bool {
    if let Ok(state) = DISPATCHER_STATE.lock() {
        state.has_focus
    } else {
        false
    }
}

pub fn reset_dispatcher_state() {
    if let Ok(mut state) = DISPATCHER_STATE.lock() {
        state.message_queue.clear();
        state.is_active = true;
        state.is_minimized = false;
        state.has_focus = true;
        state.quit_requested = None;
    }
}
