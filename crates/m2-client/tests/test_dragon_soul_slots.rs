#[path = "../src/dragon_soul_slots.rs"]
mod dragon_soul_slots;

use dragon_soul_slots::{Bonus, Deck, DragonSoulStone, DragonSoulSystem, StoneCategory};

#[test]
fn test_equip_and_unequip_stone() {
    let mut system = DragonSoulSystem::new();

    let stone = DragonSoulStone {
        category: StoneCategory::Diament,
        vnum: 12345,
        bonus: Bonus {
            hp: 100,
            ..Default::default()
        },
    };

    let previous = system.equip_stone(Deck::Deck1, stone.clone());
    assert_eq!(previous, None);

    let unequipped = system.unequip_stone(Deck::Deck1, StoneCategory::Diament);
    assert_eq!(unequipped, Some(stone));
    
    // Proba wyjecia kamienia ze zlego slotu
    let unequipped_empty = system.unequip_stone(Deck::Deck1, StoneCategory::Diament);
    assert_eq!(unequipped_empty, None);
}

#[test]
fn test_swap_stone() {
    let mut system = DragonSoulSystem::new();

    let stone1 = DragonSoulStone {
        category: StoneCategory::Rubin,
        vnum: 111,
        bonus: Bonus {
            attack: 50,
            ..Default::default()
        },
    };

    let stone2 = DragonSoulStone {
        category: StoneCategory::Rubin,
        vnum: 222,
        bonus: Bonus {
            attack: 150,
            ..Default::default()
        },
    };

    let prev1 = system.equip_stone(Deck::Deck2, stone1.clone());
    assert_eq!(prev1, None);

    let prev2 = system.equip_stone(Deck::Deck2, stone2.clone());
    assert_eq!(prev2, Some(stone1));

    let unequipped = system.unequip_stone(Deck::Deck2, StoneCategory::Rubin);
    assert_eq!(unequipped, Some(stone2));
}

#[test]
fn test_bonus_calculation_active() {
    let mut system = DragonSoulSystem::new();

    let stone1 = DragonSoulStone {
        category: StoneCategory::Jadeit,
        vnum: 333,
        bonus: Bonus {
            hp: 500,
            defense: 20,
            ..Default::default()
        },
    };

    let stone2 = DragonSoulStone {
        category: StoneCategory::Onyks,
        vnum: 444,
        bonus: Bonus {
            defense: 30,
            ..Default::default()
        },
    };

    system.equip_stone(Deck::Deck1, stone1);
    system.equip_stone(Deck::Deck1, stone2);

    // Domyslnie alchemia jest wylaczona
    assert_eq!(system.get_active_bonus(), Bonus::default());

    // Wlaczamy alchemie
    system.set_active(true);

    let active_bonus = system.get_active_bonus();
    assert_eq!(
        active_bonus,
        Bonus {
            hp: 500,
            defense: 50,
            ..Default::default()
        }
    );
}

#[test]
fn test_bonus_calculation_inactive() {
    let mut system = DragonSoulSystem::new();

    let stone = DragonSoulStone {
        category: StoneCategory::Granat,
        vnum: 555,
        bonus: Bonus {
            hp: 1000,
            ..Default::default()
        },
    };

    system.equip_stone(Deck::Deck1, stone);

    system.set_active(false);

    // Nawet z wlozonym kamieniem, na nieaktywnym systemie bonus ma byc 0
    assert_eq!(system.get_active_bonus(), Bonus::default());
}

#[test]
fn test_deck_switching() {
    let mut system = DragonSoulSystem::new();

    let stone_deck1 = DragonSoulStone {
        category: StoneCategory::Szafir,
        vnum: 666,
        bonus: Bonus {
            magic_attack: 100,
            ..Default::default()
        },
    };

    let stone_deck2 = DragonSoulStone {
        category: StoneCategory::Szafir,
        vnum: 777,
        bonus: Bonus {
            magic_defense: 200,
            ..Default::default()
        },
    };

    system.equip_stone(Deck::Deck1, stone_deck1);
    system.equip_stone(Deck::Deck2, stone_deck2);

    system.set_active(true);

    // Po aktywacji domyslna talia to Deck1
    assert_eq!(
        system.get_active_bonus(),
        Bonus {
            magic_attack: 100,
            ..Default::default()
        }
    );

    // Zmieniamy talie
    system.switch_deck(Deck::Deck2);

    assert_eq!(
        system.get_active_bonus(),
        Bonus {
            magic_defense: 200,
            ..Default::default()
        }
    );
}

#[test]
fn test_all_stones_bonus() {
    let mut system = DragonSoulSystem::new();

    let diament = DragonSoulStone { category: StoneCategory::Diament, vnum: 1, bonus: Bonus { hp: 10, ..Default::default() } };
    let rubin = DragonSoulStone { category: StoneCategory::Rubin, vnum: 2, bonus: Bonus { sp: 10, ..Default::default() } };
    let jadeit = DragonSoulStone { category: StoneCategory::Jadeit, vnum: 3, bonus: Bonus { attack: 10, ..Default::default() } };
    let szafir = DragonSoulStone { category: StoneCategory::Szafir, vnum: 4, bonus: Bonus { defense: 10, ..Default::default() } };
    let granat = DragonSoulStone { category: StoneCategory::Granat, vnum: 5, bonus: Bonus { magic_attack: 10, ..Default::default() } };
    let onyks = DragonSoulStone { category: StoneCategory::Onyks, vnum: 6, bonus: Bonus { magic_defense: 10, ..Default::default() } };

    system.equip_stone(Deck::Deck1, diament);
    system.equip_stone(Deck::Deck1, rubin);
    system.equip_stone(Deck::Deck1, jadeit);
    system.equip_stone(Deck::Deck1, szafir);
    system.equip_stone(Deck::Deck1, granat);
    system.equip_stone(Deck::Deck1, onyks);

    system.set_active(true);

    assert_eq!(
        system.get_active_bonus(),
        Bonus {
            hp: 10,
            sp: 10,
            attack: 10,
            defense: 10,
            magic_attack: 10,
            magic_defense: 10,
        }
    );
}
