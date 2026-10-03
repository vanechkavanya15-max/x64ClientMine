#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PixelPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PixelPosition {
    pub fn distance(&self, other: &PixelPosition) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MoveAfterFunc {
    pub e_func: u32,
    pub u_arg: u32,
    pub u_arg_expanded: u32,
    pub pos_dst: PixelPosition,
}

pub const FUNC_WAIT: u32 = 0;
pub const FUNC_MOVE: u32 = 1;
pub const FUNC_ATTACK: u32 = 2;
pub const FUNC_COMBO: u32 = 3;
pub const FUNC_MOB_SKILL: u32 = 4;
pub const FUNC_EMOTION: u32 = 5;
pub const FUNC_SKILL: u32 = 0x80;

pub const NAME_WAIT: u32 = 0;
pub const NAME_WALK: u32 = 1;
pub const NAME_RUN: u32 = 2;

pub const SKILL_NUM: u32 = 255;
pub const NAME_SKILL: u32 = 50;
pub const NAME_SKILL_END: u32 = NAME_SKILL + SKILL_NUM;

// From CRaceMotionData::NAME_*
pub const NAME_CLAP: u32 = NAME_SKILL_END + 1;
pub const NAME_CHEERS_1: u32 = NAME_CLAP + 1;
pub const NAME_CHEERS_2: u32 = NAME_CHEERS_1 + 1;

pub const NAME_KISS_START: u32 = NAME_CHEERS_2 + 1;
pub const NAME_KISS_WITH_WARRIOR: u32 = NAME_KISS_START + 0;
pub const NAME_KISS_WITH_ASSASSIN: u32 = NAME_KISS_START + 1;
pub const NAME_KISS_WITH_SURA: u32 = NAME_KISS_START + 2;
pub const NAME_KISS_WITH_SHAMAN: u32 = NAME_KISS_START + 3;

pub const NAME_FRENCH_KISS_START: u32 = NAME_KISS_START + 4;
pub const NAME_FRENCH_KISS_WITH_WARRIOR: u32 = NAME_FRENCH_KISS_START + 0;
pub const NAME_FRENCH_KISS_WITH_ASSASSIN: u32 = NAME_FRENCH_KISS_START + 1;
pub const NAME_FRENCH_KISS_WITH_SURA: u32 = NAME_FRENCH_KISS_START + 2;
pub const NAME_FRENCH_KISS_WITH_SHAMAN: u32 = NAME_FRENCH_KISS_START + 3;

pub const NAME_SLAP_HIT_START: u32 = NAME_FRENCH_KISS_START + 4;
pub const NAME_SLAP_HIT_WITH_WARRIOR: u32 = NAME_SLAP_HIT_START + 0;
pub const NAME_SLAP_HIT_WITH_ASSASSIN: u32 = NAME_SLAP_HIT_START + 1;
pub const NAME_SLAP_HIT_WITH_SURA: u32 = NAME_SLAP_HIT_START + 2;
pub const NAME_SLAP_HIT_WITH_SHAMAN: u32 = NAME_SLAP_HIT_START + 3;

pub const NAME_SLAP_HURT_START: u32 = NAME_SLAP_HIT_START + 4;
pub const NAME_SLAP_HURT_WITH_WARRIOR: u32 = NAME_SLAP_HURT_START + 0;
pub const NAME_SLAP_HURT_WITH_ASSASSIN: u32 = NAME_SLAP_HURT_START + 1;
pub const NAME_SLAP_HURT_WITH_SURA: u32 = NAME_SLAP_HURT_START + 2;
pub const NAME_SLAP_HURT_WITH_SHAMAN: u32 = NAME_SLAP_HURT_START + 3;

pub const NAME_DIG: u32 = NAME_SLAP_HURT_START + 4;

pub const NAME_DANCE_1: u32 = NAME_DIG + 1;
pub const NAME_DANCE_2: u32 = NAME_DANCE_1 + 1;
pub const NAME_DANCE_3: u32 = NAME_DANCE_1 + 2;
pub const NAME_DANCE_4: u32 = NAME_DANCE_1 + 3;
pub const NAME_DANCE_5: u32 = NAME_DANCE_1 + 4;
pub const NAME_DANCE_6: u32 = NAME_DANCE_1 + 5;
pub const NAME_DANCE_END: u32 = NAME_DANCE_1 + 16;

pub const NAME_CONGRATULATION: u32 = NAME_DANCE_END + 1;
pub const NAME_FORGIVE: u32 = NAME_CONGRATULATION + 1;
pub const NAME_ANGRY: u32 = NAME_FORGIVE + 1;
pub const NAME_ATTRACTIVE: u32 = NAME_ANGRY + 1;
pub const NAME_SAD: u32 = NAME_ATTRACTIVE + 1;
pub const NAME_SHY: u32 = NAME_SAD + 1;
pub const NAME_CHEERUP: u32 = NAME_SHY + 1;
pub const NAME_BANTER: u32 = NAME_CHEERUP + 1;
pub const NAME_JOY: u32 = NAME_BANTER + 1;

pub fn race_to_job(race: u32) -> u32 {
    let job_num = 4;
    race % job_num
}

#[derive(Debug, Clone, Default)]
pub struct CharacterEmotionState {
    pub vid: u32,
    pub race: u32,
    pub is_main_instance: bool,
    pub is_waiting: bool,
    pub is_walking: bool,
    pub cur_pixel_position: PixelPosition,
    pub tcp_state_enabled: bool,
    
