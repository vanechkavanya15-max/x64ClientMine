use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShopItemInfo {
    pub slot: u32,
    pub vnum: u32,
    pub count: u32,
    pub price: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShopError {
    NotEnoughGold,
    InventoryFull,
    InvalidSlot,
}

pub struct ShopNpc {
    pub shop_items: Vec<ShopItemInfo>,
    pub item_sell_prices: HashMap<u32, u32>,
    pub item_flags: HashMap<u32, u32>,
    pub inventory: Vec<Option<u32>>,
}

impl ShopNpc {
    pub fn new() -> Self {
        Self {
            shop_items: Vec::new(),
            item_sell_prices: HashMap::new(),
            item_flags: HashMap::new(),
            inventory: Vec::new(),
        }
    }

    pub fn buy_item(&mut self, slot: u32, player_gold: u32, player_inv: &mut [Option<u32>]) -> Result<u32, ShopError> {
        let item = self.shop_items.iter().find(|i| i.slot == slot).ok_or(ShopError::InvalidSlot)?;
        
        if player_gold < item.price {
            return Err(ShopError::NotEnoughGold);
        }

        let empty_slot = player_inv.iter().position(|slot| slot.is_none()).ok_or(ShopError::InventoryFull)?;
        
        player_inv[empty_slot] = Some(item.vnum);
        
        Ok(player_gold - item.price)
    }

    pub fn sell_item(&self, inv_slot: usize, count: u32) -> i64 {
        if inv_slot >= self.inventory.len() {
            return 0;
        }

        let vnum = match self.inventory[inv_slot] {
            Some(v) => v,
            None => return 0,
        };

        let sell_price = self.item_sell_prices.get(&vnum).copied().unwrap_or(0);
        let flags = self.item_flags.get(&vnum).copied().unwrap_or(0);

        let item_flag_count_per_1gold = 1 << 3;
        
        let price = if (flags & item_flag_count_per_1gold) != 0 {
            if sell_price == 0 {
                0
            } else {
                count / sell_price
            }
        } else {
            sell_price.saturating_mul(count)
        };

        (price / 5) as i64
    }
}
