use std::io::{Read, Write};
use std::net::TcpStream;
use crate::secure_cipher::SecureCipher;
use crate::ui_window_core::{WindowType, HORIZONTAL_ALIGN_CENTER};
use crate::ui_window_mgr::WindowManager;

// Constants
pub const HEADER_CG_CHARACTER_DELETE: u16 = 0x0202;
pub const HEADER_GC_CHARACTER_DELETE_SUCCESS: u16 = 0x020E;
pub const HEADER_GC_CHARACTER_DELETE_WRONG_SOCIAL_ID: u16 = 0x020F;

pub const PRIVATE_CODE_LENGTH: usize = 8;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGPlayerDelete {
    pub header: u16,
    pub length: u16,
    pub index: u8,
    pub private_code: [u8; PRIVATE_CODE_LENGTH],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCPlayerDeleteSuccess {
    pub header: u16,
    pub length: u16,
    pub account_index: u8, // C++ index / account_index
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCPlayerDeleteWrongSocialId {
    pub header: u16,
    pub length: u16,
}

impl TPacketCGPlayerDelete {
    pub fn new(index: u8, pin_code: &str) -> Self {
        let mut private_code = [0u8; PRIVATE_CODE_LENGTH];
        let bytes = pin_code.as_bytes();
        let len = std::cmp::min(bytes.len(), PRIVATE_CODE_LENGTH - 1);
        private_code[..len].copy_from_slice(&bytes[..len]);
        
        Self {
            header: HEADER_CG_CHARACTER_DELETE,
            length: std::mem::size_of::<Self>() as u16,
            index,
            private_code,
        }
    }
}

pub fn send_character_delete_packet(
    stream: &mut TcpStream,
    cipher: &mut SecureCipher,
    slot_index: u8,
    pin_code: &str,
) -> std::io::Result<()> {
    if pin_code.len() != 7 {
        println!("[UI] Blad: Kod usuniecia postaci musi miec dokladnie 7 znakow.");
        return Ok(());
    }

    let packet = TPacketCGPlayerDelete::new(slot_index, pin_code);
    let packet_size = std::mem::size_of::<TPacketCGPlayerDelete>();
    
    let buf = unsafe {
        std::slice::from_raw_parts(
            (&packet as *const TPacketCGPlayerDelete) as *const u8,
            packet_size,
        )
    };

    let mut encrypted_buf = buf.to_vec();
    cipher.encrypt_in_place(&mut encrypted_buf);

    stream.write_all(&encrypted_buf)?;
    stream.flush()?;
    println!("[NETWORK] Wyslano pakiet usuniecia postaci dla slotu {}", slot_index);

    Ok(())
}

pub fn process_character_delete_response(header: u16, payload: &[u8]) -> std::io::Result<()> {
    match header {
        HEADER_GC_CHARACTER_DELETE_SUCCESS => {
            if payload.len() < std::mem::size_of::<TPacketGCPlayerDeleteSuccess>() {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid success packet size"));
            }
            let success_packet: &TPacketGCPlayerDeleteSuccess = unsafe {
                &*(payload.as_ptr() as *const TPacketGCPlayerDeleteSuccess)
            };
            
            println!("[NETWORK] Odebrano sukces usuniecia postaci ze slotu {}", success_packet.account_index);
            
            let mut chars = crate::ui_bridge_ffi::GLOBAL_CHARACTERS.lock().unwrap();
            let mut selected_slot = crate::ui_bridge_ffi::SELECTED_CHAR_SLOT.lock().unwrap();
            
            let index = success_packet.account_index as usize;
            if index < chars.len() {
                chars[index] = crate::ui_bridge_ffi::CharacterSlot::default();
            }
            if *selected_slot == index {
                *selected_slot = 0;
            }
            Ok(())
        }
        HEADER_GC_CHARACTER_DELETE_WRONG_SOCIAL_ID => {
            println!("[NETWORK] Blad usuniecia postaci: Bledny kod social ID / PIN");
            Ok(())
        }
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Unknown delete response header",
        )),
    }
}

// ==========================================
// Prawdziwa Implementacja UI Usuwania Postaci
// ==========================================

pub static DELETE_MODAL_SLOT: std::sync::Mutex<u8> = std::sync::Mutex::new(0);
pub static DELETE_MODAL_PIN: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
pub static DELETE_MODAL_HANDLE: std::sync::Mutex<Option<u32>> = std::sync::Mutex::new(None);
pub static DELETE_MODAL_EDITLINE_HANDLE: std::sync::Mutex<Option<u32>> = std::sync::Mutex::new(None);

