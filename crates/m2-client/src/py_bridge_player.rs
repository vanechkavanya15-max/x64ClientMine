// Modul py_bridge_player.rs (Faza C)
// W pelni zastapiony kod PythonPlayerModule.cpp, PythonPlayer.cpp w Pure Rust.
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use m2_client_bridge::ffi_guard::safe_ffi_call;
use m2_client_bridge::state::game_state::GLOBAL_GAME_STATE;
use m2_client_bridge::net::send_queue::enqueue_outgoing_packet;
use m2_client_bridge::net::cg::item_use::TPacketCGItemUse;
use m2_client_bridge::net::cg::item_pickup::TPacketCGItemPickUp;

/// Pobiera wartosc wskazanego punktu (np. POINT_HP, POINT_MP) ze stanu gry.
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_status(point_type: u32) -> i64 {
    safe_ffi_call(0, || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        state.get_point(point_type as u8)
    })
}

/// Ustawia wartosc wskazanego punktu (np. POINT_HP, POINT_MP) w stanie gry.
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_set_status(point_type: u32, val: i64) -> bool {
    safe_ffi_call(false, || {
        let mut state = GLOBAL_GAME_STATE.write().unwrap();
        state.set_point(point_type as u8, val);
        true
    })
}

/// Zwraca identyfikator VNUM przedmiotu znajdujacego sie w danym slocie (komorce).
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_item_index(window: u8, slot: u16) -> u32 {
    safe_ffi_call(0, || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        state.get_item_vnum(window, slot)
    })
}

/// Zwraca ilosc przedmiotu znajdujacego sie w danym slocie (komorce).
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_item_count(window: u8, slot: u16) -> u32 {
    safe_ffi_call(0, || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        state.get_item_count(window, slot)
    })
}

/// Zwraca zsumowana ilosc przedmiotow we wszystkich oknach na podstawie VNUM.
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_item_count_by_vnum(vnum: u32) -> u32 {
    safe_ffi_call(0, || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        state.player.inventory.values()
            .filter(|item| item.vnum == vnum)
            .map(|item| item.count as u32)
            .sum()
    })
}

/// Wysyla pakiet uzycia (klikniecia) przedmiotu.
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_send_click_item(window: u8, slot: u16) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGItemUse::new(window, slot);
        let bytes = packet.to_bytes();
        enqueue_outgoing_packet(bytes)
    })
}

/// Wyszukuje najblizszego aktora (lub przedmiotu, jesli potraktowany jako aktor typu 0 lub inny)
/// w promieniu 500.0 jednostek i wysyla zadanie jego podniesienia.
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_pick_close_item() -> bool {
    safe_ffi_call(false, || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        // Wyszukujemy najblizszego aktora o dowolnym typie (actor_type = 0) w promieniu 500.0.
        // W kliencie Metin2 przedmioty (AOR_ITEM) sa tez dodawane jako instance jesli leza na ziemi.
        let nearest_vid = state.find_nearest_actor(0, 500.0);
        let packet = TPacketCGItemPickUp::new(nearest_vid);
        let bytes = packet.to_bytes();
        enqueue_outgoing_packet(bytes)
    })
}

/// Wypelnia podane wskazniki aktualna pozycja (x, y, z) glownego bohatera.
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_main_character_position(
    out_x: *mut f32,
    out_y: *mut f32,
    out_z: *mut f32,
) -> bool {
    safe_ffi_call(false, || {
        if out_x.is_null() || out_y.is_null() || out_z.is_null() {
            return false;
        }

        let state = GLOBAL_GAME_STATE.read().unwrap();
        let (x, y, z, _rot) = state.get_player_pos();
        *out_x = x;
        *out_y = y;
        *out_z = z;
        true
    })
}
