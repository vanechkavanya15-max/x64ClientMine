#[path = "../src/dragon_soul_deck_stat_cache.rs"]
mod dragon_soul_deck_stat_cache;

use dragon_soul_deck_stat_cache::{
    DragonSoulDeckStatCache, DragonSoulDeckType, DragonSoulItem, DragonSoulSlot, DragonSoulStats,
};
use std::time::{Duration, Instant};

#[test]
fn test_equip_unequip() {
    let mut cache = DragonSoulDeckStatCache::new(Duration::from_secs(1));
    let item = DragonSoulItem {
        stats: DragonSoulStats {
            attack: 10,
            defense: 5,
            hp: 0,
            sp: 0,
        },
        energy_left: 100,
    };

    cache.deck1.equip(DragonSoulSlot::Diamond, item.clone());
    assert_eq!(cache.deck1.slots[0].as_ref(), Some(&item));

    let removed = cache.deck1.unequip(DragonSoulSlot::Diamond);
    assert_eq!(removed, Some(item));
    assert_eq!(cache.deck1.slots[0], None);
}

#[test]
fn test_active_deck_stats() {
    let mut cache = DragonSoulDeckStatCache::new(Duration::from_secs(1));
    let now = Instant::now();

    cache.deck1.equip(
        DragonSoulSlot::Ruby,
        DragonSoulItem {
            stats: DragonSoulStats { attack: 20, defense: 0, hp: 0, sp: 0 },
            energy_left: 100,
        },
    );

    cache.deck2.equip(
        DragonSoulSlot::Onyx,
        DragonSoulItem {
            stats: DragonSoulStats { attack: 0, defense: 15, hp: 0, sp: 0 },
            energy_left: 100,
        },
    );

    assert_eq!(cache.get_active_stats(), DragonSoulStats::default());

    cache.toggle_deck(DragonSoulDeckType::Deck1, now).unwrap();
    assert_eq!(cache.get_active_stats().attack, 20);
    assert_eq!(cache.get_active_stats().defense, 0);

    let future = now + Duration::from_secs(2);
    cache.toggle_deck(DragonSoulDeckType::Deck2, future).unwrap();
    assert_eq!(cache.get_active_stats().attack, 0);
    assert_eq!(cache.get_active_stats().defense, 15);
}

#[test]
fn test_toggle_cooldown() {
    let mut cache = DragonSoulDeckStatCache::new(Duration::from_secs(3));
    let now = Instant::now();

    assert!(cache.toggle_deck(DragonSoulDeckType::Deck1, now).is_ok());
    
    // Attempting to toggle before cooldown expires should fail
    let soon = now + Duration::from_secs(1);
    assert!(cache.toggle_deck(DragonSoulDeckType::Deck2, soon).is_err());
    
    // Toggling after cooldown should succeed
    let later = now + Duration::from_secs(4);
    assert!(cache.toggle_deck(DragonSoulDeckType::Deck2, later).is_ok());
}

#[test]
fn test_energy_expiration() {
    let mut cache = DragonSoulDeckStatCache::new(Duration::from_secs(1));
    let now = Instant::now();

    cache.deck1.equip(
        DragonSoulSlot::Jade,
        DragonSoulItem {
            stats: DragonSoulStats { attack: 0, defense: 0, hp: 500, sp: 0 },
            energy_left: 10,
        },
    );

    cache.toggle_deck(DragonSoulDeckType::Deck1, now).unwrap();
    
    // Energy is active
    assert_eq!(cache.get_active_stats().hp, 500);

    // Drain 5 energy
    cache.tick_active_deck_energy(5);
    assert_eq!(cache.get_active_stats().hp, 500);

    // Drain remaining energy
    cache.tick_active_deck_energy(5);
    // Item should no longer provide stats
    assert_eq!(cache.get_active_stats().hp, 0);
}
