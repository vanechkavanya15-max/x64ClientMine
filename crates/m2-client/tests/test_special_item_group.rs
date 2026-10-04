use m2_client::game_special_item_group_reader::{SpecialItemGroupReader, SpecialItemDrop};

#[test]
fn test_parse_moonlight_chest() {
    let content = r#"
        Group 50011
        {
            Type drop
            1 50012 1 100
            2 27001 5 200
            3 27002 5 200
        }
    "#;

    let mut reader = SpecialItemGroupReader::new();
    let result = reader.parse_content(content);
    assert!(result.is_ok());

    let group = reader.get_group(50011).expect("Group 50011 should exist");
    assert_eq!(group.vnum, 50011);
    assert_eq!(group.group_type, "drop");
    assert_eq!(group.items.len(), 3);

    assert_eq!(group.items[0], SpecialItemDrop { vnum: 50012, count: 1, prob: 100 });
    assert_eq!(group.items[1], SpecialItemDrop { vnum: 27001, count: 5, prob: 200 });
    assert_eq!(group.items[2], SpecialItemDrop { vnum: 27002, count: 5, prob: 200 });
}

#[test]
fn test_parse_vnum_format() {
    let content = r#"
        Group Vnum 50011
        {
            Type drop
            Vnum 50012 1 100
            Vnum 27001 5 200
            Vnum 27002 5 200
        }
    "#;

    let mut reader = SpecialItemGroupReader::new();
    let result = reader.parse_content(content);
    assert!(result.is_ok());

    let group = reader.get_group(50011).expect("Group 50011 should exist");
    assert_eq!(group.vnum, 50011);
    assert_eq!(group.group_type, "drop");
    assert_eq!(group.items.len(), 3);

    assert_eq!(group.items[0], SpecialItemDrop { vnum: 50012, count: 1, prob: 100 });
    assert_eq!(group.items[1], SpecialItemDrop { vnum: 27001, count: 5, prob: 200 });
    assert_eq!(group.items[2], SpecialItemDrop { vnum: 27002, count: 5, prob: 200 });
}

#[test]
fn test_invalid_group_format() {
    let mut reader = SpecialItemGroupReader::new();
    
    // Missing brace
    let content1 = "Group 1234\nType drop\n1 12345 1 100\n}";
    assert!(reader.parse_content(content1).is_err());

    let mut reader2 = SpecialItemGroupReader::new();
    // Missing end brace
    let content2 = "Group 1234\n{\nType drop\n1 12345 1 100\n";
    assert!(reader2.parse_content(content2).is_err());
}

#[test]
fn test_missing_file_content() {
    let mut reader = SpecialItemGroupReader::new();
    let content = "";
    assert!(reader.parse_content(content).is_ok());
    assert_eq!(reader.get_all_groups().len(), 0);
}

#[test]
fn test_get_group() {
    let mut reader = SpecialItemGroupReader::new();
    let content = r#"
        // Comment
        # Another comment
        Group 1000
        {
            Type drop
            1 50012 1 100
        }
    "#;
    assert!(reader.parse_content(content).is_ok());
    assert!(reader.get_group(1000).is_some());
    assert!(reader.get_group(9999).is_none());
}

#[test]
fn test_is_valid_chest() {
    let mut reader = SpecialItemGroupReader::new();
    let content = r#"
        Group 1000
        {
            Type drop
            1 50012 1 100
        }
        Group 2000
        {
            Type drop
        }
    "#;
    
    assert!(reader.parse_content(content).is_ok());
    
    // Valid chest with items
    assert!(reader.is_valid_chest(1000));
    
    // Invalid chest (no items)
    assert!(!reader.is_valid_chest(2000));
    
    // Invalid chest (doesn't exist)
    assert!(!reader.is_valid_chest(3000));
}
