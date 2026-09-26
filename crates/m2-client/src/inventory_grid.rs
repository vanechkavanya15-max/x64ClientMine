#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemSize {
    Size1x1 = 1,
    Size1x2 = 2,
    Size1x3 = 3,
}

#[derive(Debug, Clone)]
pub struct InventoryGrid {
    slots: [bool; 180],
}

impl Default for InventoryGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl InventoryGrid {
    pub const TOTAL_SLOTS: u16 = 180;
    pub const PAGE_SLOTS: u16 = 90;
    pub const COLUMNS: u16 = 5;
    pub const ROWS_PER_PAGE: u16 = 18; // 9 wierszy * 2 pietra = 18 wierszy na strone

    pub fn new() -> Self {
        Self {
            slots: [false; 180],
        }
    }

    /// Sprawdza, czy mozna umiescic przedmiot w danym slocie
    pub fn can_put_item(&self, slot: u16, size: u8) -> bool {
        if slot >= Self::TOTAL_SLOTS {
            return false;
        }

        if size < 1 || size > 3 {
            return false;
        }

        let size_u16 = size as u16;

        // Okreslamy strone (0 lub 1)
        let _page = slot / Self::PAGE_SLOTS;
        // Pozycja slotu wewnatrz aktualnej strony
        let slot_in_page = slot % Self::PAGE_SLOTS;
        // Wiersz wewnatrz aktualnej strony
        let row = slot_in_page / Self::COLUMNS;

        // Przedmiot nie moze wychodzic poza dolna krawedz aktualnej strony
        if row + size_u16 > Self::ROWS_PER_PAGE {
            return false;
        }

        // Sprawdzanie kolizji ze wszystkimi fragmentami przedmiotu w pionie
        for i in 0..size_u16 {
            let check_slot = slot + i * Self::COLUMNS;
            
            if check_slot >= Self::TOTAL_SLOTS {
                return false;
            }

            if self.slots[check_slot as usize] {
                return false;
            }
        }

        true
    }

    /// Wklada przedmiot do inwentarza zajmujac odpowiednie kafelki
    pub fn put_item(&mut self, slot: u16, size: u8) -> Result<(), &'static str> {
        if !self.can_put_item(slot, size) {
            return Err("Kolizja lub brak miejsca w inwentarzu.");
        }

        let size_u16 = size as u16;
        for i in 0..size_u16 {
            let s = slot + i * Self::COLUMNS;
            self.slots[s as usize] = true;
        }

        Ok(())
    }

    /// Usuwa przedmiot z inwentarza zwalniajac odpowiednie kafelki
    pub fn remove_item(&mut self, slot: u16, size: u8) {
        let size_u16 = size as u16;
        for i in 0..size_u16 {
            let s = slot + i * Self::COLUMNS;
            if s < Self::TOTAL_SLOTS {
                self.slots[s as usize] = false;
            }
        }
    }

    /// Wyszukuje pierwszy wolny slot, w ktorym zmiesci sie przedmiot
    pub fn find_free_slot(&self, size: u8) -> Option<u16> {
        for slot in 0..Self::TOTAL_SLOTS {
            if self.can_put_item(slot, size) {
                return Some(slot);
            }
        }
        None
    }
}
