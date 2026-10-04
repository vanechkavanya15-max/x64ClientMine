use crate::game_net_stream_dispatcher_complete::gc;

pub fn dispatch_packet_wire_all(header: u16, data: &[u8]) -> bool {
    match header {
        gc::GUILD | 0x0730 => {
            let _ = crate::recv_guild_packets_complete::parse_guild_packet(data);
            return true;
        }
        gc::SHOP => {
            return crate::recv_shop_logic_v2::m2_net_recv_shop(data.as_ptr(), data.len());
        }
        gc::EXCHANGE => {
            return crate::recv_exchange_logic_v2::m2_net_recv_exchange(data.as_ptr(), data.len());
        }
        gc::SAFEBOX_SET | gc::SAFEBOX_DEL | gc::SAFEBOX_WRONG_PASSWORD | gc::SAFEBOX_SIZE | gc::SAFEBOX_MONEY_CHANGE | 0x0830 | 0x0831 | 0x0832 | 0x0833 | 0x0834 => {
            let _ = crate::recv_safebox_packets_complete::parse_safebox_packet(data);
            return true;
        }
        _ => {}
    }
    
    false
}
