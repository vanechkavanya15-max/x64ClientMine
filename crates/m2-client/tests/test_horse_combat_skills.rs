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

#[test]
fn test_is_charge_skill() {
    // 122 to Charge z konia
    assert_eq!(GameHorseCombatSkillManager::is_charge_skill(122), true);
    
    // Inne umiejetnosci to nie charge
    assert_eq!(GameHorseCombatSkillManager::is_charge_skill(121), false);
    assert_eq!(GameHorseCombatSkillManager::is_charge_skill(123), false);
    assert_eq!(GameHorseCombatSkillManager::is_charge_skill(1), false);
}

#[test]
fn test_is_horse_combo() {
    // Skille ataku z siodla to 121 (Ciecie), 122 (Szarza), 123 (Tapanie)
    assert_eq!(GameHorseCombatSkillManager::is_horse_combo(121), true);
    assert_eq!(GameHorseCombatSkillManager::is_horse_combo(122), true);
    assert_eq!(GameHorseCombatSkillManager::is_horse_combo(123), true);
    
    // Inne skille nie sa combami z konia
    assert_eq!(GameHorseCombatSkillManager::is_horse_combo(124), false);
    assert_eq!(GameHorseCombatSkillManager::is_horse_combo(1), false);
}

#[test]
fn test_is_horse_combo_attack() {
    // Nie jestesmy na koniu, nie mozna atakowac
    assert_eq!(GameHorseCombatSkillManager::is_horse_combo_attack(false, false), false);
    assert_eq!(GameHorseCombatSkillManager::is_horse_combo_attack(false, true), false);
    
    // Jestesmy na koniu, ale nie mamy broni (hand mode)
    assert_eq!(GameHorseCombatSkillManager::is_horse_combo_attack(true, true), false);
    
    // Jestesmy na koniu i mamy bron
    assert_eq!(GameHorseCombatSkillManager::is_horse_combo_attack(true, false), true);
}

#[test]
fn test_is_using_charge_skill() {
    // Ma buff (dash affect) -> uzywa szarzy
    assert_eq!(GameHorseCombatSkillManager::is_using_charge_skill(true, false, 0), true);
    assert_eq!(GameHorseCombatSkillManager::is_using_charge_skill(true, true, 122), true);
    
    // Nie ma buffa i skill nie jest "reserved mode"
    assert_eq!(GameHorseCombatSkillManager::is_using_charge_skill(false, false, 122), false);
    
    // Nie ma buffa, jest "reserved mode", i skill to Charge
    assert_eq!(GameHorseCombatSkillManager::is_using_charge_skill(false, true, 122), true);
    
    // Nie ma buffa, jest "reserved mode", ale skill to NIE Charge
    assert_eq!(GameHorseCombatSkillManager::is_using_charge_skill(false, true, 121), false);
}
