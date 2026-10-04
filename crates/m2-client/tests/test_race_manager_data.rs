use m2_client::race_manager_data_registry::{
    MotionType, RaceData, RaceManagerDataRegistry, RACE_WARRIOR_M,
};

#[test]
fn test_default_pc_races_registered() {
    let registry = RaceManagerDataRegistry::new();
    assert!(registry.len() >= 4);

    let warrior = registry.get_race(RACE_WARRIOR_M).expect("Wojownik musi byc zarejestrowany");
    assert_eq!(warrior.name, "warrior_m");
    assert_eq!(warrior.msm_filename, "warrior_m.msm");
    assert_eq!(warrior.get_msm_full_path(), "d:/ymir work/pc/warrior/warrior_m.msm");

    // Sprawdz sciezke animacji ruchu
    let wait_path = warrior.get_motion_path(MotionType::Wait).expect("Musi posiadac animacje Wait");
    assert_eq!(wait_path, "d:/ymir work/pc/warrior/wait.msa");

    let combo_path = warrior.get_motion_path(MotionType::Combo).expect("Musi posiadac animacje Combo");
    assert_eq!(combo_path, "d:/ymir work/pc/warrior/combo_01.msa");
}

#[test]
fn test_custom_monster_race_registration() {
    let mut registry = RaceManagerDataRegistry::new();

    let mut wolf = RaceData::new(101, "wild_dog", "d:/ymir work/monster/wild_dog", "wild_dog.msm");
    wolf.set_motion(MotionType::Wait, "wait.msa");
    wolf.set_motion(MotionType::Run, "run.msa");
    wolf.set_motion(MotionType::Dead, "dead.msa");

    registry.register_race(wolf);

    let fetched = registry.get_race(101).expect("Wilk powinien byc w rejestrze");
    assert_eq!(fetched.name, "wild_dog");
    assert_eq!(fetched.get_msm_full_path(), "d:/ymir work/monster/wild_dog/wild_dog.msm");
    assert_eq!(fetched.get_motion_path(MotionType::Dead).unwrap(), "d:/ymir work/monster/wild_dog/dead.msa");
}
