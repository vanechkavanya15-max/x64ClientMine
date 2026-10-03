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

static FISHING_VALUE: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// Pobiera lokalny slot z paska szybkiego dostepu (type, pos).
#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_local_quickslot(slot_idx: u32) -> (u32, u32) {
    safe_ffi_call((0, 0), || {
        let res = crate::recv_quickslot_sync_v2::m2_quickslot_get_v2(slot_idx as u8);
        (res.slot_type as u32, res.slot_position)
    })
}

/// Wysyla zadanie przypisania nowego elementu do lokalnego slotu szybkiego wyboru.
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_request_add_local_quickslot(
    slot_idx: u32,
    slot_type: u32,
    slot_pos: u32,
) -> bool {
    safe_ffi_call(false, || {
        // Optymistyczna synchronizacja lokalnego rejestru quickslotow
        let add_pkt = crate::recv_quickslot_sync_v2::TPacketGCQuickSlotAdd {
            header: 0,
            length: 0,
            pos: slot_idx as u8,
            slot: crate::recv_quickslot_sync_v2::TQuickSlot {
                slot_type: slot_type as u8,
                slot_position: slot_pos,
            },
        };
        crate::recv_quickslot_sync_v2::process_quickslot_add(&add_pkt as *const _ as *const u8);

        // Wyslanie pakietu sieciowego do serwera
        crate::send_quickslot_quest_v2::SendQuickSlotAddPacket(
            slot_idx as u8,
            slot_type as u8,
            slot_pos as u8,
        )
    })
}

/// Wysyla zadanie aktywacji/uzycia wskazanego slotu szybkiego dostepu.
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_request_use_local_quickslot(slot_idx: u32) -> bool {
    safe_ffi_call(false, || {
        let slot = crate::recv_quickslot_sync_v2::m2_quickslot_get_v2(slot_idx as u8);
        match slot.slot_type {
            crate::recv_quickslot_sync_v2::SLOT_TYPE_INVENTORY => {
                let packet = TPacketCGItemUse::new(1, slot.slot_position as u16);
                enqueue_outgoing_packet(packet.to_bytes())
            }
            crate::recv_quickslot_sync_v2::SLOT_TYPE_SKILL => {
                crate::send_combat::SendUseSkillPacket(slot.slot_position, 0)
            }
            crate::recv_quickslot_sync_v2::SLOT_TYPE_EMOTION => {
                m2_py_player_act_emotion(slot.slot_position)
            }
            _ => false,
        }
    })
}

/// Uruchamia animacje emocji postaci gracza.
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_act_emotion(emotion_id: u32) -> bool {
    safe_ffi_call(false, || {
        let mut state = GLOBAL_GAME_STATE.write().unwrap();
        let main_vid = state.player.vid;
        if let Some(actor) = state.actors.get_mut(&main_vid) {
            actor.current_motion = emotion_id as i32;
        }
        if let Ok(mut mgr) = crate::character_manager_core::get_character_manager().write() {
            if let Some(main_vid) = mgr.main_actor_vid {
                if let Some(inst) = mgr.get_instance_mut(main_vid) {
                    inst.active_once_motion = Some(emotion_id);
                }
            }
        }
        true
    })
}

/// Sprawdza czy gracz moze rozpoczac lowienie ryb.
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_can_fishing() -> bool {
    safe_ffi_call(false, || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        if state.player.is_mounted {
            return false;
        }
        if let Ok(mgr) = crate::character_manager_core::get_character_manager().read() {
            if let Some(main_inst) = mgr.get_main_instance() {
                if main_inst.is_dead {
                    return false;
                }
            }
        }
        true
    })
}

/// Ustawia wartosc stanu lowienia ryb (gauge/wartosc lowienia).
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_set_fishing_value(val: u32) {
    FISHING_VALUE.store(val, std::sync::atomic::Ordering::SeqCst);
}

/// Pobiera aktualna wartosc stanu lowienia ryb.
#[no_mangle]
pub unsafe extern "C" fn m2_py_player_get_fishing_value() -> u32 {
    safe_ffi_call(0, || {
        FISHING_VALUE.load(std::sync::atomic::Ordering::SeqCst)
    })
}
