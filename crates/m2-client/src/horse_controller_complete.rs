#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum RaceMotionMode {
    Reserved = 0,
    General = 1,
    OneHandSword = 2,
    TwoHandSword = 3,
    DualHandSword = 4,
    Bow = 5,
    Fan = 6,
    Bell = 7,
    Fishing = 8,
    Horse = 9,
    HorseOneHandSword = 10,
    HorseTwoHandSword = 11,
    HorseDualHandSword = 12,
    HorseBow = 13,
    HorseFan = 14,
    HorseBell = 15,
    WeddingDress = 16,
    MaxNum = 17,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct HorseActorInstance {
    pub race: u32,
    pub shape: u32,
    pub battle_hit_effect: u32,
    pub alpha_value: f32,
    pub move_speed: f32,
    pub attack_speed: f32,
    pub motion_mode: RaceMotionMode,
    pub stopped: bool,
    pub refreshed: bool,
    pub cur_pixel_position: PixelPosition,
}

impl HorseActorInstance {
    pub fn new() -> Self {
        Self {
            race: 0,
            shape: 0,
            battle_hit_effect: 0,
            alpha_value: 1.0,
            move_speed: 1.0,
            attack_speed: 1.0,
            motion_mode: RaceMotionMode::General,
            stopped: false,
            refreshed: false,
            cur_pixel_position: PixelPosition::new(0.0, 0.0, 0.0),
        }
    }

    pub fn set_race(&mut self, race: u32) -> bool {
        self.race = race;
        true
    }

    pub fn get_race(&self) -> u32 {
        self.race
    }

    pub fn set_shape(&mut self, shape: u32) {
        self.shape = shape;
    }

    pub fn set_battle_hit_effect(&mut self, hit_effect: u32) {
        self.battle_hit_effect = hit_effect;
    }

    pub fn set_alpha_value(&mut self, alpha: f32) {
        self.alpha_value = alpha;
    }

    pub fn blend_alpha_value(&mut self, _target: f32, _time: f32) {
        // stub as requested by the original engine logic
    }

    pub fn set_move_speed(&mut self, speed: f32) {
        self.move_speed = speed;
    }

    pub fn set_attack_speed(&mut self, speed: f32) {
        self.attack_speed = speed;
    }

    pub fn set_motion_mode(&mut self, mode: RaceMotionMode) {
        self.motion_mode = mode;
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn refresh_actor_instance(&mut self) {
        self.refreshed = true;
    }

    pub fn set_cur_pixel_position(&mut self, pos: PixelPosition) {
        self.cur_pixel_position = pos;
    }

    pub fn destroy(&mut self) {
        // Reset state
        *self = Self::new();
    }

    pub fn instance_base_deform(&mut self) {
        // Perform deformation logic
    }

    pub fn render(&mut self) {
        // Render
    }
}

#[derive(Debug)]
pub struct SHorse {
    is_mounting: bool,
    actor: Option<Box<HorseActorInstance>>,
}

impl Default for SHorse {
    fn default() -> Self {
        Self::new()
    }
}

impl SHorse {
    pub fn new() -> Self {
        Self {
            is_mounting: false,
            actor: None,
        }
    }

    pub fn initialize(&mut self) {
        self.is_mounting = false;
        self.actor = None;
    }

    pub fn destroy(&mut self) {
        if let Some(mut actor) = self.actor.take() {
            actor.destroy();
        }
        self.initialize();
    }

    pub fn create(&mut self, pos: PixelPosition, race: u32, hit_effect: u32) {
        assert!(
            self.actor.is_none(),
            "CInstanceBase::SHORSE::Create - ALREADY MOUNT"
        );

        let mut actor = Box::new(HorseActorInstance::new());
        if !actor.set_race(race) {
            self.actor = None;
            return;
        }

        actor.set_shape(0);
        actor.set_battle_hit_effect(hit_effect);
        actor.set_alpha_value(0.0);
        actor.blend_alpha_value(1.0, 0.5);
        actor.set_move_speed(1.0);
        actor.set_attack_speed(1.0);
        actor.set_motion_mode(RaceMotionMode::General);
        actor.stop();
        actor.refresh_actor_instance();
        actor.set_cur_pixel_position(pos);

        self.actor = Some(actor);
        self.is_mounting = true;
    }

    pub fn set_attack_speed(&mut self, atk_spd: u32) {
        if !self.is_mounting() {
            return;
        }
        if let Some(actor) = &mut self.actor {
            actor.set_attack_speed((atk_spd as f32) / 100.0);
        }
    }

    pub fn set_move_speed(&mut self, mov_spd: u32) {
        if !self.is_mounting() {
            return;
        }
        if let Some(actor) = &mut self.actor {
            actor.set_move_speed((mov_spd as f32) / 100.0);
        }
    }

    pub fn deform(&mut self) {
        if !self.is_mounting() {
            return;
        }
        if let Some(actor) = &mut self.actor {
            actor.instance_base_deform();
        }
    }

    pub fn render(&mut self) {
        if !self.is_mounting() {
            return;
        }
        if let Some(actor) = &mut self.actor {
            actor.render();
        }
    }

    pub fn get_actor_ref(&mut self) -> &mut HorseActorInstance {
        assert!(self.actor.is_some(), "CInstanceBase::SHORSE::GetActorRef");
        self.actor.as_mut().unwrap()
    }

    pub fn get_actor_ptr(&mut self) -> Option<&mut HorseActorInstance> {
        self.actor.as_deref_mut()
    }

    pub fn is_mounting(&self) -> bool {
        self.is_mounting
    }

    pub fn can_attack(&self) -> bool {
        if self.is_mounting() {
            if self.get_level() <= 1 {
                return false;
            }
        }
        true
    }

    pub fn can_use_skill(&self) -> bool {
        if self.is_mounting() {
            return 2 < self.get_level();
        }
        true
    }

    pub fn get_level(&self) -> u32 {
        if let Some(actor) = &self.actor {
            let mount = actor.get_race();
            match mount {
                20101 | 20102 | 20103 => return 1,
                20104 | 20105 | 20106 => return 2,
                20107 | 20108 | 20109 | 20110 | 20111 | 20112 | 20113 | 20114 | 20115 | 20116
                | 20117 | 20118 | 20120 | 20121 | 20122 | 20123 | 20124 | 20125 => return 3,
                20119 | 20219 | 20220 | 20221 | 20222 => return 2,
                _ => {}
            }

            if (20205 <= mount && mount <= 20208)
                || mount == 20214
                || mount == 20217
                || mount == 20224
                || mount == 20229
            {
                return 2;
            }

            if (20209 <= mount && mount <= 20212)
                || mount == 20215
                || mount == 20218
                || mount == 20220
                || mount == 20225
                || mount == 20230
            {
                return 3;
            }
        }
        0
    }

    pub fn is_new_mount(&self) -> bool {
        if let Some(actor) = &self.actor {
            let mount = actor.get_race();

            if (20205 <= mount && mount <= 20208) || mount == 20214 || mount == 20217 {
                return true;
            }

            if (20209 <= mount && mount <= 20212)
                || mount == 20215
                || mount == 20218
                || mount == 20220
            {
                return true;
            }
        }
        false
    }
}
