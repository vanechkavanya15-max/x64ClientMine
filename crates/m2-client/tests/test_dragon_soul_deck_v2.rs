use m2_client::dragon_soul_deck_v2::*;

#[test]
fn test_deck_switching() {
    let mut system = DragonSoulSystemV2::new();
    assert_eq!(system.active_deck, None);

    system.activate_deck(DeckType::Deck1);
    assert_eq!(system.active_deck, Some(DeckType::Deck1));

    system.activate_deck(DeckType::Deck2);
    assert_eq!(system.active_deck, Some(DeckType::Deck2));

    system.activate_deck(DeckType::Deck2);
    assert_eq!(system.active_deck, None);
}

#[test]
fn test_equip_unequip_stone() {
    let mut system = DragonSoulSystemV2::new();

    let stone = DragonSoulStone {
        category: StoneCategory::Diamond,
        grade: DragonSoulGrade::Normal,
        bonus: Bonus { attack: 5, defense: 5 },
    };

    let prev = system.equip_stone(DeckType::Deck1, stone.clone());
    assert_eq!(prev, None);

    let current = system.unequip_stone(DeckType::Deck1, StoneCategory::Diamond);
    assert_eq!(current, Some(stone));
    
    let none = system.unequip_stone(DeckType::Deck1, StoneCategory::Diamond);
    assert_eq!(none, None);
}

#[test]
fn test_active_bonus_no_deck_active() {
    let mut system = DragonSoulSystemV2::new();

    let stone = DragonSoulStone {
        category: StoneCategory::Diamond,
        grade: DragonSoulGrade::Normal,
        bonus: Bonus { attack: 5, defense: 5 },
    };

    system.equip_stone(DeckType::Deck1, stone);

    let bonus = system.get_active_bonus();
    assert_eq!(bonus, Bonus::default());
}

#[test]
fn test_active_bonus_with_deck_active_no_set() {
    let mut system = DragonSoulSystemV2::new();

    let stone1 = DragonSoulStone {
        category: StoneCategory::Diamond,
        grade: DragonSoulGrade::Normal,
        bonus: Bonus { attack: 5, defense: 5 },
    };
    let stone2 = DragonSoulStone {
        category: StoneCategory::Ruby,
        grade: DragonSoulGrade::Brilliant,
        bonus: Bonus { attack: 10, defense: 10 },
    };

    system.equip_stone(DeckType::Deck1, stone1);
    system.equip_stone(DeckType::Deck1, stone2);
    system.activate_deck(DeckType::Deck1);

    let bonus = system.get_active_bonus();
    assert_eq!(bonus, Bonus { attack: 15, defense: 15 });
}

#[test]
fn test_active_bonus_with_deck_active_with_set() {
    let mut system = DragonSoulSystemV2::new();

    let stones = vec![
        (StoneCategory::Diamond, Bonus { attack: 1, defense: 1 }),
        (StoneCategory::Ruby, Bonus { attack: 1, defense: 1 }),
        (StoneCategory::Jade, Bonus { attack: 1, defense: 1 }),
        (StoneCategory::Sapphire, Bonus { attack: 1, defense: 1 }),
        (StoneCategory::Garnet, Bonus { attack: 1, defense: 1 }),
        (StoneCategory::Onyx, Bonus { attack: 1, defense: 1 }),
    ];

    for (cat, bonus) in stones {
        system.equip_stone(
            DeckType::Deck2,
            DragonSoulStone {
                category: cat,
                grade: DragonSoulGrade::Brilliant,
                bonus,
            },
        );
    }

    system.activate_deck(DeckType::Deck2);

    let bonus = system.get_active_bonus();
    // 6 stones * 1 attack/defense each = 6
    // Set bonus for Brilliant = 20
    // Total = 26
    assert_eq!(bonus, Bonus { attack: 26, defense: 26 });
}

#[test]
fn test_active_bonus_with_deck_active_mixed_grades_no_set() {
    let mut system = DragonSoulSystemV2::new();

    let stones = vec![
        (StoneCategory::Diamond, DragonSoulGrade::Brilliant, Bonus { attack: 1, defense: 1 }),
        (StoneCategory::Ruby, DragonSoulGrade::Brilliant, Bonus { attack: 1, defense: 1 }),
        (StoneCategory::Jade, DragonSoulGrade::Brilliant, Bonus { attack: 1, defense: 1 }),
        (StoneCategory::Sapphire, DragonSoulGrade::Brilliant, Bonus { attack: 1, defense: 1 }),
        (StoneCategory::Garnet, DragonSoulGrade::Brilliant, Bonus { attack: 1, defense: 1 }),
        (StoneCategory::Onyx, DragonSoulGrade::Normal, Bonus { attack: 1, defense: 1 }),
    ];

    for (cat, grade, bonus) in stones {
        system.equip_stone(
            DeckType::Deck2,
            DragonSoulStone {
                category: cat,
                grade,
                bonus,
            },
        );
    }

    system.activate_deck(DeckType::Deck2);

    let bonus = system.get_active_bonus();
    // 6 stones * 1 attack/defense each = 6
    // No set bonus because grades don't match
    // Total = 6
    assert_eq!(bonus, Bonus { attack: 6, defense: 6 });
}

#[test]
fn test_ffi_m2_ds_activate_deck() {
    // Activating invalid deck
    assert_eq!(m2_ds_activate_deck(99), false);
    
    // Deactivating initially (it is already deactivated)
    let sys = get_dragon_soul_system().lock().unwrap();
    assert_eq!(sys.active_deck, None);
    drop(sys);
    
    // Activating deck 1 (idx 0)
    assert_eq!(m2_ds_activate_deck(0), true);
    let sys = get_dragon_soul_system().lock().unwrap();
    assert_eq!(sys.active_deck, Some(DeckType::Deck1));
    drop(sys);

    // Activating deck 2 (idx 1)
    assert_eq!(m2_ds_activate_deck(1), true);
    let sys = get_dragon_soul_system().lock().unwrap();
    assert_eq!(sys.active_deck, Some(DeckType::Deck2));
    drop(sys);
    
    // Toggling deck 2 (idx 1)
    assert_eq!(m2_ds_activate_deck(1), true);
    let sys = get_dragon_soul_system().lock().unwrap();
    assert_eq!(sys.active_deck, None);
    drop(sys);
}
