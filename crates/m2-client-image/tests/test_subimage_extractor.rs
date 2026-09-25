use m2_client_image::subimage_extractor::SubimageExtractor;

#[test]
fn test_integration_subimage_extractor() {
    let instance = SubimageExtractor::new();
    assert!(instance.is_ready());
}

#[test]
fn test_parse_valid_subimage() {
    let content = r#"
title "test"
version 1.0
image "d:/ymir work/ui/public/public_01.dds"
left 10
top 20
right 30
bottom 40
"#;
    let extractor = SubimageExtractor::new();
    let result = extractor.parse(content);
    assert!(result.is_ok());
    let desc = result.unwrap();
    assert_eq!(desc.image_path, "d:/ymir work/ui/public/public_01.dds");
    assert_eq!(desc.left, 10);
    assert_eq!(desc.top, 20);
    assert_eq!(desc.right, 30);
    assert_eq!(desc.bottom, 40);
}

#[test]
fn test_parse_missing_image() {
    let content = r#"
left 10
top 20
right 30
bottom 40
"#;
    let extractor = SubimageExtractor::new();
    let result = extractor.parse(content);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Brak sciezki do obrazu (image)");
}

#[test]
fn test_parse_missing_coordinates() {
    let content = r#"
image "test.dds"
left 10
top 20
"#;
    let extractor = SubimageExtractor::new();
    let result = extractor.parse(content);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Brak wymaganych wspolrzednych wycinka (left, top, right, bottom)");
}

#[test]
fn test_parse_invalid_coordinate() {
    let content = r#"
image "test.dds"
left abc
top 20
right 30
bottom 40
"#;
    let extractor = SubimageExtractor::new();
    let result = extractor.parse(content);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Nieprawidlowa wartosc dla left");
}
