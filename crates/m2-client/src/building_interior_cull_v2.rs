#[no_mangle]
pub extern "C" fn m2_building_is_inside(
    px: f32,
    py: f32,
    pz: f32,
    obb_center: *const [f32; 3],
    obb_extent: *const [f32; 3],
    obb_axes: *const [[f32; 3]; 3],
) -> bool {
    if obb_center.is_null() || obb_extent.is_null() || obb_axes.is_null() {
        return false;
    }

    let center = unsafe { &*obb_center };
    let extent = unsafe { &*obb_extent };
    let axes = unsafe { &*obb_axes };

    // Calculate vector from OBB center to the point
    let vx = px - center[0];
    let vy = py - center[1];
    let vz = pz - center[2];

    // Check each axis
    for i in 0..3 {
        // Dot product of the vector with the axis
        let distance = vx * axes[i][0] + vy * axes[i][1] + vz * axes[i][2];
        
        // If the projected distance is greater than the extent, it's outside
        if distance.abs() > extent[i] {
            return false;
        }
    }

    true
}
