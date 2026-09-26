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

/// Sprawdza czy typ okna jest prawidlowym oknem przechowujacym przedmioty (GameType.h EWindows)
pub fn is_valid_item_window(window: u8) -> bool {
    // 1: INVENTORY, 2: EQUIPMENT, 3: SAFEBOX, 4: MALL, 5: DRAGON_SOUL, 7: BELT
    matches!(window, 1 | 2 | 3 | 4 | 5 | 7)
}

/// Formatowanie i walidacja pakietu uzycia przedmiotu.
pub fn format_item_use_packet(pos: TItemPos) -> Result<Vec<u8>, ItemPacketError> {
    if !is_valid_item_window(pos.window_type) {
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
    if !is_valid_item_window(pos.window_type) {
        return Err(ItemPacketError::InvalidWindowType);
    }
    if !is_valid_item_window(change_pos.window_type) {
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
