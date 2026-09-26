use m2_client_bridge::ffi_guard::safe_ffi_call;
use m2_client_bridge::state::game_state::GLOBAL_GAME_STATE;
use std::sync::atomic::{AtomicU32, Ordering};

static COMBO_INDEX: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn m2_battle_can_attack(attacker_vid: u32, victim_vid: u32) -> bool {
    safe_ffi_call(false, || {
        if attacker_vid == 0 || victim_vid == 0 || attacker_vid == victim_vid {
            return false;
        }

        let state = match GLOBAL_GAME_STATE.read() {
            Ok(s) => s,
            Err(_) => return false,
        };

        // Znajdzmy atakujacego - to moze byc gracz lub aktor
        let (attacker_x, attacker_y, attacker_hp) = if state.get_player_vid() == attacker_vid {
            (state.player.x, state.player.y, state.get_hp())
        } else if let Some(actor) = state.get_actor(attacker_vid) {
            (actor.x, actor.y, actor.hp)
        } else {
            return false;
        };

        // Znajdzmy ofiare
        let (victim_x, victim_y, victim_hp) = if state.get_player_vid() == victim_vid {
            (state.player.x, state.player.y, state.get_hp())
        } else if let Some(actor) = state.get_actor(victim_vid) {
            (actor.x, actor.y, actor.hp)
        } else {
            return false;
        };

        // Obaj musza zyc
        if attacker_hp <= 0 || victim_hp <= 0 {
            return false;
        }

        // Dystans nie moze byc absurdalnie duzy (np. na cala mape) - ograniczmy do typowego max dystansu np 10000.0,
        // jednakze zgodnie z C++ dystans normalnego ataku to ok 150.0 - 500.0, tutaj sprawdzimy 
        // czy w ogole zyjemy. Prawdziwa kalkulacja i dystanse robia inne funkcje, tu ogolne "can_attack".
        let dx = attacker_x - victim_x;
        let dy = attacker_y - victim_y;
        let dist_sq = dx * dx + dy * dy;
        
        // Bazowy zasieg ataku + jakis margines
        if dist_sq > 250000.0 { // 500.0 * 500.0
            return false;
        }

        true
    })
}

#[no_mangle]
pub extern "C" fn m2_battle_combo_attack(_dir_rot: f32) -> bool {
    safe_ffi_call(false, || {
        let state = match GLOBAL_GAME_STATE.read() {
            Ok(s) => s,
            Err(_) => return false,
        };

        let player_hp = state.get_hp();
        if player_hp <= 0 {
            return false;
        }

        // Gracz ma hp i jest gotowy na combo - zwiekszmy index
        let current = COMBO_INDEX.load(Ordering::Relaxed);
        
        // Zaleznie czy wsiadany
        let max_combo = if state.player.is_mounted { 5 } else { 4 };
        
        if current < max_combo {
            COMBO_INDEX.store(current + 1, Ordering::Relaxed);
            true
        } else {
            // Koniec combo, nie pozwala dalej az zresetuje
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_battle_get_combo_index() -> u32 {
    safe_ffi_call(0, || {
        COMBO_INDEX.load(Ordering::Relaxed)
    })
}

#[no_mangle]
pub extern "C" fn m2_battle_reset_combo() {
    safe_ffi_call((), || {
        COMBO_INDEX.store(0, Ordering::Relaxed);
    });
}
