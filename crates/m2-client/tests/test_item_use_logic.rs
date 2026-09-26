use m2_client::item_use_logic::{ItemUseLogic, Item, ItemType, PlayerState};

#[test]
fn test_use_red_potion() {
    let mut logic = ItemUseLogic::new();
    let mut item = Item { item_type: ItemType::RedPotion(50), count: 5 };
    let mut player = PlayerState { hp: 50, max_hp: 120, sp: 10, max_sp: 100 };
    
    let res = logic.use_item(&mut item, &mut player, 500);
    assert!(res.is_ok());
    assert_eq!(item.count, 4);
    assert_eq!(player.hp, 100);
}

#[test]
fn test_use_blue_potion_max() {
    let mut logic = ItemUseLogic::new();
    let mut item = Item { item_type: ItemType::BluePotion(50), count: 1 };
    let mut player = PlayerState { hp: 100, max_hp: 120, sp: 80, max_sp: 100 };
    
    let res = logic.use_item(&mut item, &mut player, 500);
    assert!(res.is_ok());
    assert_eq!(item.count, 0);
    assert_eq!(player.sp, 100);
}

#[test]
fn test_cooldown() {
    let mut logic = ItemUseLogic::new();
    let mut item = Item { item_type: ItemType::RedPotion(50), count: 5 };
    let mut player = PlayerState { hp: 50, max_hp: 120, sp: 10, max_sp: 100 };
    
    let res = logic.use_item(&mut item, &mut player, 500);
    assert!(res.is_ok());
    
    let res2 = logic.use_item(&mut item, &mut player, 650);
    assert!(res2.is_err());
    assert_eq!(res2.unwrap_err(), "Cooldown not ready");
    assert_eq!(item.count, 4);
}

#[test]
fn test_empty_item() {
    let mut logic = ItemUseLogic::new();
    let mut item = Item { item_type: ItemType::RedPotion(50), count: 0 };
    let mut player = PlayerState { hp: 50, max_hp: 120, sp: 10, max_sp: 100 };
    
    let res = logic.use_item(&mut item, &mut player, 500);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Item empty");
}
