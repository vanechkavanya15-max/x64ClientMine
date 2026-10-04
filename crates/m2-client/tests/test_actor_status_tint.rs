use m2_client::wgpu_actor_status_tint_renderer::{
    ActorNegativeStatus, ActorStatusTintRenderer, 
};

#[test]
fn test_default_status_no_tint() {
    let tint = ActorStatusTintRenderer::calculate_status_tint(ActorNegativeStatus::None, 0);
    assert_eq!(tint.add_color, [0.0, 0.0, 0.0, 0.0]);
    assert_eq!(tint.multiply_color, [1.0, 1.0, 1.0, 1.0]);
    assert_eq!(tint.animation_speed_multiplier, 1.0);

    let base_color = [1.0, 1.0, 1.0, 1.0];
    let final_color = ActorStatusTintRenderer::apply_to_vertex_color(base_color, &tint);
    assert_eq!(final_color, [1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn test_poison_status_tint_pulse() {
    let tint_t0 = ActorStatusTintRenderer::calculate_status_tint(ActorNegativeStatus::Poison, 0);
    assert_eq!(tint_t0.animation_speed_multiplier, 1.0);
    
    // T0
    assert_eq!(tint_t0.add_color[0], 0.0);
    assert!(tint_t0.add_color[1] >= 0.0 && tint_t0.add_color[1] <= 0.5); // Pulse
    assert_eq!(tint_t0.add_color[2], 0.0);

    assert_eq!(tint_t0.multiply_color, [0.5, 1.0, 0.5, 1.0]);

    // Sprawdzenie zmiany w czasie 
    let tint_t1000 = ActorStatusTintRenderer::calculate_status_tint(ActorNegativeStatus::Poison, 1000);
    assert_ne!(tint_t0.add_color[1], tint_t1000.add_color[1]);

    let base_color = [1.0, 1.0, 1.0, 1.0];
    let final_color = ActorStatusTintRenderer::apply_to_vertex_color(base_color, &tint_t0);
    assert_eq!(final_color[0], 0.5); // 1.0 * 0.5 + 0.0
    assert_eq!(final_color[2], 0.5);
}

#[test]
fn test_burn_status_tint() {
    let tint = ActorStatusTintRenderer::calculate_status_tint(ActorNegativeStatus::Burn, 0);
    assert_eq!(tint.animation_speed_multiplier, 1.0);
    
    assert_eq!(tint.add_color, [0.3, 0.1, 0.0, 0.0]);
    assert_eq!(tint.multiply_color, [1.2, 0.8, 0.8, 1.0]);

    let base_color = [0.5, 0.5, 0.5, 1.0];
    let final_color = ActorStatusTintRenderer::apply_to_vertex_color(base_color, &tint);
    
    // R: 0.5 * 1.2 + 0.3 = 0.6 + 0.3 = 0.9
    // G: 0.5 * 0.8 + 0.1 = 0.4 + 0.1 = 0.5
    // B: 0.5 * 0.8 + 0.0 = 0.4
    
    assert!((final_color[0] - 0.9).abs() < 1e-5);
    assert!((final_color[1] - 0.5).abs() < 1e-5);
    assert!((final_color[2] - 0.4).abs() < 1e-5);
    assert_eq!(final_color[3], 1.0);
}

#[test]
fn test_freeze_and_slow_status_tint() {
    let freeze_tint = ActorStatusTintRenderer::calculate_status_tint(ActorNegativeStatus::Freeze, 0);
    assert_eq!(freeze_tint.animation_speed_multiplier, 0.0);
    assert_eq!(freeze_tint.add_color, [0.0, 0.2, 0.4, 0.0]);
    
    let slow_tint = ActorStatusTintRenderer::calculate_status_tint(ActorNegativeStatus::Slow, 0);
    assert_eq!(slow_tint.animation_speed_multiplier, 0.5);
    assert_eq!(slow_tint.add_color, [0.0, 0.2, 0.4, 0.0]);
    
    let base_color = [1.0, 1.0, 1.0, 1.0];
    let final_color = ActorStatusTintRenderer::apply_to_vertex_color(base_color, &freeze_tint);
    
    // R: 1.0 * 0.6 + 0.0 = 0.6
    // G: 1.0 * 0.8 + 0.2 = 1.0
    // B: 1.0 * 1.0 + 0.4 = 1.4 -> clamp -> 1.0
    assert!((final_color[0] - 0.6).abs() < 1e-5);
    assert!((final_color[1] - 1.0).abs() < 1e-5);
    assert!((final_color[2] - 1.0).abs() < 1e-5);
}
