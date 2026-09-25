use m2_client_bridge::state::skills::{Skill, SkillsManager};

#[test]
fn test_add_and_get_skill() {
    let mut manager = SkillsManager::new();
    let vid = 1001;

    let skill = Skill {
        vnum: 1,
        level: 20,
        cooldown_ms: 1500,
        mana_cost: 50,
    };

    manager.set_skill(vid, skill);

    let retrieved = manager.get_skill(vid, 1).expect("Skill powinien istniec w menedzerze");
    assert_eq!(retrieved.vnum, 1);
    assert_eq!(retrieved.level, 20);
    assert_eq!(retrieved.cooldown_ms, 1500);
    assert_eq!(retrieved.mana_cost, 50);
}

#[test]
fn test_get_nonexistent_skill() {
    let manager = SkillsManager::new();
    let result = manager.get_skill(9999, 1);
    assert!(result.is_none(), "Zwracanie nieistniejacego skilla powinno dawac None");
}

#[test]
fn test_update_skill() {
    let mut manager = SkillsManager::new();
    let vid = 2002;

    let mut skill = Skill {
        vnum: 5,
        level: 1,
        cooldown_ms: 2000,
        mana_cost: 30,
    };

    manager.set_skill(vid, skill);

    // Zmiana wlasciwosci skilla (np. po awansie poziomu)
    skill.level = 2;
    skill.cooldown_ms = 1900;
    manager.set_skill(vid, skill);

    let retrieved = manager.get_skill(vid, 5).unwrap();
    assert_eq!(retrieved.level, 2);
    assert_eq!(retrieved.cooldown_ms, 1900);
}

#[test]
fn test_remove_skill() {
    let mut manager = SkillsManager::new();
    let vid = 3003;

    let skill = Skill {
        vnum: 10,
        level: 1,
        cooldown_ms: 0,
        mana_cost: 10,
    };

    manager.set_skill(vid, skill);
    assert!(manager.get_skill(vid, 10).is_some());

    manager.remove_skill(vid, 10);
    assert!(manager.get_skill(vid, 10).is_none(), "Skill powinien zostac usuniety");
}

#[test]
fn test_remove_character() {
    let mut manager = SkillsManager::new();
    let vid = 4004;

    manager.set_skill(vid, Skill { vnum: 1, level: 1, cooldown_ms: 0, mana_cost: 0 });
    manager.set_skill(vid, Skill { vnum: 2, level: 1, cooldown_ms: 0, mana_cost: 0 });

    assert!(manager.get_skill(vid, 1).is_some());
    assert!(manager.get_skill(vid, 2).is_some());

    // Usuniecie postaci ze swiata (np. smierc, wylogowanie, znikniecie z pola widzenia)
    manager.remove_character(vid);

    assert!(manager.get_skill(vid, 1).is_none());
    assert!(manager.get_skill(vid, 2).is_none());
}

#[test]
fn test_add_character_explicitly() {
    let mut manager = SkillsManager::new();
    let vid = 5005;

    manager.add_character(vid);
    
    // Sprawdzenie ze postac zostala dodana, ale nie ma zadnych skilli
    let result = manager.get_skill(vid, 1);
    assert!(result.is_none());
    
    // Mozna swobodnie dodac umiejetnosc bez wczesniejszego wywolania add_character, co bylo sprawdzone w poprzednich testach
}
