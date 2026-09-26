/// Calculates the normal, tangent, and binormal vectors for a terrain vertex
/// given its adjacent vertices and the UV coordinates.
///
/// It follows the logic used in the C++ MapOutdoorRender for computing the 
/// Tangent Basis (Tangent and Binormal) for bump mapping.
/// 
/// Returns `true` if successful (always).
#[no_mangle]
pub extern "C" fn m2_terrain_calc_tangent_basis(
    pos_center: &[f32; 3],
    pos_right: &[f32; 3], // v1
    pos_up: &[f32; 3],    // v2
    uv_center: &[f32; 2],
    uv_right: &[f32; 2],  // uv1
    uv_up: &[f32; 2],     // uv2
    out_normal: &mut [f32; 3],
    out_tangent: &mut [f32; 3],
    out_binormal: &mut [f32; 3],
) -> bool {
    let edge1 = [
        pos_right[0] - pos_center[0],
        pos_right[1] - pos_center[1],
        pos_right[2] - pos_center[2],
    ];
    let edge2 = [
        pos_up[0] - pos_center[0],
        pos_up[1] - pos_center[1],
        pos_up[2] - pos_center[2],
    ];

    let delta_uv1 = [
        uv_right[0] - uv_center[0],
        uv_right[1] - uv_center[1],
    ];
    let delta_uv2 = [
        uv_up[0] - uv_center[0],
        uv_up[1] - uv_center[1],
    ];

    let det = delta_uv1[0] * delta_uv2[1] - delta_uv2[0] * delta_uv1[1];
    
    let f = if det.abs() < 1e-6 {
        0.0 // Avoid division by zero
    } else {
        1.0 / det
    };

    let tangent = [
        f * (delta_uv2[1] * edge1[0] - delta_uv1[1] * edge2[0]),
        f * (delta_uv2[1] * edge1[1] - delta_uv1[1] * edge2[1]),
        f * (delta_uv2[1] * edge1[2] - delta_uv1[1] * edge2[2]),
    ];

    let binormal = [
        f * (-delta_uv2[0] * edge1[0] + delta_uv1[0] * edge2[0]),
        f * (-delta_uv2[0] * edge1[1] + delta_uv1[0] * edge2[1]),
        f * (-delta_uv2[0] * edge1[2] + delta_uv1[0] * edge2[2]),
    ];

    let normal = [
        edge1[1] * edge2[2] - edge1[2] * edge2[1],
        edge1[2] * edge2[0] - edge1[0] * edge2[2],
        edge1[0] * edge2[1] - edge1[1] * edge2[0],
    ];

    fn normalize(v: &[f32; 3]) -> [f32; 3] {
        let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        if len < 1e-6 {
            [0.0, 0.0, 0.0]
        } else {
            [v[0] / len, v[1] / len, v[2] / len]
        }
    }

    *out_tangent = normalize(&tangent);
    *out_binormal = normalize(&binormal);
    *out_normal = normalize(&normal);

    true
}
