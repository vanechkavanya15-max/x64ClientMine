use crate::ffi_guard::safe_ffi_call;

/// Faza A: Zwraca VID najblizszego moba (aktora).
/// Obecnie zwraca mockowana wartosc 0 (brak znalezienia), 
/// aby spelnialo definicje interfejsu C FFI.
///
/// Atrybuty `actor_type` okreslaja jakiego typu aktora szukamy,
/// a `max_distance` to maksymalna odleglosc wyszukiwania.
#[no_mangle]
pub extern "C" fn m2_find_nearest_actor(actor_type: u32, max_distance: f32) -> u32 {
    safe_ffi_call(0, || {
        // Implementacja w przyszlosci znajdzie i zwroci 
        // poprawne VID na podstawie aktora.
        // Faza A: mock (0 = brak aktora).
        let _ = actor_type;
        let _ = max_distance;
        0
    })
}
