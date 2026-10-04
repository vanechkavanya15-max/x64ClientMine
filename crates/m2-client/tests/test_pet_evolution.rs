use m2_client::game_pet_evolution_system::*;
use std::ffi::CString;

#[test]
fn test_pet_summon_and_feeding() {
    let name = CString::new("Mini Azrael").unwrap();
    let pet_id = m2_pet_summon(34001, name.as_ptr(), 25);
    assert!(pet_id > 0);

    assert_eq!(m2_pet_get_level(pet_id), 25);
    assert_eq!(m2_pet_get_stage(pet_id), PetEvolutionStage::Young as u8);

    assert!(m2_pet_feed(pet_id, 3600));

    assert!(m2_pet_dismiss(pet_id));
}

#[test]
fn test_pet_evolution_stages() {
    let name = CString::new("Fire Phoenix").unwrap();
    let pet_id = m2_pet_summon(34002, name.as_ptr(), 45);

    // Level 45 is eligible for Wild stage
    let stage = m2_pet_evolve(pet_id);
    assert_eq!(stage, PetEvolutionStage::Wild as u8);
}
