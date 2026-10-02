use m2_client::ui_in_game_hud_complete::{PlayerStatus, ExpBar, TaskBar, MinimapHud};
use m2_client::recv_points::GlobalGameState;
use m2_client::stamina_system::StaminaSystem;
use m2_client::recv_quickslot_sync_v2::SLOT_TYPE_SKILL;

#[test]
fn test_player_status_calculations() {
    let mut state = GlobalGameState::new();
    state.hp = 500;
    state.max_hp = 1000;
    state.sp = 200;
    state.max_sp = 800;

    let mut stamina_sys = StaminaSystem::new();
    stamina_sys.current_stamina = 750.0;
    stamina_sys.max_stamina = 1000.0;

    let mut status = PlayerStatus::new();
    status.update(&state, &stamina_sys);

    assert_eq!(status.hp_percent, 0.5);
    assert_eq!(status.sp_percent, 0.25);
    assert_eq!(status.stamina_percent, 0.75);

    // Edge case: overflow / 0 max
    state.max_hp = 0;
    status.update(&state, &stamina_sys);
    assert_eq!(status.hp_percent, 0.0);
}

#[test]
fn test_exp_bar_globes() {
    let mut state = GlobalGameState::new();
    let mut exp_bar = ExpBar::new();

    // 0%
    state.exp = 0;
    state.next_exp = 1000;
    exp_bar.update(&state);
    assert_eq!(exp_bar.globes, [0.0, 0.0, 0.0, 0.0]);

    // 12.5% (Half of first globe)
    state.exp = 125;
    exp_bar.update(&state);
    assert_eq!(exp_bar.globes, [50.0, 0.0, 0.0, 0.0]);

    // 30% (First globe full, second is 5% which is 5/25 = 20% full)
    state.exp = 300;
    exp_bar.update(&state);
    assert!((exp_bar.globes[0] - 100.0).abs() < f32::EPSILON);
    assert!((exp_bar.globes[1] - 20.0).abs() < 1e-5);
    assert!((exp_bar.globes[2] - 0.0).abs() < f32::EPSILON);
    assert!((exp_bar.globes[3] - 0.0).abs() < f32::EPSILON);

    // 100%
    state.exp = 1000;
    exp_bar.update(&state);
    assert_eq!(exp_bar.globes, [100.0, 100.0, 100.0, 100.0]);
}

#[test]
fn test_taskbar_cooldown() {
    let mut taskbar = TaskBar::new();
    
    // add an item
    taskbar.quickslot_window.add_quickslot(0, SLOT_TYPE_SKILL, 123);
    
    // start cooldown: duration 10s at t=5.0
    taskbar.set_cooldown(0, 5.0, 10.0);

    // t=5.0 => 100% cooldown
    assert_eq!(taskbar.get_cooldown_percent(0, 5.0), 1.0);
    
    // t=10.0 => elapsed 5s, 5/10 = 0.5 (50% remaining)
    assert_eq!(taskbar.get_cooldown_percent(0, 10.0), 0.5);

    // t=15.0 => 0% cooldown remaining
    assert_eq!(taskbar.get_cooldown_percent(0, 15.0), 0.0);
}

#[test]
fn test_taskbar_key_press() {
    let mut taskbar = TaskBar::new();
    taskbar.quickslot_window.add_quickslot(0, SLOT_TYPE_SKILL, 123);
    
    let result = taskbar.on_key_pressed(0x31, false);
    assert!(result.is_some());
    let slot = result.unwrap();
    assert_eq!(slot.slot_type, SLOT_TYPE_SKILL);
    let pos = { slot.slot_position };
    assert_eq!(pos, 123);

    // Change page
    let none_result = taskbar.on_key_pressed(0x32, true);
    assert!(none_result.is_none());
    assert_eq!(taskbar.quickslot_window.active_page, 1);
}

#[test]
fn test_minimap_hud_zoom_and_pins() {
    let mut minimap = MinimapHud::new(100, 100.0);
    assert_eq!(minimap.window.state.zoom_scale, 1.0);

    let _ = minimap.zoom_in();
    assert_eq!(minimap.window.state.zoom_scale, 2.0);
    
    let res = minimap.zoom_in();
    assert!(res.is_err()); // Max zoom

    let _ = minimap.zoom_out();
    assert_eq!(minimap.window.state.zoom_scale, 1.0);
    
    // Add pins and check logic
    minimap.window.state.player_x = 0.0;
    minimap.window.state.player_y = 0.0;
    
    minimap.add_pin_player(1, 0.0, 0.0);
    minimap.add_pin_monster(2, 1000.0, 0.0); // Out of bounds screen mapping clamp
    minimap.add_pin_npc(3, 10.0, -10.0);

    let pin_screen_data = minimap.get_pins_with_screen_pos();
    assert_eq!(pin_screen_data.len(), 3);
    
    assert_eq!(pin_screen_data[0].0, 1); // Player
    assert_eq!(pin_screen_data[0].2, 0xFFFFFFFF);
    
    assert_eq!(pin_screen_data[1].0, 2); // Monster
    assert_eq!(pin_screen_data[1].2, 0xFFFF0000);
    assert_eq!(pin_screen_data[1].1.y, 0.0); // clamped or on line
    
    assert_eq!(pin_screen_data[2].0, 3); // NPC
    assert_eq!(pin_screen_data[2].2, 0xFFFFFF00);

    minimap.update_camera_rotation(90.0);
    assert_eq!(minimap.compass_rotation, 90.0);
}
