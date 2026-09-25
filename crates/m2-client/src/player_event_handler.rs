//! Modul `player_event_handler` — Obsluga zdarzen animacji gracza i wyzwalanie pakietow ciosu
//! Wzorzec referencyjny C++: `source/UserInterface/PythonPlayerEventHandler.cpp`
//! Opis: Odwzorowanie CPythonPlayerEventHandler: odbieranie sygnalu klatki uderzenia (OnHit) i natychmiastowe generowanie pakietu ataku na cel.

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct PixelPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct State {
    pub p_pos_self: PixelPosition,
    pub adv_rot_self: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct SVictim {
    pub vid: u32,
    pub pixel_x: i32,
    pub pixel_y: i32,
}

pub trait NetworkStream {
    fn send_character_state_packet(&self, pos: PixelPosition, rot: f32, func: u32, arg: u32);
    fn send_attack_packet(&self, skill_index: u32, vid: u32);
    fn send_sync_position_element_packet(&self, vid: u32, x: i32, y: i32);
    fn send_sequence(&self);
    fn send_fly_targeting_packet(&self, vid: u32, pos: PixelPosition);
    fn send_shoot_packet(&self, skill_index: u32);
    fn send(&self, size: usize, data: &[u8]);
}

pub trait Player {
    fn clear_affects(&self);
    fn set_affect(&self, affect: u32);
    fn reset_affect(&self, affect: u32);
    fn new_stop(&self);
    fn set_target(&self, vid: u32, is_pc: bool);
}

pub trait ActorInstance {
    fn get_virtual_id(&self) -> u32;
    fn is_pushing(&self) -> bool;
    fn get_race(&self) -> u32;
    fn new_get_last_pixel_position_ref(&self) -> PixelPosition;
    fn temp_push(&self, x: i32, y: i32);
}

pub trait CharacterManager {
    fn adjust_collision_with_other_objects(&self, actor: &dyn ActorInstance);
    fn get_instance_ptr(&self, vid: u32) -> Option<Rc<RefCell<dyn InstanceBase>>>;
}

pub trait GraphicThingInstance {
    fn get_target_rotation(&self) -> f32;
    fn on_get_fly_target_position(&self) -> PixelPosition;
    fn on_shoot_damage(&self);
}

pub trait InstanceBase {
    fn new_get_pixel_position(&self) -> PixelPosition;
    fn get_graphic_thing_instance_ptr(&self) -> Rc<RefCell<dyn GraphicThingInstance>>;
    fn get_virtual_id(&self) -> u32;
}

pub trait FlyEventHandler {
    fn on_set_fly_target(&mut self);
    fn on_shoot(&mut self, skill_index: u32);
    fn on_no_target(&mut self);
    fn on_exploding_out_of_range(&mut self);
    fn on_exploding_at_background(&mut self);
    fn on_exploding_at_another_target(&mut self, skill_index: u32, vid: u32);
    fn on_exploding_at_target(&mut self, skill_index: u32);
}

// Stale funkcyjne z CInstanceBase
pub const FUNC_WAIT: u32 = 0;
pub const FUNC_MOVE: u32 = 1;
pub const FUNC_COMBO: u32 = 2;
pub const FUNC_SKILL: u32 = 0x80;

pub const HEADER_CG_SYNC_POSITION: u8 = 10; // Zalozenie, poniewaz nie mamy stalych pakietow.

pub fn get_msec() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(std::time::Duration::ZERO)
        .as_millis() as u32
}

pub fn is_huge_race(vnum: u32) -> bool {
    // Prosta atrapa funkcji C++: extern bool IS_HUGE_RACE(unsigned int vnum);
    // W testach mozemy mockowac to uzywajac specyficznych vnumow.
    vnum >= 8000
}

#[derive(Clone, Default)]
pub struct PlayerEventHandler {
    pub initialized: bool,
    pub prev_combo_index: u32,
    pub next_waiting_notify_time: u32,
    pub next_moving_notify_time: u32,
    pub p_pos_prev_waiting: PixelPosition,
    pub victims: Vec<SVictim>,
    
    // Zaleznosci:
    pub network_stream: Option<Arc<dyn NetworkStream>>,
    pub player: Option<Arc<dyn Player>>,
    pub character_manager: Option<Arc<dyn CharacterManager>>,
}

impl PlayerEventHandler {
    pub fn new() -> Self {
        Self {
            initialized: true,
            prev_combo_index: 0,
            next_waiting_notify_time: 0,
            next_moving_notify_time: 0,
            p_pos_prev_waiting: PixelPosition::default(),
            victims: Vec::new(),
            network_stream: None,
            player: None,
            character_manager: None,
        }
    }

