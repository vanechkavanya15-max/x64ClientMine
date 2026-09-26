use m2_client::item_attributes::{ItemAttribute, ITEM_ATTRIBUTE_SLOT_MAX_NUM};
use m2_client::item_stacking_attrs_v2::{ItemV2, InventoryV2, ITEM_FLAG_STACKABLE, m2_item_can_stack};
use std::sync::{OnceLock, Mutex};

static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn get_lock() -> std::sync::MutexGuard<'static, ()> {
    TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

fn create_base_item() -> ItemV2 {
    ItemV2 {
        vnum: 12345,
        count: 10,
        flags: ITEM_FLAG_STACKABLE,
        sockets: [0, 0, 0],
        attributes: [ItemAttribute::default(); ITEM_ATTRIBUTE_SLOT_MAX_NUM],
    }
}

#[test]
fn test_can_stack_identical_items() {
    let _lock = get_lock();
    let a = create_base_item();
    let mut b = create_base_item();
    b.count = 20;

    assert!(a.can_stack_with(&b));
}

#[test]
fn test_cannot_stack_different_vnum() {
    let _lock = get_lock();
    let a = create_base_item();
    let mut b = create_base_item();
    b.vnum = 54321;

    assert!(!a.can_stack_with(&b));
}

#[test]
fn test_cannot_stack_not_stackable_flag() {
    let _lock = get_lock();
    let mut a = create_base_item();
    let mut b = create_base_item();
    a.flags = 0;

    assert!(!a.can_stack_with(&b));

    a.flags = ITEM_FLAG_STACKABLE;
    b.flags = 0;
    assert!(!a.can_stack_with(&b));
}

#[test]
fn test_cannot_stack_different_sockets() {
    let _lock = get_lock();
    let a = create_base_item();
    let mut b = create_base_item();
    b.sockets[0] = 1;

    assert!(!a.can_stack_with(&b));
}

#[test]
fn test_cannot_stack_different_attributes() {
    let _lock = get_lock();
    let a = create_base_item();
    let mut b = create_base_item();
    b.attributes[0] = ItemAttribute { attr_type: 1, value: 10 };

    assert!(!a.can_stack_with(&b));
}

#[test]
fn test_cannot_stack_exceeding_max_count() {
    let _lock = get_lock();
    let mut a = create_base_item();
    a.count = 150;
    let mut b = create_base_item();
    b.count = 60;

    assert!(!a.can_stack_with(&b)); // 150 + 60 = 210 > 200

    a.count = 100;
    b.count = 100;
    assert!(a.can_stack_with(&b)); // 100 + 100 = 200 <= 200
}

#[test]
fn test_ffi_m2_item_can_stack() {
    let _lock = get_lock();
    let a = create_base_item();
    let b = create_base_item();

    let result = m2_item_can_stack(&a as *const ItemV2, &b as *const ItemV2);
    assert!(result);

    let mut c = create_base_item();
    c.vnum = 999;
    let result_diff = m2_item_can_stack(&a as *const ItemV2, &c as *const ItemV2);
    assert!(!result_diff);

    let result_null = m2_item_can_stack(std::ptr::null(), &b as *const ItemV2);
    assert!(!result_null);
}

#[test]
fn test_split_stack() {
    let _lock = get_lock();
    let mut inv = InventoryV2::new(5);
    
    let mut item = create_base_item();
    item.count = 50;
    inv.slots[0] = Some(item);

    let res = inv.split_stack(0, 1, 20);
    assert!(res.is_ok());

    let src = inv.slots[0].unwrap();
    let dst = inv.slots[1].unwrap();

    assert_eq!(src.count, 30);
    assert_eq!(dst.count, 20);
    assert_eq!(src.vnum, dst.vnum);
}

#[test]
fn test_split_stack_invalid_cases() {
    let _lock = get_lock();
    let mut inv = InventoryV2::new(5);
    
    let mut item = create_base_item();
    item.count = 50;
    inv.slots[0] = Some(item);

    // split to same slot
    assert!(inv.split_stack(0, 0, 10).is_err());

    // split zero
    assert!(inv.split_stack(0, 1, 0).is_err());

    // split more than available
    assert!(inv.split_stack(0, 1, 60).is_err());

    // split full amount (not allowed in SplitStack usually, it's a move)
    assert!(inv.split_stack(0, 1, 50).is_err());

    // split to non-empty slot
    inv.slots[1] = Some(create_base_item());
    assert!(inv.split_stack(0, 1, 10).is_err());
}
