//! Modul `ui_select_screen` — Kompletny interaktywny Ekran Wyboru i Tworzenia Postaci
//! Wzorzec referencyjny C++: `source/UserInterface/PythonNetworkStreamPhaseSelect.cpp`
//! Opis: Zbudowanie Ekranu Wyboru Postaci: 4 sloty postaci, podglad 3D wybranej postaci, paski statystyk (WIT, INT, SIL, ZRE), przyciski Start (Wejdz do gry), Stworz nowa postac, Usun.

pub const CHARACTER_NAME_MAX_LEN: usize = 24;
pub const PLAYER_PER_ACCOUNT4: usize = 4;
pub const PRIVATE_CODE_LENGTH: usize = 8;

#[derive(Debug, Clone, Default)]
pub struct SimplePlayerInformation {
    pub id: u32,
    pub name: String,
    pub job: u8,
    pub level: u8,
    pub play_minutes: u32,
    pub st: u8,
    pub ht: u8,
    pub dx: u8,
    pub iq: u8,
    pub main_part: u16,
    pub change_name: u8,
    pub hair_part: u16,
    pub x: i32,
    pub y: i32,
    pub addr: i32,
    pub port: u16,
    pub skill_group: u8,
}

#[derive(Debug, Clone, Default)]
pub struct UiSelectScreen {
    pub initialized: bool,
    pub players: [Option<SimplePlayerInformation>; PLAYER_PER_ACCOUNT4],
    pub selected_slot: Option<usize>,
    pub empire_id: u32,
}

impl UiSelectScreen {
    pub fn new() -> Self {
        Self {
            initialized: true,
            players: [None, None, None, None],
            selected_slot: None,
            empire_id: 0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_ui_select_screen() {
        let instance = UiSelectScreen::new();
        assert!(instance.is_ready());
    }
}

impl UiSelectScreen {
    pub fn select_slot(&mut self, slot: usize) -> bool {
        if slot >= PLAYER_PER_ACCOUNT4 {
            return false;
        }
        if self.players[slot].is_some() {
            self.selected_slot = Some(slot);
            true
        } else {
            self.selected_slot = None;
            false
        }
    }

    pub fn get_selected_character_preview(&self) -> Option<&SimplePlayerInformation> {
        self.selected_slot.and_then(|slot| self.players[slot].as_ref())
    }

    pub fn get_selected_character_stats(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_selected_character_preview()
            .map(|info| (info.ht, info.iq, info.st, info.dx)) // WIT (HT), INT (IQ), SIL (ST), ZRE (DX)
    }

    // Network simulations for sending
    pub fn send_select_empire_packet(&mut self, empire_id: u32) -> bool {
        self.empire_id = empire_id;
        true
    }

    pub fn send_select_character_packet(&self, index: u8) -> bool {
        if (index as usize) < PLAYER_PER_ACCOUNT4 && self.players[index as usize].is_some() {
            true
        } else {
            false
        }
    }

    pub fn send_destroy_character_packet(&self, index: u8, _private_code: &str) -> bool {
        if (index as usize) < PLAYER_PER_ACCOUNT4 {
            true
        } else {
            false
        }
    }

    pub fn send_create_character_packet(
        &self,
        index: u8,
        name: &str,
        _job: u8,
        _shape: u8,
        _con: u8,
        _int: u8,
        _str: u8,
        _dex: u8,
    ) -> bool {
        if (index as usize) < PLAYER_PER_ACCOUNT4 && name.len() <= CHARACTER_NAME_MAX_LEN {
            true
        } else {
            false
        }
    }

    pub fn send_change_name_packet(&self, index: u8, name: &str) -> bool {
        if (index as usize) < PLAYER_PER_ACCOUNT4 && name.len() <= CHARACTER_NAME_MAX_LEN {
            true
        } else {
            false
        }
    }

    // Network simulations for receiving
    pub fn recv_player_create_success_packet(&mut self, slot: u8, info: SimplePlayerInformation) -> bool {
        let slot = slot as usize;
        if slot >= PLAYER_PER_ACCOUNT4 {
            return false;
        }
        self.players[slot] = Some(info);
        true
    }

    pub fn recv_player_create_failure_packet(&self, _type_id: u8) -> bool {
        true
    }

    pub fn recv_player_destroy_success_packet(&mut self, account_index: u8) -> bool {
        let slot = account_index as usize;
        if slot >= PLAYER_PER_ACCOUNT4 {
            return false;
        }
        self.players[slot] = None;
        if self.selected_slot == Some(slot) {
            self.selected_slot = None;
        }
        true
    }

    pub fn recv_player_destroy_failure_packet(&self) -> bool {
        true
    }

    pub fn recv_change_name(&mut self, pid: u32, new_name: &str) -> bool {
        for player in self.players.iter_mut().flatten() {
            if player.id == pid {
                if new_name.len() <= CHARACTER_NAME_MAX_LEN {
                    player.name = new_name.to_string();
                    player.change_name = 0;
                    return true;
                }
            }
        }
        false
    }
}
