use std::collections::VecDeque;

pub const INVENTORY_MAX_NUM: usize = 90;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShopCoinType {
    Gold = 0,
    SecondaryCoin = 1,
}

#[derive(Debug, Clone, Copy)]
pub struct ShopCartItem {
    pub vnum: u32,
    pub price: u32,
    pub count: u8,
    pub item_size: u8,
    pub coin_type: ShopCoinType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShopTransactionError {
    NotEnoughGold,
    NotEnoughSecondaryCoin,
    InventoryFull,
    CartFull,
    InvalidItemSize,
}

pub struct ShopTransactionProcessor {
    pub cart: Vec<ShopCartItem>,
}

impl ShopTransactionProcessor {
    pub fn new() -> Self {
        Self {
            cart: Vec::with_capacity(40),
        }
    }

    pub fn clear(&mut self) {
        self.cart.clear();
    }

    pub fn add_item(&mut self, item: ShopCartItem) -> Result<(), ShopTransactionError> {
        if self.cart.len() >= 40 {
            return Err(ShopTransactionError::CartFull);
        }
        if item.item_size == 0 || item.item_size > 3 {
            return Err(ShopTransactionError::InvalidItemSize);
        }
        self.cart.push(item);
        Ok(())
    }

    pub fn build_shop_price(&self, coin_type: ShopCoinType) -> u64 {
        self.cart
            .iter()
            .filter(|i| i.coin_type == coin_type)
            .map(|i| i.price as u64)
            .sum()
    }

    pub fn can_buy_item(
        player_gold: u64,
        player_secondary_coin: u64,
        inventory_grid: &[Option<u32>; INVENTORY_MAX_NUM],
        item: &ShopCartItem,
    ) -> Result<(), ShopTransactionError> {
        match item.coin_type {
            ShopCoinType::Gold => {
                if player_gold < item.price as u64 {
                    return Err(ShopTransactionError::NotEnoughGold);
                }
            }
            ShopCoinType::SecondaryCoin => {
                if player_secondary_coin < item.price as u64 {
                    return Err(ShopTransactionError::NotEnoughSecondaryCoin);
                }
            }
        }

        if !Self::check_inventory_space(inventory_grid, item.item_size) {
            return Err(ShopTransactionError::InventoryFull);
        }

        Ok(())
    }

    pub fn can_buy_items(
        &self,
        player_gold: u64,
        player_secondary_coin: u64,
        inventory_grid: &[Option<u32>; INVENTORY_MAX_NUM],
    ) -> Result<(), ShopTransactionError> {
        let total_gold = self.build_shop_price(ShopCoinType::Gold);
        if player_gold < total_gold {
            return Err(ShopTransactionError::NotEnoughGold);
        }

        let total_secondary_coin = self.build_shop_price(ShopCoinType::SecondaryCoin);
        if player_secondary_coin < total_secondary_coin {
            return Err(ShopTransactionError::NotEnoughSecondaryCoin);
        }

        let mut temp_grid = *inventory_grid;

        for item in &self.cart {
            if !Self::place_item_in_grid(&mut temp_grid, item.item_size) {
                return Err(ShopTransactionError::InventoryFull);
            }
        }

        Ok(())
    }

    fn check_inventory_space(grid: &[Option<u32>; INVENTORY_MAX_NUM], item_size: u8) -> bool {
        let mut temp_grid = *grid;
        Self::place_item_in_grid(&mut temp_grid, item_size)
    }

    fn place_item_in_grid(grid: &mut [Option<u32>; INVENTORY_MAX_NUM], item_size: u8) -> bool {
        for row in 0..(INVENTORY_MAX_NUM / 5) {
            for col in 0..5 {
                let mut can_place = true;
                for h in 0..item_size as usize {
                    let r = row + h;
                    if r >= (INVENTORY_MAX_NUM / 5) {
                        can_place = false;
                        break;
                    }
                    let index = r * 5 + col;
                    if grid[index].is_some() {
                        can_place = false;
                        break;
                    }
                }
                
                if can_place {
                    for h in 0..item_size as usize {
                        let r = row + h;
                        let index = r * 5 + col;
                        grid[index] = Some(1); // placeholder for item taking space
                    }
                    return true;
                }
            }
        }
        false
    }
}
