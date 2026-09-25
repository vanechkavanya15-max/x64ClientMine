use m2_client_bridge::state::skills::{SkillsContainer, m2_skills_container_new, m2_skills_container_free, m2_skills_container_set_skill, m2_skills_container_can_use, m2_skills_container_trigger_cooldown};

#[test]
fn test_skills_container_set_skill() {
    let mut container = SkillsContainer::new();
    let vnum = 1;
    
    // Skill nie istnieje na poczatku, nie mozna go uzyc
    assert!(!container.can_use(vnum, 0));

    // Ustaw skill (poziom 1)
    container.set_skill(vnum, 1);
    
    // Skill mozna uzyc (cooldown to 0)
    assert!(container.can_use(vnum, 0));

    // Ustaw na 0 poziom
    container.set_skill(vnum, 0);
    assert!(!container.can_use(vnum, 0));
}

#[test]
fn test_skills_container_cooldown() {
    let mut container = SkillsContainer::new();
    let vnum = 2;
    container.set_skill(vnum, 1);
    
    // Wywolanie cooldown (1000ms), przy czasie 1000, next_use_time = 2000
    container.trigger_cooldown(vnum, 1000, 1000);

    // Przed uplywem czasu (1500)
    assert!(!container.can_use(vnum, 1500));
    // Dokladnie o czasie uplywu (2000)
    assert!(container.can_use(vnum, 2000));
    // Po uplywie czasu (2500)
    assert!(container.can_use(vnum, 2500));
}

#[test]
fn test_skills_ffi() {
    unsafe {
        let ptr = m2_skills_container_new();
        assert!(!ptr.is_null());

        let vnum = 3;
        m2_skills_container_set_skill(ptr, vnum, 1);

        assert!(m2_skills_container_can_use(ptr, vnum, 1000));

        m2_skills_container_trigger_cooldown(ptr, vnum, 5000, 1000);

        // Przed cooldownem (1000 + 5000 = 6000)
        assert!(!m2_skills_container_can_use(ptr, vnum, 3000));
        assert!(m2_skills_container_can_use(ptr, vnum, 6000));

        m2_skills_container_free(ptr);
    }
}
