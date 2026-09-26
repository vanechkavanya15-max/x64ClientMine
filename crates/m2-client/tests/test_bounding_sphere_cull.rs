use m2_client::bounding_sphere_cull::{Frustum, Plane, ViewState};

fn get_standard_frustum() -> Frustum {
    // A simple orthographic-like frustum box centered at origin
    // Planes: Left, Right, Bottom, Top, Near, Far
    Frustum::new([
        Plane::new(1.0, 0.0, 0.0, 10.0),   // Left: x >= -10 => 1*x + 10 >= 0
        Plane::new(-1.0, 0.0, 0.0, 10.0),  // Right: x <= 10 => -1*x + 10 >= 0
        Plane::new(0.0, 1.0, 0.0, 10.0),   // Bottom: y >= -10 => 1*y + 10 >= 0
        Plane::new(0.0, -1.0, 0.0, 10.0),  // Top: y <= 10 => -1*y + 10 >= 0
        Plane::new(0.0, 0.0, 1.0, 10.0),   // Near: z >= -10 => 1*z + 10 >= 0
        Plane::new(0.0, 0.0, -1.0, 10.0),  // Far: z <= 10 => -1*z + 10 >= 0
    ])
}

#[test]
fn test_sphere_inside() {
    let frustum = get_standard_frustum();
    let state = frustum.cull_sphere((0.0, 0.0, 0.0), 2.0);
    // dist to planes = 10.0. 10.0 is not < -2.0. 10.0 is not < 2.0.
    // So has_intersected = false -> Inside.
    assert_eq!(state, ViewState::Inside);
}

#[test]
fn test_sphere_outside() {
    let frustum = get_standard_frustum();
    // Sphere completely outside to the right: x = 20.0
    // Right plane: -1.0 * 20.0 + 10.0 = -10.0
    // dist (-10.0) < -radius (-2.0) -> Outside
    let state = frustum.cull_sphere((20.0, 0.0, 0.0), 2.0);
    assert_eq!(state, ViewState::Outside);
}

#[test]
fn test_sphere_partial() {
    let frustum = get_standard_frustum();
    // Sphere on the right edge: x = 10.0
    // Right plane: -1.0 * 10.0 + 10.0 = 0.0
    // dist (0.0) is not < -radius (-2.0)
    // dist (0.0) < radius (2.0) -> has_intersected = true -> Partial
    let state = frustum.cull_sphere((10.0, 0.0, 0.0), 2.0);
    assert_eq!(state, ViewState::Partial);
}

#[test]
fn test_sphere_zero_radius_inside() {
    let frustum = get_standard_frustum();
    let state = frustum.cull_sphere((0.0, 0.0, 0.0), 0.0);
    assert_eq!(state, ViewState::Inside);
}

#[test]
fn test_sphere_zero_radius_outside() {
    let frustum = get_standard_frustum();
    let state = frustum.cull_sphere((11.0, 0.0, 0.0), 0.0);
    // Right plane: -1 * 11 + 10 = -1
    // dist (-1) < -radius (0) -> Outside
    assert_eq!(state, ViewState::Outside);
}

#[test]
fn test_sphere_zero_radius_boundary() {
    let frustum = get_standard_frustum();
    let state = frustum.cull_sphere((10.0, 0.0, 0.0), 0.0);
    // Right plane: -1 * 10 + 10 = 0
    // dist (0.0) < -0.0 -> false
    // dist (0.0) < 0.0 -> false
    // -> Inside
    assert_eq!(state, ViewState::Inside);
}
