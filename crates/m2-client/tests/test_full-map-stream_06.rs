
use m2_client::property_manager_parser::{parse_property_file, PropertyType};

#[test]
fn test_parse_valid_building_prb() {
    let content = r#"YPRT
123456789
propertyname		"house_01"
propertytype		"Building"
isattributedata		"1"
buildingfile		"d:/ymir work/zone/b/obj/house_01.gr2"
"#;
    let prop = parse_property_file(content).expect("Failed to parse valid .prb file");
    assert_eq!(prop.id, 123456789);
    assert_eq!(prop.name, "house_01");
    assert_eq!(prop.property_type, PropertyType::Building);
    assert_eq!(prop.is_attribute_data, Some(true));
    assert_eq!(prop.building_file, Some("d:/ymir work/zone/b/obj/house_01.gr2".to_string()));
}

#[test]
fn test_parse_valid_decoration_prd() {
    let content = r#"YPRT
987654321
propertyname		"statue_01"
propertytype		"Decoration"
buildingfile		"d:/ymir work/zone/d/obj/statue_01.gr2"
"#;
    let prop = parse_property_file(content).expect("Failed to parse valid .prd file");
    assert_eq!(prop.id, 987654321);
    assert_eq!(prop.name, "statue_01");
    assert_eq!(prop.property_type, PropertyType::Decoration);
    assert_eq!(prop.building_file, Some("d:/ymir work/zone/d/obj/statue_01.gr2".to_string()));
    assert_eq!(prop.is_attribute_data, None);
}

#[test]
fn test_parse_missing_yprt_header() {
    let content = r#"NOT_YPRT
123456789
propertyname		"house_01"
propertytype		"Building"
"#;
    let prop = parse_property_file(content);
    assert!(prop.is_none());
}

#[test]
fn test_parse_invalid_id() {
    let content = r#"YPRT
not_a_number
propertyname		"house_01"
propertytype		"Building"
"#;
    let prop = parse_property_file(content);
    assert!(prop.is_none());
}

#[test]
fn test_parse_unknown_property_type_and_extra_fields() {
    let content = r#"YPRT
5555
propertyname		"weird_obj"
propertytype		"CustomType"
unknown_field		"12.34"
"#;
    let prop = parse_property_file(content).expect("Failed to parse file with unknown types");
    assert_eq!(prop.id, 5555);
    assert_eq!(prop.name, "weird_obj");
    assert_eq!(prop.property_type, PropertyType::Unknown("CustomType".to_string()));
    assert_eq!(prop.extra_fields.get("unknown_field"), Some(&"12.34".to_string()));
}
