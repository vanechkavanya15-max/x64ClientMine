use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::item_attributes::{ItemAttribute, ITEM_ATTRIBUTE_SLOT_MAX_NUM, APPLY_NONE};

/// Reprezentuje pojedynczy możliwy wylosowany bonus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AttributeProbability {
    pub attr_type: u8,
    pub min_value: i16,
    pub max_value: i16,
}

pub struct BonusAttributeChanger {
    // Tutaj normalnie byłaby jakaś konfiguracja, np. z bazy danych
    // lub pliku txt, z której losujemy bonusy.
    possible_attributes: Vec<AttributeProbability>,
}

impl BonusAttributeChanger {
    pub fn new() -> Self {
        Self {
            possible_attributes: Vec::new(),
        }
    }

    pub fn with_possible_attributes(attributes: Vec<AttributeProbability>) -> Self {
        Self {
            possible_attributes: attributes,
        }
    }

    /// Zwraca ilość normalnych bonusów (1-5)
    pub fn get_normal_attribute_count(&self, attributes: &[ItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM]) -> usize {
        let mut count = 0;
        for i in 0..5 {
            if attributes[i].attr_type != APPLY_NONE {
                count += 1;
            }
        }
        count
    }

    /// Dodaje nowy bonus (1-5) z danym prawdopodobieństwem i wartościami
    pub fn add_normal_attribute(&self, attributes: &mut [ItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM]) -> bool {
        let count = self.get_normal_attribute_count(attributes);
        if count >= 5 {
            return false;
        }

        // Musimy uniknąć dodania bonusu, który już istnieje.
        let existing_types: HashSet<u8> = attributes[0..5]
            .iter()
            .map(|a| a.attr_type)
            .filter(|&t| t != APPLY_NONE)
            .collect();

        let available_attrs: Vec<&AttributeProbability> = self.possible_attributes
            .iter()
            .filter(|a| !existing_types.contains(&a.attr_type))
            .collect();

        if available_attrs.is_empty() {
            return false;
        }

        let mut rng = rand::thread_rng();
        if let Some(&attr) = available_attrs.choose(&mut rng) {
            let value = rng.gen_range(attr.min_value..=attr.max_value);
            attributes[count] = ItemAttribute {
                attr_type: attr.attr_type,
                value,
            };
            return true;
        }

        false
    }
    
    pub fn add_rare_attribute(&self, attributes: &mut [ItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM]) -> bool {
        // Find empty slot (6 or 7, index 5 or 6 normally)
        let slot = if attributes[5].attr_type == APPLY_NONE {
            5
        } else if attributes[6].attr_type == APPLY_NONE {
            6
        } else {
            return false;
        };
        
        let existing_types: HashSet<u8> = attributes[5..=6]
            .iter()
            .map(|a| a.attr_type)
            .filter(|&t| t != APPLY_NONE)
            .collect();

        let available_attrs: Vec<&AttributeProbability> = self.possible_attributes
            .iter()
            .filter(|a| !existing_types.contains(&a.attr_type))
            .collect();

        if available_attrs.is_empty() {
            return false;
        }

        let mut rng = rand::thread_rng();
        if let Some(&attr) = available_attrs.choose(&mut rng) {
            let value = rng.gen_range(attr.min_value..=attr.max_value);
            attributes[slot] = ItemAttribute {
                attr_type: attr.attr_type,
                value,
            };
            return true;
        }
        
        false
    }

    /// Zmienia istniejące normalne bonusy (1-5 lub do 6 jeśli obecny)
    pub fn change_normal_attributes(&self, attributes: &mut [ItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM]) -> bool {
        let count = self.get_normal_attribute_count(attributes);
        if count == 0 {
            return false;
        }

        let mut rng = rand::thread_rng();
        let mut new_types = HashSet::new();

        for i in 0..count {
            let available_attrs: Vec<&AttributeProbability> = self.possible_attributes
                .iter()
                .filter(|a| !new_types.contains(&a.attr_type))
                .collect();

            if available_attrs.is_empty() {
                return false;
            }

            let &attr = available_attrs.choose(&mut rng).unwrap();
            let value = rng.gen_range(attr.min_value..=attr.max_value);
            
            attributes[i] = ItemAttribute {
                attr_type: attr.attr_type,
                value,
            };
            new_types.insert(attr.attr_type);
        }

        true
    }
    
    pub fn change_rare_attributes(&self, attributes: &mut [ItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM]) -> bool {
        let mut count = 0;
        if attributes[5].attr_type != APPLY_NONE { count += 1; }
        if attributes[6].attr_type != APPLY_NONE { count += 1; }
        
        if count == 0 {
            return false;
        }
        
        let mut rng = rand::thread_rng();
        let mut new_types = HashSet::new();

        for i in 0..count {
            let available_attrs: Vec<&AttributeProbability> = self.possible_attributes
                .iter()
                .filter(|a| !new_types.contains(&a.attr_type))
                .collect();

            if available_attrs.is_empty() {
                return false; // Nie udało się wylosować
            }

            let &attr = available_attrs.choose(&mut rng).unwrap();
            let value = rng.gen_range(attr.min_value..=attr.max_value);
            
            attributes[5 + i] = ItemAttribute {
                attr_type: attr.attr_type,
                value,
            };
            new_types.insert(attr.attr_type);
        }

        true
    }
}
