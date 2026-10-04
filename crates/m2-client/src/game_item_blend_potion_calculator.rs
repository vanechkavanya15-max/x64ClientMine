//! Game Item Blend Potion Calculator Module
//!
//! Ladowanie i parsowanie pliku konfiguracyjnego `blend.txt` oraz kalkulacja
//! losowych lub odczytanych atrybutow mikstur ziolowych (tzw. "roski" - blend potions).

use std::collections::HashMap;

/// Konfiguracja pojedynczego przepisu na miksture roski z blend.txt.
#[derive(Debug, Clone, PartialEq)]
pub struct BlendPotionRecipe {
    pub item_vnum: u32,
    pub apply_type: u32,
    pub apply_values: Vec<i32>,
    pub apply_durations: Vec<u32>, // w sekundach
}

impl BlendPotionRecipe {
    pub fn new(item_vnum: u32, apply_type: u32) -> Self {
        Self {
            item_vnum,
            apply_type,
            apply_values: Vec::new(),
            apply_durations: Vec::new(),
        }
    }

    /// Pobiera minimalna i maksymalna mozliwa wartosc atrybutu.
    pub fn value_range(&self) -> Option<(i32, i32)> {
        if self.apply_values.is_empty() {
            None
        } else {
            let min = *self.apply_values.iter().min().unwrap();
            let max = *self.apply_values.iter().max().unwrap();
            Some((min, max))
        }
    }

    /// Pobiera minimalny i maksymalny czas trwania w sekundach.
    pub fn duration_range(&self) -> Option<(u32, u32)> {
        if self.apply_durations.is_empty() {
            None
        } else {
            let min = *self.apply_durations.iter().min().unwrap();
            let max = *self.apply_durations.iter().max().unwrap();
            Some((min, max))
        }
    }
}

/// Wynik uzycia / wytworzenia konkretnej butelki roski z wlasciwosciami.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlendPotionInstance {
    pub item_vnum: u32,
    pub apply_type: u32,
    pub apply_value: i32,
    pub duration_seconds: u32,
}

/// Menedzer i kalkulator systemu mikstur roskowych.
#[derive(Debug, Default, Clone)]
pub struct GameItemBlendPotionCalculator {
    recipes: HashMap<u32, BlendPotionRecipe>,
}

impl GameItemBlendPotionCalculator {
    pub fn new() -> Self {
        Self {
            recipes: HashMap::new(),
        }
    }

    /// Rejestruje przepis w pamieci.
    pub fn add_recipe(&mut self, recipe: BlendPotionRecipe) {
        self.recipes.insert(recipe.item_vnum, recipe);
    }

    /// Pobiera przepis dla danego item_vnum.
    pub fn get_recipe(&self, item_vnum: u32) -> Option<&BlendPotionRecipe> {
        self.recipes.get(&item_vnum)
    }

    /// Liczba zarejestrowanych przepisow.
    pub fn len(&self) -> usize {
        self.recipes.len()
    }

    /// Czy rejestr jest pusty.
    pub fn is_empty(&self) -> bool {
        self.recipes.is_empty()
    }

    /// Parsuje zawartosc tekstowa pliku `blend.txt`.
    /// Format linii:
    /// section
    ///   item_vnum 50821
    ///   apply_type 54
    ///   apply_value 8 10 12 15 20
    ///   apply_duration 60 120 180 300 600
    /// end
    pub fn load_from_text(&mut self, text: &str) -> usize {
        let mut count = 0;
        let mut current_vnum: Option<u32> = None;
        let mut current_apply: Option<u32> = None;
        let mut current_values = Vec::new();
        let mut current_durations = Vec::new();

        for raw_line in text.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
                continue;
            }

            if line.eq_ignore_ascii_case("section") {
                current_vnum = None;
                current_apply = None;
                current_values.clear();
                current_durations.clear();
            } else if line.eq_ignore_ascii_case("end") {
                if let (Some(vnum), Some(apply)) = (current_vnum, current_apply) {
                    let mut recipe = BlendPotionRecipe::new(vnum, apply);
                    recipe.apply_values = current_values.clone();
                    recipe.apply_durations = current_durations.clone();
                    self.recipes.insert(vnum, recipe);
                    count += 1;
                }
            } else {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    match parts[0].to_ascii_lowercase().as_str() {
                        "item_vnum" => {
                            current_vnum = parts[1].parse().ok();
                        }
                        "apply_type" => {
                            current_apply = parts[1].parse().ok();
                        }
                        "apply_value" => {
                            current_values = parts[1..].iter().filter_map(|s| s.parse().ok()).collect();
                        }
                        "apply_duration" => {
                            current_durations = parts[1..].iter().filter_map(|s| s.parse().ok()).collect();
                        }
                        _ => {}
                    }
                }
            }
        }

        count
    }

    /// Sprawdza czy podane wartosci butelki sa dopuszczalne w ramach receptury.
    pub fn validate_instance(&self, instance: &BlendPotionInstance) -> bool {
        if let Some(recipe) = self.get_recipe(instance.item_vnum) {
            recipe.apply_type == instance.apply_type
                && (recipe.apply_values.is_empty() || recipe.apply_values.contains(&instance.apply_value))
                && (recipe.apply_durations.is_empty() || recipe.apply_durations.contains(&instance.duration_seconds))
        } else {
            false
        }
    }
}