    pub current_motion: u32,
    pub motion_loop: bool,
    pub motion_speed_ratio: f32,
    
    pub rotation: f32,
    pub advancing_rotation: f32,
    pub target_rotation: f32,

    pub tcp_move_after_func: MoveAfterFunc,
}


impl CharacterEmotionState {
    pub fn new(vid: u32, race: u32, is_main: bool) -> Self {
        Self {
            vid,
            race,
            is_main_instance: is_main,
            ..Default::default()
        }
    }

    pub fn act_emotion(&mut self, motion_number: u32) {
        self.current_motion = motion_number;
        self.motion_loop = false;
        // Simplified pushing motion logic
    }

    pub fn is_acting_emotion(&self) -> bool {
        // According to IsActEmotion in C++
        match self.current_motion {
            NAME_FRENCH_KISS_WITH_WARRIOR..=NAME_FRENCH_KISS_WITH_SHAMAN |
            NAME_KISS_WITH_WARRIOR..=NAME_KISS_WITH_SHAMAN => true,
            _ => false,
        }
    }

    pub fn process_function_emotion(
        &mut self,
        motion_number: u32,
        target_vid: u32,
        pos_dst: PixelPosition,
        target_instance: &mut CharacterEmotionState,
    ) {
        if self.is_walking {
            self.is_walking = false;
        }

        self.tcp_state_enabled = true;
        self.cur_pixel_position = pos_dst;

        if target_instance.vid == target_vid {
            target_instance.tcp_state_enabled = true;
            if target_instance.is_walking {
                target_instance.is_walking = false;
            }

            let motion_number_1 = (motion_number >> 16) & 0xFFFF;
            let motion_number_2 = motion_number & 0xFFFF;

            let src_job = race_to_job(self.race);
            let dst_job = race_to_job(target_instance.race);

            // Set motions with proper job offsets
            self.current_motion = motion_number_1 + dst_job;
            self.motion_loop = false;

            target_instance.current_motion = motion_number_2 + src_job;
            target_instance.motion_loop = false;
            
            // Note: actual look_at rotation and advancing_rotation are handled conceptually
            // For testing logic, we just modify the states above.
        }
    }

    pub fn act_dual_emotion(
        &mut self,
        target_instance: &mut CharacterEmotionState,
        motion_number_1: u16,
        motion_number_2: u16,
        server_time: u32,
    ) {
        if !self.is_waiting {
            self.current_motion = NAME_WAIT;
            self.motion_loop = true;
        }
        if !target_instance.is_waiting {
            target_instance.current_motion = NAME_WAIT;
            target_instance.motion_loop = true;
        }

        let emotion_distance = 100.0f32;
        let dx = self.cur_pixel_position.x - target_instance.cur_pixel_position.x;
        let dy = self.cur_pixel_position.y - target_instance.cur_pixel_position.y;
        
        let mut distance = (dx * dx + dy * dy).sqrt();
        if distance == 0.0 {
            distance = 1.0;
        }

        let mut dst_position = PixelPosition::default();
        dst_position.x = target_instance.cur_pixel_position.x + (dx / distance) * emotion_distance;
        dst_position.y = target_instance.cur_pixel_position.y + (dy / distance) * emotion_distance;

        // dwCurTime = serverTime + 500
        let _cur_time = server_time + 500;

        let motion_combined = ((motion_number_1 as u32) << 16) | (motion_number_2 as u32);
        
        self.tcp_move_after_func = MoveAfterFunc {
            e_func: FUNC_EMOTION,
            u_arg: motion_combined,
            u_arg_expanded: target_instance.vid,
            pos_dst: dst_position,
        };

        self.tcp_state_enabled = false;
        target_instance.tcp_state_enabled = false;

        // Conceptually, StartEmotionProcess() on main instance is called here if either is main.
    }
}

#[no_mangle]
pub extern "C" fn m2_character_act_emotion(
    state: *mut CharacterEmotionState,
    motion_number: u32,
) {
    if state.is_null() {
        return;
    }
    unsafe {
        (*state).act_emotion(motion_number);
    }
}

#[no_mangle]
pub extern "C" fn m2_character_is_acting_emotion(state: *const CharacterEmotionState) -> bool {
    if state.is_null() {
        return false;
    }
    unsafe { (*state).is_acting_emotion() }
}

#[no_mangle]
pub extern "C" fn m2_character_process_function_emotion(
    state: *mut CharacterEmotionState,
    motion_number: u32,
    target_vid: u32,
    pos_dst: PixelPosition,
    target_state: *mut CharacterEmotionState,
) {
    if state.is_null() || target_state.is_null() {
        return;
    }
    unsafe {
        (*state).process_function_emotion(
            motion_number,
            target_vid,
            pos_dst,
            &mut *target_state,
        );
    }
}

#[no_mangle]
pub extern "C" fn m2_character_act_dual_emotion(
    state: *mut CharacterEmotionState,
    target_state: *mut CharacterEmotionState,
    motion_number_1: u16,
    motion_number_2: u16,
    server_time: u32,
) {
    if state.is_null() || target_state.is_null() {
        return;
    }
    unsafe {
        (*state).act_dual_emotion(
            &mut *target_state,
            motion_number_1,
            motion_number_2,
            server_time,
        );
    }
}
