use m2_client::speedtree_forest_mgr_v2::{
    m2_forest_get_tree_count, SpeedTreeForestManager, SptDefinition, TreeInstance, get_forest_manager,
    SECTOR_SIZE_CM,
};

#[test]
fn test_spt_definition_parse_valid_srt() {
    let mut data = Vec::new();
    data.extend_from_slice(b"SRT ");
    data.extend_from_slice(&1u32.to_le_bytes());
    data.extend_from_slice(b"dummy_payload");

    let parsed = SptDefinition::parse(&data).expect("Should parse valid SRT");
    assert_eq!(parsed.magic, "SRT ");
    assert_eq!(parsed.version, 1);
    assert_eq!(parsed.raw_data, b"dummy_payload");
}

#[test]
fn test_spt_definition_parse_valid_spt() {
    let mut data = Vec::new();
    data.extend_from_slice(b"SPT ");
    data.extend_from_slice(&2u32.to_le_bytes());
    data.extend_from_slice(b"dummy_payload2");

    let parsed = SptDefinition::parse(&data).expect("Should parse valid SPT");
    assert_eq!(parsed.magic, "SPT ");
    assert_eq!(parsed.version, 2);
    assert_eq!(parsed.raw_data, b"dummy_payload2");
}

#[test]
fn test_spt_definition_parse_invalid_length() {
    let data = b"SRT";
    let res = SptDefinition::parse(data);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "File too short for SPT definition");
}

#[test]
fn test_spt_definition_parse_invalid_magic() {
    let mut data = Vec::new();
    data.extend_from_slice(b"BAD ");
    data.extend_from_slice(&1u32.to_le_bytes());
    data.extend_from_slice(b"dummy");

    let res = SptDefinition::parse(&data);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Invalid magic signature in SPT definition");
}

#[test]
fn test_forest_manager_add_instance_and_sectors() {
    let mut mgr = SpeedTreeForestManager::new();

    let inst1 = TreeInstance {
        tree_id: 1,
        pos_x: 0.0,
        pos_y: 0.0,
        pos_z: 100.0,
        scale: 1.0,
        rotation: 0.0,
    };

    let inst2 = TreeInstance {
        tree_id: 2,
        pos_x: SECTOR_SIZE_CM + 10.0, // Sector (1, 0)
        pos_y: 10.0,
        pos_z: 100.0,
        scale: 1.0,
        rotation: 0.0,
    };

    let inst3 = TreeInstance {
        tree_id: 3,
        pos_x: SECTOR_SIZE_CM * 2.0 + 10.0, // Sector (2, 3)
        pos_y: SECTOR_SIZE_CM * 3.0 + 10.0,
        pos_z: 100.0,
        scale: 1.0,
        rotation: 0.0,
    };

    mgr.add_instance(inst1.clone());
    mgr.add_instance(inst2.clone());
    mgr.add_instance(inst3.clone());

    assert_eq!(mgr.sectors.get(&(0, 0)).unwrap().len(), 1);
    assert_eq!(mgr.sectors.get(&(1, 0)).unwrap().len(), 1);
    assert_eq!(mgr.sectors.get(&(2, 3)).unwrap().len(), 1);
}

#[test]
fn test_forest_manager_negative_coordinates() {
    let mut mgr = SpeedTreeForestManager::new();

    let inst = TreeInstance {
        tree_id: 1,
        pos_x: -1000.0,
        pos_y: -2000.0,
        pos_z: 0.0,
        scale: 1.0,
        rotation: 0.0,
    };

    mgr.add_instance(inst);
    // Negative coordinates are clamped to 0.0 via max(0.0) -> sector (0,0)
    assert_eq!(mgr.sectors.get(&(0, 0)).unwrap().len(), 1);
}

#[test]
fn test_ffi_m2_forest_get_tree_count() {
    // Clear global state first
    get_forest_manager().write().unwrap().clear();

    {
        let mut mgr = get_forest_manager().write().unwrap();
        mgr.add_instance(TreeInstance {
            tree_id: 1,
            pos_x: 10.0,
            pos_y: 10.0,
            pos_z: 0.0,
            scale: 1.0,
            rotation: 0.0,
        });

        mgr.add_instance(TreeInstance {
            tree_id: 2,
            pos_x: 20.0,
            pos_y: 20.0,
            pos_z: 0.0,
            scale: 1.0,
            rotation: 0.0,
        });

        mgr.add_instance(TreeInstance {
            tree_id: 3,
            pos_x: SECTOR_SIZE_CM + 5.0,
            pos_y: 10.0,
            pos_z: 0.0,
            scale: 1.0,
            rotation: 0.0,
        });
    }

    assert_eq!(m2_forest_get_tree_count(0, 0), 2);
    assert_eq!(m2_forest_get_tree_count(1, 0), 1);
    assert_eq!(m2_forest_get_tree_count(99, 99), 0);
}

#[test]
fn test_manager_clear_and_default() {
    let mut mgr = SpeedTreeForestManager::default();
    
    mgr.register_definition(1, SptDefinition {
        magic: "SPT ".to_string(),
        version: 1,
        raw_data: vec![],
    });

    mgr.add_instance(TreeInstance {
        tree_id: 1,
        pos_x: 0.0,
        pos_y: 0.0,
        pos_z: 0.0,
        scale: 1.0,
        rotation: 0.0,
    });

    assert_eq!(mgr.definitions.len(), 1);
    assert_eq!(mgr.sectors.len(), 1);

    mgr.clear();
    
    assert_eq!(mgr.definitions.len(), 0);
    assert_eq!(mgr.sectors.len(), 0);
}
