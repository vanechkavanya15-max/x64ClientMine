//! Modul `instance_motion` — Kolejka animacji i trybow broni aktora
//! Wzorzec referencyjny C++: `source/UserInterface/InstanceBaseMotion.cpp oraz source/GameLib/ActorInstanceMotion.cpp`
//! Opis: Odwzorowanie kolejki animacji: tryby broni (GENERAL, ONEHAND_SWORD, TWOHAND_SWORD, BOW, BELL, FAN, HORSE), stany WAIT, WALK, RUN, COMBO_ATTACK, DEAD.

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionMode {
    Reserved = 0,
    General = 1,
    OnehandSword = 2,
    TwohandSword = 3,
    DualhandSword = 4,
    Bow = 5,
    Fan = 6,
    Bell = 7,
    Fishing = 8,
    Horse = 9,
    HorseOnehandSword = 10,
    HorseTwohandSword = 11,
    HorseDualhandSword = 12,
    HorseBow = 13,
    HorseFan = 14,
    HorseBell = 15,
    WeddingDress = 16,
    MaxNum = 17,
}

impl Default for MotionMode {
    fn default() -> Self {
        MotionMode::General
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionName {
    None = 0,
    Wait = 1,
    Walk = 2,
    Run = 3,
    ChangeWeapon = 4,
    Damage = 5,
    DamageFlying = 6,
    StandUp = 7,
    DamageBack = 8,
    DamageFlyingBack = 9,
    StandUpBack = 10,
    Dead = 11,
    DeadBack = 12,
    NormalAttack = 13,
    ComboAttack1 = 14,
    ComboAttack2 = 15,
    ComboAttack3 = 16,
    ComboAttack4 = 17,
    ComboAttack5 = 18,
    ComboAttack6 = 19,
    ComboAttack7 = 20,
    ComboAttack8 = 21,
    IntroWait = 22,
    IntroSelected = 23,
    IntroNotSelected = 24,
    Spawn = 25,
    FishingThrow = 26,
    FishingWait = 27,
    FishingStop = 28,
    FishingReact = 29,
    FishingCatch = 30,
    FishingFail = 31,
    Stop = 32,
    Special1 = 33,
    Special2 = 34,
    Special3 = 35,
    Special4 = 36,
    Special5 = 37,
    Special6 = 38,
    Skill = 50,
}

impl Default for MotionName {
    fn default() -> Self {
        MotionName::None
    }
}

impl MotionName {
    pub fn from_u32(val: u32) -> Self {
        match val {
            0 => MotionName::None,
            1 => MotionName::Wait,
            2 => MotionName::Walk,
            3 => MotionName::Run,
            4 => MotionName::ChangeWeapon,
            5 => MotionName::Damage,
            6 => MotionName::DamageFlying,
            7 => MotionName::StandUp,
            8 => MotionName::DamageBack,
            9 => MotionName::DamageFlyingBack,
            10 => MotionName::StandUpBack,
            11 => MotionName::Dead,
            12 => MotionName::DeadBack,
            13 => MotionName::NormalAttack,
            14 => MotionName::ComboAttack1,
            15 => MotionName::ComboAttack2,
            16 => MotionName::ComboAttack3,
            17 => MotionName::ComboAttack4,
            18 => MotionName::ComboAttack5,
            19 => MotionName::ComboAttack6,
            20 => MotionName::ComboAttack7,
            21 => MotionName::ComboAttack8,
            22 => MotionName::IntroWait,
            23 => MotionName::IntroSelected,
            24 => MotionName::IntroNotSelected,
            25 => MotionName::Spawn,
            26 => MotionName::FishingThrow,
            27 => MotionName::FishingWait,
            28 => MotionName::FishingStop,
            29 => MotionName::FishingReact,
            30 => MotionName::FishingCatch,
            31 => MotionName::FishingFail,
            32 => MotionName::Stop,
            33 => MotionName::Special1,
            34 => MotionName::Special2,
            35 => MotionName::Special3,
            36 => MotionName::Special4,
            37 => MotionName::Special5,
            38 => MotionName::Special6,
            50 => MotionName::Skill,
            _ => MotionName::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionPushType {
    None,
    Once,
    Loop,
}

impl Default for MotionPushType {
    fn default() -> Self {
        MotionPushType::None
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReservingMotionNode {
    pub motion_type: MotionPushType,
    pub motion_key: u32,
    pub start_time: f32,
    pub blend_time: f32,
    pub speed_ratio: f32,
    pub duration: f32,
}

#[derive(Debug, Clone, Default)]
pub struct CurrentMotionNode {
    pub motion_type: MotionPushType,
    pub motion_key: u32,
    pub cur_frame: u32,
    pub frame_count: u32,
    pub start_time: f32,
    pub end_time: f32,
    pub blend_time: f32,
    pub speed_ratio: f32,
    pub skill: u32,
    pub loop_count: i32,
}

#[derive(Debug, Clone, Default)]
pub struct InstanceMotion {
    pub initialized: bool,
    pub motion_mode: MotionMode,
    pub motion_deque: VecDeque<ReservingMotionNode>,
    pub cur_mot_node: CurrentMotionNode,
    pub is_dead: bool,
    pub is_faint: bool,
    pub local_time: f32,
}

impl InstanceMotion {
    pub fn new() -> Self {
        Self {
            initialized: true,
            motion_mode: MotionMode::General,
            motion_deque: VecDeque::new(),
            cur_mot_node: CurrentMotionNode::default(),
            is_dead: false,
            is_faint: false,
            local_time: 0.0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn set_local_time(&mut self, time: f32) {
        self.local_time = time;
    }
    
    pub fn get_local_time(&self) -> f32 {
        self.local_time
    }

    pub fn set_motion_mode(&mut self, mode: MotionMode) {
        self.motion_mode = mode;
    }

    pub fn get_motion_mode(&self) -> MotionMode {
        self.motion_mode
    }

    pub fn clear_motion(&mut self) {
        self.motion_deque.clear();
        self.cur_mot_node = CurrentMotionNode::default();
    }
    
    // Zwraca czas trwania animacji na podstawie klucza (tymczasowy mock)
    pub fn get_motion_duration(&self, _motion_key: u32) -> f32 {
        1.0 // Zastepuje m_pkCurRaceData->GetMotionDuration
    }

    // Oblicza kiedy zaczac kolejna animacje (odpowiada GetLastMotionTime)
    pub fn get_last_motion_time(&self, blend_time: f32) -> f32 {
        if self.motion_deque.is_empty() {
            if self.cur_mot_node.motion_type == MotionPushType::Once {
                return self.cur_mot_node.end_time - blend_time;
            }
            return self.get_local_time();
        }

        if let Some(last_node) = self.motion_deque.back() {
            return last_node.start_time + last_node.duration - blend_time;
        }

        self.get_local_time()
    }

    pub fn push_motion(&mut self, motion_type: MotionPushType, motion_name: MotionName, blend_time: f32, speed_ratio: f32) {
        let motion_key = motion_name as u32; 
        
        let start_time = self.get_last_motion_time(blend_time);
        let duration = self.get_motion_duration(motion_key);

        self.motion_deque.push_back(ReservingMotionNode {
            motion_type,
            motion_key,
            start_time,
            blend_time,
            speed_ratio,
            duration,
        });
    }

    pub fn push_once_motion(&mut self, motion: MotionName, blend_time: f32, speed_ratio: f32) {
        self.push_motion(MotionPushType::Once, motion, blend_time, speed_ratio);
    }

    pub fn push_loop_motion(&mut self, motion: MotionName, blend_time: f32, speed_ratio: f32) {
        self.push_motion(MotionPushType::Loop, motion, blend_time, speed_ratio);
    }

    pub fn set_loop_motion(&mut self, motion: MotionName, blend_time: f32, speed_ratio: f32) {
        self.clear_motion();
        self.push_loop_motion(motion, blend_time, speed_ratio);
    }
    
    pub fn intercept_loop_motion(&mut self, motion: MotionName, blend_time: f32, speed_ratio: f32) {
        // Skrot dla logiki InterceptMotion
        self.clear_motion();
        self.push_loop_motion(motion, blend_time, speed_ratio);
    }

    // Odpowiednik CActorInstance::ReservingMotionProcess
    pub fn reserving_motion_process(&mut self) {
        if self.motion_deque.is_empty() {
            return;
        }
        
        let current_time = self.get_local_time();
        let front = self.motion_deque.front().unwrap().clone();
        
        if front.start_time > current_time {
            return;
        }
        
        let next_motion = MotionName::from_u32(front.motion_key);
        match next_motion {
            MotionName::StandUp | MotionName::StandUpBack => {
                if self.is_faint {
                    // SetEndStopMotion (mock)
                    // Opuznienie pozostalych elementow kolejki
                    for node in self.motion_deque.iter_mut() {
                        node.start_time += 1.0;
                    }
                    return;
                }
            }
            _ => {}
        }
        
        let prev_motion_node = self.cur_mot_node.clone();
        self.motion_deque.pop_front();
        
        match next_motion {
            MotionName::StandUp | MotionName::StandUpBack => {
                if self.is_dead {
                    self.cur_mot_node = prev_motion_node;
                    self.clear_motion();
                    return;
                }
            }
            _ => {}
        }
        
        let i_loop_count = if front.motion_type == MotionPushType::Once { 1 } else { 0 };
        
        // Mock set_motion behavior, assigning duration directly
        let duration = self.get_motion_duration(front.motion_key) / front.speed_ratio;
        let end_time = front.start_time + duration;
        
        self.cur_mot_node.skill = 0;
        self.cur_mot_node.motion_type = front.motion_type;
        self.cur_mot_node.speed_ratio = front.speed_ratio;
        self.cur_mot_node.start_time = front.start_time;
        self.cur_mot_node.end_time = end_time;
        self.cur_mot_node.motion_key = front.motion_key;
        self.cur_mot_node.cur_frame = 0;
        self.cur_mot_node.frame_count = (duration / (1.0 / 60.0)) as u32; // Zalozenie: 60 FPS
        self.cur_mot_node.loop_count = i_loop_count;
    }
    
    // Odpowiednik CActorInstance::CurrentMotionProcess
    pub fn current_motion_process(&mut self) {
        if self.cur_mot_node.motion_type == MotionPushType::Loop {
            if self.cur_mot_node.cur_frame >= self.cur_mot_node.frame_count {
                self.cur_mot_node.cur_frame = 0;
            }
        }
        
        if self.is_dead {
            return;
        }
        
        if !self.motion_deque.is_empty() {
            return;
        }
        
        let current_time = self.get_local_time();
        let motion_index = MotionName::from_u32(self.cur_mot_node.motion_key);
        
        // Brak zlozonych warunkow m_pkCurRaceMotionData->IsLoopMotion(), implementacja oparta o fEndTime
        let is_looping = false; // Mock
        
        if !is_looping {
            if current_time > self.cur_mot_node.end_time {
                if self.cur_mot_node.motion_type == MotionPushType::Once {
                    match motion_index {
                        MotionName::DamageFlying | 
                        MotionName::DamageFlyingBack | 
                        MotionName::Dead | 
                        MotionName::IntroSelected | 
                        MotionName::IntroNotSelected => {
                            self.cur_mot_node.end_time += 3.0;
                            // SetEndStopMotion mock
                        }
                        _ => {
                            self.intercept_loop_motion(MotionName::Wait, 0.1, 1.0);
                        }
                    }
                } else if self.cur_mot_node.motion_type == MotionPushType::Loop {
                    if motion_index == MotionName::Wait {
                        self.push_loop_motion(MotionName::Wait, 0.5, 1.0);
                    }
                }
            }
        }
    }
}
