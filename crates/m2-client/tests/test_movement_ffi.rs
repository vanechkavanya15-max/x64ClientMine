use m2_client::movement_ffi::{
    m2_move_to, m2_get_height_at, m2_is_walkable, m2_stop_movement, get_movement_state,
    set_terrain_height, set_terrain_attr
};
use m2_client::terrain_height_calc::{TerrainHeightCalculator, RawHeightMap, HEIGHTMAP_RAW_SIZE, HEIGHTMAP_RAW_XSIZE};
use m2_client::terrain_attr_map::{TerrainAttrMap, ATTR_BLOCK};
use std::sync::Mutex;

static TEST_MUTEX: std::sync::OnceLock<Mutex<()>> = std::sync::OnceLock::new();

fn get_mutex() -> &'static Mutex<()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(()))
}

fn setup_test_terrain() {
    let mut raw_data = [0u16; HEIGHTMAP_RAW_SIZE];
    for y in 0..=3 {
        for x in 0..=3 {
            raw_data[y * HEIGHTMAP_RAW_XSIZE + x] = 100;
        }
    }
    let raw_map = RawHeightMap::new(raw_data);
    let calc = TerrainHeightCalculator::new(raw_map, 1.0);
    set_terrain_height(calc);

    let mut attr_grid = vec![0u8; 128 * 128];
    // Srodek 12800.0 / 200.0 = 64
    attr_grid[64 * 128 + 64] = ATTR_BLOCK;
    let attr_map = TerrainAttrMap::new(128, 128, attr_grid).unwrap();
    set_terrain_attr(attr_map);
}

#[test]
fn test_move_to() {
    let _guard = get_mutex().lock().unwrap();
    setup_test_terrain();
    let target_x = 1000.0;
    let target_y = 1000.0;

    let result = m2_move_to(target_x, target_y);
    assert!(result, "Ruch do dozowolonego punktu powinien zwrocic true");

    let (x, y, is_moving) = get_movement_state();
    assert_eq!(x, target_x, "Oczekiwano zaktualizowania target_x");
    assert_eq!(y, target_y, "Oczekiwano zaktualizowania target_y");
    assert!(is_moving, "Oczekiwano, ze stan ruchu bedzie true");
}

#[test]
fn test_move_to_unwalkable() {
    let _guard = get_mutex().lock().unwrap();
    setup_test_terrain();
    let target_x = -100.0;
    let target_y = -100.0;

    let result = m2_move_to(target_x, target_y);
    assert!(!result, "Ruch do punktu poza mapa powinien zwrocic false");
}

#[test]
fn test_get_height_at() {
    let _guard = get_mutex().lock().unwrap();
    setup_test_terrain();
    let height1 = m2_get_height_at(0.0, 0.0);
    assert_eq!(height1, 100.0, "Wysokosc w punkcie 0,0 powinna byc 100.0 na podstawie funkcji sin/cos");

    let height2 = m2_get_height_at(100.0, 100.0);
    assert!(height2.is_finite(), "Wysokosc powinna byc liczba skonczona");
}

#[test]
fn test_is_walkable_bounds() {
    let _guard = get_mutex().lock().unwrap();
    setup_test_terrain();
    assert!(!m2_is_walkable(-1.0, 5000.0), "Punkt x < 0 powinien byc niemozliwy do wejscia");
    assert!(!m2_is_walkable(25601.0, 5000.0), "Punkt x > map_size powinien byc niemozliwy do wejscia");
    assert!(!m2_is_walkable(5000.0, -1.0), "Punkt y < 0 powinien byc niemozliwy do wejscia");
    assert!(!m2_is_walkable(5000.0, 25601.0), "Punkt y > map_size powinien byc niemozliwy do wejscia");
    assert!(m2_is_walkable(1000.0, 1000.0), "Zwykly punkt na mapie powinien byc mozliwy do wejscia");
}

#[test]
fn test_is_walkable_center_obstacle() {
    let _guard = get_mutex().lock().unwrap();
    setup_test_terrain();
    let center_x = 25600.0 / 2.0;
    let center_y = 25600.0 / 2.0;
    
    assert!(!m2_is_walkable(center_x, center_y), "Punkt w centrum mapy (przeszkoda) powinien byc niemozliwy do wejscia");
    assert!(m2_is_walkable(center_x + 600.0, center_y), "Punkt oddalony od centrum bardziej niz promien przeszkody powinien byc ok");
}

#[test]
fn test_stop_movement() {
    let _guard = get_mutex().lock().unwrap();
    setup_test_terrain();
    m2_move_to(2000.0, 2000.0);
    let (_, _, is_moving_before) = get_movement_state();
    assert!(is_moving_before, "Powinno poruszac sie przed zatrzymaniem");

    m2_stop_movement();

    let (_, _, is_moving_after) = get_movement_state();
    assert!(!is_moving_after, "Nie powinno poruszac sie po zatrzymaniu");
}
