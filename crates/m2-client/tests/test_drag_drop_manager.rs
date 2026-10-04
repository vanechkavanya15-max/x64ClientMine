use m2_client::ui_drag_drop_global_manager::{
    DraggedType, DropTarget, SlotWindowType, UiDragDropGlobalManager,
};

#[test]
fn test_drag_drop_item_lifecycle() {
    let mut manager = UiDragDropGlobalManager::new();
    assert!(!manager.is_dragging());

    // 1. Rozpoczecie przeciagania przedmiotu
    manager.attach_item(10, 1, SlotWindowType::Inventory, 5);
    assert!(manager.is_dragging());

    // 2. Ruch kursorem
    manager.update_cursor(300, 450);
    let pos = manager.get_cursor_position();
    assert_eq!(pos.x, 300);
    assert_eq!(pos.y, 450);

    // 3. Upuszczenie na inny slot ekwipunku
    let dropped = manager.execute_drop(DropTarget::Slot {
        window: SlotWindowType::Inventory,
        slot_index: 12,
    });

    assert!(!manager.is_dragging());
    assert_eq!(
        dropped,
        Some(DraggedType::Item {
            vnum: 10,
            count: 1,
            source_slot: 5,
            source_window: SlotWindowType::Inventory,
        })
    );
}

#[test]
fn test_drag_drop_skill_lifecycle() {
    let mut manager = UiDragDropGlobalManager::new();

    manager.attach_skill(1, 20);
    assert!(manager.is_dragging());

    let dropped = manager.execute_drop(DropTarget::QuickSlot { index: 2 });
    assert!(!manager.is_dragging());
    assert_eq!(dropped, Some(DraggedType::Skill { skill_id: 1, skill_level: 20 }));
}
