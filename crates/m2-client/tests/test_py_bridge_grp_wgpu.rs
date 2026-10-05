// crates/m2-client/tests/test_py_bridge_grp_wgpu.rs
use m2_client::py_bridge_grp_wgpu::*;

#[test]
fn test_generate_color_argb() {
    // Bialy (1.0, 1.0, 1.0, 1.0) -> 0xFFFFFFFF
    let white = m2_grp_generate_color(1.0, 1.0, 1.0, 1.0);
    assert_eq!(white, 0xFFFFFFFF);

    // Czarny (0.0, 0.0, 0.0, 1.0) -> 0xFF000000
    let black = m2_grp_generate_color(0.0, 0.0, 0.0, 1.0);
    assert_eq!(black, 0xFF000000);

    // Czerwony (1.0, 0.0, 0.0, 1.0) -> 0xFFFF0000
    let red = m2_grp_generate_color(1.0, 0.0, 0.0, 1.0);
    assert_eq!(red, 0xFFFF0000);

    // Zielony (0.0, 1.0, 0.0, 1.0) -> 0xFF00FF00
    let green = m2_grp_generate_color(0.0, 1.0, 0.0, 1.0);
    assert_eq!(green, 0xFF00FF00);

    // Niebieski z 50% alfa
    let semi_blue = m2_grp_generate_color(0.0, 0.0, 1.0, 0.5);
    let alpha = (semi_blue >> 24) & 0xFF;
    assert_eq!(alpha, 128);
}

#[test]
fn test_render_state_stack() {
    m2_grp_set_game_render_state();
    let state_game = get_current_grp_state();
    assert_eq!(state_game.mode, GrpRenderMode::Game3D);
    assert!(state_game.depth_test);
    assert!(state_game.depth_write);

    // Push Game state
    m2_grp_push_state();

    // Switch to UI state
    m2_grp_set_interface_render_state();
    let state_ui = get_current_grp_state();
    assert_eq!(state_ui.mode, GrpRenderMode::Interface2D);
    assert!(!state_ui.depth_test);
    assert!(!state_ui.depth_write);

    // Pop state back to Game
    m2_grp_pop_state();
    let state_restored = get_current_grp_state();
    assert_eq!(state_restored.mode, GrpRenderMode::Game3D);
    assert!(state_restored.depth_test);
}

#[test]
fn test_clear_depth_buffer() {
    m2_grp_clear_depth_buffer();
    let state = get_current_grp_state();
    assert!(state.depth_clear_requested);
}
