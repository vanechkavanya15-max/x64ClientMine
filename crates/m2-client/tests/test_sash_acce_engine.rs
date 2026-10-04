use m2_client::game_sash_acce_combination_engine::{
    SashGrade,
    SashItem,
    calculate_absorbed_bonus,
    combine_sashes,
    upgrade_custom_sash_absorption,
};

#[test]
fn test_sash_creation_and_absorption_rates() {
    let simple = SashItem::new(SashGrade::Simple, None);
    assert_eq!(simple.grade, SashGrade::Simple);
    assert_eq!(simple.absorption_rate, 1);

    let fine = SashItem::new(SashGrade::Fine, None);
    assert_eq!(fine.grade, SashGrade::Fine);
    assert_eq!(fine.absorption_rate, 5);

    let noble = SashItem::new(SashGrade::Noble, None);
    assert_eq!(noble.grade, SashGrade::Noble);
    assert_eq!(noble.absorption_rate, 10);

    let custom_default = SashItem::new(SashGrade::Custom, None);
    assert_eq!(custom_default.grade, SashGrade::Custom);
    assert_eq!(custom_default.absorption_rate, 11);

    let custom_high = SashItem::new(SashGrade::Custom, Some(20));
    assert_eq!(custom_high.grade, SashGrade::Custom);
    assert_eq!(custom_high.absorption_rate, 20);

    // Test bounds
    let custom_below = SashItem::new(SashGrade::Custom, Some(5));
    assert_eq!(custom_below.absorption_rate, 11); // Clamp to 11

    let custom_above = SashItem::new(SashGrade::Custom, Some(30));
    assert_eq!(custom_above.absorption_rate, 25); // Clamp to 25
}

#[test]
fn test_absorption_calculation() {
    // 0 values should return 0
    assert_eq!(calculate_absorbed_bonus(0, 10), 0);

    // 1% of 100 is 1
    assert_eq!(calculate_absorbed_bonus(100, 1), 1);

    // 1% of 1 is 0.01, but minimum is 1
    assert_eq!(calculate_absorbed_bonus(1, 1), 1);

    // 10% of 25 is 2.5, using integer math it's 2
    assert_eq!(calculate_absorbed_bonus(25, 10), 2);

    // 25% of 1000 is 250
    assert_eq!(calculate_absorbed_bonus(1000, 25), 250);
    
    // Negative values
    assert_eq!(calculate_absorbed_bonus(-100, 10), -10);
    
    // Minimum 1 applies to negatives too (it will return -1)
    assert_eq!(calculate_absorbed_bonus(-1, 5), -1);
}

#[test]
fn test_sash_combination() {
    let simple1 = SashItem::new(SashGrade::Simple, None);
    let simple2 = SashItem::new(SashGrade::Simple, None);
    
    // Test successful combination Simple -> Fine
    let success = combine_sashes(&simple1, &simple2, 100, 50); // 100% chance, roll 50
    assert!(success.is_ok());
    let fine = success.unwrap();
    assert_eq!(fine.grade, SashGrade::Fine);
    assert_eq!(fine.absorption_rate, 5);

    // Test failed combination
    let failure = combine_sashes(&simple1, &simple2, 50, 75); // 50% chance, roll 75
    assert!(failure.is_err());

    // Test Fine -> Noble
    let fine1 = SashItem::new(SashGrade::Fine, None);
    let fine2 = SashItem::new(SashGrade::Fine, None);
    let noble = combine_sashes(&fine1, &fine2, 100, 1).unwrap();
    assert_eq!(noble.grade, SashGrade::Noble);
    
    // Test Noble -> Custom
    let noble1 = SashItem::new(SashGrade::Noble, None);
    let noble2 = SashItem::new(SashGrade::Noble, None);
    let custom = combine_sashes(&noble1, &noble2, 100, 1).unwrap();
    assert_eq!(custom.grade, SashGrade::Custom);
    assert_eq!(custom.absorption_rate, 11);
    
    // Test mismatched grades
    assert!(combine_sashes(&simple1, &fine1, 100, 1).is_err());
    
    // Test custom grade combining (should fail)
    let custom1 = SashItem::new(SashGrade::Custom, None);
    let custom2 = SashItem::new(SashGrade::Custom, None);
    assert!(combine_sashes(&custom1, &custom2, 100, 1).is_err());
}

#[test]
fn test_upgrade_custom_absorption() {
    let mut custom = SashItem::new(SashGrade::Custom, Some(11));
    
    // Test successful upgrade
    let result = upgrade_custom_sash_absorption(&mut custom, true, 5);
    assert!(result.is_ok());
    assert_eq!(custom.absorption_rate, 16);
    
    // Test failed upgrade
    let result = upgrade_custom_sash_absorption(&mut custom, false, 5);
    assert!(result.is_err());
    assert_eq!(custom.absorption_rate, 16);
    
    // Test hitting max cap
    let result = upgrade_custom_sash_absorption(&mut custom, true, 20);
    assert!(result.is_ok());
    assert_eq!(custom.absorption_rate, 25);
    
    // Test past max cap
    let result = upgrade_custom_sash_absorption(&mut custom, true, 5);
    assert!(result.is_err()); // Already at 25
    
    // Test non-custom sash (should fail)
    let mut simple = SashItem::new(SashGrade::Simple, None);
    assert!(upgrade_custom_sash_absorption(&mut simple, true, 5).is_err());
}
