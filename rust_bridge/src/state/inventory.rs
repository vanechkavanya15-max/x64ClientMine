#[derive(Debug, PartialEq, Eq)]
pub enum InventoryError {
    InvalidIndex,
}

pub struct Inventory {
    // 180 slotow (4 strony po 45 slotow)
    slots: [u32; 180],
}

impl Inventory {
    // Tworzy nowa, pusta instancje ekwipunku
    pub fn new() -> Self {
        Self {
            slots: [0; 180],
        }
    }

    // Pobiera VID przedmiotu na podstawie indeksu
    pub fn get(&self, index: usize) -> Result<u32, InventoryError> {
        if index >= 180 {
            return Err(InventoryError::InvalidIndex);
        }
        Ok(self.slots[index])
    }

    // Ustawia VID przedmiotu w danym slocie
    pub fn set(&mut self, index: usize, vid: u32) -> Result<(), InventoryError> {
        if index >= 180 {
            return Err(InventoryError::InvalidIndex);
        }
        // Brak przedmiotu (pusty slot) reprezentujemy przez 0
        // Jesli pojawia sie jakies specyficzne bledne VID (np. u32::MAX),
        // mozemy to zablokowac. Tutaj kazde VID przejdzie bezpiecznie.
        self.slots[index] = vid;
        Ok(())
    }

    // Zamienia miejscami dwa sloty
    pub fn swap(&mut self, index1: usize, index2: usize) -> Result<(), InventoryError> {
        if index1 >= 180 || index2 >= 180 {
            return Err(InventoryError::InvalidIndex);
        }
        self.slots.swap(index1, index2);
        Ok(())
    }

    // Znajduje pierwszy darmowy slot (oznaczony jako 0)
    pub fn find_empty(&self) -> Option<usize> {
        self.slots.iter().position(|&vid| vid == 0)
    }
}