    pub fn with_dependencies(
        network_stream: Arc<dyn NetworkStream>,
        player: Arc<dyn Player>,
        character_manager: Arc<dyn CharacterManager>,
    ) -> Self {
        let mut handler = Self::new();
        handler.network_stream = Some(network_stream);
        handler.player = Some(player);
        handler.character_manager = Some(character_manager);
        handler
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
}

impl PlayerEventHandler {
    pub fn on_syncing(&mut self, state: &State) {
        self.p_pos_prev_waiting = state.p_pos_self;
    }

    pub fn on_waiting(&mut self, state: &State) {
        let cur_time = get_msec();
        if self.next_waiting_notify_time > cur_time {
            return;
        }

        self.next_waiting_notify_time = cur_time + 100;

        let dx = self.p_pos_prev_waiting.x - state.p_pos_self.x;
        let dy = self.p_pos_prev_waiting.y - state.p_pos_self.y;
        let len = (dx * dx + dy * dy).sqrt();

        if len < 1.0 {
            return;
        }

        self.p_pos_prev_waiting = state.p_pos_self;
        if let Some(ref ns) = self.network_stream {
            ns.send_character_state_packet(
                state.p_pos_self,
                state.adv_rot_self,
                FUNC_WAIT,
                0,
            );
        }
    }

    pub fn on_moving(&mut self, state: &State) {
        let cur_time = get_msec();
        if self.next_moving_notify_time > cur_time {
            return;
        }

        self.next_moving_notify_time = cur_time + 300;
        if let Some(ref ns) = self.network_stream {
            ns.send_character_state_packet(
                state.p_pos_self,
                state.adv_rot_self,
                FUNC_MOVE,
                0,
            );
        }
    }

    pub fn on_move(&mut self, state: &State) {
        let cur_time = get_msec();
        self.next_waiting_notify_time = cur_time + 100;
        self.next_moving_notify_time = cur_time + 300;

        if let Some(ref ns) = self.network_stream {
            ns.send_character_state_packet(
                state.p_pos_self,
                state.adv_rot_self,
                FUNC_MOVE,
                0,
            );
        }
    }

    pub fn on_stop(&mut self, state: &State) {
        if let Some(ref ns) = self.network_stream {
            ns.send_character_state_packet(
                state.p_pos_self,
                state.adv_rot_self,
                FUNC_WAIT,
                0,
            );
        }
    }

    pub fn on_warp(&mut self, state: &State) {
        if let Some(ref ns) = self.network_stream {
            ns.send_character_state_packet(
                state.p_pos_self,
                state.adv_rot_self,
                FUNC_WAIT,
                0,
            );
        }
    }
}

impl PlayerEventHandler {
    pub fn on_attack(&mut self, state: &State, motion_index: u16) {
        assert!(motion_index < 255);
        if let Some(ref ns) = self.network_stream {
            ns.send_character_state_packet(
                state.p_pos_self,
                state.adv_rot_self,
                FUNC_COMBO,
                motion_index as u32,
            );
        }
    }

    pub fn on_use_skill(&mut self, state: &State, mot_skill: u32, arg: u32) {
        if let Some(ref ns) = self.network_stream {
            ns.send_character_state_packet(
                state.p_pos_self,
                state.adv_rot_self,
                FUNC_SKILL | mot_skill,
                arg,
            );
        }
    }

    pub fn on_hit(
        &mut self,
        skill: u32,
        actor_victim: &dyn ActorInstance,
        is_send_packet: bool,
    ) {
        let vid_victim = actor_victim.get_virtual_id();

        // Update Target
        if let Some(ref pl) = self.player {
            pl.set_target(vid_victim, false);
        }
        // Update Target

        if is_send_packet {
            if let Some(ref ns) = self.network_stream {
                ns.send_attack_packet(skill, vid_victim);
            }
        }

        if !actor_victim.is_pushing() {
            return;
        }

        if is_huge_race(actor_victim.get_race()) {
            return;
        }

        if let Some(ref cm) = self.character_manager {
            cm.adjust_collision_with_other_objects(actor_victim);
        }

        let p_pos_last = actor_victim.new_get_last_pixel_position_ref();

        let victim = SVictim {
            vid: vid_victim,
            pixel_x: p_pos_last.x as i32,
            pixel_y: p_pos_last.y as i32,
        };

        actor_victim.temp_push(victim.pixel_x, victim.pixel_y);

        self.victims.push(victim);
    }

