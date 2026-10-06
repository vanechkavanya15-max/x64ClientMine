use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCCharacterDelete {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
}

pub fn parse_character_delete(data: &[u8]) -> Result<(), &'static str> {
    if data.len() < size_of::<TPacketGCCharacterDelete>() {
        return Err("Buffer too short");
    }

    let packet = unsafe { &*(data.as_ptr() as *const TPacketGCCharacterDelete) };
    let vid = packet.dwVID;

    #[cfg(not(test))]
    {
        // 1. Remove from NetworkActorMgr
        if let Ok(mut mgr) = crate::actor_spawn_v2::get_network_actor_mgr().write() {
            mgr.remove_actor(vid);
        }

        // 2. Remove picking actor
        crate::actor_picking_v2::remove_picking_actor_v2(vid);

        // 3. Clear TextTail
        if let Ok(mut state) = crate::py_bridge_texttail_runtime_complete::get_text_tail_state().lock() {
            state.character_text_tail_map.remove(&vid);
            if let Some(pos) = state.character_text_tail_list.iter().position(|&x| x == vid) {
                state.character_text_tail_list.remove(pos);
            }
        }

        // 4. Reset target if it was the selected target
        if let Ok(state) = crate::py_bridge_player_target::get_player_target_state().lock() {
            if state.target_vid == vid {
                unsafe { crate::py_bridge_player_target::m2_py_player_clear_target_vid(); }
            }
        }

        // 5. Hide private shop in UI
        let game_wnd = crate::py_bridge_net::get_phase_window(crate::py_bridge_net::PHASE_WINDOW_GAME);
        if game_wnd != 0 {
            crate::py_event_dispatcher::call_python_object_method1_i32(
                game_wnd, 
                "BINARY_PrivateShop_Disappear", 
                vid as i32
            );
        }
    }

    #[cfg(test)]
    {
        // Mocks called in test environment
        if let Ok(mut mgr) = super::crate_mock::actor_spawn_v2::get_network_actor_mgr().write() {
            mgr.remove_actor(vid);
        }
        super::crate_mock::actor_picking_v2::remove_picking_actor_v2(vid);
        if let Ok(mut state) = super::crate_mock::py_bridge_texttail_runtime_complete::get_text_tail_state().lock() {
            state.character_text_tail_map.remove(&vid);
            if let Some(pos) = state.character_text_tail_list.iter().position(|&x| x == vid) {
                state.character_text_tail_list.remove(pos);
            }
        }
        if let Ok(state) = super::crate_mock::py_bridge_player_target::get_player_target_state().lock() {
            if state.target_vid == vid {
                unsafe { super::crate_mock::py_bridge_player_target::m2_py_player_clear_target_vid(); }
            }
        }
        let game_wnd = super::crate_mock::py_bridge_net::get_phase_window(super::crate_mock::py_bridge_net::PHASE_WINDOW_GAME);
        if game_wnd != 0 {
            super::crate_mock::py_event_dispatcher::call_python_object_method1_i32(
                game_wnd, 
                "BINARY_PrivateShop_Disappear", 
                vid as i32
            );
        }
    }

    Ok(())
}
