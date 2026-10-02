use std::mem::size_of;
use m2_client_bridge::state::game_state::GLOBAL_GAME_STATE;
use std::sync::LazyLock;
use std::sync::RwLock;
use std::collections::HashSet;
use std::collections::HashMap;

pub const SKILL_MAX_NUM: usize = 255;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct TPacketGCDamageInfo {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub flag: u8,
    pub damage: i32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct TPacketGCFlyTargeting {
    pub header: u16,
    pub length: u16,
    pub dwShooterVID: u32,
    pub dwTargetVID: u32,
    pub lX: i32,
    pub lY: i32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct TPacketGCCreateFly {
    pub header: u16,
    pub length: u16,
    pub bType: u8,
    pub dwStartVID: u32,
    pub dwEndVID: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct TPacketGCPVP {
    pub header: u16,
    pub length: u16,
    pub dwVIDSrc: u32,
    pub dwVIDDst: u32,
    pub bMode: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct TPacketGCDuelStart {
    pub header: u16,
    pub length: u16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct TPacketGCSkillLevel {
    pub header: u16,
    pub length: u16,
    pub abSkillLevels: [u8; SKILL_MAX_NUM],
}

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct TPlayerSkill {
    pub bMasterType: u8,
    pub bLevel: u8,
    pub tNextRead: u64, // time_t equivalent
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct TPacketGCSkillLevelNew {
    pub header: u16,
    pub length: u16,
    pub skills: [TPlayerSkill; SKILL_MAX_NUM],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct TPacketGCSkillCoolTimeEnd {
    pub header: u16,
    pub length: u16,
    pub bSkill: u8,
}

// Full business logic emulation state.
// In actual C++ client, things like "FlyMgr" and "CPythonPlayer" hold this.
pub struct FlyTarget {
    pub target_vid: u32,
    pub static_pos: Option<(i32, i32, f32)>,
}

pub struct ActiveFly {
    pub fly_type: u8,
    pub start_vid: u32,
    pub end_vid: u32,
}

pub struct CombatState {
    pub challenge_instances: HashSet<u32>,
    pub cant_fight_instances: HashSet<u32>,
    pub duel_keys: HashSet<(u32, u32)>,
    pub duel_mode_by_vid: HashMap<u32, u32>,
    pub active_combo_skill: u32,
    pub active_riding_skill: u32,
    pub skill_cooldowns: HashSet<u32>,
    pub actor_skills: HashMap<u32, HashMap<u32, u8>>,
    
    // Internal state to verify logic ran
    pub fly_targets_assigned: std::collections::HashMap<u32, FlyTarget>,
    pub active_flies: Vec<ActiveFly>,
    pub last_damage_effect: Option<(u32, i32, u8, bool, bool)>, // vid, damage, flag, is_self, is_target
}

impl CombatState {
    fn new() -> Self {
        Self {
            challenge_instances: HashSet::new(),
            cant_fight_instances: HashSet::new(),
            duel_keys: HashSet::new(),
            duel_mode_by_vid: HashMap::new(),
            active_combo_skill: 0,
            active_riding_skill: 0,
            skill_cooldowns: HashSet::new(),
            actor_skills: HashMap::new(),
            fly_targets_assigned: std::collections::HashMap::new(),
            active_flies: Vec::new(),
            last_damage_effect: None,
        }
    }
}

pub static GLOBAL_COMBAT_STATE: LazyLock<RwLock<CombatState>> = LazyLock::new(|| RwLock::new(CombatState::new()));

pub fn process_damage_info(data: &[u8]) -> Result<(), &'static str> {
    if data.len() < size_of::<TPacketGCDamageInfo>() {
        return Err("Buffer too short for DamageInfo packet");
    }
    let packet = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const TPacketGCDamageInfo) };
    
    let vid = packet.dwVID;
    let flag = packet.flag;
    let damage = packet.damage;
    
    if damage >= 0 {
        let mut gs = GLOBAL_GAME_STATE.write().map_err(|_| "Failed to lock game state")?;
        let is_self = vid == gs.player.vid;
        
        let mut target_selected = false;
        if let Some(target) = gs.actors.get_mut(&vid) {
            target.name = format!("{} (-{})", target.name, damage);
            target_selected = true; // Simulating bTarget if picked
        }
        
        let mut cs = GLOBAL_COMBAT_STATE.write().map_err(|_| "Failed to lock combat state")?;
        cs.last_damage_effect = Some((vid, damage, flag, is_self, target_selected));
    }
    
    Ok(())
}

pub fn process_fly_targeting(data: &[u8]) -> Result<(), &'static str> {
    if data.len() < size_of::<TPacketGCFlyTargeting>() {
        return Err("Buffer too short for FlyTargeting packet");
    }
    let packet = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const TPacketGCFlyTargeting) };
    
    let shooter_vid = packet.dwShooterVID;
    let target_vid = packet.dwTargetVID;
    let x = packet.lX;
    let y = packet.lY;
    
    let gs = GLOBAL_GAME_STATE.read().map_err(|_| "Failed to lock game state")?;
    let mut cs = GLOBAL_COMBAT_STATE.write().map_err(|_| "Failed to lock combat state")?;
    
    if gs.actors.contains_key(&shooter_vid) {
        if target_vid != 0 && gs.actors.contains_key(&target_vid) {
            cs.fly_targets_assigned.insert(shooter_vid, FlyTarget {
                target_vid,
                static_pos: None,
            });
        } else {
            let h = 60.0; // TEMPORARY HEIGHT based on CPythonBackground::Instance().GetHeight(kPacket.lX, kPacket.lY)
            cs.fly_targets_assigned.insert(shooter_vid, FlyTarget {
                target_vid: 0,
                static_pos: Some((x, y, h)),
            });
        }
    }
    
    Ok(())
}

pub fn process_create_fly(data: &[u8]) -> Result<(), &'static str> {
    if data.len() < size_of::<TPacketGCCreateFly>() {
        return Err("Buffer too short for CreateFly packet");
    }
    let packet = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const TPacketGCCreateFly) };
    
    let fly_type = packet.bType;
    let start_vid = packet.dwStartVID;
    let end_vid = packet.dwEndVID;
    
    let gs = GLOBAL_GAME_STATE.read().map_err(|_| "Failed to lock game state")?;
    let mut cs = GLOBAL_COMBAT_STATE.write().map_err(|_| "Failed to lock combat state")?;
    
    if gs.actors.contains_key(&start_vid) && gs.actors.contains_key(&end_vid) {
        cs.active_flies.push(ActiveFly {
            fly_type,
            start_vid,
            end_vid,
        });
    }
    
    Ok(())
}