    pub fn flush_victim_list(&mut self) {
        if self.victims.is_empty() {
            return;
        }

        let _ui_victim_count = self.victims.len();

        // In C++ it manually calculates size using sizeof
        // TPacketCGSyncPosition = bHeader (1) + wSize (2) = 3 + padding = ?
        // TPacketCGSyncPositionElement = dwVID (4) + lPixelX (4) + lPixelY (4) = 12
        // We'll represent packet building dynamically or via a trait.
        // C++: rkStream.Send(sizeof(kPacketSyncPos), &kPacketSyncPos);
        // Since we are mocking, we can just call `send` with some dummy array for header.
        let header_packet = [HEADER_CG_SYNC_POSITION, 0, 0, 0];
        let header_packet_size = 4; // Typical padded size for 1 byte + 2 byte word in MSVC, let's just send 4.
        
        if let Some(ref ns) = self.network_stream {
            ns.send(header_packet_size, &header_packet);

            for victim in &self.victims {
                ns.send_sync_position_element_packet(
                    victim.vid,
                    victim.pixel_x,
                    victim.pixel_y,
                );
            }

            ns.send_sequence();
        }
        
        self.victims.clear();
    }
}

impl PlayerEventHandler {
    pub fn on_clear_affects(&self) {
        if let Some(ref pl) = self.player {
            pl.clear_affects();
        }
    }

    pub fn on_set_affect(&self, affect: u32) {
        if let Some(ref pl) = self.player {
            pl.set_affect(affect);
        }
    }

    pub fn on_reset_affect(&self, affect: u32) {
        if let Some(ref pl) = self.player {
            pl.reset_affect(affect);
        }
    }

    pub fn on_update(&self) {
        // empty in C++
    }

    pub fn on_change_shape(&self) {
        if let Some(ref pl) = self.player {
            pl.new_stop();
        }
    }

    pub fn get_normal_bow_attack_fly_event_handler(
        &self,
        p_inst_main: Rc<RefCell<dyn InstanceBase>>,
        p_inst_target: Rc<RefCell<dyn InstanceBase>>,
    ) -> NormalBowAttackFlyEventHandlerAutoClear {
        let mut handler = NormalBowAttackFlyEventHandlerAutoClear::default();
        handler.set(self.clone(), p_inst_main, p_inst_target);
        handler
    }

    pub fn change_fly_target(
        handler: &mut NormalBowAttackFlyEventHandlerAutoClear,
        p_inst_target: Rc<RefCell<dyn InstanceBase>>,
    ) {
        handler.set_target(p_inst_target);
    }
}

pub struct NormalBowAttackFlyEventHandlerAutoClear {
    p_parent: Option<PlayerEventHandler>,
    p_inst_main: Option<Rc<RefCell<dyn InstanceBase>>>,
    p_inst_target: Option<Rc<RefCell<dyn InstanceBase>>>,
}

impl Default for NormalBowAttackFlyEventHandlerAutoClear {
    fn default() -> Self {
        Self {
            p_parent: None,
            p_inst_main: None,
            p_inst_target: None,
        }
    }
}

impl FlyEventHandler for NormalBowAttackFlyEventHandlerAutoClear {
    fn on_set_fly_target(&mut self) {
        let p_inst_main = self.p_inst_main.as_ref().unwrap().borrow();
        let p_inst_target = self.p_inst_target.as_ref().unwrap().borrow();
        
        let p_pos_self = p_inst_main.new_get_pixel_position();
        let adv_rot_self = p_inst_main.get_graphic_thing_instance_ptr().borrow().get_target_rotation();

        let _state = State {
            p_pos_self,
            adv_rot_self,
        };

        if let Some(p_parent) = &self.p_parent {
            if let Some(ref ns) = p_parent.network_stream {
                ns.send_fly_targeting_packet(
                    p_inst_target.get_virtual_id(),
                    p_inst_target.get_graphic_thing_instance_ptr().borrow().on_get_fly_target_position(),
                );
            }
        }
    }

    fn on_shoot(&mut self, skill_index: u32) {
        if let Some(p_parent) = &self.p_parent {
            if let Some(ref ns) = p_parent.network_stream {
                ns.send_shoot_packet(skill_index);
            }
        }
    }

    fn on_no_target(&mut self) {
        // empty
    }

    fn on_exploding_out_of_range(&mut self) {
        // empty
    }

    fn on_exploding_at_background(&mut self) {
        // empty
    }

    fn on_exploding_at_another_target(&mut self, _skill_index: u32, _vid: u32) {
        // Return is the very first line of function in C++, logic below is skipped.
        // trace is commented out or unreachable
        return;
    }

    fn on_exploding_at_target(&mut self, _skill_index: u32) {
        // empty
    }
}

impl NormalBowAttackFlyEventHandlerAutoClear {
    pub fn set(
        &mut self,
        p_parent: PlayerEventHandler,
        p_inst_main: Rc<RefCell<dyn InstanceBase>>,
        p_inst_target: Rc<RefCell<dyn InstanceBase>>,
    ) {
        self.p_parent = Some(p_parent);
        self.p_inst_main = Some(p_inst_main);
        self.p_inst_target = Some(p_inst_target);
    }

    pub fn set_target(&mut self, p_inst_target: Rc<RefCell<dyn InstanceBase>>) {
        self.p_inst_target = Some(p_inst_target);
    }
}
