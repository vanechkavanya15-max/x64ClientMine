// Definicja dostepnych slotow ekwipunku postaci.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquipmentSlot {
    Armor = 0,
    Helmet = 1,
    Shield = 2,
    Shoes = 3,
    Weapon = 4,
    Jewelry = 5,
    Sash = 6,
    Belt = 7,
}

impl EquipmentSlot {
    // Funkcja pomocnicza do konwersji z liczby na enum, umozliwia bezpieczna obsluge indeksow.
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(EquipmentSlot::Armor),
            1 => Some(EquipmentSlot::Helmet),
            2 => Some(EquipmentSlot::Shield),
            3 => Some(EquipmentSlot::Shoes),
            4 => Some(EquipmentSlot::Weapon),
            5 => Some(EquipmentSlot::Jewelry),
            6 => Some(EquipmentSlot::Sash),
            7 => Some(EquipmentSlot::Belt),
            _ => None,
        }
    }
}

// Menedzer stanu ekwipunku postaci.
#[derive(Debug, Clone)]
pub struct EquipmentManager {
    // Tablica przechowujaca VID (Virtual ID) przedmiotow.
    // Wartosc 0 oznacza brak przedmiotu (pusty slot).
    slots: [u32; 8],
}

impl Default for EquipmentManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EquipmentManager {
    // Tworzy nowa, pusta instancje menedzera ekwipunku.
    pub fn new() -> Self {
        Self {
            slots: [0; 8],
        }
    }

    // Zaklada przedmiot (okreslony przez VID) w podanym slocie (indeks).
    // Zwraca true jesli operacja zakonczyla sie sukcesem, w przeciwnym razie false.
    // Zapobiega przypisaniu blednego VID rownego 0 oraz indeksow poza zakresem.
    pub fn equip_item(&mut self, slot_index: usize, vid: u32) -> bool {
        if vid == 0 {
            return false; // VID rowne 0 jest nieprawidlowe, gdyz 0 oznacza pusty slot.
        }

        if let Some(slot) = EquipmentSlot::from_index(slot_index) {
            self.slots[slot as usize] = vid;
            true
        } else {
            false // Indeks poza dopuszczalnym zakresem.
        }
    }

    // Zdejmuje przedmiot z podanego slotu (indeks) i zwraca jego VID.
    // Zwraca 0, jesli indeks jest poza zakresem lub slot byl juz pusty.
    pub fn unequip_item(&mut self, slot_index: usize) -> u32 {
        if let Some(slot) = EquipmentSlot::from_index(slot_index) {
            let vid = self.slots[slot as usize];
            self.slots[slot as usize] = 0;
            vid
        } else {
            0 // Indeks poza dopuszczalnym zakresem.
        }
    }

    // Pobiera VID przedmiotu w danym slocie bez jego zdejmowania.
    // Zwraca 0 jesli slot jest pusty lub indeks znajduje sie poza zakresem.
    pub fn get_item(&self, slot_index: usize) -> u32 {
        if let Some(slot) = EquipmentSlot::from_index(slot_index) {
            self.slots[slot as usize]
        } else {
            0 // Indeks poza dopuszczalnym zakresem.
        }
    }
}
