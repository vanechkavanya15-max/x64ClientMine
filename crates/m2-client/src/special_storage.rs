// Kategoria magazynu specjalnego
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemCategory {
    SkillBook,
    UpgradeItems,
    Stone,
}

// Reprezentacja przedmiotu w magazynie specjalnym
#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub category: ItemCategory,
    // Dodatkowe pola mozna dodac wedlug potrzeb
}

// Zdefiniowanie wielkosci magazynu (45 slotow)
pub const SPECIAL_STORAGE_SIZE: usize = 45;

// Struktura reprezentujaca magazyn specjalny
pub struct SpecialStorage {
    // Trzy kategorie: SkillBook (0), UpgradeItems (1), Stone (2)
    slots: [[Option<Item>; SPECIAL_STORAGE_SIZE]; 3],
}

impl SpecialStorage {
    // Inicjalizacja pustego magazynu
    pub fn new() -> Self {
        Self {
            slots: [
                [None; SPECIAL_STORAGE_SIZE],
                [None; SPECIAL_STORAGE_SIZE],
                [None; SPECIAL_STORAGE_SIZE],
            ],
        }
    }

    // Zamiana kategorii na indeks tablicy
    fn category_to_index(category: ItemCategory) -> usize {
        match category {
            ItemCategory::SkillBook => 0,
            ItemCategory::UpgradeItems => 1,
            ItemCategory::Stone => 2,
        }
    }

    // Sprawdza ilosc wolnego miejsca dla danej kategorii
    pub fn get_free_space(&self, category: ItemCategory) -> usize {
        let index = Self::category_to_index(category);
        self.slots[index].iter().filter(|&slot| slot.is_none()).count()
    }

    // Umieszcza przedmiot w konkretnym slocie magazynu specjalnego
    // Zwraca Err() jezeli indeks jest poza zakresem lub slot jest zajety
    pub fn put_item(&mut self, category: ItemCategory, slot: usize, item: Item) -> Result<(), &'static str> {
        if slot >= SPECIAL_STORAGE_SIZE {
            return Err("Index poza zakresem");
        }

        let index = Self::category_to_index(category);
        if self.slots[index][slot].is_some() {
            return Err("Slot jest juz zajety");
        }

        self.slots[index][slot] = Some(item);
        Ok(())
    }

    // Usuwa przedmiot z danego slotu
    pub fn remove_item(&mut self, category: ItemCategory, slot: usize) -> Option<Item> {
        if slot >= SPECIAL_STORAGE_SIZE {
            return None;
        }

        let index = Self::category_to_index(category);
        self.slots[index][slot].take()
    }

    // Automatycznie sortuje przedmiot do odpowiedniej kategorii
    // Szuka pierwszego wolnego slotu i umieszcza tam przedmiot
    pub fn auto_sort_item(&mut self, item: Item) -> Result<usize, &'static str> {
        let index = Self::category_to_index(item.category);
        
        for slot in 0..SPECIAL_STORAGE_SIZE {
            if self.slots[index][slot].is_none() {
                self.slots[index][slot] = Some(item);
                return Ok(slot);
            }
        }
        
        Err("Brak wolnego miejsca w magazynie specjalnym")
    }
}
