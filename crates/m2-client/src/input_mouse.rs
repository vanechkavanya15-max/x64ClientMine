//! Modul `input_mouse` — Sterowanie postacia myszka (LPM chodzenie/atak, PPM kamera)
//! Wzorzec referencyjny C++: `source/UserInterface/PythonPlayerInputMouse.cpp`
//! Opis: Odwzorowanie __OnClickActor, __OnClickGround, __OnPressActor: klikniecie w ziemie wyznacza cel marszu, klikniecie na potwora uruchamia podbiegniecie i atak.

pub const DIK_LCONTROL: i32 = 0x1D;
pub const DIK_RCONTROL: i32 = 0x9D;

pub const CURSOR_NORMAL: i32 = 0;
pub const CURSOR_CAMERA_ROTATE: i32 = 1;
pub const CURSOR_MODE_HARDWARE: i32 = 1;

pub const EFFECT_PICK: i32 = 0;
pub const FUNC_WAIT: i32 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButtonType {
    Left = 0,
    Right = 1,
    Middle = 2,
    Num = 3,
}

impl TryFrom<usize> for MouseButtonType {
    type Error = ();
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == MouseButtonType::Left as usize => Ok(MouseButtonType::Left),
            x if x == MouseButtonType::Right as usize => Ok(MouseButtonType::Right),
            x if x == MouseButtonType::Middle as usize => Ok(MouseButtonType::Middle),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButtonFunc {
    Smart = 0,
    Move = 1,
    Camera = 2,
    Attack = 3,
    Skill = 4,
    Auto = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButtonState {
    Click = 0,
    Press = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReservedMode {
    #[default]
    None = 0,
    ClickPosition = 1,
    ClickItem = 2,
    ClickActor = 3,
    UseSkill = 4,
}

pub trait InputMouseContext {
    fn can_move(&self) -> bool;
    fn get_main_actor_ptr(&self) -> bool;
    fn is_open_private_shop(&self) -> bool;
    fn is_processing_emotion(&self) -> bool;
    fn is_sleep(&self) -> bool;
    fn is_lock(&self) -> bool;
    fn is_movie_mode(&self) -> bool;

    fn get_mouse_position(&self) -> (i32, i32);
    fn get_screen_size(&self) -> (i32, i32);
    fn get_mouse_dir_rotation(&self, mouse_x: f32, mouse_y: f32) -> f32;
    fn move_to_direction(&mut self, dir_rot: f32) -> bool;
    fn stop(&mut self);

    fn has_current_camera(&self) -> bool;
    fn begin_camera_drag(&mut self, mouse_x: i32, mouse_y: i32);
    fn end_camera_drag(&mut self) -> bool;
    fn is_map_ready(&self) -> bool;
    fn set_cursor_num(&mut self, cursor_num: i32);
    fn get_cursor_mode(&self) -> i32;
    fn set_cursor_visible(&mut self, visible: bool, force: bool);

    fn change_target_to_picked_instance(&mut self);
    fn get_picked_actor_ptr(&self) -> Option<u32>;
    fn open_character_menu(&mut self, virtual_id: u32);
    
    fn get_picked_item_id(&self) -> Option<u32>;
    fn get_picked_actor_id(&self) -> Option<u32>;
    fn get_picked_ground_pos(&self) -> Option<(f32, f32, f32)>;
    fn get_main_character_index(&self) -> u32;

    fn on_press_item(&mut self, item_id: u32);
    fn on_press_actor(&mut self, actor_id: u32, is_auto: bool);
    fn on_press_ground(&mut self, pos: (f32, f32, f32));
    fn on_press_screen(&mut self);

    fn on_click_item(&mut self, item_id: u32);
    fn on_click_actor(&mut self, actor_id: u32, is_auto: bool);
    fn on_click_ground(&mut self, pos: (f32, f32, f32));

    fn show_effect(&mut self, effect_id: i32, pos: (f32, f32, f32));

    fn is_pressed(&self, key: i32) -> bool;
    fn attack(&mut self);
    fn use_current_skill(&mut self);

    fn get_ground_item_position(&self, iid: u32) -> Option<(f32, f32, f32)>;
    fn get_distance_from_dest_pixel_position(&self, pos: (f32, f32, f32)) -> f32;
    fn get_pixel_position(&self) -> (f32, f32, f32);
    fn get_rotation(&self) -> f32;
    fn send_character_state_packet(&mut self, pos: (f32, f32, f32), rot: f32, func: i32, arg: i32);
    fn send_click_item_packet(&mut self, iid: u32);
    fn move_to_dest_pixel_position_direction(&mut self, pos: (f32, f32, f32));
    fn is_empty_reserved_delay_time(&self, elapsed_time: f32) -> bool;
    fn get_global_elapsed_time(&self) -> f32;
    
    fn reserve_process_click_actor(&mut self);
    fn find_actor_ptr(&self, vid: u32) -> bool;
    fn get_distance(&self, vid: u32) -> f32;
    fn is_huge_race(&self, vid: u32) -> bool;
    fn set_target(&mut self, vid: u32);
    fn use_skill(&mut self, slot_index: u32) -> bool;
    fn move_to_dest_instance_direction(&mut self, vid: u32);
    fn set_multi_dir_key_state(&mut self, left: bool, right: bool, up: bool, down: bool);
    fn get_combo_index(&self) -> i32;
    
    fn clear_reserved_action(&mut self);
    fn get_target_vid(&self) -> u32;
    fn check_dash_affect(&self) -> bool;
    fn is_attackable_instance(&self, vid: u32) -> bool;
    fn can_attack(&self) -> bool;
    fn is_clickable_distance_dest_instance(&self, vid: u32) -> bool;
    fn is_npc(&self, vid: u32) -> bool;
    fn send_click_actor_packet(&mut self, vid: u32);
    fn set_auto_attack_target_actor_id(&mut self, vid: u32);
    fn is_bow_mode(&self) -> bool;
    fn can_shot(&self, vid: u32) -> bool;
    fn attack_to_dest_instance_direction(&mut self, vid: u32);
    fn is_movable_ground_distance(&self, pos: (f32, f32, f32)) -> bool;
    fn move_to_dest_pixel_position_direction_bool(&mut self, pos: (f32, f32, f32)) -> bool;
    fn clear_auto_attack_target_actor_id(&mut self);
    fn cancel_fishing(&mut self) -> bool;
}

#[derive(Debug, Clone)]
pub struct InputMouse {
    pub initialized: bool,
    pub m_ae_mbf_button: [MouseButtonFunc; MouseButtonType::Num as usize],
    
    pub m_is_dir_mov: bool,
    pub m_is_smt_mov: bool,
    pub m_is_dir_key: bool,
    pub m_is_atk_key: bool,
    
    pub m_is_left: bool,
    pub m_is_right: bool,
    pub m_is_up: bool,
    pub m_is_down: bool,

    pub m_e_reserved_mode: ReservedMode,
    pub m_dw_iid_reserved: u32,
    pub m_k_ppos_reserved: (f32, f32, f32),
    pub m_dw_vid_reserved: u32,
    pub m_dw_skill_range_reserved: u32,
    pub m_dw_skill_slot_index_reserved: u32,
    
    pub m_i_combo_old: i32,
}

impl Default for InputMouse {
    fn default() -> Self {
        Self {
            initialized: true,
            m_ae_mbf_button: [
                MouseButtonFunc::Smart,
                MouseButtonFunc::Move,
                MouseButtonFunc::Camera,
            ],
            m_is_dir_mov: false,
            m_is_smt_mov: false,
            m_is_dir_key: false,
            m_is_atk_key: false,
            
            m_is_left: false,
            m_is_right: false,
            m_is_up: false,
            m_is_down: false,

            m_e_reserved_mode: ReservedMode::None,
            m_dw_iid_reserved: 0,
            m_k_ppos_reserved: (0.0, 0.0, 0.0),
            m_dw_vid_reserved: 0,
            m_dw_skill_range_reserved: 0,
            m_dw_skill_slot_index_reserved: 0,
            
            m_i_combo_old: 0,
        }
    }
}

impl InputMouse {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn set_mouse_move_state(&mut self, ctx: &mut dyn InputMouseContext, state: MouseButtonState) {
        if state == MouseButtonState::Press {
            self.move_to_mouse_screen_direction(ctx);
            self.m_is_dir_mov = true;
        } else if state == MouseButtonState::Click {
            ctx.stop();
            self.m_is_dir_mov = false;
        }
    }

    pub fn move_to_mouse_screen_direction(&mut self, ctx: &mut dyn InputMouseContext) -> bool {
        let (mouse_x, mouse_y) = ctx.get_mouse_position();
        let (scr_width, scr_height) = ctx.get_screen_size();
        
        let f_mouse_x = mouse_x as f32 / scr_width as f32;
        let f_mouse_y = mouse_y as f32 / scr_height as f32;

        let f_dir_rot = ctx.get_mouse_dir_rotation(f_mouse_x, f_mouse_y);
        ctx.move_to_direction(f_dir_rot)
    }

    pub fn set_mouse_camera_state(&mut self, ctx: &mut dyn InputMouseContext, state: MouseButtonState) {
        if ctx.has_current_camera() {
            if state == MouseButtonState::Press {
                let (mouse_x, mouse_y) = ctx.get_mouse_position();
                ctx.begin_camera_drag(mouse_x, mouse_y);
                
                if !ctx.is_map_ready() {
                    return;
                }
                
                ctx.set_cursor_num(CURSOR_CAMERA_ROTATE);
                if ctx.get_cursor_mode() == CURSOR_MODE_HARDWARE {
                    ctx.set_cursor_visible(false, true);
                }
            } else if state == MouseButtonState::Click {
                let is_camera_drag = ctx.end_camera_drag();
                
                if !ctx.is_map_ready() {
                    return;
                }
                
                ctx.set_cursor_num(CURSOR_NORMAL);
                if ctx.get_cursor_mode() == CURSOR_MODE_HARDWARE {
                    ctx.set_cursor_visible(true, false);
                }
                
                if !is_camera_drag {
                    ctx.change_target_to_picked_instance();
                    if let Some(vid) = ctx.get_picked_actor_ptr() {
                        ctx.open_character_menu(vid);
                    }
                }
            }
        }
    }

    pub fn set_mouse_smart_state(&mut self, ctx: &mut dyn InputMouseContext, state: MouseButtonState, is_auto: bool) {
        if !ctx.get_main_actor_ptr() {
            return;
        }
        
        if ctx.is_open_private_shop() {
            self.m_is_smt_mov = false;
            return;
        }
        
        if ctx.is_processing_emotion() {
            return;
        }
        
        if ctx.is_sleep() {
            return;
        }
        
        if state == MouseButtonState::Press {
            self.m_is_smt_mov = true;
            self.on_press_smart(ctx, is_auto);
        } else if state == MouseButtonState::Click {
            self.m_is_smt_mov = false;
            self.on_click_smart(ctx, is_auto);
        }
    }

    fn on_press_smart(&mut self, ctx: &mut dyn InputMouseContext, is_auto: bool) {
        if let Some(item_id) = ctx.get_picked_item_id() {
            ctx.on_press_item(item_id);
        } else if let Some(actor_id) = ctx.get_picked_actor_id() {
            if actor_id != ctx.get_main_character_index() {
                self.on_press_actor(ctx, actor_id, is_auto);
            } else if let Some(pos) = ctx.get_picked_ground_pos() {
                self.on_press_ground(ctx, pos);
            } else {
                ctx.on_press_screen();
            }
        } else if let Some(pos) = ctx.get_picked_ground_pos() {
            self.on_press_ground(ctx, pos);
        } else {
            ctx.on_press_screen();
        }
    }

    fn on_click_smart(&mut self, ctx: &mut dyn InputMouseContext, is_auto: bool) {
        if let Some(item_id) = ctx.get_picked_item_id() {
            ctx.on_click_item(item_id);
        } else if let Some(actor_id) = ctx.get_picked_actor_id() {
            self.on_click_actor(ctx, actor_id, is_auto);
        } else if let Some(pos) = ctx.get_picked_ground_pos() {
            self.on_click_ground(ctx, pos);
        } else {
            ctx.stop();
        }
    }

    pub fn show_picked_effect(&mut self, ctx: &mut dyn InputMouseContext, pos: (f32, f32, f32)) {
        if !ctx.get_main_actor_ptr() {
            return;
        }
        
        if ctx.is_movie_mode() {
            return;
        }
        
        ctx.show_effect(EFFECT_PICK, pos);
    }

    pub fn set_mouse_func(&mut self, mbt: MouseButtonType, mbf: MouseButtonFunc) -> bool {
        let index = mbt as usize;
        if index >= MouseButtonType::Num as usize {
            return false;
        }
        self.m_ae_mbf_button[index] = mbf;
        true
    }

    pub fn get_mouse_func(&self, mbt: MouseButtonType) -> Option<MouseButtonFunc> {
        let index = mbt as usize;
        if index >= MouseButtonType::Num as usize {
            return None;
        }
        Some(self.m_ae_mbf_button[index])
    }

    pub fn set_quick_camera_mode(&mut self, ctx: &mut dyn InputMouseContext, is_enable: bool) {
        if !is_enable {
            self.set_mouse_camera_state(ctx, MouseButtonState::Click);
        }
    }

    pub fn set_mouse_state(&mut self, ctx: &mut dyn InputMouseContext, mbt: MouseButtonType, mbs: MouseButtonState) -> bool {
        let index = mbt as usize;
        if index >= MouseButtonType::Num as usize {
            return false;
        }
        
        let mbf = self.m_ae_mbf_button[index];
        match mbf {
            MouseButtonFunc::Move => {
                if ctx.can_move() {
                    self.set_mouse_move_state(ctx, mbs);
                }
            },
            MouseButtonFunc::Smart => {
                if ctx.is_pressed(DIK_LCONTROL) || ctx.is_pressed(DIK_RCONTROL) {
                    ctx.attack();
                } else {
                    self.set_mouse_smart_state(ctx, mbs, false);
                }
            },
            MouseButtonFunc::Camera => {
                self.set_mouse_camera_state(ctx, mbs);
            },
            MouseButtonFunc::Auto => {
                self.set_mouse_smart_state(ctx, mbs, true);
            },
            MouseButtonFunc::Attack => {
                ctx.attack();
            },
            MouseButtonFunc::Skill => {
                if ctx.is_pressed(DIK_LCONTROL) {
                    self.set_mouse_camera_state(ctx, mbs);
                } else {
                    if mbs == MouseButtonState::Press {
                        ctx.change_target_to_picked_instance();
                        ctx.use_current_skill();
                    }
                }
            },
        }
        true
    }

    pub fn set_mouse_middle_button_state(&mut self, ctx: &mut dyn InputMouseContext, state: MouseButtonState) {
        self.set_mouse_camera_state(ctx, state);
    }

    pub fn refresh_mouse_walking_direction(&mut self, ctx: &mut dyn InputMouseContext) {
        if !ctx.get_main_actor_ptr() {
            return;
        }
        
        match self.m_e_reserved_mode {
            ReservedMode::ClickItem => {
                if let Some(picked_item_pos) = ctx.get_ground_item_position(self.m_dw_iid_reserved) {
                    if ctx.get_distance_from_dest_pixel_position(picked_item_pos) < 20.0 {
                        let cur_pos = ctx.get_pixel_position();
                        let cur_rot = ctx.get_rotation();
                        ctx.send_character_state_packet(cur_pos, cur_rot, FUNC_WAIT, 0);
                        ctx.send_click_item_packet(self.m_dw_iid_reserved);
                        ctx.stop();
                        ctx.clear_reserved_action();
                    } else {
                        ctx.move_to_dest_pixel_position_direction(picked_item_pos);
                    }
                } else {
                    ctx.clear_reserved_action();
                }
            },
            ReservedMode::ClickActor => {
                ctx.reserve_process_click_actor();
            },
            ReservedMode::ClickPosition => {
                if !ctx.is_lock() {
                    let elapsed_time = ctx.get_global_elapsed_time();
                    if ctx.is_empty_reserved_delay_time(elapsed_time) {
                        ctx.move_to_dest_pixel_position_direction(self.m_k_ppos_reserved);
                        ctx.clear_reserved_action();
                    }
                }
            },
            ReservedMode::UseSkill => {
                if ctx.find_actor_ptr(self.m_dw_vid_reserved) {
                    let mut distance = ctx.get_distance(self.m_dw_vid_reserved);
                    if ctx.is_huge_race(self.m_dw_vid_reserved) {
                        distance -= 200.0;
                    }
                    if distance < self.m_dw_skill_range_reserved as f32 {
                        ctx.set_target(self.m_dw_vid_reserved);
                        if ctx.use_skill(self.m_dw_skill_slot_index_reserved) {
                            ctx.clear_reserved_action();
                        }
                    } else {
                        ctx.move_to_dest_instance_direction(self.m_dw_vid_reserved);
                    }
                } else {
                    ctx.clear_reserved_action();
                }
            },
            ReservedMode::None => {}
        }

        if self.m_is_smt_mov {
            self.on_press_smart(ctx, false);
        }

        if self.m_is_dir_mov {
            self.move_to_mouse_screen_direction(ctx);
        }

        if self.m_is_dir_key {
            ctx.set_multi_dir_key_state(self.m_is_left, self.m_is_right, self.m_is_up, self.m_is_down);
        }

        if self.m_is_atk_key {
            ctx.attack();
        }

        self.m_i_combo_old = ctx.get_combo_index();
    }
    
    pub fn is_right_button_skill_mode(&self) -> bool {
        self.m_ae_mbf_button[MouseButtonType::Right as usize] == MouseButtonFunc::Skill
    }
}

impl InputMouse {
    pub fn on_click_actor(&mut self, ctx: &mut dyn InputMouseContext, dw_picked_actor_id: u32, is_auto: bool) {
        if self.m_e_reserved_mode == ReservedMode::UseSkill {
            if ctx.get_target_vid() == dw_picked_actor_id {
                return;
            }
            if ctx.check_dash_affect() {
                self.m_dw_vid_reserved = dw_picked_actor_id;
                return;
            }
        }
        
        ctx.clear_reserved_action();
        
        if !ctx.find_actor_ptr(dw_picked_actor_id) {
            return;
        }
        
        if ctx.is_attackable_instance(dw_picked_actor_id) {
            if !ctx.can_attack() {
                return;
            }
        }
        
        if !ctx.is_clickable_distance_dest_instance(dw_picked_actor_id) {
            self.m_e_reserved_mode = ReservedMode::ClickActor;
            self.m_dw_vid_reserved = dw_picked_actor_id;
            return;
        }
        
        if ctx.is_npc(dw_picked_actor_id) {
            ctx.send_click_actor_packet(dw_picked_actor_id);
        }
        
        ctx.stop();
    }
    
    pub fn on_press_actor(&mut self, ctx: &mut dyn InputMouseContext, dw_picked_actor_id: u32, is_auto: bool) {
        if self.m_e_reserved_mode == ReservedMode::UseSkill {
            if ctx.get_target_vid() == dw_picked_actor_id {
                return;
            }
            if ctx.check_dash_affect() {
                self.m_dw_vid_reserved = dw_picked_actor_id;
                return;
            }
        }
        
        ctx.change_target_to_picked_instance();
        ctx.clear_reserved_action();
        
        if !ctx.can_attack() {
            return;
        }
        
        if !ctx.find_actor_ptr(dw_picked_actor_id) {
            return;
        }
        
        if is_auto {
            if ctx.is_attackable_instance(dw_picked_actor_id) {
                ctx.set_auto_attack_target_actor_id(dw_picked_actor_id);
            }
        }
        
        if ctx.is_bow_mode() {
            if ctx.is_attackable_instance(dw_picked_actor_id) {
                if !ctx.can_shot(dw_picked_actor_id) {
                    return;
                }
            }
        }
        
        if !ctx.is_clickable_distance_dest_instance(dw_picked_actor_id) {
            self.m_e_reserved_mode = ReservedMode::ClickActor;
            self.m_dw_vid_reserved = dw_picked_actor_id;
            return;
        }
        
        if !ctx.is_attackable_instance(dw_picked_actor_id) {
            return;
        }
        
        ctx.attack_to_dest_instance_direction(dw_picked_actor_id);
    }
    
    pub fn on_click_ground(&mut self, ctx: &mut dyn InputMouseContext, pos: (f32, f32, f32)) {
        if !ctx.is_movable_ground_distance(pos) {
            return;
        }
        
        if ctx.move_to_dest_pixel_position_direction_bool(pos) {
            self.show_picked_effect(ctx, pos);
        }
    }
    
    pub fn on_press_ground(&mut self, ctx: &mut dyn InputMouseContext, pos: (f32, f32, f32)) {
        ctx.clear_reserved_action();
        ctx.clear_auto_attack_target_actor_id();
        
        if ctx.cancel_fishing() {
            return;
        }
        
        if !ctx.is_movable_ground_distance(pos) {
            return;
        }
        
        if !ctx.move_to_dest_pixel_position_direction_bool(pos) {
            self.m_e_reserved_mode = ReservedMode::ClickPosition;
            self.m_k_ppos_reserved = pos;
        }
    }
}
