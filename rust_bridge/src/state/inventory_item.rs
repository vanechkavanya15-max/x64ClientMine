// Reprezentacja pojedynczego przedmiotu w ekwipunku.
// Zawiera podstawowe informacje takie jak vid, vnum, count,
// a takze tablice na 3 gniazda i 7 bonusow.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ItemAttribute {
    pub b_type: u8,
    pub value: i16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InventoryItem {
    pub vid: u32,
    pub vnum: u32,
    pub count: u8,
    pub sockets: [i32; 3],
    pub attributes: [ItemAttribute; 7],
}

impl InventoryItem {
    // Tworzy nowy przedmiot.
    pub fn new(vid: u32, vnum: u32, count: u8) -> Self {
        Self {
            vid,
            vnum,
            count,
            sockets: [0; 3],
            attributes: [ItemAttribute { b_type: 0, value: 0 }; 7],
        }
    }

    // Sprawdza czy przedmiot ma poprawne ID (vid > 0).
    pub fn is_valid(&self) -> bool {
        self.vid > 0
    }

    // Ustawia wartosc gniazda pod zadanym indeksem.
    // Zwraca blad, jesli indeks jest poza zakresem (0-2) lub przedmiot ma niepoprawne vid.
    pub fn set_socket(&mut self, index: usize, value: i32) -> Result<(), &'static str> {
        if !self.is_valid() {
            return Err("Niepoprawne VID przedmiotu");
        }
        if index >= self.sockets.len() {
            return Err("Indeks gniazda poza zakresem");
        }
        self.sockets[index] = value;
        Ok(())
    }

    // Pobiera wartosc gniazda pod zadanym indeksem.
    // Zwraca None, jesli indeks jest poza zakresem (0-2) lub przedmiot ma niepoprawne vid.
    pub fn get_socket(&self, index: usize) -> Option<i32> {
        if !self.is_valid() {
            return None;
        }
        if index >= self.sockets.len() {
            return None;
        }
        Some(self.sockets[index])
    }

    // Ustawia bonus pod zadanym indeksem.
    // Zwraca blad, jesli indeks jest poza zakresem (0-6) lub przedmiot ma niepoprawne vid.
    pub fn set_attribute(&mut self, index: usize, b_type: u8, value: i16) -> Result<(), &'static str> {
        if !self.is_valid() {
            return Err("Niepoprawne VID przedmiotu");
        }
        if index >= self.attributes.len() {
            return Err("Indeks bonusu poza zakresem");
        }
        self.attributes[index] = ItemAttribute { b_type, value };
        Ok(())
    }

    // Pobiera bonus pod zadanym indeksem.
    // Zwraca None, jesli indeks jest poza zakresem (0-6) lub przedmiot ma niepoprawne vid.
    pub fn get_attribute(&self, index: usize) -> Option<ItemAttribute> {
        if !self.is_valid() {
            return None;
        }
        if index >= self.attributes.len() {
            return None;
        }
        Some(self.attributes[index])
    }
}
