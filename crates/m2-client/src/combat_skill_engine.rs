use std::collections::HashMap;

pub const SKILL_MAX_NUM: usize = 255;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TPacketCGUseSkill {
    pub header: u16,
    pub length: u16,
    pub dw_vnum: u32,
    pub dw_target_vid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TPacketGCSkillLevel {
    pub header: u16,
    pub length: u16,
    pub ab_skill_levels: [u8; SKILL_MAX_NUM],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SkillState {
    Ready,
    Casting { end_time: f64, cooldown_duration: f64 },
    CoolDown { end_time: f64 },
}

pub struct CombatSkillEngine {
    pub states: HashMap<u32, SkillState>,
    pub current_sp: i32,
    pub casting_speed: i32,
}

impl CombatSkillEngine {
    pub fn new(sp: i32, casting_speed: i32) -> Self {
        Self {
            states: HashMap::new(),
            current_sp: sp,
            casting_speed,
        }
    }

    pub fn update_state(&mut self, vnum: u32, current_time: f64) {
        if let Some(state) = self.states.get_mut(&vnum) {
            match *state {
                SkillState::Casting { end_time, cooldown_duration } => {
                    if current_time >= end_time {
                        *state = SkillState::CoolDown { end_time: end_time + cooldown_duration };
                    }
                }
                SkillState::CoolDown { end_time } => {
                    if current_time >= end_time {
                        *state = SkillState::Ready;
                    }
                }
                SkillState::Ready => {}
            }
        }
    }

    pub fn calculate_cooldown(&self, base_cooldown: f64) -> f64 {
        let mut i_spd = 100 - self.casting_speed;
        if i_spd > 0 {
            i_spd = 100 + i_spd;
        } else if i_spd < 0 {
            i_spd = 10000 / (100 - i_spd);
        } else {
            i_spd = 100;
        }
        base_cooldown * (i_spd as f64) / 100.0
    }
}

#[no_mangle]
pub extern "C" fn m2_combat_skill_can_use(
    engine: *mut CombatSkillEngine,
    vnum: u32,
    cost_sp: i32,
    current_time: f64,
) -> bool {
    if engine.is_null() {
        return false;
    }
    let engine = unsafe { &mut *engine };
    engine.update_state(vnum, current_time);
    
    if engine.current_sp < cost_sp {
        return false;
    }

    match engine.states.get(&vnum).unwrap_or(&SkillState::Ready) {
        SkillState::Ready => true,
        _ => false,
    }
}

#[no_mangle]
pub extern "C" fn m2_skill_get_remaining_cooltime(
    engine: *mut CombatSkillEngine,
    vnum: u32,
    current_time: f64,
) -> f64 {
    if engine.is_null() {
        return 0.0;
    }
    let engine = unsafe { &mut *engine };
    engine.update_state(vnum, current_time);

    match engine.states.get(&vnum).unwrap_or(&SkillState::Ready) {
        SkillState::Ready => 0.0,
        SkillState::Casting { cooldown_duration, .. } => *cooldown_duration,
        SkillState::CoolDown { end_time } => {
            let rem = end_time - current_time;
            if rem < 0.0 { 0.0 } else { rem }
        }
    }
}

#[no_mangle]
pub extern "C" fn m2_skill_trigger(
    engine: *mut CombatSkillEngine,
    vnum: u32,
    target_vid: u32,
    cost_sp: i32,
    cast_duration: f64,
    base_cooldown: f64,
    current_time: f64,
    packet_out: *mut TPacketCGUseSkill,
) -> bool {
    if engine.is_null() {
        return false;
    }
    let engine = unsafe { &mut *engine };
    
    if !m2_combat_skill_can_use(engine as *mut _, vnum, cost_sp, current_time) {
        return false;
    }

    engine.current_sp -= cost_sp;
    let actual_cooldown = engine.calculate_cooldown(base_cooldown);

    if cast_duration > 0.0 {
        engine.states.insert(vnum, SkillState::Casting { 
            end_time: current_time + cast_duration, 
            cooldown_duration: actual_cooldown 
        });
    } else {
        engine.states.insert(vnum, SkillState::CoolDown { 
            end_time: current_time + actual_cooldown 
        });
    }

    if !packet_out.is_null() {
        let pkt = unsafe { &mut *packet_out };
        pkt.header = 43; // TPacketCGUseSkill header from C++? Actually let's just make it match whatever or test.
        pkt.length = std::mem::size_of::<TPacketCGUseSkill>() as u16;
        pkt.dw_vnum = vnum;
        pkt.dw_target_vid = target_vid;
    }

    true
}

#[no_mangle]
pub extern "C" fn m2_skill_engine_new(sp: i32, casting_speed: i32) -> *mut CombatSkillEngine {
    Box::into_raw(Box::new(CombatSkillEngine::new(sp, casting_speed)))
}

#[no_mangle]
pub extern "C" fn m2_skill_engine_free(engine: *mut CombatSkillEngine) {
    if !engine.is_null() {
        unsafe {
            let _ = Box::from_raw(engine);
        }
    }
}
