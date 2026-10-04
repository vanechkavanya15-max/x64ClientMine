use m2_client::wgpu_speedtree_wind_pipeline::{
    compute_wind_displacement,
    compute_billboard_view_facing,
};
use glam::{Mat4, Vec3};

#[test]
fn test_wind_zero_weight_no_displacement() {
    let position = Vec3::new(10.0, 5.0, 3.0);
    // Arbitralna macierz wiatru przesuwajaca wszystko o +10 w Y
    let wind_matrix = Mat4::from_translation(Vec3::new(0.0, 10.0, 0.0));
    let wind_weight = 0.0; // ZERO wiatru
    
    let result = compute_wind_displacement(position, wind_matrix, wind_weight);
    
    // Wierzcholek powinien pozostac w pierwotnej pozycji
    assert_eq!(result.x, 10.0);
    assert_eq!(result.y, 5.0);
    assert_eq!(result.z, 3.0);
}

#[test]
fn test_wind_full_weight_full_displacement() {
    let position = Vec3::new(10.0, 5.0, 3.0);
    // Arbitralna macierz wiatru obracajaca o 90 stopni wzgledem osi Z (w uproszczeniu tutaj uzywamy przesuniecia testowego)
    let wind_matrix = Mat4::from_translation(Vec3::new(5.0, -2.0, 1.0));
    let wind_weight = 1.0; // MAX wiatru
    
    let result = compute_wind_displacement(position, wind_matrix, wind_weight);
    
    // Wierzcholek powinien byc w pelni przesuniety macierza
    assert_eq!(result.x, 15.0);
    assert_eq!(result.y, 3.0);
    assert_eq!(result.z, 4.0);
}

#[test]
fn test_wind_partial_weight_interpolated_displacement() {
    let position = Vec3::new(10.0, 5.0, 3.0);
    // Przesuniecie o +10.0 w X, Y, Z
    let wind_matrix = Mat4::from_translation(Vec3::new(10.0, 10.0, 10.0));
    let wind_weight = 0.5; // POLOWA wiatru
    
    let result = compute_wind_displacement(position, wind_matrix, wind_weight);
    
    // Wierzcholek powinien byc przesuniety w polowie pomiedzy oryginalem a p_wind_full
    assert_eq!(result.x, 15.0); // 10.0 + (20.0 - 10.0) * 0.5
    assert_eq!(result.y, 10.0); // 5.0 + (15.0 - 5.0) * 0.5
    assert_eq!(result.z, 8.0);  // 3.0 + (13.0 - 3.0) * 0.5
}

#[test]
fn test_billboard_view_facing_calculation() {
    let center = Vec3::new(100.0, 100.0, 50.0);
    let offset_x = 10.0;
    let offset_y = 20.0;
    
    // Klasyczna macierz widoku kamery - look at
    let view_matrix = Mat4::look_at_rh(
        Vec3::new(0.0, 0.0, 0.0), // oko
        Vec3::new(0.0, 1.0, 0.0), // cel (patrzy wzdluz osi Y)
        Vec3::new(0.0, 0.0, 1.0), // Z up
    );
    
    let result = compute_billboard_view_facing(center, offset_x, offset_y, view_matrix);
    
    // Dla tej macierzy widoku: wektor "w prawo" z perspektywy kamery na Y to X (albo -X)
    // wektor "w gore" kamery to Z
    // right = (1, 0, 0), up = (0, 0, 1) - zalezy od dokladnej implementacji look_at
    
    // Sprawdzamy zaleznosc wektorow
    let right = Vec3::new(view_matrix.x_axis.x, view_matrix.y_axis.x, view_matrix.z_axis.x);
    let up = Vec3::new(view_matrix.x_axis.y, view_matrix.y_axis.y, view_matrix.z_axis.y);
    
    let expected = center + (right * offset_x) + (up * offset_y);
    
    // Sprawdzamy dokladnosc obliczen
    assert!((result.x - expected.x).abs() < 1e-4);
    assert!((result.y - expected.y).abs() < 1e-4);
    assert!((result.z - expected.z).abs() < 1e-4);
}
