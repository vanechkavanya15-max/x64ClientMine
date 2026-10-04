use crate::character_create_delete_flow::Job;
use crate::ui_window_mgr::get_window_manager;
use m2_client_bridge::net::cg::character_select::CharacterSelectPacket;

#[derive(Debug, Clone)]
pub struct CharacterSlotData {
    pub name: String,
    pub level: u8,
    pub job: Job,
    pub str: u8,
    pub dex: u8,
    pub vit: u8,
    pub int: u8,
    pub play_time: u32,
    pub armor: u32,
    pub weapon: u32,
    pub is_empty: bool,
}

impl Default for CharacterSlotData {
    fn default() -> Self {
        Self {
            name: String::new(),
            level: 0,
            job: Job::WarriorM,
            str: 0,
            dex: 0,
            vit: 0,
            int: 0,
            play_time: 0,
            armor: 0,
            weapon: 0,
            is_empty: true,
        }
    }
}

pub struct CharacterSelectScreen {
    pub slots: [CharacterSlotData; 4],
    pub selected_slot: Option<u8>,
    pub camera_rotation: f32,
}

impl CharacterSelectScreen {
    pub fn new() -> Self {
        Self {
            slots: [
                CharacterSlotData::default(),
                CharacterSlotData::default(),
                CharacterSlotData::default(),
                CharacterSlotData::default(),
            ],
            selected_slot: None,
            camera_rotation: 0.0,
        }
    }

    pub fn set_slot(&mut self, index: u8, data: CharacterSlotData) {
        if index < 4 {
            self.slots[index as usize] = data;
        }
    }

    pub fn select_slot(&mut self, index: u8) {
        if index < 4 {
            if !self.slots[index as usize].is_empty {
                if self.selected_slot != Some(index) {
                    self.selected_slot = Some(index);
                    self.camera_rotation = 0.0;
                    
                    // Zamiast stuba - symulacja animacji "IntroWait" lub podobnej 
                    // wzorowanej na zachowaniu z CPythonNetworkStreamPhaseSelect
                    if let Some(mut chr_mgr) = crate::character_manager_core::get_character_manager().write().ok() {
                         if let Some(main_actor) = chr_mgr.get_main_instance() {
                              let vid = main_actor.vid;
                              if let Some(actor) = chr_mgr.get_instance_mut(vid) {
                                   // Ustaw animacje wprowadzajaca
                                   actor.active_once_motion = Some(1); // MOTION_INTRO_WAIT (1)
                              }
                         }
                    }
                }
            }
        }
    }

    pub fn get_selected_character_packet(&self) -> Option<CharacterSelectPacket> {
        self.selected_slot.map(|slot| CharacterSelectPacket::new(slot))
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.selected_slot.is_some() {
            self.camera_rotation += delta_time * 10.0; // Rotate camera when a character is selected
            if self.camera_rotation >= 360.0 {
                self.camera_rotation -= 360.0;
            }
        }
    }
}

impl CharacterSelectScreen {
    pub fn handle_button_click(&mut self, button_name: &str) -> bool {
        match button_name {
            "StartGame" => {
                if let Some(slot) = self.selected_slot {
                    let packet = CharacterSelectPacket::new(slot);
                    let mut sent = false;
                    if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
                        if let Some((stream, cipher)) = sock_guard.as_mut() {
                            use std::io::Write;
                            let bytes = packet.to_bytes();
                            let mut encrypted_buf = bytes.clone();
                            cipher.encrypt_in_place(&mut encrypted_buf);
                            if stream.write_all(&encrypted_buf).is_ok() {
                                let _ = stream.flush();
                                sent = true;
                            }
                        }
                    }
                    if !sent {
                        m2_client_bridge::net::send_queue::enqueue_outgoing_packet(packet.to_bytes());
                    }
                    return true;
                }
                false
            }
            "CreateCharacter" => {
                let mut wm = get_window_manager();
                // Pokaz okno tworzenia postaci
                if let Some(win) = wm.windows.iter_mut().find(|(_, w)| w.name == "CreateCharacterWindow") {
                    win.1.is_show = true;
                }
                true
            }
            "DeleteCharacter" => {
                if let Some(slot) = self.selected_slot {
                     let mut wm = get_window_manager();
                     crate::character_delete_flow::open_character_delete_modal(&mut wm, slot);
                     return true;
                }
                false
            }
            "Exit" => {
                std::process::exit(0);
            }
            _ => false,
        }
    }
}
