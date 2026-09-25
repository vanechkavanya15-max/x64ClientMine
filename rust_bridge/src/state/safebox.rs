use std::collections::HashMap;

/// Reprezentuje pojedynczy przedmiot w magazynie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub vnum: u32,
    pub count: u8,
}

/// Reprezentuje stan pojedynczego magazynu.
#[derive(Debug, Clone)]
pub struct SafeboxState {
    pub items: Vec<Option<Item>>,
    pub gold: i64, // zloto moze byc bardzo duze
}

impl SafeboxState {
    /// Tworzy nowy magazyn o podanym rozmiarze z zerowym stanem zlota.
    pub fn new(size: usize) -> Self {
        SafeboxState {
            items: vec![None; size],
            gold: 0,
        }
    }
}

/// Menedzer zarzadzajacy magazynami dla wielu VID (Virtual ID).
#[derive(Debug)]
pub struct SafeboxManager {
    safeboxes: HashMap<u32, SafeboxState>,
}

impl SafeboxManager {
    /// Tworzy nowa, pusta instancje menedzera magazynow.
    pub fn new() -> Self {
        SafeboxManager {
            safeboxes: HashMap::new(),
        }
    }

    /// Otwiera (lub nadpisuje) magazyn dla podanego VID o zadanym rozmiarze.
    pub fn open_safebox(&mut self, vid: u32, size: usize) {
        self.safeboxes.insert(vid, SafeboxState::new(size));
    }

    /// Dodaje przedmiot na okreslonym indeksie w magazynie.
    /// Zwraca blad (w postaci komunikatu String), jezeli VID nie istnieje,
    /// albo jezeli indeks jest poza zakresem magazynu.
    pub fn add_item(&mut self, vid: u32, index: usize, item: Item) -> Result<(), String> {
        if let Some(state) = self.safeboxes.get_mut(&vid) {
            if index < state.items.len() {
                state.items[index] = Some(item);
                Ok(())
            } else {
                Err("Index poza zakresem magazynu".to_string())
            }
        } else {
            Err("Magazyn dla podanego VID nie istnieje".to_string())
        }
    }

    /// Pobiera sklonowany przedmiot z okreslonego indeksu.
    /// Zwraca None, jezeli VID nie istnieje, indeks jest poza zakresem, 
    /// lub gdy na tym indeksie nie ma przedmiotu.
    pub fn get_item(&self, vid: u32, index: usize) -> Option<Item> {
        self.safeboxes.get(&vid).and_then(|state| {
            if index < state.items.len() {
                state.items[index].clone()
            } else {
                None
            }
        })
    }

    /// Usuwa przedmiot z okreslonego indeksu w magazynie.
    /// Zwraca blad, jezeli VID nie istnieje lub indeks jest poza zakresem.
    pub fn remove_item(&mut self, vid: u32, index: usize) -> Result<(), String> {
        if let Some(state) = self.safeboxes.get_mut(&vid) {
            if index < state.items.len() {
                state.items[index] = None;
                Ok(())
            } else {
                Err("Index poza zakresem magazynu".to_string())
            }
        } else {
            Err("Magazyn dla podanego VID nie istnieje".to_string())
        }
    }

    /// Dodaje zloto do magazynu podanego VID.
    pub fn add_gold(&mut self, vid: u32, amount: i64) -> Result<(), String> {
        if amount < 0 {
            return Err("Kwota musi byc nieujemna".to_string());
        }
        if let Some(state) = self.safeboxes.get_mut(&vid) {
            state.gold = state.gold.saturating_add(amount);
            Ok(())
        } else {
            Err("Magazyn dla podanego VID nie istnieje".to_string())
        }
    }

    /// Odejmuje zloto z magazynu podanego VID.
    /// Zwraca blad jesli w magazynie nie ma wystarczajacej ilosci zlota.
    pub fn sub_gold(&mut self, vid: u32, amount: i64) -> Result<(), String> {
        if amount < 0 {
            return Err("Kwota musi byc nieujemna".to_string());
        }
        if let Some(state) = self.safeboxes.get_mut(&vid) {
            if state.gold >= amount {
                state.gold -= amount;
                Ok(())
            } else {
                Err("Niewystarczajaca ilosc zlota w magazynie".to_string())
            }
        } else {
            Err("Magazyn dla podanego VID nie istnieje".to_string())
        }
    }

    /// Pobiera ilosc zlota z magazynu podanego VID. Zwraca 0 jezeli VID nie istnieje.
    pub fn get_gold(&self, vid: u32) -> i64 {
        self.safeboxes.get(&vid).map(|state| state.gold).unwrap_or(0)
    }
}
