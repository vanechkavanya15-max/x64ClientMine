use m2_client::area_loader::{AreaLoader, Position};

#[test]
fn test_integration_area_loader() {
    let instance = AreaLoader::new();
    assert!(instance.is_ready());
}

#[test]
fn test_load_standard_objects() {
    let mut loader = AreaLoader::new();
    let content = r#"
ScriptType	AreaData

areadatafile	1
objectcount	2

object000	10.0	20.0	30.0	12345	45.0	2.5	1	2	3
object001	-5.0	-10.0	0.0	54321	0.0	0.0
"#;
    
    assert!(loader.load_object_from_string(content));
    assert_eq!(loader.get_object_data_count(), 2);
    
    let obj0 = loader.get_object_data(0).unwrap();
    assert_eq!(obj0.position, Position { x: 10.0, y: 20.0, z: 30.0 });
    assert_eq!(obj0.crc, 12345);
    assert_eq!(obj0.roll, 45.0);
    assert_eq!(obj0.yaw, 0.0);
    assert_eq!(obj0.pitch, 0.0);
    assert_eq!(obj0.height_bias, 2.5);
    assert_eq!(obj0.portal_id[0], 1);
    assert_eq!(obj0.portal_id[1], 2);
    assert_eq!(obj0.portal_id[2], 3);
    assert_eq!(obj0.portal_id[3], 0);

    let obj1 = loader.get_object_data(1).unwrap();
    assert_eq!(obj1.position, Position { x: -5.0, y: -10.0, z: 0.0 });
    assert_eq!(obj1.crc, 54321);
    assert_eq!(obj1.roll, 0.0);
    assert_eq!(obj1.height_bias, 0.0);
    assert_eq!(obj1.portal_id[0], 0);
}

#[test]
fn test_load_advanced_rotations() {
    let mut loader = AreaLoader::new();
    let content = r#"
ScriptType	AreaData

areadatafile	1
objectcount	1

object000	10.0	20.0	30.0	12345	90.0#45.0#180.0
"#;

    assert!(loader.load_object_from_string(content));
    assert_eq!(loader.get_object_data_count(), 1);
    
    let obj0 = loader.get_object_data(0).unwrap();
    assert_eq!(obj0.yaw, 90.0);
    assert_eq!(obj0.pitch, 45.0);
    assert_eq!(obj0.roll, 180.0);
}

#[test]
fn test_load_ambience_objects() {
    let mut loader = AreaLoader::new();
    let content = r#"
ScriptType	AreaAmbienceData

areaambiencedatafile	1
objectcount	2

object000	10.0	20.0	30.0	12345	500	0.75
object001	-5.0	-10.0	0.0	54321	1000
"#;

    assert!(loader.load_ambience_from_string(content));
    assert_eq!(loader.get_object_data_count(), 2);
    
    let obj0 = loader.get_object_data(0).unwrap();
    assert_eq!(obj0.position, Position { x: 10.0, y: 20.0, z: 30.0 });
    assert_eq!(obj0.crc, 12345);
    assert_eq!(obj0.range, 500);
    assert_eq!(obj0.max_volume_area_percentage, 0.75);
    assert_eq!(obj0.yaw, 0.0);
    assert_eq!(obj0.pitch, 0.0);
    assert_eq!(obj0.roll, 0.0);
    assert_eq!(obj0.height_bias, 0.0);
    
    let obj1 = loader.get_object_data(1).unwrap();
    assert_eq!(obj1.crc, 54321);
    assert_eq!(obj1.range, 1000);
    assert_eq!(obj1.max_volume_area_percentage, 0.0);
}

#[test]
fn test_load_invalid_formats() {
    let mut loader = AreaLoader::new();
    
    // Missing areadatafile
    let content_no_areadatafile = r#"
objectcount	1
object000	10.0	20.0	30.0	12345	0.0
"#;
    assert!(!loader.load_object_from_string(content_no_areadatafile));
    
    // Missing objectcount
    let content_no_objectcount = r#"
areadatafile	1
object000	10.0	20.0	30.0	12345	0.0
"#;
    assert!(!loader.load_object_from_string(content_no_objectcount));
    
    // Invalid objectcount format
    let content_invalid_count = r#"
areadatafile	1
objectcount	abc
object000	10.0	20.0	30.0	12345	0.0
"#;
    assert!(!loader.load_object_from_string(content_invalid_count));
}

#[test]
fn test_build_object_instances_sorting() {
    let mut loader = AreaLoader::new();
    let content = r#"
areadatafile	1
objectcount	3

object000	0.0	0.0	0.0	300	0.0
object001	0.0	0.0	0.0	100	0.0
object002	0.0	0.0	0.0	200	0.0
"#;

    assert!(loader.load_object_from_string(content));
    assert_eq!(loader.get_object_data_count(), 3);
    
    // Before sorting
    assert_eq!(loader.get_object_data(0).unwrap().crc, 300);
    assert_eq!(loader.get_object_data(1).unwrap().crc, 100);
    assert_eq!(loader.get_object_data(2).unwrap().crc, 200);
    
    loader.build_object_instances();
    
    // After sorting
    assert_eq!(loader.get_object_data(0).unwrap().crc, 100);
    assert_eq!(loader.get_object_data(1).unwrap().crc, 200);
    assert_eq!(loader.get_object_data(2).unwrap().crc, 300);
}
