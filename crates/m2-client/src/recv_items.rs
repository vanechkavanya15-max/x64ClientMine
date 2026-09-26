use m2_client_bridge as bridge;
use bridge::net::gc::item_set::TPacketGCItemSet;
use bridge::net::gc::item_del::TPacketGCItemDel;
use bridge::state::game_state::{GLOBAL_GAME_STATE, ItemData};

pub fn handle_item_set(data: &[u8]) -> bool {
    let packet = match TPacketGCItemSet::from_bytes(data) {
        Ok(p) => p,
        Err(_) => return false,
    };

    let mut attrs = [(0u8, 0i16); 7];
    for (i, attr) in packet.aAttr.iter().enumerate() {
        attrs[i] = (attr.bType, attr.sValue);
    }

    let item_data = ItemData {
        vnum: packet.vnum,
        count: packet.count,
        flags: packet.flags,
        anti_flags: packet.anti_flags,
        sockets: packet.alSockets,
        attrs,
    };

    if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
        state.set_item(packet.pos.window_type, packet.pos.cell, item_data);
    }

    true
}

pub fn handle_item_del(data: &[u8]) -> bool {
    let packet = match TPacketGCItemDel::from_bytes(data) {
        Ok(p) => p,
        Err(_) => return false,
    };

    if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
        state.del_item(packet.pos.window_type, packet.pos.cell);
    }

    true
}
