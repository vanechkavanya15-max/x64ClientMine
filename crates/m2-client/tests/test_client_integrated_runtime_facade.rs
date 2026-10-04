use m2_client::client_integrated_runtime_facade::ClientIntegratedRuntime;
use glam::Vec3;

#[test]
fn test_client_integrated_runtime_lifecycle() {
    let mut runtime = ClientIntegratedRuntime::new();
    assert!(runtime.is_running);
    assert_eq!(runtime.current_phase, 1);

    runtime.switch_phase(5);
    assert_eq!(runtime.current_phase, 5);

    runtime.update(0.016);
    assert!(runtime.is_running);
}

#[test]
fn test_splash_aoe_calculation() {
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let direction = Vec3::new(0.0, 0.0, 1.0);
    let max_dist = 500.0;
    let angle_deg = 90.0;

    let target_in_cone = Vec3::new(10.0, 0.0, 200.0);
    assert!(ClientIntegratedRuntime::is_point_in_cone_splash(
        origin, direction, max_dist, angle_deg, target_in_cone
    ));

    let target_behind = Vec3::new(0.0, 0.0, -100.0);
    assert!(!ClientIntegratedRuntime::is_point_in_cone_splash(
        origin, direction, max_dist, angle_deg, target_behind
    ));
}

#[test]
fn test_belt_inventory_validation() {
    assert!(ClientIntegratedRuntime::can_store_in_belt(27001));
    assert!(!ClientIntegratedRuntime::can_store_in_belt(10));
}
