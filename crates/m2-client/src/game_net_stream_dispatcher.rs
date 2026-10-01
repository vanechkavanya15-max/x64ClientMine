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