pub fn create_character_delete_modal(wm: &mut WindowManager, sw: u32, sh: u32) {
    let board_w = 260;
    let board_h = 130;
    let modal_board = wm.register(WindowType::Box);
    wm.set_name(modal_board, "DeleteCharacterModal");
    wm.set_position(modal_board, (sw as i32 - board_w) / 2, (sh as i32 - board_h) / 2);
    wm.set_size(modal_board, board_w, board_h);
    wm.set_color(modal_board, [0.0, 0.0, 0.0, 0.9]); // Black transparent background
    
    // Title
    let title = wm.register(WindowType::TextLine);
    wm.set_parent(title, modal_board);
    wm.set_position(title, 0, 10);
    wm.set_text(title, "Podaj 7-cyfrowy kod PIN do usuniecia postaci:");
    wm.set_align(title, HORIZONTAL_ALIGN_CENTER, crate::ui_window_core::VERTICAL_ALIGN_TOP);
    
    // EditLine Background Box
    let edit_bg = wm.register(WindowType::Box);
    wm.set_parent(edit_bg, modal_board);
    wm.set_position(edit_bg, (board_w - 100) / 2, 40);
    wm.set_size(edit_bg, 100, 20);
    wm.set_color(edit_bg, [0.1, 0.1, 0.1, 1.0]);
    
    // EditLine Text (The actual pin text)
    let edit_text = wm.register(WindowType::TextLine);
    wm.set_parent(edit_text, edit_bg);
    wm.set_position(edit_text, 0, 2);
    wm.set_text(edit_text, "");
    wm.set_align(edit_text, HORIZONTAL_ALIGN_CENTER, crate::ui_window_core::VERTICAL_ALIGN_TOP);
    wm.set_name(edit_text, "DeleteCharacterPinText");
    
    // Confirm Button
    let btn_ok = wm.register(WindowType::Button);
    wm.set_parent(btn_ok, modal_board);
    wm.set_position(btn_ok, 30, 80);
    wm.set_size(btn_ok, 80, 25);
    wm.set_name(btn_ok, "DeleteCharacterConfirmButton");
    wm.set_button_visuals(
        btn_ok,
        "d:/ymir work/ui/public/middle_button_01.sub",
        "d:/ymir work/ui/public/middle_button_02.sub",
        "d:/ymir work/ui/public/middle_button_03.sub",
    );
    wm.set_text(btn_ok, "Usun");
    
    // Cancel Button
    let btn_cancel = wm.register(WindowType::Button);
    wm.set_parent(btn_cancel, modal_board);
    wm.set_position(btn_cancel, board_w - 30 - 80, 80);
    wm.set_size(btn_cancel, 80, 25);
    wm.set_name(btn_cancel, "DeleteCharacterCancelButton");
    wm.set_button_visuals(
        btn_cancel,
        "d:/ymir work/ui/public/middle_button_01.sub",
        "d:/ymir work/ui/public/middle_button_02.sub",
        "d:/ymir work/ui/public/middle_button_03.sub",
    );
    wm.set_text(btn_cancel, "Anuluj");

    // Hide initially
    wm.hide(modal_board);
    
    *DELETE_MODAL_HANDLE.lock().unwrap() = Some(modal_board);
    *DELETE_MODAL_EDITLINE_HANDLE.lock().unwrap() = Some(edit_text);
}

pub fn open_character_delete_modal(wm: &mut WindowManager, slot_index: u8) {
    if let Some(modal_handle) = *DELETE_MODAL_HANDLE.lock().unwrap() {
        wm.show(modal_handle);
        *DELETE_MODAL_SLOT.lock().unwrap() = slot_index;
        DELETE_MODAL_PIN.lock().unwrap().clear();
        
        if let Some(edit_handle) = *DELETE_MODAL_EDITLINE_HANDLE.lock().unwrap() {
            wm.set_text(edit_handle, "");
        }
        
        println!("[UI] Otwarto modal usuwania postaci dla slotu {}", slot_index);
    }
}

pub fn close_character_delete_modal(wm: &mut WindowManager) {
    if let Some(modal_handle) = *DELETE_MODAL_HANDLE.lock().unwrap() {
        wm.hide(modal_handle);
        DELETE_MODAL_PIN.lock().unwrap().clear();
    }
}

pub fn handle_pin_input(wm: &mut WindowManager, char_code: char) {
    let modal_handle = *DELETE_MODAL_HANDLE.lock().unwrap();
    if modal_handle.is_none() { return; }
    let modal_handle = modal_handle.unwrap();
    
    // Check if window is visible (we use basic assumption for now)
    let is_visible = wm.windows.get(&modal_handle).map_or(false, |w| w.is_show);
    if !is_visible { return; }
    
    let mut pin = DELETE_MODAL_PIN.lock().unwrap();
    if char_code == '\x08' {
        // Backspace
        pin.pop();
    } else if char_code.is_digit(10) && pin.len() < 7 {
        pin.push(char_code);
    }
    
    if let Some(edit_handle) = *DELETE_MODAL_EDITLINE_HANDLE.lock().unwrap() {
        wm.set_text(edit_handle, &pin);
    }
}

pub fn handle_delete_modal_click(name: &str, wm: &mut WindowManager) -> bool {
    match name {
        "DeleteCharacterConfirmButton" => {
            let pin = DELETE_MODAL_PIN.lock().unwrap().clone();
            let slot = *DELETE_MODAL_SLOT.lock().unwrap();
            
            if pin.len() != 7 {
                println!("[UI] Blad: Kod usuniecia postaci musi miec dokladnie 7 znakow.");
                return true;
            }
            
            // Wyslanie pakietu (wymaga contextu sieci)
            let mut sock_guard = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock().unwrap();
            if let Some((stream, cipher)) = sock_guard.as_mut() {
                if let Err(e) = send_character_delete_packet(stream, cipher, slot, &pin) {
                    println!("[UI] Blad wysylania pakietu usuwania postaci: {}", e);
                }
            } else {
                println!("[UI] Blad: Brak aktywnego polaczenia z serwerem gry.");
            }
            
            close_character_delete_modal(wm);
            true
        }
        "DeleteCharacterCancelButton" => {
            close_character_delete_modal(wm);
            true
        }
        _ => false,
    }
}
