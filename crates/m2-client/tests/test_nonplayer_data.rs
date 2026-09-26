#[path = "../src/nonplayer_data.rs"]
mod nonplayer_data;

use nonplayer_data::{
    NonPlayerData, NonPlayerDatabase, RANK_BOSS, RANK_KING, RANK_KNIGHT, RANK_PAWN, RANK_S_KNIGHT,
    TYPE_MONSTER, TYPE_NPC,
};

#[test]
fn test_register_and_read_mob() {
    let mut db = NonPlayerDatabase::new();
    let mob = NonPlayerData {
        race_vnum: 101,
        mob_name: "Wild Dog".to_string(),
        rank: RANK_PAWN,
        mob_type: TYPE_MONSTER,
        is_aggressive: false,
        attack_speed: 100,
        move_speed: 100,
    };

    db.register_mob(mob);

    assert_eq!(db.get_monster_name(101), "Wild Dog");
    assert_eq!(db.is_boss(101), false);
    assert_eq!(db.is_aggressive(101), false);
}

#[test]
fn test_is_boss_and_king() {
    let mut db = NonPlayerDatabase::new();
    
    // Boss
    let boss_mob = NonPlayerData {
        race_vnum: 1093,
        mob_name: "Death Reaper".to_string(),
        rank: RANK_BOSS,
        mob_type: TYPE_MONSTER,
        is_aggressive: true,
        attack_speed: 150,
        move_speed: 120,
    };
    db.register_mob(boss_mob);
    assert_eq!(db.is_boss(1093), true);

    // King
    let king_mob = NonPlayerData {
        race_vnum: 1094,
        mob_name: "King Spider".to_string(),
        rank: RANK_KING,
        mob_type: TYPE_MONSTER,
        is_aggressive: true,
        attack_speed: 200,
        move_speed: 150,
    };
    db.register_mob(king_mob);
    assert_eq!(db.is_boss(1094), true);

    // Non-Boss (Knight)
    let knight_mob = NonPlayerData {
        race_vnum: 102,
        mob_name: "Wolf".to_string(),
        rank: RANK_KNIGHT,
        mob_type: TYPE_MONSTER,
        is_aggressive: false,
        attack_speed: 110,
        move_speed: 105,
    };
    db.register_mob(knight_mob);
    assert_eq!(db.is_boss(102), false);
}

#[test]
fn test_is_aggressive() {
    let mut db = NonPlayerDatabase::new();

    let aggressive_mob = NonPlayerData {
        race_vnum: 103,
        mob_name: "Alpha Wolf".to_string(),
        rank: RANK_S_KNIGHT,
        mob_type: TYPE_MONSTER,
        is_aggressive: true,
        attack_speed: 120,
        move_speed: 110,
    };
    db.register_mob(aggressive_mob);

    assert_eq!(db.is_aggressive(103), true);
}

#[test]
fn test_unknown_vnum_defaults() {
    let db = NonPlayerDatabase::new();

    assert_eq!(db.get_monster_name(9999), "");
    assert_eq!(db.is_boss(9999), false);
    assert_eq!(db.is_aggressive(9999), false);
}

#[test]
fn test_edge_cases() {
    let mut db = NonPlayerDatabase::new();

    // Max u32 VNUM
    let max_vnum = u32::MAX;
    db.register_mob(NonPlayerData {
        race_vnum: max_vnum,
        mob_name: "Glitch Mob".to_string(),
        rank: RANK_PAWN,
        mob_type: TYPE_MONSTER,
        is_aggressive: false,
        attack_speed: 0,
        move_speed: 0,
    });
    assert_eq!(db.get_monster_name(max_vnum), "Glitch Mob");

    // Overwriting a mob
    db.register_mob(NonPlayerData {
        race_vnum: 101,
        mob_name: "Dog 1".to_string(),
        rank: RANK_PAWN,
        mob_type: TYPE_MONSTER,
        is_aggressive: false,
        attack_speed: 100,
        move_speed: 100,
    });
    assert_eq!(db.get_monster_name(101), "Dog 1");

    db.register_mob(NonPlayerData {
        race_vnum: 101,
        mob_name: "Dog 2".to_string(),
        rank: RANK_PAWN,
        mob_type: TYPE_MONSTER,
        is_aggressive: true, // changed
        attack_speed: 100,
        move_speed: 100,
    });
    assert_eq!(db.get_monster_name(101), "Dog 2");
    assert_eq!(db.is_aggressive(101), true);

    // Special characters in name
    db.register_mob(NonPlayerData {
        race_vnum: 9001,
        mob_name: "Npc z ogonkiem ąęść".to_string(),
        rank: RANK_PAWN,
        mob_type: TYPE_NPC,
        is_aggressive: false,
        attack_speed: 0,
        move_speed: 0,
    });
    assert_eq!(db.get_monster_name(9001), "Npc z ogonkiem ąęść");
}
