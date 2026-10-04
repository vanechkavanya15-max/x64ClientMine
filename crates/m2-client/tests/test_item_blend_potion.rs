use m2_client::game_item_blend_potion_calculator::{
    BlendPotionInstance, BlendPotionRecipe, GameItemBlendPotionCalculator,
};

#[test]
fn test_blend_recipe_manual_registration() {
    let mut calc = GameItemBlendPotionCalculator::new();
    assert!(calc.is_empty());

    let mut recipe = BlendPotionRecipe::new(50821, 54); // Roska ataku krytycznego
    recipe.apply_values = vec![8, 10, 12, 15, 20];
    recipe.apply_durations = vec![60, 120, 180, 300, 600];

    calc.add_recipe(recipe);

    assert_eq!(calc.len(), 1);
    let r = calc.get_recipe(50821).expect("Przepis 50821 musi istniec");
    assert_eq!(r.apply_type, 54);
    assert_eq!(r.value_range(), Some((8, 20)));
    assert_eq!(r.duration_range(), Some((60, 600)));

    // Poprawna instancja
    let inst_valid = BlendPotionInstance {
        item_vnum: 50821,
        apply_type: 54,
        apply_value: 15,
        duration_seconds: 300,
    };
    assert!(calc.validate_instance(&inst_valid));

    // Niepoprawna instancja (nieistniejaca wartosc)
    let inst_invalid = BlendPotionInstance {
        item_vnum: 50821,
        apply_type: 54,
        apply_value: 99,
        duration_seconds: 300,
    };
    assert!(!calc.validate_instance(&inst_invalid));
}

#[test]
fn test_blend_recipe_parse_text() {
    let raw_text = r#"
# Konfiguracja rosek
section
    item_vnum 50822
    apply_type 55
    apply_value 5 10 15
    apply_duration 120 300
end

section
    item_vnum 50823
    apply_type 16
    apply_value 10 20
    apply_duration 600
end
"#;

    let mut calc = GameItemBlendPotionCalculator::new();
    let loaded = calc.load_from_text(raw_text);
    assert_eq!(loaded, 2);
    assert_eq!(calc.len(), 2);

    let r1 = calc.get_recipe(50822).unwrap();
    assert_eq!(r1.apply_type, 55);
    assert_eq!(r1.apply_values, vec![5, 10, 15]);

    let r2 = calc.get_recipe(50823).unwrap();
    assert_eq!(r2.apply_type, 16);
    assert_eq!(r2.apply_durations, vec![600]);
}