pub fn process_pvp(data: &[u8]) -> Result<(), &'static str> {
    if data.len() < size_of::<TPacketGCPVP>() {
        return Err("Buffer too short for PVP packet");
    }
    let packet = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const TPacketGCPVP) };
    
    let src = packet.dwVIDSrc;
    let dst = packet.dwVIDDst;
    let mode = packet.bMode;
    
    let gs = GLOBAL_GAME_STATE.read().map_err(|_| "Failed to lock game state")?;
    let main_vid = gs.player.vid;
    
    let mut cs = GLOBAL_COMBAT_STATE.write().map_err(|_| "Failed to lock combat state")?;
    
    // mode 1 is PVP_MODE_AGREE
    if mode == 1 {
        if main_vid == dst {
            cs.challenge_instances.insert(src);
        }
        if main_vid == src {
            cs.cant_fight_instances.insert(dst);
        }
    }
    
    Ok(())
}

pub fn process_duel_start(data: &[u8]) -> Result<(), &'static str> {
    if data.len() < size_of::<TPacketGCDuelStart>() {
        return Err("Buffer too short for DuelStart packet base");
    }
    let packet = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const TPacketGCDuelStart) };
    
    let base_len = size_of::<TPacketGCDuelStart>();
    let remaining_len = (packet.length as usize).saturating_sub(base_len);
    
    if data.len() < base_len + remaining_len {
        return Err("Buffer too short for DuelStart dynamic array");
    }
    
    let count = remaining_len / size_of::<u32>();
    
    let gs = GLOBAL_GAME_STATE.read().map_err(|_| "Failed to lock game state")?;
    let main_vid = gs.player.vid;
    
    let mut cs = GLOBAL_COMBAT_STATE.write().map_err(|_| "Failed to lock combat state")?;
    let data_slice = unsafe { std::slice::from_raw_parts(data.as_ptr().add(base_len), remaining_len) };
    
    for i in 0..count {
        if let Ok(dest_bytes) = data_slice[i * 4..(i + 1) * 4].try_into() {
            let dest_vid = u32::from_le_bytes(dest_bytes);
            cs.duel_keys.insert((main_vid, dest_vid));
        }
    }
    
    if gs.actors.contains_key(&main_vid) {
        if count == 0 {
             cs.duel_mode_by_vid.insert(main_vid, 0); // 0 = DUEL_CANNOTATTACK
        } else {
             cs.duel_mode_by_vid.insert(main_vid, 1); // 1 = DUEL_START
        }
    }
    
    Ok(())
}

