#[path = "../src/path_validation.rs"]
mod path_validation;

use path_validation::validate_path;

#[test]
fn test_open_field_path() {
    let result = validate_path(0.0, 0.0, 200.0, 0.0, |_, _| true);
    assert!(!result.has_obstacle);
    assert_eq!(result.end_x, 200.0);
    assert_eq!(result.end_y, 0.0);
}

#[test]
fn test_obstacle_in_path() {
    // Sciezka od 0,0 do 200,0. Przeszkoda na x >= 100.0
    let result = validate_path(0.0, 0.0, 200.0, 0.0, |x, _| x < 100.0);
    assert!(result.has_obstacle);
    // Przed dojsciem do 100, ostatnim bezpiecznym punktem probkowanym jest 50.0 
    // (probki co 50.0: 0.0 -> OK, 50.0 -> OK, 100.0 -> FAIL)
    assert_eq!(result.end_x, 50.0);
    assert_eq!(result.end_y, 0.0);
}

#[test]
fn test_safe_point_before_wall() {
    // Cel na granicy sciany. Sciana zaczyna sie w x >= 150.0. Cel na 150.0, 0.0
    // Probki: 0.0 (OK), 50.0 (OK), 100.0 (OK). Cel 150.0 sprawdza osobno na koncu jesli wejdzie:
    // Pętla current_distance < 150.0: probkuje 0, 50, 100.
    // Potem is_walkable_fn dla 150.0 -> FAIL.
    // Wiec ostatni wazny punkt to 100.0
    let result = validate_path(0.0, 0.0, 150.0, 0.0, |x, _| x < 150.0);
    assert!(result.has_obstacle);
    assert_eq!(result.end_x, 100.0);
    assert_eq!(result.end_y, 0.0);
}

#[test]
fn test_zero_distance() {
    let result = validate_path(10.0, 20.0, 10.0, 20.0, |_, _| false);
    assert!(!result.has_obstacle);
    assert_eq!(result.end_x, 10.0);
    assert_eq!(result.end_y, 20.0);
}

#[test]
fn test_obstacle_at_start() {
    // Przeszkoda od razu na startowym polu
    let result = validate_path(0.0, 0.0, 100.0, 0.0, |_, _| false);
    assert!(result.has_obstacle);
    // last_valid pozostaje start_x, start_y
    assert_eq!(result.end_x, 0.0);
    assert_eq!(result.end_y, 0.0);
}

#[test]
fn test_diagonal_path_with_obstacle() {
    // 3,4,5 trójkąt proporcje. Start: 0,0, End: 300, 400. Distance = 500
    // dir_x = 0.6, dir_y = 0.8
    // Probki co 50 pkt (czyli +30x, +40y)
    // 0: 0,0 (OK)
    // 50: 30,40 (OK)
    // 100: 60,80 (OK)
    // 150: 90,120 (FAIL) - np. x >= 90
    let result = validate_path(0.0, 0.0, 300.0, 400.0, |x, _| x < 90.0);
    assert!(result.has_obstacle);
    // Przed 90, ostatnie valid probka to 60, 80
    // Uwaga na floating point inprecision
    assert!((result.end_x - 60.0).abs() < 1e-4);
    assert!((result.end_y - 80.0).abs() < 1e-4);
}
