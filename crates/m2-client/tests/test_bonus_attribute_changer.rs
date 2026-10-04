use m2_client::item_attributes::{ItemAttribute, APPLY_NONE, ITEM_ATTRIBUTE_SLOT_MAX_NUM};
use m2_client::game_item_bonus_attribute_changer::{BonusAttributeChanger, AttributeProbability};

fn get_test_attributes() -> Vec<AttributeProbability> {
    vec![
        AttributeProbability { attr_type: 1, min_value: 500, max_value: 2000 },
        AttributeProbability { attr_type: 2, min_value: 10, max_value: 20 },
        AttributeProbability { attr_type: 3, min_value: 5, max_value: 10 },
        AttributeProbability { attr_type: 4, min_value: 10, max_value: 15 },
        AttributeProbability { attr_type: 5, min_value: 5, max_value: 10 },
        AttributeProbability { attr_type: 6, min_value: 5, max_value: 15 },
    ]
}

#[test]
fn test_add_normal_attribute() {
    let changer = BonusAttributeChanger::with_possible_attributes(get_test_attributes());
    let mut attrs = [ItemAttribute::default(); ITEM_ATTRIBUTE_SLOT_MAX_NUM];

    assert!(changer.add_normal_attribute(&mut attrs));
    assert_eq!(changer.get_normal_attribute_count(&attrs), 1);
    assert_ne!(attrs[0].attr_type, APPLY_NONE);

    assert!(changer.add_normal_attribute(&mut attrs));
    assert_eq!(changer.get_normal_attribute_count(&attrs), 2);
    assert_ne!(attrs[1].attr_type, APPLY_NONE);
    assert_ne!(attrs[0].attr_type, attrs[1].attr_type); // Unikalne bonusy
}

#[test]
fn test_change_normal_attributes() {
    let changer = BonusAttributeChanger::with_possible_attributes(get_test_attributes());
    let mut attrs = [ItemAttribute::default(); ITEM_ATTRIBUTE_SLOT_MAX_NUM];
    
    for _ in 0..5 {
        changer.add_normal_attribute(&mut attrs);
    }
    assert_eq!(changer.get_normal_attribute_count(&attrs), 5);
    
    assert!(changer.change_normal_attributes(&mut attrs));
    
    // Upewniamy się, że po zmianie wciąż mamy 5 bonusy
    assert_eq!(changer.get_normal_attribute_count(&attrs), 5);
    
    // Sprawdzamy unikalność po zmianie
    let mut types = std::collections::HashSet::new();
    for i in 0..5 {
        assert_ne!(attrs[i].attr_type, APPLY_NONE);
        assert!(types.insert(attrs[i].attr_type));
    }
}

#[test]
fn test_add_rare_attribute() {
    let changer = BonusAttributeChanger::with_possible_attributes(get_test_attributes());
    let mut attrs = [ItemAttribute::default(); ITEM_ATTRIBUTE_SLOT_MAX_NUM];

    assert!(changer.add_rare_attribute(&mut attrs));
    assert_ne!(attrs[5].attr_type, APPLY_NONE);

    assert!(changer.add_rare_attribute(&mut attrs));
    assert_ne!(attrs[6].attr_type, APPLY_NONE);
    assert_ne!(attrs[5].attr_type, attrs[6].attr_type);
}

#[test]
fn test_change_rare_attributes() {
    let changer = BonusAttributeChanger::with_possible_attributes(get_test_attributes());
    let mut attrs = [ItemAttribute::default(); ITEM_ATTRIBUTE_SLOT_MAX_NUM];

    changer.add_rare_attribute(&mut attrs);
    changer.add_rare_attribute(&mut attrs);
    
    assert!(changer.change_rare_attributes(&mut attrs));
    assert_ne!(attrs[5].attr_type, APPLY_NONE);
    assert_ne!(attrs[6].attr_type, APPLY_NONE);
    assert_ne!(attrs[5].attr_type, attrs[6].attr_type);
}
