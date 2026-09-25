use m2_client::player_skill::{PlayerSkill, PointType, SkillData, PlayerState};

struct MockSkillData {
    skill_type: u8,
    is_toggle: bool,
    can_use: bool,
    attack: bool,
    need_empty_bottle: bool,
    need_poison_bottle: bool,
    horse_skill: bool,
    charge_skill: bool,
    need_bow: bool,
    use_hp_skill: bool,
    need_sp: i32,
    use_if_not_enough: bool,
    cool_time: f32,
}

impl Default for MockSkillData {
    fn default() -> Self {
        Self {
            skill_type: 1,
            is_toggle: false,
            can_use: true,
            attack: false,
            need_empty_bottle: false,
            need_poison_bottle: false,
            horse_skill: false,
            charge_skill: false,
            need_bow: false,
            use_hp_skill: false,
            need_sp: 10,
            use_if_not_enough: false,
            cool_time: 5.0,
        }
    }
}

impl SkillData for MockSkillData {
    fn get_type(&self) -> u8 { self.skill_type }
    fn is_toggle_skill(&self) -> bool { self.is_toggle }
    fn is_can_use_skill(&self) -> bool { self.can_use }
    fn is_standing_skill(&self) -> bool { true }
    fn is_attack_skill(&self) -> bool { self.attack }
    fn is_need_empty_bottle(&self) -> bool { self.need_empty_bottle }
    fn is_need_poison_bottle(&self) -> bool { self.need_poison_bottle }
    fn is_horse_skill(&self) -> bool { self.horse_skill }
    fn is_charge_skill(&self) -> bool { self.charge_skill }
    fn is_need_bow(&self) -> bool { self.need_bow }
    fn is_use_hp_skill(&self) -> bool { self.use_hp_skill }
    fn get_need_sp(&self, _cur: f32) -> i32 { self.need_sp }
    fn can_use_if_not_enough(&self) -> bool { self.use_if_not_enough }
    fn can_use_weapon_type(&self, _w: i32) -> bool { true }
    fn get_skill_cool_time(&self, _cur: f32) -> f32 { self.cool_time }
    fn get_target_range(&self) -> i32 { 0 }
    fn get_target_count(&self, _cur: f32) -> i32 { 1 }
    fn is_fan_range(&self) -> bool { false }
    fn is_circle_range(&self) -> bool { false }
    fn is_no_motion(&self) -> bool { false }
    fn get_skill_motion_index(&self, _g: i32) -> i32 { 0 }
    fn get_motion_loop_count(&self, _c: f32) -> i32 { 1 }
    fn is_moving_skill(&self) -> bool { false }
    fn is_only_for_guild_war(&self) -> bool { false }
}

struct MockPlayerState {
    sp: i32,
    hp: i32,
    casting_speed: i32,
    current_time: f32,
    is_safe: bool,
    has_arrow: bool,
    dash_affect: bool,
}

impl Default for MockPlayerState {
    fn default() -> Self {
        Self {
            sp: 100,
            hp: 100,
            casting_speed: 0,
            current_time: 10.0,
            is_safe: false,
            has_arrow: false,
            dash_affect: false,
        }
    }
}

impl PlayerState for MockPlayerState {
    fn get_status(&self, point_type: PointType) -> i32 {
        match point_type {
            PointType::Sp => self.sp,
            PointType::Hp => self.hp,
            PointType::CastingSpeed => self.casting_speed,
            _ => 0,
        }
    }
    fn is_fishing_mode(&self) -> bool { false }
    fn get_weapon_type(&self) -> i32 { 1 }
    fn has_item(&self, _item_vnum: u32) -> bool { true }
    fn has_enough_arrow(&self) -> bool { self.has_arrow }
    fn is_in_safe(&self) -> bool { self.is_safe }
    fn is_mounting_horse(&self) -> bool { false }
    fn check_dash_affect(&self) -> bool { self.dash_affect }
    fn get_current_second(&self) -> f32 { self.current_time }
}

#[test]
fn test_integration_player_skill_basic() {
    let instance = PlayerSkill::new();
    assert!(instance.is_ready());
}

#[test]
fn test_use_skill_success() {
    let mut skill = PlayerSkill::new();
    let slot = 1;
    skill.skills[slot].level = 1; // skill learned
    
    let mut skill_data = MockSkillData::default();
    skill_data.need_sp = 20;
    
    let mut player_state = MockPlayerState::default();
    player_state.sp = 50;
    player_state.current_time = 100.0;
    
    // First use should succeed
    let can_use = skill.use_skill(slot, &skill_data, &player_state, true);
    assert!(can_use);
    
    // Check what the cool_time actually became
    // Default casting speed is 0. spd = 100 - 0 = 100 -> spd becomes 200.
    // cool_time = 5.0 * 200 / 100 = 10.0
    assert_eq!(skill.skills[slot].cool_time, 10.0);
    
    // Time hasn't advanced, skill should be on cooldown
    assert!(!skill.check_skill_usable(slot, &skill_data, &player_state, true));
    
    // Advance time past cooldown
    player_state.current_time = 111.0;
    assert!(skill.check_skill_usable(slot, &skill_data, &player_state, true));
}

#[test]
fn test_not_enough_sp() {
    let mut skill = PlayerSkill::new();
    let slot = 1;
    skill.skills[slot].level = 1;
    
    let mut skill_data = MockSkillData::default();
    skill_data.need_sp = 50;
    
    let mut player_state = MockPlayerState::default();
    player_state.sp = 10;
    
    assert!(!skill.use_skill(slot, &skill_data, &player_state, true));
}

#[test]
fn test_use_hp_skill() {
    let mut skill = PlayerSkill::new();
    let slot = 1;
    skill.skills[slot].level = 1;
    
    let mut skill_data = MockSkillData::default();
    skill_data.use_hp_skill = true;
    skill_data.need_sp = 50; // In this mock need_sp maps to need_hp based on how check_short_life calls get_need_sp
    
    let mut player_state = MockPlayerState::default();
    player_state.hp = 10;
    
    assert!(!skill.use_skill(slot, &skill_data, &player_state, true));
    
    player_state.hp = 100;
    assert!(skill.use_skill(slot, &skill_data, &player_state, true));
}

#[test]
fn test_casting_speed_modifier() {
    let mut skill = PlayerSkill::new();
    let slot = 1;
    skill.skills[slot].level = 1;
    
    let skill_data = MockSkillData::default(); // cool_time = 5.0
    
    let mut player_state = MockPlayerState::default();
    player_state.casting_speed = 50; 
    skill.run_cool_time(slot, &skill_data, &player_state);
    assert_eq!(skill.skills[slot].cool_time, 7.5);
    
    player_state.casting_speed = 150; 
    skill.run_cool_time(slot, &skill_data, &player_state);
    assert_eq!(skill.skills[slot].cool_time, 3.3);
}


#[test]
fn test_toggle_skill() {
    let mut skill = PlayerSkill::new();
    let slot = 1;
    skill.skills[slot].level = 1;
    
    let mut skill_data = MockSkillData::default();
    skill_data.is_toggle = true;
    
    let player_state = MockPlayerState::default();
    
    // Toggle on
    assert!(skill.use_skill(slot, &skill_data, &player_state, true));
    assert!(skill.is_skill_active(slot));
    
    // Cool down for toggle skills shouldn't prevent turning it off, but wait, toggle skills 
    // are checked in __UseSkill C++. Let's verify toggling logic.
    let res = skill.use_skill(slot, &skill_data, &player_state, true);
    assert!(!res); // C++ explicitly returns false when turning toggle off
    assert!(!skill.is_skill_active(slot));
}
