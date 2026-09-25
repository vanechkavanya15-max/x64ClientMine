/// Maksymalna liczba przedmiotow w oknie handlu
pub const TRADE_MAX_NUM: usize = 12;

/// Struktura reprezentujaca przedmiot w oknie handlu
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TradeItem {
    pub vnum: u32,
    pub count: u32,
}

/// Struktura przechowujaca stan handlu z innym graczem
#[derive(Debug, Clone)]
pub struct TradeState {
    /// Czy okno handlu jest otwarte
    pub is_open: bool,
    /// ID partnera w handlu (0 oznacza brak partnera)
    pub partner_vid: u32,
    /// Sloty z wlasnymi przedmiotami
    pub own_items: [Option<TradeItem>; TRADE_MAX_NUM],
    /// Sloty z przedmiotami partnera
    pub partner_items: [Option<TradeItem>; TRADE_MAX_NUM],
    /// Wlasne zloto w oknie handlu
    pub own_gold: u64,
    /// Zloto partnera w oknie handlu
    pub partner_gold: u64,
    /// Czy wlasna czesc jest zablokowana (zaakceptowana)
    pub is_own_locked: bool,
    /// Czy czesc partnera jest zablokowana (zaakceptowana)
    pub is_partner_locked: bool,
}

impl Default for TradeState {
    fn default() -> Self {
        Self::new()
    }
}

impl TradeState {
    /// Tworzy nowy, pusty stan handlu
    pub fn new() -> Self {
        Self {
            is_open: false,
            partner_vid: 0,
            own_items: Default::default(),
            partner_items: Default::default(),
            own_gold: 0,
            partner_gold: 0,
            is_own_locked: false,
            is_partner_locked: false,
        }
    }

    /// Otwiera okno handlu z podanym partnerem
    pub fn open(&mut self, partner_vid: u32) -> Result<(), &'static str> {
        if partner_vid == 0 {
            return Err("Nieprawidlowe VID partnera.");
        }
        if self.is_open {
            return Err("Handel jest juz otwarty.");
        }
        
        self.clear();
        self.is_open = true;
        self.partner_vid = partner_vid;
        Ok(())
    }

    /// Zamyka okno handlu i czysci stan
    pub fn close(&mut self) {
        self.clear();
    }

    /// Czysci wewnetrzny stan handlu
    fn clear(&mut self) {
        self.is_open = false;
        self.partner_vid = 0;
        self.own_items = Default::default();
        self.partner_items = Default::default();
        self.own_gold = 0;
        self.partner_gold = 0;
        self.is_own_locked = false;
        self.is_partner_locked = false;
    }

    /// Dodaje wlasny przedmiot do handlu
    pub fn set_own_item(&mut self, slot: usize, vnum: u32, count: u32) -> Result<(), &'static str> {
        if !self.is_open {
            return Err("Handel nie jest otwarty.");
        }
        if self.is_own_locked {
            return Err("Wlasna czesc handlu jest zablokowana.");
        }
        if slot >= TRADE_MAX_NUM {
            return Err("Indeks slotu poza zakresem.");
        }
        self.own_items[slot] = Some(TradeItem { vnum, count });
        Ok(())
    }

    /// Usuwa wlasny przedmiot z handlu
    pub fn remove_own_item(&mut self, slot: usize) -> Result<(), &'static str> {
        if !self.is_open {
            return Err("Handel nie jest otwarty.");
        }
        if self.is_own_locked {
            return Err("Wlasna czesc handlu jest zablokowana.");
        }
        if slot >= TRADE_MAX_NUM {
            return Err("Indeks slotu poza zakresem.");
        }
        self.own_items[slot] = None;
        Ok(())
    }

    /// Dodaje przedmiot partnera do handlu
    pub fn set_partner_item(&mut self, slot: usize, vnum: u32, count: u32) -> Result<(), &'static str> {
        if !self.is_open {
            return Err("Handel nie jest otwarty.");
        }
        if self.is_partner_locked {
            return Err("Czesc partnera jest zablokowana.");
        }
        if slot >= TRADE_MAX_NUM {
            return Err("Indeks slotu poza zakresem.");
        }
        self.partner_items[slot] = Some(TradeItem { vnum, count });
        Ok(())
    }

    /// Usuwa przedmiot partnera z handlu
    pub fn remove_partner_item(&mut self, slot: usize) -> Result<(), &'static str> {
        if !self.is_open {
            return Err("Handel nie jest otwarty.");
        }
        if self.is_partner_locked {
            return Err("Czesc partnera jest zablokowana.");
        }
        if slot >= TRADE_MAX_NUM {
            return Err("Indeks slotu poza zakresem.");
        }
        self.partner_items[slot] = None;
        Ok(())
    }

    /// Ustawia ilosc wlasnego zlota w handlu
    pub fn set_own_gold(&mut self, gold: u64) -> Result<(), &'static str> {
        if !self.is_open {
            return Err("Handel nie jest otwarty.");
        }
        if self.is_own_locked {
            return Err("Wlasna czesc handlu jest zablokowana.");
        }
        self.own_gold = gold;
        Ok(())
    }

    /// Ustawia ilosc zlota partnera w handlu
    pub fn set_partner_gold(&mut self, gold: u64) -> Result<(), &'static str> {
        if !self.is_open {
            return Err("Handel nie jest otwarty.");
        }
        if self.is_partner_locked {
            return Err("Czesc partnera jest zablokowana.");
        }
        self.partner_gold = gold;
        Ok(())
    }

    /// Blokuje wlasna czesc handlu (akceptacja)
    pub fn lock_own(&mut self) -> Result<(), &'static str> {
        if !self.is_open {
            return Err("Handel nie jest otwarty.");
        }
        self.is_own_locked = true;
        Ok(())
    }

    /// Odblokowuje wlasna czesc handlu (np. po anulowaniu akceptacji)
    pub fn unlock_own(&mut self) -> Result<(), &'static str> {
        if !self.is_open {
            return Err("Handel nie jest otwarty.");
        }
        self.is_own_locked = false;
        Ok(())
    }

    /// Blokuje czesc partnera w handlu
    pub fn lock_partner(&mut self) -> Result<(), &'static str> {
        if !self.is_open {
            return Err("Handel nie jest otwarty.");
        }
        self.is_partner_locked = true;
        Ok(())
    }

    /// Odblokowuje czesc partnera
    pub fn unlock_partner(&mut self) -> Result<(), &'static str> {
        if !self.is_open {
            return Err("Handel nie jest otwarty.");
        }
        self.is_partner_locked = false;
        Ok(())
    }
}
