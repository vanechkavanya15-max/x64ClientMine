use m2_client::game_horse_combat_skill_manager::GameHorseCombatSkillManager;

#[test]
fn test_can_attack_horse_level() {
    // Brak dosiadanego wierzchowca
    assert_eq!(GameHorseCombatSkillManager::can_attack_horse_level(false, 2, false, 0, 0), false);
    
    // Zwykly kon, poziom 1 (nie moze atakowac)
    assert_eq!(GameHorseCombatSkillManager::can_attack_horse_level(true, 1, false, 0, 0), false);
    
    // Zwykly kon bojowy, poziom 2 (moze atakowac)
    assert_eq!(GameHorseCombatSkillManager::can_attack_horse_level(true, 2, false, 0, 0), true);
    
    // Zwykly kon militarny, poziom 3 (moze atakowac)
    assert_eq!(GameHorseCombatSkillManager::can_attack_horse_level(true, 3, false, 0, 0), true);
    
    // Nowy mount z poziomem umiejetnosci za malym (od 11)
    assert_eq!(GameHorseCombatSkillManager::can_attack_horse_level(true, 2, true, 0, 10), false);
    
    // Nowy mount z wystarczajacym poziomem umiejetnosci (11+)
    assert_eq!(GameHorseCombatSkillManager::can_attack_horse_level(true, 2, true, 0, 11), true);
    
    // Nowy mount z grade = 1 (zawsze ok niezaleznie od levelu jesli grade wystarcza)
    assert_eq!(GameHorseCombatSkillManager::can_attack_horse_level(true, 2, true, 1, 0), true);
}

#[test]
fn test_can_use_horse_skill() {
    // Brak wierzchowca
    assert_eq!(GameHorseCombatSkillManager::can_use_skill(false, 3, false, 0, 0), false);
    
    // Zwykly kon, poziom 2 (bojowy) - nie moze uzywac skilli
    assert_eq!(GameHorseCombatSkillManager::can_use_skill(true, 2, false, 0, 0), false);
    
    // Zwykly kon, poziom 3 (militarny) - moze uzywac skilli
    assert_eq!(GameHorseCombatSkillManager::can_use_skill(true, 3, false, 0, 0), true);
    
    // Nowy mount, skill < 20 - zablokowane
    assert_eq!(GameHorseCombatSkillManager::can_use_skill(true, 3, true, 0, 19), false);
    
    // Nowy mount, skill 20+ - dozwolone
    assert_eq!(GameHorseCombatSkillManager::can_use_skill(true, 3, true, 0, 20), true);
    
    // Nowy mount z grade 1 - dozwolone
    assert_eq!(GameHorseCombatSkillManager::can_use_skill(true, 3, true, 1, 0), true);
}

#[test]
fn test_check_skill_usable_on_horse() {
    // 1. Gracz na koniu uzywa skilla konnego -> OK
    assert_eq!(GameHorseCombatSkillManager::check_skill_usable_on_horse(true, true), Ok(()));
    
    // 2. Gracz na koniu uzywa ZWYKLEGO skilla (nie konnego) -> BLAD NOT_HORSE_SKILL
    assert_eq!(GameHorseCombatSkillManager::check_skill_usable_on_horse(true, false), Err("NOT_HORSE_SKILL"));
    
    // 3. Gracz na piechote uzywa skilla konnego -> BLAD HAVE_TO_RIDE
    assert_eq!(GameHorseCombatSkillManager::check_skill_usable_on_horse(false, true), Err("HAVE_TO_RIDE"));
    
    // 4. Gracz na piechote uzywa zwyklego skilla -> OK
    assert_eq!(GameHorseCombatSkillManager::check_skill_usable_on_horse(false, false), Ok(()));
}
