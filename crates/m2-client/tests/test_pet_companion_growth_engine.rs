use m2_client::pet_companion_growth_engine::{PetCompanion, PetState, PetType, PetSkill};

#[test]
fn test_pet_initialization() {
    let pet = PetCompanion::new(1001, PetType::Type1, 3600);
    
    assert_eq!(pet.id, 1001);
    assert_eq!(pet.state, PetState::Unsummoned);
    assert_eq!(pet.stats.level, 1);
    assert_eq!(pet.stats.exp, 0);
    assert_eq!(pet.stats.max_exp, 100);
    assert_eq!(pet.stats.duration_seconds, 3600);
    assert_eq!(pet.stats.max_duration_seconds, 3600);
    assert_eq!(pet.stats.pet_type, PetType::Type1);
}

#[test]
fn test_pet_summon_state_transitions() {
    let mut pet = PetCompanion::new(1002, PetType::Type2, 3600);
    
    // Test summon success
    assert!(pet.handle_summon_packet(true));
    assert_eq!(pet.state, PetState::Summoned);
    
    // Test double summon fails
    assert!(!pet.summon());
    assert_eq!(pet.state, PetState::Summoned);
    
    // Test unsummon success
    assert!(pet.handle_summon_packet(false));
    assert_eq!(pet.state, PetState::Unsummoned);
}

#[test]
fn test_pet_duration_decay_and_hungry_state() {
    let mut pet = PetCompanion::new(1003, PetType::Type3, 1000);
    pet.summon();
    
    // Tick down to just above hungry threshold (10% of 1000 = 100)
    pet.tick_duration(899);
    assert_eq!(pet.stats.duration_seconds, 101);
    assert_eq!(pet.state, PetState::Summoned);
    
    // Tick down into hungry state
    pet.tick_duration(2);
    assert_eq!(pet.stats.duration_seconds, 99);
    assert_eq!(pet.state, PetState::Hungry);
    
    // Tick down to death (unsummoned)
    pet.tick_duration(100);
    assert_eq!(pet.stats.duration_seconds, 0);
    assert_eq!(pet.state, PetState::Unsummoned);
}

#[test]
fn test_pet_summon_when_low_duration() {
    let mut pet = PetCompanion::new(1004, PetType::Type4, 1000);
    pet.stats.duration_seconds = 50; // Below 10%
    
    pet.summon();
    assert_eq!(pet.state, PetState::Hungry);
}

#[test]
fn test_pet_summon_when_dead() {
    let mut pet = PetCompanion::new(1005, PetType::Type5, 1000);
    pet.stats.duration_seconds = 0;
    
    assert!(!pet.summon());
    assert_eq!(pet.state, PetState::Unsummoned);
}

#[test]
fn test_pet_feeding() {
    let mut pet = PetCompanion::new(1006, PetType::Type6, 1000);
    pet.stats.duration_seconds = 50; // Hungry threshold
    pet.summon();
    assert_eq!(pet.state, PetState::Hungry);
    
    // Feed partial amount, still hungry
    pet.feed(20);
    assert_eq!(pet.stats.duration_seconds, 70);
    assert_eq!(pet.state, PetState::Hungry);
    
    // Feed past hungry threshold
    pet.feed(50);
    assert_eq!(pet.stats.duration_seconds, 120);
    assert_eq!(pet.state, PetState::Summoned);
    
    // Feed past max (should clamp)
    pet.feed(2000);
    assert_eq!(pet.stats.duration_seconds, 1000);
}

#[test]
fn test_pet_exp_and_level_up() {
    let mut pet = PetCompanion::new(1007, PetType::Type7, 3600);
    
    // Initial max_exp = 100
    pet.add_exp(90);
    assert_eq!(pet.stats.level, 1);
    assert_eq!(pet.stats.exp, 90);
    
    // Level up
    pet.add_exp(20);
    assert_eq!(pet.stats.level, 2);
    assert_eq!(pet.stats.exp, 10);
    // new max_exp = 100 + (2 * 50) = 200
    assert_eq!(pet.stats.max_exp, 200);
}

#[test]
fn test_pet_evolution_state() {
    let mut pet = PetCompanion::new(1008, PetType::Type8, 3600);
    pet.stats.level = 39;
    pet.stats.exp = 0;
    pet.stats.max_exp = 100 + (39 * 50);
    pet.summon();
    
    // Push over the level threshold (40)
    pet.add_exp(pet.stats.max_exp + 10);
    assert_eq!(pet.stats.level, 40);
    assert_eq!(pet.state, PetState::Evolving);
    
    // Complete evolution
    pet.complete_evolution();
    assert_eq!(pet.state, PetState::Summoned);
}

#[test]
fn test_pet_skills() {
    let mut pet = PetCompanion::new(1009, PetType::Type1, 3600);
    
    assert_eq!(pet.get_skill(0), Some(PetSkill::None));
    
    assert!(pet.set_skill(0, PetSkill::WarriorResistance));
    assert_eq!(pet.get_skill(0), Some(PetSkill::WarriorResistance));
    
    assert!(pet.set_skill(2, PetSkill::Berserk));
    assert_eq!(pet.get_skill(2), Some(PetSkill::Berserk));
    
    assert!(!pet.set_skill(3, PetSkill::Pierce));
    assert_eq!(pet.get_skill(3), None);
}

#[test]
fn test_pet_age() {
    let mut pet = PetCompanion::new(1010, PetType::Type1, 3600);
    assert_eq!(pet.stats.age_days, 0);
    
    pet.increment_age();
    assert_eq!(pet.stats.age_days, 1);
}

#[test]
fn test_pet_type_from_u8() {
    assert_eq!(PetType::from_u8(1), Some(PetType::Type1));
    assert_eq!(PetType::from_u8(8), Some(PetType::Type8));
    assert_eq!(PetType::from_u8(0), None);
    assert_eq!(PetType::from_u8(9), None);
}
