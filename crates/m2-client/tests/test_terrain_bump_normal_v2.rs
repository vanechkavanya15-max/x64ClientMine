#[path = "../src/terrain_bump_normal_v2.rs"]
mod terrain_bump_normal_v2;
use terrain_bump_normal_v2::*;

#[test]
fn test_m2_terrain_calc_tangent_basis() {
    let pos_center = [0.0, 0.0, 0.0];
    let pos_right = [1.0, 0.0, 0.0];
    let pos_up = [0.0, 1.0, 0.0];
    let uv_center = [0.0, 0.0];
    let uv_right = [1.0, 0.0];
    let uv_up = [0.0, 1.0];
    let mut normal = [0.0; 3];
    let mut tangent = [0.0; 3];
    let mut binormal = [0.0; 3];

    assert_eq!(
        m2_terrain_calc_tangent_basis(
            &pos_center, &pos_right, &pos_up,
            &uv_center, &uv_right, &uv_up,
            &mut normal, &mut tangent, &mut binormal
        ),
        true
    );

    assert_eq!(normal, [0.0, 0.0, 1.0]);
    assert_eq!(tangent, [1.0, 0.0, 0.0]);
    assert_eq!(binormal, [0.0, 1.0, 0.0]);
}

#[test]
fn test_calc_tangent_basis_flat_plane() {
    let pos_center = [0.0, 0.0, 0.0];
    let pos_right = [2.0, 0.0, 0.0];
    let pos_up = [0.0, 2.0, 0.0];
    
    let uv_center = [0.0, 0.0];
    let uv_right = [1.0, 0.0];
    let uv_up = [0.0, 1.0];
    
    let mut normal = [0.0; 3];
    let mut tangent = [0.0; 3];
    let mut binormal = [0.0; 3];
    
    m2_terrain_calc_tangent_basis(
        &pos_center, &pos_right, &pos_up,
        &uv_center, &uv_right, &uv_up,
        &mut normal, &mut tangent, &mut binormal
    );
    
    assert_eq!(normal, [0.0, 0.0, 1.0]);
    assert_eq!(tangent, [1.0, 0.0, 0.0]);
    assert_eq!(binormal, [0.0, 1.0, 0.0]);
}

#[test]
fn test_calc_tangent_basis_sloped_plane() {
    let pos_center = [0.0, 0.0, 0.0];
    let pos_right = [1.0, 0.0, 1.0];
    let pos_up = [0.0, 1.0, 0.0];
    
    let uv_center = [0.0, 0.0];
    let uv_right = [1.0, 0.0];
    let uv_up = [0.0, 1.0];
    
    let mut normal = [0.0; 3];
    let mut tangent = [0.0; 3];
    let mut binormal = [0.0; 3];
    
    m2_terrain_calc_tangent_basis(
        &pos_center, &pos_right, &pos_up,
        &uv_center, &uv_right, &uv_up,
        &mut normal, &mut tangent, &mut binormal
    );
    
    let len_t = (1.0_f32 + 1.0).sqrt();
    assert!((tangent[0] - 1.0/len_t).abs() < 1e-5);
    assert_eq!(tangent[1], 0.0);
    assert!((tangent[2] - 1.0/len_t).abs() < 1e-5);
    
    let len_n = (1.0_f32 + 1.0).sqrt();
    assert!((normal[0] - -1.0/len_n).abs() < 1e-5);
    assert_eq!(normal[1], 0.0);
    assert!((normal[2] - 1.0/len_n).abs() < 1e-5);
}

#[test]
fn test_calc_tangent_basis_flipped_uvs() {
    let pos_center = [0.0, 0.0, 0.0];
    let pos_right = [1.0, 0.0, 0.0];
    let pos_up = [0.0, 1.0, 0.0];
    
    let uv_center = [0.0, 0.0];
    let uv_right = [-1.0, 0.0];
    let uv_up = [0.0, -1.0];
    
    let mut normal = [0.0; 3];
    let mut tangent = [0.0; 3];
    let mut binormal = [0.0; 3];
    
    m2_terrain_calc_tangent_basis(
        &pos_center, &pos_right, &pos_up,
        &uv_center, &uv_right, &uv_up,
        &mut normal, &mut tangent, &mut binormal
    );
    
    assert_eq!(tangent, [-1.0, 0.0, 0.0]);
    assert_eq!(binormal, [0.0, -1.0, 0.0]);
    assert_eq!(normal, [0.0, 0.0, 1.0]);
}

#[test]
fn test_calc_tangent_basis_zero_uv_area() {
    let pos_center = [0.0, 0.0, 0.0];
    let pos_right = [1.0, 0.0, 0.0];
    let pos_up = [0.0, 1.0, 0.0];
    
    let uv_center = [0.0, 0.0];
    let uv_right = [0.0, 0.0];
    let uv_up = [0.0, 0.0];
    
    let mut normal = [0.0; 3];
    let mut tangent = [0.0; 3];
    let mut binormal = [0.0; 3];
    
    m2_terrain_calc_tangent_basis(
        &pos_center, &pos_right, &pos_up,
        &uv_center, &uv_right, &uv_up,
        &mut normal, &mut tangent, &mut binormal
    );
    
    // With zero UV area, the denominator `f` is 1/0 (infinity) or NaN.
    // So the output should be handled gracefully (e.g., zeroes or NaNs, but it should not crash).
    // Based on the code above, the values will likely be NaNs if not handled.
    // Let's modify the rust file to handle it and check for zeroes.
    assert_eq!(tangent, [0.0, 0.0, 0.0]);
    assert_eq!(binormal, [0.0, 0.0, 0.0]);
}

#[test]
fn test_calc_tangent_basis_complex_slope() {
    let pos_center = [1.0, 2.0, 3.0];
    let pos_right = [3.0, 3.0, 5.0];
    let pos_up = [0.0, 4.0, 2.0];
    
    let uv_center = [0.2, 0.3];
    let uv_right = [0.8, 0.4];
    let uv_up = [0.3, 0.9];
    
    let mut normal = [0.0; 3];
    let mut tangent = [0.0; 3];
    let mut binormal = [0.0; 3];
    
    m2_terrain_calc_tangent_basis(
        &pos_center, &pos_right, &pos_up,
        &uv_center, &uv_right, &uv_up,
        &mut normal, &mut tangent, &mut binormal
    );
    
    // verify orthogonal properties
    let t_dot_n = tangent[0]*normal[0] + tangent[1]*normal[1] + tangent[2]*normal[2];
    assert!(t_dot_n.abs() < 1e-5);
    
    let b_dot_n = binormal[0]*normal[0] + binormal[1]*normal[1] + binormal[2]*normal[2];
    assert!(b_dot_n.abs() < 1e-5);
}

#[test]
fn test_calc_tangent_basis_zero_edge() {
    let pos_center = [0.0, 0.0, 0.0];
    let pos_right = [0.0, 0.0, 0.0]; // Zero length edge
    let pos_up = [0.0, 1.0, 0.0];
    
    let uv_center = [0.0, 0.0];
    let uv_right = [1.0, 0.0];
    let uv_up = [0.0, 1.0];
    
    let mut normal = [0.0; 3];
    let mut tangent = [0.0; 3];
    let mut binormal = [0.0; 3];
    
    m2_terrain_calc_tangent_basis(
        &pos_center, &pos_right, &pos_up,
        &uv_center, &uv_right, &uv_up,
        &mut normal, &mut tangent, &mut binormal
    );
    
    assert_eq!(normal, [0.0, 0.0, 0.0]); // Normal has zero length
}
