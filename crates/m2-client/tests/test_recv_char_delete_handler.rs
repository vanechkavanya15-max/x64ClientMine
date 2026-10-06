#[path = "../src/recv_char_delete_handler.rs"]
mod recv_char_delete_handler;

use recv_char_delete_handler::{TPacketGCCharacterDelete, parse_character_delete};
use std::mem::size_of;

// Mocks to bypass the unresolved imports when testing locally as a standalone module
mod crate_mock {
    pub mod actor_spawn_v2 {
        use std::sync::RwLock;
        pub struct NetworkActorMgr {}
        impl NetworkActorMgr {
            pub fn remove_actor(&mut self, _vid: u32) {}
        }
        pub fn get_network_actor_mgr() -> &'static RwLock<NetworkActorMgr> {
            static MGR: std::sync::OnceLock<RwLock<NetworkActorMgr>> = std::sync::OnceLock::new();
            MGR.get_or_init(|| RwLock::new(NetworkActorMgr {}))
        }
    }
    
    pub mod actor_picking_v2 {
        pub fn remove_picking_actor_v2(_vid: u32) {}
    }
    
    pub mod py_bridge_texttail_runtime_complete {
        use std::sync::Mutex;
        use std::collections::HashMap;
        pub struct TextTailState {
            pub character_text_tail_map: HashMap<u32, ()>,
            pub character_text_tail_list: Vec<u32>,
        }
        pub fn get_text_tail_state() -> &'static Mutex<TextTailState> {
            static STATE: std::sync::OnceLock<Mutex<TextTailState>> = std::sync::OnceLock::new();
            STATE.get_or_init(|| Mutex::new(TextTailState {
                character_text_tail_map: HashMap::new(),
                character_text_tail_list: Vec::new(),
            }))
        }
    }
    
    pub mod py_bridge_player_target {
        use std::sync::Mutex;
        pub struct PlayerTargetState {
            pub target_vid: u32,
        }
        pub fn get_player_target_state() -> &'static Mutex<PlayerTargetState> {
            static STATE: std::sync::OnceLock<Mutex<PlayerTargetState>> = std::sync::OnceLock::new();
            STATE.get_or_init(|| Mutex::new(PlayerTargetState { target_vid: 0 }))
        }
        pub unsafe fn m2_py_player_clear_target_vid() {}
    }
    
    pub mod py_bridge_net {
        pub const PHASE_WINDOW_GAME: usize = 5;
        pub fn get_phase_window(_phase: usize) -> usize {
            1 // mock handle
        }
    }
    
    pub mod py_event_dispatcher {
        pub fn call_python_object_method1_i32(_wnd: usize, _method: &str, _arg: i32) {}
    }
}

#[test]
fn test_parse_character_delete_success() {
    let packet = TPacketGCCharacterDelete {
        header: 0x0208 as u16,
        length: size_of::<TPacketGCCharacterDelete>() as u16,
        dwVID: 12345,
    };
    
    let data = unsafe {
        std::slice::from_raw_parts(
            &packet as *const _ as *const u8,
            size_of::<TPacketGCCharacterDelete>(),
        )
    };
    
    let result = parse_character_delete(data);
    assert!(result.is_ok());
}

#[test]
fn test_parse_character_delete_buffer_too_short() {
    let data: [u8; 4] = [0, 0, 0, 0];
    
    let result = parse_character_delete(&data);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "Buffer too short");
}

#[test]
fn test_parse_character_delete_empty_buffer() {
    let data: [u8; 0] = [];
    
    let result = parse_character_delete(&data);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "Buffer too short");
}
