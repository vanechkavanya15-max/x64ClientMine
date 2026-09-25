//! Modul `input_keyboard` — Sterowanie postacia z klawiatury (WASD, skille, quicksloty)
//! Wzorzec referencyjny C++: `source/UserInterface/PythonPlayerInputKeyboard.cpp`
//! Opis: Odwzorowanie sterowania klawiatura: ruch kierunkowy WASD/Strzalki, klawisz Spacji (atak ciagly), skroty 1-4 i F1-F4 (uzycie slotow podrecznych), podnoszenie przedmiotow klawiszem Z/\`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DikKey {
    Up,
    Down,
    Left,
    Right,
    W,
    A,
    S,
    D,
    Space,
    Num1,
    Num2,
    Num3,
    Num4,
    F1,
    F2,
    F3,
    F4,
    Z,
    Grave, // Klawisz \`
    Other(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirKey {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardUd {
    None,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardLr {
    None,
    Left,
    Right,
}

#[derive(Debug, Clone, Default)]
pub struct InputKeyboard {
    pub initialized: bool,
    pub is_up: bool,
    pub is_down: bool,
    pub is_left: bool,
    pub is_right: bool,
    pub is_atk_key: bool,
    pub is_dir_key: bool,
    // Pola pomocnicze do symulacji stanu C++
    pub is_fishing_mode: bool,
    pub last_move_direction: f32,
    pub stopped: bool,
    pub attack_called: bool,
    // Pola dla nowych funkcjonalnosci
    pub last_quickslot_used: Option<u32>,
    pub pickup_called: bool,
}

impl InputKeyboard {
    pub fn new() -> Self {
        Self {
            initialized: true,
            ..Default::default()
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn set_attack_key_state(&mut self, is_press: bool) {
        if is_press {
            if self.is_fishing_mode {
                self.new_fishing();
                return;
            }
        }
        self.is_atk_key = is_press;
    }

    pub fn set_single_dik_key_state(&mut self, e_dik_key: DikKey, is_press: bool) {
        if self.new_cancel_fishing() {
            return;
        }

        match e_dik_key {
            // Ruch kierunkowy: Strzalki i WASD
            DikKey::Up | DikKey::W => self.set_single_dir_key_state(DirKey::Up, is_press),
            DikKey::Down | DikKey::S => self.set_single_dir_key_state(DirKey::Down, is_press),
            DikKey::Left | DikKey::A => self.set_single_dir_key_state(DirKey::Left, is_press),
            DikKey::Right | DikKey::D => self.set_single_dir_key_state(DirKey::Right, is_press),
            
            // Obsluga uzycia quickslotow (skroty 1-4, F1-F4)
            DikKey::Num1 if is_press => self.request_use_local_quick_slot(0),
            DikKey::Num2 if is_press => self.request_use_local_quick_slot(1),
            DikKey::Num3 if is_press => self.request_use_local_quick_slot(2),
            DikKey::Num4 if is_press => self.request_use_local_quick_slot(3),
            DikKey::F1 if is_press => self.request_use_local_quick_slot(4),
            DikKey::F2 if is_press => self.request_use_local_quick_slot(5),
            DikKey::F3 if is_press => self.request_use_local_quick_slot(6),
            DikKey::F4 if is_press => self.request_use_local_quick_slot(7),

            // Obsluga podnoszenia przedmiotow (klawisze Z lub \`)
            DikKey::Z | DikKey::Grave if is_press => self.request_item_pickup(),
            
            // Obsluga ataku (Spacja)
            DikKey::Space => self.set_attack_key_state(is_press),

            _ => {}
        }
    }

    pub fn set_single_dir_key_state(&mut self, e_dir_key: DirKey, is_press: bool) {
        match e_dir_key {
            DirKey::Up => self.is_up = is_press,
            DirKey::Down => self.is_down = is_press,
            DirKey::Left => self.is_left = is_press,
            DirKey::Right => self.is_right = is_press,
        }

        self.is_dir_key = self.is_up || self.is_down || self.is_left || self.is_right;

        // Przekazanie stanu: isLeft, isRight, isUp, isDown
        self.set_multi_dir_key_state(self.is_left, self.is_right, self.is_up, self.is_down);
    }

    pub fn set_multi_dir_key_state(
        &mut self,
        is_left: bool,
        is_right: bool,
        is_up: bool,
        is_down: bool,
    ) {
        if !self.can_move() {
            return;
        }

        let is_any = is_left || is_right || is_up || is_down;

        if is_any {
            let f_dir_rot = self.get_multi_key_dir_rotation(is_left, is_right, is_up, is_down);

            if !self.move_to_direction(f_dir_rot) {
                // Tracen("CPythonPlayer::NEW_SetMultiKeyState - NEW_Move -> ERROR");
            }
        } else {
            self.stop();
        }
    }

    pub fn get_degree_from_direction(&self, i_ud: KeyboardUd, i_lr: KeyboardLr) -> f32 {
        match i_ud {
            KeyboardUd::Up => {
                if i_lr == KeyboardLr::Left {
                    return 45.0;
                } else if i_lr == KeyboardLr::Right {
                    return -45.0;
                }
                0.0
            }
            KeyboardUd::Down => {
                if i_lr == KeyboardLr::Left {
                    return 135.0;
                } else if i_lr == KeyboardLr::Right {
                    return -135.0;
                }
                180.0
            }
            KeyboardUd::None => {
                if i_lr == KeyboardLr::Left {
                    return 90.0;
                } else if i_lr == KeyboardLr::Right {
                    return -90.0;
                }
                0.0
            }
        }
    }

    // Metody symulujace / zaleznosci (w C++ w innych plikach, tutaj jako mocki na potrzeby logiki i testow)

    pub fn new_fishing(&mut self) {
        // Symulacja akcji lowienia
    }

    pub fn new_cancel_fishing(&mut self) -> bool {
        // Zgodnie z C++, anuluje lowienie, jesli w toku i zwraca true, wpp false.
        // W testach ustawiamy to z zewnatrz lub mozna dac jakas logike.
        false
    }

    pub fn can_move(&self) -> bool {
        true
    }

    pub fn get_multi_key_dir_rotation(
        &self,
        is_left: bool,
        is_right: bool,
        is_up: bool,
        is_down: bool,
    ) -> f32 {
        let mut f_scr_x = 0.5;
        let mut f_scr_y = 0.5;

        if is_left {
            f_scr_x = 0.0;
        } else if is_right {
            f_scr_x = 1.0;
        }

        if is_up {
            f_scr_y = 0.0;
        } else if is_down {
            f_scr_y = 1.0;
        }

        // Zamiast uzywac GetMouseDirRotation i GetDegreeFromPosition (matematyka na wektorach D3DX),
        // obliczamy to prosciej na podstawie polozenia "myszy"
        
        let ix = (1024.0 * f_scr_x) as i32;
        let iy = (768.0 * f_scr_y) as i32;
        let i_half_width = 512;
        let i_half_height = 384;
        
        self.get_degree_from_position(ix, iy, i_half_width, i_half_height)
    }
    
    pub fn get_degree_from_position(&self, ix: i32, iy: i32, i_half_width: i32, i_half_height: i32) -> f32 {
        let dx = (ix - i_half_width) as f32;
        let dy = (iy - i_half_height) as f32;
        
        if dx == 0.0 && dy == 0.0 {
            return 0.0;
        }
        
        // Zgodnie z implementacja C++ (uproszczona)
        let len = (dx * dx + dy * dy).sqrt();
        let nx = dx / len;
        let ny = dy / len;
        
        // Dot product z (0, -1, 0) => 0*nx + (-1)*ny + 0*nz = -ny
        let dot = -ny;
        let acos_val = dot.acos();
        
        // Konwersja na radiany, w C++ to bylo D3DXToDegree
        let mut ret = acos_val.to_degrees();
        
        if nx < 0.0 {
            ret = 360.0 - ret;
        }
        
        360.0 - ret
    }

    pub fn move_to_direction(&mut self, f_dir_rot: f32) -> bool {
        self.last_move_direction = f_dir_rot;
        self.stopped = false;
        true
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn request_use_local_quick_slot(&mut self, slot_index: u32) {
        // Symulacja wywolania uzycia quickslotu (np. CPythonPlayer::RequestUseLocalQuickSlot w C++)
        self.last_quickslot_used = Some(slot_index);
    }

    pub fn request_item_pickup(&mut self) {
        // Symulacja wywolania podnoszenia przedmiotow dookola (np. z uzyciem opkodow sieciowych w C++)
        self.pickup_called = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_input_keyboard() {
        let instance = InputKeyboard::new();
        assert!(instance.is_ready());
    }
}
