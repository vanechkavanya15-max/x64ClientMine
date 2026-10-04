use std::mem::size_of;

use crate::net_shop_private_packets::{HEADER_CG_SHOP, SHOP_SUB_BUY, TPacketCGShop};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemCategory {
    Weapon,
    Armor,
    Jewelry,
    Upgrade,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortCriteria {
    Price,
    Level,
    Count,
}

#[derive(Debug, Clone)]
pub struct CatalogItem {
    pub item_id: u32,
    pub shop_id: u32,
    pub name: String,
    pub category: ItemCategory,
    pub level: u32,
    pub price: u64,
    pub count: u32,
    pub pos: u8,
}

#[derive(Debug, Clone, Default)]
pub struct SearchFilters {
    pub category: Option<ItemCategory>,
    pub min_level: Option<u32>,
    pub max_level: Option<u32>,
    pub min_price: Option<u64>,
    pub max_price: Option<u64>,
    pub name_substring: Option<String>,
}

pub struct OfflineShopCatalogEngine {
    items: Vec<CatalogItem>,
}

impl OfflineShopCatalogEngine {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: CatalogItem) {
        self.items.push(item);
    }

    pub fn search(&self, filters: &SearchFilters, sort_criteria: Option<SortCriteria>, sort_order: Option<SortOrder>) -> Vec<CatalogItem> {
        let mut results: Vec<CatalogItem> = self.items.iter().filter(|item| {
            if let Some(category) = filters.category {
                if item.category != category {
                    return false;
                }
            }
            if let Some(min_level) = filters.min_level {
                if item.level < min_level {
                    return false;
                }
            }
            if let Some(max_level) = filters.max_level {
                if item.level > max_level {
                    return false;
                }
            }
            if let Some(min_price) = filters.min_price {
                if item.price < min_price {
                    return false;
                }
            }
            if let Some(max_price) = filters.max_price {
                if item.price > max_price {
                    return false;
                }
            }
            if let Some(name_substring) = &filters.name_substring {
                if !item.name.to_lowercase().contains(&name_substring.to_lowercase()) {
                    return false;
                }
            }
            true
        }).cloned().collect();

        if let Some(criteria) = sort_criteria {
            let order = sort_order.unwrap_or(SortOrder::Ascending);
            results.sort_by(|a, b| {
                let cmp = match criteria {
                    SortCriteria::Price => a.price.cmp(&b.price),
                    SortCriteria::Level => a.level.cmp(&b.level),
                    SortCriteria::Count => a.count.cmp(&b.count),
                };
                match order {
                    SortOrder::Ascending => cmp,
                    SortOrder::Descending => cmp.reverse(),
                }
            });
        }

        results
    }

    pub fn generate_purchase_packet(&self, shop_id: u32, item_id: u32) -> Option<Vec<u8>> {
        let item = self.items.iter().find(|i| i.shop_id == shop_id && i.item_id == item_id)?;
        
        let packet = TPacketCGShop {
            header: HEADER_CG_SHOP,
            length: (size_of::<TPacketCGShop>() + 2) as u16,
            subheader: SHOP_SUB_BUY,
        };

        let mut bytes = Vec::with_capacity(packet.length as usize);
        let packet_bytes: [u8; size_of::<TPacketCGShop>()] = unsafe { std::mem::transmute(packet) };
        bytes.extend_from_slice(&packet_bytes);
        
        // Count then Pos (same as m2_net_send_shop_buy_packet)
        let bCount = 1u8;
        bytes.push(bCount);
        bytes.push(item.pos);

        Some(bytes)
    }
}
