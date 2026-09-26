use m2_client::item_anti_flags_v2::*;

#[test]
fn test_can_equip_full_sex() {
    // 0 = MALE, 1 = FEMALE
    
    // Male trying to equip female-only item (MALE anti-flag)
    assert_eq!(
        can_equip_full(ITEM_ANTIFLAG_MALE, LIMIT_NONE, 0, 0, 0, 1, (10, 10, 10, 10), 0),
        false
    );
    
    // Male trying to equip male item (no anti-flag)
    assert_eq!(
        can_equip_full(0, LIMIT_NONE, 0, 0, 0, 1, (10, 10, 10, 10), 0),
        true
    );

    // Female trying to equip male-only item (FEMALE anti-flag)
    assert_eq!(
        can_equip_full(ITEM_ANTIFLAG_FEMALE, LIMIT_NONE, 0, 0, 1, 1, (10, 10, 10, 10), 0),
        false
    );
    
    // Female trying to equip female item
    assert_eq!(
        can_equip_full(0, LIMIT_NONE, 0, 0, 1, 1, (10, 10, 10, 10), 0),
        true
    );
}

#[test]
fn test_can_equip_full_class() {
    // WARRIOR = 0, ASSASSIN = 1, SURA = 2, SHAMAN = 3
    
    // Warrior cannot equip if WARRIOR anti-flag is set
    assert_eq!(
        can_equip_full(ITEM_ANTIFLAG_WARRIOR, LIMIT_NONE, 0, 0, 0, 1, (10, 10, 10, 10), 0),
        false
    );
    
    // Warrior can equip if ASSASSIN anti-flag is set
    assert_eq!(
        can_equip_full(ITEM_ANTIFLAG_ASSASSIN, LIMIT_NONE, 0, 0, 0, 1, (10, 10, 10, 10), 0),
        true
    );

    // Shaman cannot equip if SHAMAN anti-flag is set
    assert_eq!(
        can_equip_full(ITEM_ANTIFLAG_SHAMAN, LIMIT_NONE, 0, 3, 0, 1, (10, 10, 10, 10), 0),
        false
    );
}

#[test]
fn test_can_equip_full_level_limit() {
    // Limit is 50, player is 40 -> false
    assert_eq!(
        can_equip_full(0, LIMIT_LEVEL, 50, 0, 0, 40, (10, 10, 10, 10), 0),
        false
    );

    // Limit is 50, player is 50 -> true
    assert_eq!(
        can_equip_full(0, LIMIT_LEVEL, 50, 0, 0, 50, (10, 10, 10, 10), 0),
        true
    );

    // Limit is 50, player is 60 -> true
    assert_eq!(
        can_equip_full(0, LIMIT_LEVEL, 50, 0, 0, 60, (10, 10, 10, 10), 0),
        true
    );
}

#[test]
fn test_can_equip_full_stat_limits() {
    // LIMIT_STR limit is 90, stat is 80 -> false
    assert_eq!(
        can_equip_full(0, LIMIT_STR, 90, 0, 0, 1, (80, 10, 10, 10), 0),
        false
    );
    
    // LIMIT_STR limit is 90, stat is 90 -> true
    assert_eq!(
        can_equip_full(0, LIMIT_STR, 90, 0, 0, 1, (90, 10, 10, 10), 0),
        true
    );

    // LIMIT_DEX
    assert_eq!(can_equip_full(0, LIMIT_DEX, 90, 0, 0, 1, (10, 80, 10, 10), 0), false);
    assert_eq!(can_equip_full(0, LIMIT_DEX, 90, 0, 0, 1, (10, 90, 10, 10), 0), true);

    // LIMIT_INT
    assert_eq!(can_equip_full(0, LIMIT_INT, 90, 0, 0, 1, (10, 10, 80, 10), 0), false);
    assert_eq!(can_equip_full(0, LIMIT_INT, 90, 0, 0, 1, (10, 10, 90, 10), 0), true);

    // LIMIT_CON
    assert_eq!(can_equip_full(0, LIMIT_CON, 90, 0, 0, 1, (10, 10, 10, 80), 0), false);
    assert_eq!(can_equip_full(0, LIMIT_CON, 90, 0, 0, 1, (10, 10, 10, 90), 0), true);
}

#[test]
fn test_can_equip_full_real_time_limit() {
    // Current time is 100, expiry time is 50 -> expired -> false
    assert_eq!(
        can_equip_full(0, LIMIT_REAL_TIME, 50, 0, 0, 1, (10, 10, 10, 10), 100),
        false
    );
    
    // Current time is 100, expiry time is 150 -> not expired -> true
    assert_eq!(
        can_equip_full(0, LIMIT_REAL_TIME, 150, 0, 0, 1, (10, 10, 10, 10), 100),
        true
    );

    // Current time is 100, limit is 0 (not used yet) -> true
    assert_eq!(
        can_equip_full(0, LIMIT_REAL_TIME, 0, 0, 0, 1, (10, 10, 10, 10), 100),
        true
    );
}

#[test]
fn test_can_equip_full_timer_based_wear() {
    // Remaining time is 0 -> false
    assert_eq!(
        can_equip_full(0, LIMIT_TIMER_BASED_ON_WEAR, 0, 0, 0, 1, (10, 10, 10, 10), 100),
        false
    );
    
    // Remaining time is > 0 -> true
    assert_eq!(
        can_equip_full(0, LIMIT_TIMER_BASED_ON_WEAR, 60, 0, 0, 1, (10, 10, 10, 10), 100),
        true
    );
}

#[test]
fn test_ffi_can_equip_full() {
    assert_eq!(
        m2_item_can_equip_full(ITEM_ANTIFLAG_WARRIOR, LIMIT_LEVEL, 50, 0, 0, 40, 10, 10, 10, 10, 0),
        false
    );
    assert_eq!(
        m2_item_can_equip_full(0, LIMIT_LEVEL, 50, 0, 0, 60, 10, 10, 10, 10, 0),
        true
    );
}
