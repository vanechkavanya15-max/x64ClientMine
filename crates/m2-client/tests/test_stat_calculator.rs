use m2_client::game_character_stat_calculator::*;

#[test]
fn test_character_stat_formulas() {
    m2_game_stats_set_attributes(10, 20, 15, 25, 12, 5);

    // Max HP = 700 + 20*40 + 10*20 = 700 + 800 + 200 = 1700
    assert_eq!(m2_game_stats_get_max_hp(), 1700);

    // Max SP = 250 + 15*20 + 10*10 = 250 + 300 + 100 = 650
    assert_eq!(m2_game_stats_get_max_sp(), 650);

    // Defense = 20 + 10*2 = 40
    assert_eq!(m2_game_stats_get_defense(), 40);

    // Attack Power = 25*2 + 10*2 = 50 + 20 = 70
    assert_eq!(m2_game_stats_get_attack(), 70);

    // Increase VIT
    assert!(m2_game_stats_increase(0));
    assert_eq!(m2_game_stats_get_max_hp(), 1740); // +40 HP
}
