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

impl ItemUseLogic {
    pub fn new() -> Self {
        Self { last_use_time: 0 }
    }
    
    pub fn use_item(&mut self, item: &mut Item, player: &mut PlayerState, current_time: u64) -> Result<(), &'static str> {
        if current_time < self.last_use_time + 200 {
            return Err("Cooldown not ready");
        }
        
        if item.count == 0 {
            return Err("Item empty");
        }
        
        match item.item_type {
            ItemType::RedPotion(amount) => {
                player.hp = std::cmp::min(player.max_hp, player.hp.saturating_add(amount));
            }
            ItemType::BluePotion(amount) => {
                player.sp = std::cmp::min(player.max_sp, player.sp.saturating_add(amount));
            }
            ItemType::GreenPotion(_) => {
                // Apply green potion effect
            }
            ItemType::PurplePotion(_) => {
                // Apply purple potion effect
            }
            ItemType::TeleportScroll => {
                // Apply teleport scroll effect
            }
            ItemType::TreasureChest => {
                // Open chest
            }
        }
        
        item.count -= 1;
        self.last_use_time = current_time;
        
        Ok(())
    }
}
