use m2_client::locale_string_dictionary_loader::LocaleDictionary;

#[test]
fn test_locale_string_parsing() {
    let content = r#"
"Original Text 1";
"Translated Text 1";

"Original Text 2";
"Translated Text 2";

"Original\nText\t3";
"Translated\nText\t3";
"#;

    let mut dict = LocaleDictionary::new();
    dict.load_locale_string(content);

    assert_eq!(dict.translate("Original Text 1"), "Translated Text 1");
    assert_eq!(dict.translate("Original Text 2"), "Translated Text 2");
    assert_eq!(dict.translate("Original\nText\t3"), "Translated\nText\t3");
}

#[test]
fn test_locale_game_parsing() {
    let content = r#"
# Comment line
// Another comment
-- Yet another comment

KEY_1	Value 1
KEY_2	Value 2 with spaces
KEY_3 Value 3 fallback
"#;

    let mut dict = LocaleDictionary::new();
    dict.load_locale_game(content);

    assert_eq!(dict.translate("KEY_1"), "Value 1");
    assert_eq!(dict.translate("KEY_2"), "Value 2 with spaces");
    assert_eq!(dict.translate("KEY_3"), "Value 3 fallback");
}

#[test]
fn test_translate_fallback() {
    let mut dict = LocaleDictionary::new();
    dict.load_locale_game("KNOWN_KEY\tKnown Value");

    assert_eq!(dict.translate("KNOWN_KEY"), "Known Value");
    assert_eq!(dict.translate("UNKNOWN_KEY"), "UNKNOWN_KEY");
}

#[test]
fn test_empty_lines_and_whitespaces() {
    let content = "   \n\n\t\nKEY_X\tValue X\n\n  \n";
    let mut dict = LocaleDictionary::new();
    dict.load_locale_game(content);

    assert_eq!(dict.translate("KEY_X"), "Value X");
}

#[test]
fn test_escaped_quotes_in_locale_string() {
    let content = r#"
"She said \"Hello\"";
"Powiedziała \"Cześć\"";
"#;

    let mut dict = LocaleDictionary::new();
    dict.load_locale_string(content);

    assert_eq!(dict.translate("She said \"Hello\""), "Powiedziała \"Cześć\"");
}
