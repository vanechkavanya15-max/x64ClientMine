use m2_client_pyapi::mod_item::ModItem;

#[test]
fn test_integration_mod_item() {
    let mut instance = ModItem::new();
    assert!(instance.is_ready());

    // Dodanie prototypu przedmiotu
    instance.add_item_prototype(11299, "Zbroja z Czarnej Stali".to_string(), 2, 50000);
    instance.add_item_prototype(19, "Miecz+9".to_string(), 1, 12000);

    // Wybor przedmiotu ktory nie istnieje
    assert_eq!(instance.select_item(99999), false);
    assert_eq!(instance.get_item_name(), None);
    assert_eq!(instance.get_item_type(), None);
    assert_eq!(instance.get_item_price(), None);

    // Wybor przedmiotu 11299
    assert_eq!(instance.select_item(11299), true);
    assert_eq!(instance.get_item_name(), Some("Zbroja z Czarnej Stali".to_string()));
    assert_eq!(instance.get_item_type(), Some(2));
    assert_eq!(instance.get_item_price(), Some(50000));

    // Wybor przedmiotu 19
    assert_eq!(instance.select_item(19), true);
    assert_eq!(instance.get_item_name(), Some("Miecz+9".to_string()));
    assert_eq!(instance.get_item_type(), Some(1));
    assert_eq!(instance.get_item_price(), Some(12000));
}