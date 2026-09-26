use m2_client::player_stat_calc_v2::*;

#[test]
fn test_player_stat_calc_max_hp() {
    let calc = PlayerStatCalc {
        con: 10,
        base_hp: 50,
        equip_hp: 100,
        ..Default::default()
    };
    // 10 * 20 + 50 + 100 = 200 + 150 = 350
    assert_eq!(calc.max_hp(), 350);
}

#[test]
fn test_player_stat_calc_max_sp() {
    let calc = PlayerStatCalc {
        int_: 5,
        base_sp: 20,
        equip_sp: 30,
        ..Default::default()
    };
    // 5 * 20 + 20 + 30 = 100 + 50 = 150
    assert_eq!(calc.max_sp(), 150);
}

#[test]
fn test_player_stat_calc_atk_grade() {
    let calc = PlayerStatCalc {
        str_: 15,
        level: 10,
        equip_atk: 50,
        ..Default::default()
    };
    // 15 * 2 + 10 * 2 + 50 = 30 + 20 + 50 = 100
    assert_eq!(calc.atk_grade(), 100);
}

#[test]
fn test_player_stat_calc_def_grade() {
    let calc = PlayerStatCalc {
        con: 20,
        equip_def: 40,
        ..Default::default()
    };
    // 20 + 40 = 60
    assert_eq!(calc.def_grade(), 60);
}

#[test]
fn test_player_stat_calc_mag_atk() {
    let calc = PlayerStatCalc {
        int_: 25,
        level: 30,
        ..Default::default()
    };
    // 25 * 2 + 30 * 2 = 50 + 60 = 110
    assert_eq!(calc.mag_atk(), 110);
}

#[test]
fn test_player_stat_calc_evasion() {
    let calc = PlayerStatCalc {
        dex: 15,
        ..Default::default()
    };
    // 30 * (2 * 15 + 5) / (15 + 95) = 1050 / 110 = 9
    assert_eq!(calc.evasion(), 9);

    let calc90 = PlayerStatCalc {
        dex: 90,
        ..Default::default()
    };
    // 30 * (180 + 5) / (90 + 95) = 30 * 185 / 185 = 30
    assert_eq!(calc90.evasion(), 30);
}

#[test]
fn test_m2_stat_calculate_ffi() {
    // m2_stat_calculate(con, int_, str_, dex, level)
    let res = m2_stat_calculate(10, 5, 15, 15, 10);
    // hp: 10 * 20 = 200
    // sp: 5 * 20 = 100
    // atk: 15 * 2 + 10 * 2 = 30 + 20 = 50
    // def: 10
    assert_eq!(res.hp, 200);
    assert_eq!(res.sp, 100);
    assert_eq!(res.atk, 50);
    assert_eq!(res.def, 10);
}

#[test]
fn test_player_stat_calc_overflows() {
    let calc = PlayerStatCalc {
        con: i32::MAX,
        base_hp: i32::MAX,
        equip_hp: i32::MAX,
        ..Default::default()
    };
    let hp = calc.max_hp();
    // i32::MAX * 20 + i32::MAX + i32::MAX -> valid calculation in i64
    let expected = (i32::MAX as i64) * 20 + (i32::MAX as i64) + (i32::MAX as i64);
    assert_eq!(hp, expected);
}
