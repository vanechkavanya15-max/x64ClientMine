use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionMode {
    General = 1,
    Horse = 9,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PixelPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PixelPosition {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

pub struct GraphicThingInstance {
    pub pos: PixelPosition,
    pub stopped: bool,
    pub refreshed: bool,
    pub horse: Option<Rc<RefCell<HorseActorInstance>>>,
    pub move_speed: f32,
    pub attack_speed: f32,
    pub attached_models: Vec<String>,
}

impl GraphicThingInstance {
    pub fn new() -> Self {
        Self {
            pos: PixelPosition::new(0.0, 0.0, 0.0),
            stopped: false,
            refreshed: false,
            horse: None,
            move_speed: 1.0,
            attack_speed: 1.0,
            attached_models: Vec::new(),
        }
    }

    pub fn set_move_speed(&mut self, move_speed: u32) {
        self.move_speed = (move_speed as f32) / 100.0;
    }

    pub fn set_attack_speed(&mut self, attack_speed: u32) {
        self.attack_speed = (attack_speed as f32) / 100.0;
    }

    pub fn get_cur_pixel_position_ref(&self) -> PixelPosition {
        self.pos
    }

    pub fn mount_horse(&mut self, horse: Option<Rc<RefCell<HorseActorInstance>>>) {
        self.horse = horse;
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn refresh_actor_instance(&mut self) {
        self.refreshed = true;
    }

    pub fn attach_model_instance(
        &mut self,
        _part: u32,
        bone_name: &str,
        _target: &GraphicThingInstance,
        _target_part: u32,
    ) {
        if !self.attached_models.iter().any(|b| b == bone_name) {
            self.attached_models.push(bone_name.to_string());
        }
    }

    pub fn detach_model_instance(
        &mut self,
        _part: u32,
        bone_name: &str,
        _target: &GraphicThingInstance,
        _target_part: u32,
    ) {
        self.attached_models.retain(|b| b != bone_name);
    }
}

pub struct HorseActorInstance {
    pub pos: PixelPosition,
    pub race: u32,
    pub hit_effect: u32,
    pub move_speed: f32,
    pub attack_speed: f32,
    pub attached_models: Vec<String>,
}

impl HorseActorInstance {
    pub fn new() -> Self {
        Self {
            pos: PixelPosition::new(0.0, 0.0, 0.0),
            race: 0,
            hit_effect: 0,
            move_speed: 1.0,
            attack_speed: 1.0,
            attached_models: Vec::new(),
        }
    }

    pub fn get_race(&self) -> u32 {
        self.race
    }

    pub fn set_move_speed(&mut self, speed: f32) {
        self.move_speed = speed;
    }

    pub fn set_attack_speed(&mut self, speed: f32) {
        self.attack_speed = speed;
    }

    pub fn attach_model_instance(
        &mut self,
        _part: u32,
        bone_name: &str,
        _target: &GraphicThingInstance,
        _target_part: u32,
    ) {
        if !self.attached_models.iter().any(|b| b == bone_name) {
            self.attached_models.push(bone_name.to_string());
        }
    }

    pub fn detach_model_instance(
        &mut self,
        _part: u32,
        bone_name: &str,
        _target: &GraphicThingInstance,
        _target_part: u32,
    ) {
        self.attached_models.retain(|b| b != bone_name);
    }
}

pub struct SHorse {
    pub is_mounting: bool,
    pub actor: Option<Rc<RefCell<HorseActorInstance>>>,
}

impl SHorse {
    pub fn new() -> Self {
        Self {
            is_mounting: false,
            actor: None,
        }
    }

    pub fn destroy(&mut self) {
        self.actor = None;
        self.is_mounting = false;
    }

    pub fn create(&mut self, pos: PixelPosition, race: u32, hit_effect: u32) {
        if self.actor.is_some() {
            panic!("ALREADY MOUNT");
        }

        let mut actor = HorseActorInstance::new();
        actor.race = race;
        actor.hit_effect = hit_effect;
        actor.pos = pos;

        self.actor = Some(Rc::new(RefCell::new(actor)));
        self.is_mounting = true;
    }

    pub fn get_actor_ptr(&self) -> Option<Rc<RefCell<HorseActorInstance>>> {
        self.actor.clone()
    }

    pub fn is_mounting(&self) -> bool {
        self.is_mounting
    }

    pub fn get_level(&self) -> u32 {
        if let Some(actor) = &self.actor {
            let mount = actor.borrow().get_race();
            match mount {
                20101..=20103 => return 1,
                20104..=20106 => return 2,
                20107..=20118 | 20120..=20125 => return 3,
                20119 | 20219..=20222 => return 2,
                20205..=20208 | 20214 | 20217 | 20224 | 20229 => return 2,
                20209..=20212 | 20215 | 20218 | 20220 | 20225 | 20230 => return 3,
                _ => {}
            }
        }
        0
    }

    pub fn is_new_mount(&self) -> bool {
        if let Some(actor) = &self.actor {
            let mount = actor.borrow().get_race();
            match mount {
                20205..=20208 | 20214 | 20217 => return true,
                20209..=20212 | 20215 | 20218 | 20220 => return true,
                _ => {}
            }
        }
        false
    }

    pub fn set_move_speed(&mut self, move_speed: u32) {
        if !self.is_mounting() {
            return;
        }
        if let Some(actor) = &self.actor {
            actor
                .borrow_mut()
                .set_move_speed((move_speed as f32) / 100.0);
        }
    }

    pub fn set_attack_speed(&mut self, attack_speed: u32) {
        if !self.is_mounting() {
            return;
        }
        if let Some(actor) = &self.actor {
            actor
                .borrow_mut()
                .set_attack_speed((attack_speed as f32) / 100.0);
        }
    }
}

// EFFECT_HIT is enum value 0 in ms_adwCRCAffectEffect
pub const EFFECT_HIT_CRC_INDEX: usize = 0;

pub struct InstanceBase {
    pub horse: SHorse,
    pub graphic_thing_instance: GraphicThingInstance,
    pub motion_mode: MotionMode,
    pub rotation_speed: f32,
    pub default_horse_rotation_speed: f32,
    pub ms_adw_crc_affect_effect: [u32; 1],
    pub base_move_speed: f32,
    pub base_attack_speed: f32,
}

impl InstanceBase {
    pub fn new(hit_effect_crc: u32) -> Self {
        Self {
            horse: SHorse::new(),
            graphic_thing_instance: GraphicThingInstance::new(),
            motion_mode: MotionMode::General,
            rotation_speed: 1.0,
            default_horse_rotation_speed: 1.5,
            ms_adw_crc_affect_effect: [hit_effect_crc],
            base_move_speed: 1.0,
            base_attack_speed: 1.0,
        }
    }

    pub fn is_mounting_horse(&self) -> bool {
        self.horse.is_mounting()
    }

    pub fn is_new_mount(&self) -> bool {
        self.horse.is_new_mount()
    }

    pub fn mount_horse(&mut self, race: u32) {
        self.horse.destroy();
        self.horse.create(
            self.graphic_thing_instance.get_cur_pixel_position_ref(),
            race,
            self.ms_adw_crc_affect_effect[EFFECT_HIT_CRC_INDEX],
        );

        self.motion_mode = MotionMode::Horse;
        self.rotation_speed = self.default_horse_rotation_speed;

        self.graphic_thing_instance
            .mount_horse(self.horse.get_actor_ptr());
        self.graphic_thing_instance.stop();
        self.graphic_thing_instance.refresh_actor_instance();

        self.attach_horse_saddle();
    }

    pub fn dismount_horse(&mut self) {
        self.detach_horse_saddle();
        self.horse.destroy();
        self.graphic_thing_instance.mount_horse(None);
        self.motion_mode = MotionMode::General;
        self.rotation_speed = 1.0;
    }

    pub fn attach_horse_saddle(&mut self) {
        if !self.is_mounting_horse() {
            return;
        }

        if let Some(actor) = self.horse.get_actor_ptr() {
            actor
                .borrow_mut()
                .attach_model_instance(0, "saddle", &self.graphic_thing_instance, 0);
        }
    }

    pub fn detach_horse_saddle(&mut self) {
        if !self.is_mounting_horse() {
            return;
        }

        if let Some(actor) = self.horse.get_actor_ptr() {
            actor
                .borrow_mut()
                .detach_model_instance(0, "saddle", &self.graphic_thing_instance, 0);
        }
    }

    pub fn set_move_speed(&mut self, move_speed: u32) {
        self.base_move_speed = (move_speed as f32) / 100.0;
        self.graphic_thing_instance.set_move_speed(move_speed);
        self.horse.set_move_speed(move_speed);
    }

    pub fn set_attack_speed(&mut self, attack_speed: u32) {
        self.base_attack_speed = (attack_speed as f32) / 100.0;
        self.graphic_thing_instance.set_attack_speed(attack_speed);
        self.horse.set_attack_speed(attack_speed);
    }

    pub fn can_attack_horse_level(&self) -> bool {
        if self.is_mounting_horse() {
            if self.horse.get_level() <= 1 {
                return false;
            }
        }
        true
    }
}
