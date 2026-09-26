use m2_client_bridge::net::cg::item_use::{TPacketCGItemUse, TItemPos};
use m2_client_bridge::net::cg::item_move::TPacketCGItemMove;
use m2_client_bridge::net::cg::item_pickup::TPacketCGItemPickUp;

/// Blad podczas budowania lub walidacji pakietow przedmiotu.
#[derive(Debug, PartialEq, Eq)]
pub enum ItemPacketError {
    InvalidWindowType,
    InvalidItemCount,
    InvalidTargetWindowType,
    InvalidEntityId,
}

/// Formatowanie i walidacja pakietu uzycia przedmiotu.
pub fn format_item_use_packet(pos: TItemPos) -> Result<Vec<u8>, ItemPacketError> {
    // Sprawdzenie poprawnosci struktury (window_type). 
    // W zaleznosci od implementacji, zwykle: INVENTORY = 1, EQUIPMENT = 2, SAFEBOX = 3, MALL = 4.
    // Ograniczamy do prawidlowych podstawowych typow okien dla uzycia przedmiotu (1 lub 2).
    if pos.window_type == 0 || pos.window_type > 4 {
        return Err(ItemPacketError::InvalidWindowType);
    }

    let packet = TPacketCGItemUse::new_with_pos(pos);
    Ok(packet.to_bytes())
}

/// Formatowanie i walidacja pakietu przeniesienia przedmiotu.
pub fn format_item_move_packet(
    pos: TItemPos,
    change_pos: TItemPos,
    count: u8,
) -> Result<Vec<u8>, ItemPacketError> {
    // Sprawdzenie poprawnosci struktury
    if pos.window_type == 0 || pos.window_type > 4 {
        return Err(ItemPacketError::InvalidWindowType);
    }
    if change_pos.window_type == 0 || change_pos.window_type > 4 {
        return Err(ItemPacketError::InvalidTargetWindowType);
    }
    if count == 0 {
        return Err(ItemPacketError::InvalidItemCount);
    }

    let packet = TPacketCGItemMove::new(
        pos.window_type,
        pos.cell,
        change_pos.window_type,
        change_pos.cell,
        count,
    );
    Ok(packet.to_bytes())
}

/// Formatowanie i walidacja pakietu podniesienia przedmiotu z ziemi.
pub fn format_item_pickup_packet(vid: u32) -> Result<Vec<u8>, ItemPacketError> {
    // Sprawdzenie poprawnosci struktury: VID nie moze byc 0 (z reguly VID=0 to brak bytu).
    if vid == 0 {
        return Err(ItemPacketError::InvalidEntityId);
    }

    let packet = TPacketCGItemPickUp::new(vid);
    Ok(packet.to_bytes())
}
