use m2_client_terrain::map_setting::MapSetting;

#[test]
fn test_integration_map_setting() {
    let instance = MapSetting::new();
    assert!(!instance.is_ready());
}

#[test]
fn test_parse_valid_content() {
    let content = r#"
        # Skonfigurowana mapa z komentarzami
        ScriptType    MapSetting
        
        MapSize        2 2
        BasePosition   0 0
        HeightScale    1.5
        
        // Komentarz z ukosnikami
        TextureSet     "textureset/snow.txt"
        Environment    "environment/snow.msenv"
    "#;

    let result = MapSetting::parse(content);
    assert!(result.is_ok(), "Parsowanie prawidlowego pliku powinno sie udac");
    
    let setting = result.unwrap();
    assert!(setting.is_ready());
    assert_eq!(setting.map_size_x, 2);
    assert_eq!(setting.map_size_y, 2);
    assert_eq!(setting.height_scale, 1.5);
    assert_eq!(setting.texture_set, "textureset/snow.txt");
    assert_eq!(setting.environment, "environment/snow.msenv");
}

#[test]
fn test_parse_missing_required_fields() {
    let content = r#"
        MapSize 2 2
        TextureSet "textureset/snow.txt"
    "#;
    
    let result = MapSetting::parse(content);
    assert!(result.is_err(), "Powinien zwrocic blad jesli brakuje height_scale i environment");
    assert_eq!(result.unwrap_err(), "Brak wymaganych pol w pliku konfiguracyjnym");
}

#[test]
fn test_parse_invalid_map_size() {
    let content = r#"
        MapSize 2
        HeightScale 1.5
        TextureSet "textureset/snow.txt"
        Environment "environment/snow.msenv"
    "#;
    
    let result = MapSetting::parse(content);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Nieprawidlowa liczba argumentow dla MapSize");
}

#[test]
fn test_parse_invalid_height_scale() {
    let content = r#"
        MapSize 2 2
        HeightScale abc
        TextureSet "textureset/snow.txt"
        Environment "environment/snow.msenv"
    "#;
    
    let result = MapSetting::parse(content);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Blad parsowania HeightScale");
}

#[test]
fn test_parse_quotes_handling() {
    let content = r#"
        MapSize 4 4
        HeightScale 2.0
        TextureSet "  textureset/custom.txt  "
        Environment environment/plain.msenv
    "#;

    let result = MapSetting::parse(content);
    assert!(result.is_ok());
    
    let setting = result.unwrap();
    assert_eq!(setting.texture_set, "textureset/custom.txt");
    assert_eq!(setting.environment, "environment/plain.msenv");
}