pub fn process_skill_level(data: &[u8]) -> Result<(), &'static str> {
    if data.len() < size_of::<TPacketGCSkillLevel>() {
        return Err("Buffer too short for SkillLevel packet");
    }
    let packet = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const TPacketGCSkillLevel) };
    
    let gs = GLOBAL_GAME_STATE.read().map_err(|_| "Failed to lock game state")?;
    let main_vid = gs.player.vid;
    
    let mut cs = GLOBAL_COMBAT_STATE.write().map_err(|_| "Failed to lock combat state")?;
    
    if gs.actors.contains_key(&main_vid) {
        let mut new_skills = HashMap::new();
        for i in 0..SKILL_MAX_NUM {
            let lvl = packet.abSkillLevels[i];
            new_skills.insert(i as u32, lvl);
        }
        cs.actor_skills.insert(main_vid, new_skills);
    }
    
    Ok(())
}

pub fn process_skill_level_new(data: &[u8]) -> Result<(), &'static str> {
    if data.len() < size_of::<TPacketGCSkillLevelNew>() {
        return Err("Buffer too short for SkillLevelNew packet");
    }
    let packet = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const TPacketGCSkillLevelNew) };
    
    let gs = GLOBAL_GAME_STATE.read().map_err(|_| "Failed to lock game state")?;
    let main_vid = gs.player.vid;
    
    let mut cs = GLOBAL_COMBAT_STATE.write().map_err(|_| "Failed to lock combat state")?;
    
    if gs.actors.contains_key(&main_vid) {
        let mut new_skills = HashMap::new();
        for i in 0..SKILL_MAX_NUM {
            let skill = &packet.skills[i];
            
            if i >= 112 && i <= 115 && skill.bLevel > 0 {
                cs.active_combo_skill = i as u32; 
            }
            if i >= 116 && i <= 119 && skill.bLevel > 0 {
                cs.active_riding_skill = i as u32; 
            }
            
            new_skills.insert(i as u32, skill.bLevel);
        }
        cs.actor_skills.insert(main_vid, new_skills);
    }
    
    Ok(())
}

pub fn process_skill_cooltime_end(data: &[u8]) -> Result<(), &'static str> {
    if data.len() < size_of::<TPacketGCSkillCoolTimeEnd>() {
        return Err("Buffer too short for SkillCoolTimeEnd packet");
    }
    let packet = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const TPacketGCSkillCoolTimeEnd) };
    
    let skill_id = packet.bSkill;
    
    let mut cs = GLOBAL_COMBAT_STATE.write().map_err(|_| "Failed to lock combat state")?;
    cs.skill_cooldowns.remove(&(skill_id as u32));
    
    Ok(())
}

#[no_mangle]
pub extern "C" fn m2_recv_damage_info(data_ptr: *const u8, len: usize) -> bool {
    let slice = unsafe { std::slice::from_raw_parts(data_ptr, len) };
    process_damage_info(slice).is_ok()
}

#[no_mangle]
pub extern "C" fn m2_recv_fly_targeting(data_ptr: *const u8, len: usize) -> bool {
    let slice = unsafe { std::slice::from_raw_parts(data_ptr, len) };
    process_fly_targeting(slice).is_ok()
}

#[no_mangle]
pub extern "C" fn m2_recv_create_fly(data_ptr: *const u8, len: usize) -> bool {
    let slice = unsafe { std::slice::from_raw_parts(data_ptr, len) };
    process_create_fly(slice).is_ok()
}

#[no_mangle]
pub extern "C" fn m2_recv_pvp(data_ptr: *const u8, len: usize) -> bool {
    let slice = unsafe { std::slice::from_raw_parts(data_ptr, len) };
    process_pvp(slice).is_ok()
}

#[no_mangle]
pub extern "C" fn m2_recv_duel_start(data_ptr: *const u8, len: usize) -> bool {
    let slice = unsafe { std::slice::from_raw_parts(data_ptr, len) };
    process_duel_start(slice).is_ok()
}

#[no_mangle]
pub extern "C" fn m2_recv_skill_level(data_ptr: *const u8, len: usize) -> bool {
    let slice = unsafe { std::slice::from_raw_parts(data_ptr, len) };
    process_skill_level(slice).is_ok()
}

#[no_mangle]
pub extern "C" fn m2_recv_skill_level_new(data_ptr: *const u8, len: usize) -> bool {
    let slice = unsafe { std::slice::from_raw_parts(data_ptr, len) };
    process_skill_level_new(slice).is_ok()
}

#[no_mangle]
pub extern "C" fn m2_recv_skill_cooltime_end(data_ptr: *const u8, len: usize) -> bool {
    let slice = unsafe { std::slice::from_raw_parts(data_ptr, len) };
    process_skill_cooltime_end(slice).is_ok()
}
