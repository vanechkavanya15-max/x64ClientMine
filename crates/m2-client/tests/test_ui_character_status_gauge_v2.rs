#[path = "../src/ui_character_status_gauge_v2.rs"]
mod ui_character_status_gauge_v2;
use ui_character_status_gauge_v2::*;

#[test]
fn test_gauge_hp_percentage_and_clipping() {
    let mut gauge = GaugeState::new(200.0, 20.0);
    gauge.set_value(50.0, 100.0);
    
    // Test instant display bypass to check percentage
    gauge.display_value = 50.0;
    assert!((gauge.get_percentage() - 0.5).abs() < f64::EPSILON);
    
    let clip_rect = gauge.get_clip_rect();
    assert_eq!(clip_rect.left, 0.0);
    assert_eq!(clip_rect.top, 0.0);
    assert_eq!(clip_rect.right, 100.0);
    assert_eq!(clip_rect.bottom, 20.0);
}

#[test]
fn test_gauge_hp_flash_animation() {
    let mut gauge = GaugeState::new(100.0, 15.0);
    gauge.set_value(100.0, 100.0);
    
    assert!(!gauge.is_flashing);
    assert_eq!(gauge.flash_alpha, 0.0);

    // Taking damage should trigger flash
    gauge.set_value(50.0, 100.0);
    assert!(gauge.is_flashing);
    assert_eq!(gauge.flash_alpha, 1.0);

    // Update time to fade out
    gauge.update(0.25);
    assert!(gauge.is_flashing);
    assert!((gauge.flash_alpha - 0.5).abs() < f32::EPSILON);

    // Finish fade out
    gauge.update(0.25);
    assert!(!gauge.is_flashing);
    assert_eq!(gauge.flash_alpha, 0.0);
}

#[test]
fn test_exp_taskbar_gauge_orbs() {
    let mut exp_gauge = ExpTaskbarGauge::new();
    
    // 0% EXP
    exp_gauge.set_exp(0, 100);
    let orbs = exp_gauge.get_orbs_percentages();
    assert_eq!(orbs, [0.0, 0.0, 0.0, 0.0]);

    // 12.5% EXP (Half of first bulb)
    exp_gauge.set_exp(125, 1000);
    let orbs = exp_gauge.get_orbs_percentages();
    assert_eq!(orbs, [0.5, 0.0, 0.0, 0.0]);

    // 50% EXP (Two full bulbs)
    exp_gauge.set_exp(50, 100);
    let orbs = exp_gauge.get_orbs_percentages();
    assert_eq!(orbs, [1.0, 1.0, 0.0, 0.0]);

    // 80% EXP (Three full bulbs, fourth is 20% filled -> 0.8 / 0.25 = 3.2 bulbs -> 4th bulb is 0.2)
    exp_gauge.set_exp(80, 100);
    let orbs = exp_gauge.get_orbs_percentages();
    assert_eq!(orbs[0], 1.0);
    assert_eq!(orbs[1], 1.0);
    assert_eq!(orbs[2], 1.0);
    assert!((orbs[3] - 0.2).abs() < f32::EPSILON);
    
    // 100% EXP (All full)
    exp_gauge.set_exp(100, 100);
    let orbs = exp_gauge.get_orbs_percentages();
    assert_eq!(orbs, [1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn test_c_ffi_hp_gauge() {
    m2_ui_gauge_v2_set_hp(100.0, 100.0);
    m2_ui_gauge_v2_set_hp(50.0, 100.0); // take damage to trigger flash

    assert!(m2_ui_gauge_v2_is_hp_flashing());
    assert_eq!(m2_ui_gauge_v2_get_hp_flash_alpha(), 1.0);

    m2_ui_gauge_v2_update(0.25);
    
    assert!(m2_ui_gauge_v2_is_hp_flashing());
    assert!((m2_ui_gauge_v2_get_hp_flash_alpha() - 0.5).abs() < f32::EPSILON);
    
    let mut rect = RectClip { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 };
    m2_ui_gauge_v2_get_hp_clip_rect(&mut rect);
    
    assert_eq!(rect.left, 0.0);
    assert_eq!(rect.top, 0.0);
    assert_eq!(rect.bottom, 15.0);
}

#[test]
fn test_c_ffi_exp_gauge() {
    m2_ui_gauge_v2_set_exp(75, 100);
    
    let mut orbs = [0.0f32; 4];
    m2_ui_gauge_v2_get_exp_orbs(orbs.as_mut_ptr());
    
    assert_eq!(orbs[0], 1.0);
    assert_eq!(orbs[1], 1.0);
    assert_eq!(orbs[2], 1.0);
    assert_eq!(orbs[3], 0.0);
}

#[test]
fn test_enum_coverage() {
    let _t = GaugeType::Hp;
    let _t = GaugeType::Sp;
    let _t = GaugeType::Stamina;
}
