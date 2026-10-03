use m2_client_bridge::net::send_queue::enqueue_outgoing_packet;
use m2_client_bridge::ffi_guard::safe_ffi_call;

#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    RedPotion(u32), // Heal amount
    BluePotion(u32), // SP amount
    GreenPotion(u32), // Duration
    PurplePotion(u32), // Duration
    TeleportScroll,
    TreasureChest,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub item_type: ItemType,
    pub count: u32,
}

#[derive(Debug, Clone, Default)]
pub struct PlayerState {
    pub hp: u32,
    pub max_hp: u32,
    pub sp: u32,
    pub max_sp: u32,
}

pub struct ItemUseLogic {
    pub last_use_time: u64,
}

#[repr(C, packed)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

#[repr(C, packed)]
pub struct TPacketCGItemUse {
    pub header: u16,
    pub pos: TItemPos,
}

impl TPacketCGItemUse {
    pub const HEADER_CG_ITEM_USE: u16 = 0x0501;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    pub fn new(window_type: u8, cell: u16) -> Self {
        Self {
            header: Self::HEADER_CG_ITEM_USE,
            pos: TItemPos { window_type, cell },
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.push(self.pos.window_type);
        buf.extend_from_slice(&self.pos.cell.to_le_bytes());
        buf
    }
}

impl ItemUseLogic {
    pub fn new() -> Self {
        Self { last_use_time: 0 }
    }
    
    pub fn use_item(&mut self, item: &mut Item, window_type: u8, cell: u16, current_time: u64) -> Result<(), &'static str> {
        if current_time < self.last_use_time + 200 {
            return Err("Cooldown not ready");
        }
        
        if item.count == 0 {
            return Err("Item empty");
        }
        
        // Construct and send network packet
        let packet = TPacketCGItemUse::new(window_type, cell);
        enqueue_outgoing_packet(packet.to_bytes());
        
        self.last_use_time = current_time;
        
        Ok(())
    }
}
