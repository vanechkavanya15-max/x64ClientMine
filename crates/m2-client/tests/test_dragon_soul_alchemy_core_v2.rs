#[path = "../src/dragon_soul_alchemy_core_v2.rs"]
mod dragon_soul_alchemy_core_v2;

use dragon_soul_alchemy_core_v2::{
    DragonSoulDeckType, DragonSoulGradeTypes, DragonSoulRefineError, DragonSoulStepTypes,
    DragonSoulSubType, DragonSoulVnum, DS_DECK_MAX_NUM, DRAGON_SOUL_GRADE_MAX, DRAGON_SOUL_STEP_MAX,
    DS_SLOT_NUM_TYPES, DS_MAX_REFINE_LEVEL,
};

#[test]
fn test_enum_sizes_and_constants() {
    assert_eq!(DS_DECK_MAX_NUM, 2);
    assert_eq!(DRAGON_SOUL_GRADE_MAX, 5);
    assert_eq!(DRAGON_SOUL_STEP_MAX, 5);
    assert_eq!(DS_SLOT_NUM_TYPES, 6);
    assert_eq!(DS_MAX_REFINE_LEVEL, 6);

    assert_eq!(DragonSoulDeckType::DS_DECK_1 as u8, 0);
    assert_eq!(DragonSoulDeckType::DS_DECK_2 as u8, 1);

    assert_eq!(DragonSoulSubType::Diamond as u8, 0);
    assert_eq!(DragonSoulSubType::Onyx as u8, 5);

    assert_eq!(DragonSoulGradeTypes::Normal as u8, 0);
    assert_eq!(DragonSoulGradeTypes::Legendary as u8, 4);

    assert_eq!(DragonSoulStepTypes::Lowest as u8, 0);
    assert_eq!(DragonSoulStepTypes::Highest as u8, 4);
}

#[test]
fn test_dragon_soul_vnum_roundtrip() {
    // 1(Diamond = 1) (Rare = 2) (Mid = 2) (Level = 4) => 112240
    let vnum_val = 112240;
    
    let ds = DragonSoulVnum::parse(vnum_val).expect("Failed to parse valid vnum");
    assert_eq!(ds.category, DragonSoulSubType::Diamond);
    assert_eq!(ds.grade, DragonSoulGradeTypes::Rare);
    assert_eq!(ds.step, DragonSoulStepTypes::Mid);
    assert_eq!(ds.level, 4);

    assert_eq!(ds.to_vnum(), vnum_val);

    // 1(Onyx = 6) (Legendary = 4) (Highest = 4) (Level = 6) => 164460
    let vnum_max = 164460;
    let ds_max = DragonSoulVnum::parse(vnum_max).expect("Failed to parse max vnum");
    assert_eq!(ds_max.category, DragonSoulSubType::Onyx);
    assert_eq!(ds_max.grade, DragonSoulGradeTypes::Legendary);
    assert_eq!(ds_max.step, DragonSoulStepTypes::Highest);
    assert_eq!(ds_max.level, 6);

    assert_eq!(ds_max.to_vnum(), vnum_max);
}

#[test]
fn test_dragon_soul_vnum_invalid() {
    // Under range
    assert_eq!(DragonSoulVnum::parse(99999), Err(DragonSoulRefineError::InvalidVnum));
    
    // Over range
    assert_eq!(DragonSoulVnum::parse(170001), Err(DragonSoulRefineError::InvalidVnum));

    // Invalid category index (e.g. 0 would mean 10XXXX but our valid is 11XXXX to 16XXXX)
    assert_eq!(DragonSoulVnum::parse(100000), Err(DragonSoulRefineError::InvalidVnum));

    // Invalid grade (5) -> 1(Ruby=2)5000 => 125000
    assert_eq!(DragonSoulVnum::parse(125000), Err(DragonSoulRefineError::InvalidVnum));
    
    // Invalid level (7) -> 1(Ruby=2)0070 => 120070
    assert_eq!(DragonSoulVnum::parse(120070), Err(DragonSoulRefineError::InvalidVnum));
}

#[test]
fn test_dragon_soul_upgrade_grade() {
    let base = DragonSoulVnum::parse(121110).unwrap(); // Ruby, Brilliant, Low, Level 1
    
    // Success upgrade: upgrades grade to Rare, resets step to Lowest, resets level to 0
    let upgraded = base.upgrade_grade(true).unwrap();
    assert_eq!(upgraded.category, DragonSoulSubType::Ruby);
    assert_eq!(upgraded.grade, DragonSoulGradeTypes::Rare);
    assert_eq!(upgraded.step, DragonSoulStepTypes::Lowest);
    assert_eq!(upgraded.level, 0);
    assert_eq!(upgraded.to_vnum(), 122000);

    // Fail upgrade
    assert_eq!(base.upgrade_grade(false), Err(DragonSoulRefineError::RefineFailed));

    // Max grade upgrade
    let max_grade = DragonSoulVnum::parse(124000).unwrap(); // Ruby, Legendary
    assert_eq!(max_grade.upgrade_grade(true), Err(DragonSoulRefineError::MaxGradeReached));
}

#[test]
fn test_dragon_soul_improve_step() {
    let base = DragonSoulVnum::parse(122240).unwrap(); // Ruby, Rare, Mid, Level 4
    
    // Success improve: improves step to High, keeps grade, resets level to 0
    let improved = base.improve_step(true).unwrap();
    assert_eq!(improved.category, DragonSoulSubType::Ruby);
    assert_eq!(improved.grade, DragonSoulGradeTypes::Rare);
    assert_eq!(improved.step, DragonSoulStepTypes::High);
    assert_eq!(improved.level, 0);
    assert_eq!(improved.to_vnum(), 122300);

    // Fail improve
    assert_eq!(base.improve_step(false), Err(DragonSoulRefineError::RefineFailed));

    // Max step improve
    let max_step = DragonSoulVnum::parse(122400).unwrap(); // Ruby, Rare, Highest
    assert_eq!(max_step.improve_step(true), Err(DragonSoulRefineError::MaxStepReached));
}

#[test]
fn test_dragon_soul_refine_level() {
    let base = DragonSoulVnum::parse(122240).unwrap(); // Ruby, Rare, Mid, Level 4
    
    // Success refine level
    let leveled = base.refine_level(true).unwrap();
    assert_eq!(leveled.category, DragonSoulSubType::Ruby);
    assert_eq!(leveled.grade, DragonSoulGradeTypes::Rare);
    assert_eq!(leveled.step, DragonSoulStepTypes::Mid);
    assert_eq!(leveled.level, 5);
    assert_eq!(leveled.to_vnum(), 122250);

    // Fail refine
    assert_eq!(base.refine_level(false), Err(DragonSoulRefineError::RefineFailed));

    // Max level refine
    let max_level = DragonSoulVnum::parse(122260).unwrap(); // Level 6
    assert_eq!(max_level.refine_level(true), Err(DragonSoulRefineError::MaxLevelReached));
}
