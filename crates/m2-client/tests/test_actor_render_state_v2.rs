use m2_client::actor_render_state_v2::*;

#[test]
fn test_m2_actor_get_render_state_hair() {
    let res = m2_actor_get_render_state(PART_HAIR);
    assert_eq!(res.cull_mode, D3DCULL_NONE); // Two-sided
    assert_eq!(res.alpha_ref, 128);
    assert_eq!(res.specular_enable, false);
}

#[test]
fn test_m2_actor_get_render_state_main() {
    let res = m2_actor_get_render_state(PART_MAIN);
    assert_eq!(res.cull_mode, D3DCULL_CW);
    assert_eq!(res.alpha_ref, 128);
    assert_eq!(res.specular_enable, true);
}

#[test]
fn test_m2_actor_get_render_state_weapon() {
    let res = m2_actor_get_render_state(PART_WEAPON);
    assert_eq!(res.cull_mode, D3DCULL_CW);
    assert_eq!(res.alpha_ref, 128);
    assert_eq!(res.specular_enable, false);
}

#[test]
fn test_m2_actor_get_render_state_head() {
    let res = m2_actor_get_render_state(PART_HEAD);
    assert_eq!(res.cull_mode, D3DCULL_CW);
    assert_eq!(res.alpha_ref, 128);
    assert_eq!(res.specular_enable, false);
}

#[test]
fn test_armor_specular_plus_6_and_below() {
    for vnum in [11290, 11291, 11292, 11293, 11294, 11295, 11296] {
        let power = m2_actor_get_specular_power(vnum);
        assert_eq!(power, 0.0, "Vnum {} should have 0 specular power", vnum);
    }
}

#[test]
fn test_armor_specular_plus_7() {
    let power = m2_actor_get_specular_power(11297);
    assert_eq!(power, 0.5);
}

#[test]
fn test_armor_specular_plus_8() {
    let power = m2_actor_get_specular_power(11298);
    assert_eq!(power, 0.7);
}

#[test]
fn test_armor_specular_plus_9() {
    let power = m2_actor_get_specular_power(11299);
    assert_eq!(power, 1.0);
}
