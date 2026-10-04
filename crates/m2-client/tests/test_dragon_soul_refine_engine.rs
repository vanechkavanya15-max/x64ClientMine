use m2_client::dragon_soul_refine_engine::{
    DragonSoulRefineEngine, DragonSoulRefineWindowRefineType, DragonSoulRefineError,
    DragonSoulSub, RefineResult,
};
use m2_client::dragon_soul_deck_v2::{DragonSoulRefine, DragonSoulGrade, DragonSoulClarity, StoneCategory};

#[test]
fn test_add_material_invalid_slot() {
    let mut engine = DragonSoulRefineEngine::new(DragonSoulRefineWindowRefineType::Upgrade);
    let result = engine.add_material(15, 1, 0, 110000);
    assert_eq!(result, Err(DragonSoulRefineError::InvalidSlot));
}

#[test]
fn test_add_material_invalid_stone() {
    let mut engine = DragonSoulRefineEngine::new(DragonSoulRefineWindowRefineType::Upgrade);
    let result = engine.add_material(0, 1, 0, 10);
    assert_eq!(result, Err(DragonSoulRefineError::InvalidMaterial));
}

#[test]
fn test_add_material_upgrade_mismatched_category() {
    let mut engine = DragonSoulRefineEngine::new(DragonSoulRefineWindowRefineType::Upgrade);
    
    // Diamond, Normal, Lowest, Level 0
    let vnum1 = DragonSoulRefine::make_vnum(StoneCategory::Diamond, DragonSoulGrade::Normal, DragonSoulClarity::Lowest, 0);
    // Ruby, Normal, Lowest, Level 0
    let vnum2 = DragonSoulRefine::make_vnum(StoneCategory::Ruby, DragonSoulGrade::Normal, DragonSoulClarity::Lowest, 0);

    assert_eq!(engine.add_material(0, 1, 0, vnum1), Ok(()));
    assert_eq!(engine.add_material(1, 1, 1, vnum2), Err(DragonSoulRefineError::MismatchedCategory));
}

#[test]
fn test_add_material_upgrade_mismatched_grade() {
    let mut engine = DragonSoulRefineEngine::new(DragonSoulRefineWindowRefineType::Upgrade);
    
    // Diamond, Normal, Lowest, Level 0
    let vnum1 = DragonSoulRefine::make_vnum(StoneCategory::Diamond, DragonSoulGrade::Normal, DragonSoulClarity::Lowest, 0);
    // Diamond, Brilliant, Lowest, Level 0
    let vnum2 = DragonSoulRefine::make_vnum(StoneCategory::Diamond, DragonSoulGrade::Brilliant, DragonSoulClarity::Lowest, 0);

    assert_eq!(engine.add_material(0, 1, 0, vnum1), Ok(()));
    assert_eq!(engine.add_material(1, 1, 1, vnum2), Err(DragonSoulRefineError::MismatchedGrade));
}

#[test]
fn test_add_material_improvement_mismatched_clarity() {
    let mut engine = DragonSoulRefineEngine::new(DragonSoulRefineWindowRefineType::Improvement);
    
    // Diamond, Normal, Lowest, Level 0
    let vnum1 = DragonSoulRefine::make_vnum(StoneCategory::Diamond, DragonSoulGrade::Normal, DragonSoulClarity::Lowest, 0);
    // Diamond, Normal, Low, Level 0
    let vnum2 = DragonSoulRefine::make_vnum(StoneCategory::Diamond, DragonSoulGrade::Normal, DragonSoulClarity::Low, 0);

    assert_eq!(engine.add_material(0, 1, 0, vnum1), Ok(()));
    assert_eq!(engine.add_material(1, 1, 1, vnum2), Err(DragonSoulRefineError::MismatchedClarity));
}

#[test]
fn test_add_material_refine_valid() {
    let mut engine = DragonSoulRefineEngine::new(DragonSoulRefineWindowRefineType::Refine);
    
    // Diamond, Normal, Lowest, Level 0
    let vnum1 = DragonSoulRefine::make_vnum(StoneCategory::Diamond, DragonSoulGrade::Normal, DragonSoulClarity::Lowest, 0);
    
    assert_eq!(engine.add_material(0, 1, 0, vnum1), Ok(()));
    assert_eq!(engine.add_material(1, 1, 1, 70251), Ok(())); // Green bean
    assert_eq!(engine.add_material(2, 1, 2, 70252), Ok(())); // Blue bean
    assert_eq!(engine.add_material(3, 1, 3, 70253), Ok(())); // Pink bean
}

#[test]
fn test_add_material_refine_invalid_bean() {
    let mut engine = DragonSoulRefineEngine::new(DragonSoulRefineWindowRefineType::Refine);
    
    // Diamond, Normal, Lowest, Level 0
    let vnum1 = DragonSoulRefine::make_vnum(StoneCategory::Diamond, DragonSoulGrade::Normal, DragonSoulClarity::Lowest, 0);
    
    assert_eq!(engine.add_material(0, 1, 0, vnum1), Ok(()));
    assert_eq!(engine.add_material(1, 1, 1, 99999), Err(DragonSoulRefineError::InvalidMaterial)); // Invalid bean
}

#[test]
fn test_remove_material() {
    let mut engine = DragonSoulRefineEngine::new(DragonSoulRefineWindowRefineType::Upgrade);
    let vnum = DragonSoulRefine::make_vnum(StoneCategory::Diamond, DragonSoulGrade::Normal, DragonSoulClarity::Lowest, 0);
    
    assert_eq!(engine.add_material(0, 1, 0, vnum), Ok(()));
    assert!(engine.slots[0].is_some());

    assert_eq!(engine.remove_material(0), Ok(()));
    assert!(engine.slots[0].is_none());
}

#[test]
fn test_parse_server_response() {
    assert_eq!(DragonSoulRefineEngine::parse_server_response(0, 1, 2), Some(RefineResult::Open));
    assert_eq!(DragonSoulRefineEngine::parse_server_response(11, 3, 4), Some(RefineResult::Succeed { window_type: 3, cell: 4 }));
    assert_eq!(
        DragonSoulRefineEngine::parse_server_response(5, 5, 6),
        Some(RefineResult::Fail { reason: DragonSoulSub::RefineFail, window_type: 5, cell: 6 })
    );
    assert_eq!(
        DragonSoulRefineEngine::parse_server_response(7, 7, 8),
        Some(RefineResult::Fail { reason: DragonSoulSub::RefineFailInvalidMaterial, window_type: 7, cell: 8 })
    );
    assert_eq!(DragonSoulRefineEngine::parse_server_response(99, 1, 2), None);
}
