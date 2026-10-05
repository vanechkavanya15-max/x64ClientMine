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
}

impl Default for BonusAttributeChanger {
    fn default() -> Self {
        Self::new()
    }
}

impl BonusAttributeChanger {
    pub fn with_possible_attributes(attributes: Vec<AttributeProbability>) -> Self {
        Self {
            possible_attributes: attributes,
        }
    }

    /// Zwraca ilość normalnych bonusów (1-5)
    pub fn get_normal_attribute_count(&self, attributes: &[ItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM]) -> usize {
        attributes[0..5]
            .iter()
            .filter(|a| a.attr_type != APPLY_NONE)
            .count()
    }

    /// Dodaje nowy bonus (1-5) z danym prawdopodobieństwem i wartościami
    pub fn add_normal_attribute(&self, attributes: &mut [ItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM]) -> bool {
        let count = self.get_normal_attribute_count(attributes);
        if count >= 5 {
            return false;
        }

        // Musimy uniknąć dodania bonusu, który już istnieje.
        let available_attrs: Vec<&AttributeProbability> = self.possible_attributes
            .iter()
            .filter(|a| {
                !attributes[0..5].iter().any(|attr| attr.attr_type == a.attr_type && attr.attr_type != APPLY_NONE)
            })
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
        let empty_slot_index = attributes[5..=6]
            .iter()
            .position(|a| a.attr_type == APPLY_NONE);

        let slot = match empty_slot_index {
            Some(idx) => 5 + idx,
            None => return false,
        };
        
        let available_attrs: Vec<&AttributeProbability> = self.possible_attributes
            .iter()
            .filter(|a| {
                !attributes[5..=6].iter().any(|attr| attr.attr_type == a.attr_type && attr.attr_type != APPLY_NONE)
            })
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
        
        // Możemy po prostu potasować dostępne i wziąć pierwszych `count` elementów,
        // co zagwarantuje nam unikalność oraz bycie o wiele szybszym w działaniu.
        let mut shuffled_attrs = self.possible_attributes.clone();
        shuffled_attrs.shuffle(&mut rng);
        
        if shuffled_attrs.len() < count {
            return false; // Nie ma wystarczającej liczby unikalnych bonusów
        }

        for (attr_dest, attr_src) in attributes[0..count].iter_mut().zip(shuffled_attrs.iter()) {
            let value = rng.gen_range(attr_src.min_value..=attr_src.max_value);
            *attr_dest = ItemAttribute {
                attr_type: attr_src.attr_type,
                value,
            };
        }

        true
    }
    
    pub fn change_rare_attributes(&self, attributes: &mut [ItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM]) -> bool {
        let count = attributes[5..=6]
            .iter()
            .filter(|a| a.attr_type != APPLY_NONE)
            .count();
        
        if count == 0 {
            return false;
        }
        
        let mut rng = rand::thread_rng();
        
        let mut shuffled_attrs = self.possible_attributes.clone();
        shuffled_attrs.shuffle(&mut rng);
        
        if shuffled_attrs.len() < count {
            return false; // Nie udało się wylosować
        }

        for (attr_dest, attr_src) in attributes[5..5+count].iter_mut().zip(shuffled_attrs.iter()) {
            let value = rng.gen_range(attr_src.min_value..=attr_src.max_value);
            *attr_dest = ItemAttribute {
                attr_type: attr_src.attr_type,
                value,
            };
        }

        true
    }
}
