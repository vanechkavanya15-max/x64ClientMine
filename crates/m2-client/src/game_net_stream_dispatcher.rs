use crate::game_net_receiver::gc_opcode;
use std::mem::size_of;

/// Dispatcher pakietow sieciowych fazy Game rozszerzony o obsluge dodatkowych systemow
pub fn dispatch_packet_extended(header: u16, data: &[u8]) -> bool {
    match header {
        gc_opcode::TARGET | 0x0A10 => {
            if data.len() >= size_of::<crate::recv_target_info_v2::TPacketGCTarget>() {
                let packet = unsafe { &*(data.as_ptr() as *const crate::recv_target_info_v2::TPacketGCTarget) };
                crate::recv_target_info_v2::parse_target_packet(packet);
                return true;
            }
        }
        gc_opcode::TARGET_UPDATE | 0x0A11 => {
            if data.len() >= 16 {
                let id = i32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
                let x = i32::from_le_bytes(data[8..12].try_into().unwrap_or_default());
                let y = i32::from_le_bytes(data[12..16].try_into().unwrap_or_default());
                crate::game_net_receiver::push_event(crate::game_net_receiver::GameEvent::TargetUpdate { id, x, y });
                return true;
            }
        }
        gc_opcode::TARGET_DELETE | 0x0A12 => {
            if data.len() >= 8 {
                let id = i32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
                crate::game_net_receiver::push_event(crate::game_net_receiver::GameEvent::TargetDelete { id });
                return true;
            }
        }
        gc_opcode::SHOP_SIGN | 0x0811 => {
            if data.len() >= 8 {
                let vid = u32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
                let sign = if data.len() > 8 {
                    let sign_data = &data[8..];
                    let sign_end = sign_data.iter().take(33).position(|&b| b == 0).unwrap_or(sign_data.len().min(33));
                    String::from_utf8_lossy(&sign_data[..sign_end]).to_string()
                } else {
                    String::new()
                };
                crate::game_net_receiver::push_event(crate::game_net_receiver::GameEvent::ShopSign { vid, sign });
                return true;
            }
        }
        gc_opcode::SAFEBOX_SIZE | 0x0833 => {
            if data.len() >= 5 {
                let size = data[4];
                crate::recv_safebox_packets_complete::m2_safebox_recv_open(size);
                crate::game_net_receiver::push_event(crate::game_net_receiver::GameEvent::SafeboxSize { size });
                return true;
            }
        }
        gc_opcode::SAFEBOX_MONEY_CHANGE | 0x0834 => {
            if data.len() >= 8 {
                let money = i32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
                crate::recv_safebox_packets_complete::m2_safebox_set_money(money);
                crate::game_net_receiver::push_event(crate::game_net_receiver::GameEvent::SafeboxMoneyChange { money });
                return true;
            }
        }
        gc_opcode::PARTY_INVITE | 0x0710 => {
            if data.len() >= 8 {
                let leader_pid = u32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
                let game_wnd = crate::py_bridge_net::get_phase_window(crate::py_bridge_net::PHASE_WINDOW_GAME);
                if game_wnd != 0 {
                    crate::py_event_dispatcher::call_python_object_method1_i32(game_wnd, "RecvPartyInviteQuestion", leader_pid as i32);
                }
                crate::game_net_receiver::push_event(crate::game_net_receiver::GameEvent::PartyInvite { leader_pid });
                return true;
            }
        }
        gc_opcode::PARTY_UPDATE | 0x0712 => {
            if data.len() >= size_of::<crate::recv_party_logic_v2::TPacketGCPartyUpdate>() {
                let packet = unsafe { &*(data.as_ptr() as *const crate::recv_party_logic_v2::TPacketGCPartyUpdate) };
                crate::recv_party_logic_v2::PARTY_MANAGER.handle_party_update(packet);
                let pid = packet.pid;
                let game_wnd = crate::py_bridge_net::get_phase_window(crate::py_bridge_net::PHASE_WINDOW_GAME);
                if game_wnd != 0 {
                    crate::py_event_dispatcher::call_python_object_method1_i32(game_wnd, "UpdatePartyMemberInfo", pid as i32);
                }
                crate::game_net_receiver::push_event(crate::game_net_receiver::GameEvent::PartyUpdate { pid });
                return true;
            }
        }
        gc_opcode::SKILL_COOLTIME_END | 0x021C => {
            if crate::recv_combat_skills_complete::process_skill_cooltime_end(data).is_ok() {
                if data.len() >= 5 {
                    crate::game_net_receiver::push_event(crate::game_net_receiver::GameEvent::SkillCooltimeEnd { skill: data[4] });
                }
                return true;
            }
        }
        gc_opcode::SCRIPT | gc_opcode::QUEST_INFO | 0x0910 | 0x0912 => {
            // Skrypty uzywaja m2_quest_recv_script_packet ktory obsluguje globalny stan dialogow
            return crate::recv_quest_dialog_v2::m2_quest_recv_script_packet(data.as_ptr(), data.len());
        }
        0x051D => { // gc_opcode::REFINE_INFORMATION (not in gc_opcode enum currently, using raw hex)
            return crate::recv_refine_blacksmith_v2::m2_refine_get_info(
                data.as_ptr(),
                data.len(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
        }
        0x0810 => { // SHOP
            return crate::recv_shop_logic_v2::m2_net_recv_shop(data.as_ptr(), data.len());
        }
        0x051C => { // EXCHANGE
            return crate::recv_exchange_logic_v2::m2_net_recv_exchange(data.as_ptr(), data.len());
        }
        0x0306 => { // WARP
            return crate::recv_warp_phase_v2::m2_net_recv_warp(data.as_ptr(), data.len());
        }
        0x0411..=0x0413 => { // FLY BULLET
            return crate::recv_fly_bullet_v2::m2_net_recv_fly(data.as_ptr(), data.len());
        }
        0x0741 => { // MESSENGER
            return crate::recv_messenger_friends_v2::m2_client_parse_messenger(data.as_ptr(), data.len());
        }
        _ => {}
    }
    
    false
}