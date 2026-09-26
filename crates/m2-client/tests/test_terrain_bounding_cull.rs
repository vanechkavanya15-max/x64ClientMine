use m2_client::terrain_bounding_cull::AABB3D;

#[test]
fn test_aabb_creation_from_sector() {
    let sector_x = 1000.0;
    let sector_y = 2000.0;
    let min_h = 50.0;
    let max_h = 150.0;
    let sector_size = 25600.0; // 128 * 200

    let aabb = AABB3D::from_sector(sector_x, sector_y, min_h, max_h, sector_size);

    assert_eq!(aabb.min, (1000.0, 2000.0, 50.0));
    assert_eq!(aabb.max, (26600.0, 27600.0, 150.0));
}

#[test]
fn test_intersects_aabb_overlapping() {
    let aabb1 = AABB3D::new((0.0, 0.0, 0.0), (10.0, 10.0, 10.0));
    let aabb2 = AABB3D::new((5.0, 5.0, 5.0), (15.0, 15.0, 15.0));
    
    assert!(aabb1.intersects_aabb(&aabb2));
    assert!(aabb2.intersects_aabb(&aabb1));

    // Wewnątrz
    let aabb3 = AABB3D::new((2.0, 2.0, 2.0), (8.0, 8.0, 8.0));
    assert!(aabb1.intersects_aabb(&aabb3));
    assert!(aabb3.intersects_aabb(&aabb1));
}

#[test]
fn test_intersects_aabb_disjoint() {
    let aabb1 = AABB3D::new((0.0, 0.0, 0.0), (10.0, 10.0, 10.0));
    let aabb2 = AABB3D::new((11.0, 11.0, 11.0), (20.0, 20.0, 20.0));
    
    assert!(!aabb1.intersects_aabb(&aabb2));
    assert!(!aabb2.intersects_aabb(&aabb1));

    // Stykające się tylko na jednej krawędzi/ścianie (często liczone jako przecięcie w grach, u nas min <= max)
    let aabb3 = AABB3D::new((10.0, 0.0, 0.0), (20.0, 10.0, 10.0));
    assert!(aabb1.intersects_aabb(&aabb3));
}

#[test]
fn test_contains_point() {
    let aabb = AABB3D::new((0.0, 0.0, 0.0), (10.0, 10.0, 10.0));

    // Wewnątrz
    assert!(aabb.contains_point((5.0, 5.0, 5.0)));
    
    // Na granicy
    assert!(aabb.contains_point((0.0, 0.0, 0.0)));
    assert!(aabb.contains_point((10.0, 10.0, 10.0)));
    assert!(aabb.contains_point((5.0, 10.0, 5.0)));

    // Na zewnątrz
    assert!(!aabb.contains_point((-1.0, 5.0, 5.0)));
    assert!(!aabb.contains_point((11.0, 5.0, 5.0)));
    assert!(!aabb.contains_point((5.0, 5.0, -1.0)));
    assert!(!aabb.contains_point((5.0, 5.0, 11.0)));
}

#[test]
fn test_is_in_frustum_planes() {
    // Prosta scena, bryła od (0,0,0) do (10,10,10)
    let aabb = AABB3D::new((0.0, 0.0, 0.0), (10.0, 10.0, 10.0));

    // Płaszczyzna (normalna, dystans). a*x + b*y + c*z + d = 0
    // Przyjmijmy płaszczyznę: x >= -5. Normalna to (1, 0, 0), dystans d = 5.
    // 1*x + 0*y + 0*z + 5 >= 0 -> x >= -5.
    let plane_x_ge_minus_5 = (1.0, 0.0, 0.0, 5.0);
    // Cały AABB ma x w [0, 10], więc x >= -5 jest spełnione dla wszystkich punktów.
    assert!(aabb.is_in_frustum_planes(&[plane_x_ge_minus_5, plane_x_ge_minus_5, plane_x_ge_minus_5, plane_x_ge_minus_5, plane_x_ge_minus_5, plane_x_ge_minus_5]));

    // Płaszczyzna, która całkowicie odrzuca AABB: x <= -5 -> -x - 5 >= 0.
    // Normalna (-1, 0, 0), d = -5.
    // Max wartość: -1 * 0 + 0*0 + 0*0 - 5 = -5 < 0. Zatem odrzucone.
    let plane_x_le_minus_5 = (-1.0, 0.0, 0.0, -5.0);
    assert!(!aabb.is_in_frustum_planes(&[plane_x_le_minus_5, plane_x_ge_minus_5, plane_x_ge_minus_5, plane_x_ge_minus_5, plane_x_ge_minus_5, plane_x_ge_minus_5]));

    // Płaszczyzna przecinająca AABB: x >= 5.
    // Normalna (1, 0, 0), d = -5. 
    // Punkt min (0,0,0) -> 1*0 - 5 = -5 < 0 (poza)
    // Punkt max (10,10,10) -> 1*10 - 5 = 5 >= 0 (wewnątrz)
    // is_in_frustum_planes powinno zwrócić true (nie całkowicie odrzucone).
    let plane_x_ge_5 = (1.0, 0.0, 0.0, -5.0);
    assert!(aabb.is_in_frustum_planes(&[plane_x_ge_5, plane_x_ge_minus_5, plane_x_ge_minus_5, plane_x_ge_minus_5, plane_x_ge_minus_5, plane_x_ge_minus_5]));

    // Płaszczyzna pochylona odrzucająca wszystko.
    // x + y + z + d >= 0.
    // Max x,y,z to 10. Więc max x+y+z = 30.
    // Jeśli d = -31, to max wartość to 30 - 31 = -1 < 0. Odrzucone.
    let plane_angled_out = (1.0, 1.0, 1.0, -31.0);
    assert!(!aabb.is_in_frustum_planes(&[plane_angled_out, plane_x_ge_minus_5, plane_x_ge_minus_5, plane_x_ge_minus_5, plane_x_ge_minus_5, plane_x_ge_minus_5]));

    // Jeśli d = -29, max to 30 - 29 = 1 >= 0. Przecina, więc false (w sensie cullingu). Funkcja zwraca true.
    let plane_angled_in = (1.0, 1.0, 1.0, -29.0);
    assert!(aabb.is_in_frustum_planes(&[plane_angled_in, plane_x_ge_minus_5, plane_x_ge_minus_5, plane_x_ge_minus_5, plane_x_ge_minus_5, plane_x_ge_minus_5]));
}
