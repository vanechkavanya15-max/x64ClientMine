use std::panic::catch_unwind;
use crate::state::inventory_item::InventoryItem;

/// Stala dla menedzera handlu
pub const TRADE_MANAGER: u8 = 0;

/// Globalna instancja TradeManagera dla C FFI (poniewaz sygnatura FFI nie przyjmuje wskaznika)
pub static mut GLOBAL_TRADE_MANAGER: *mut TradeManager = std::ptr::null_mut();

/// Struktura TradeManager (32-bit Little Endian, zgodna z Win32 MSVC - co do zasady layoutu)
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct TradeManager {
    pub is_active: bool,
    pub my_gold: i64,
    pub partner_gold: i64,
    pub my_locked: bool,
    pub partner_locked: bool,
    pub my_items: [Option<InventoryItem>; 12],
    pub partner_items: [Option<InventoryItem>; 12],
}

impl Default for TradeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TradeManager {
    /// Tworzy nowy, pusty stan handlu
    pub fn new() -> Self {
        Self {
            is_active: false,
            my_gold: 0,
            partner_gold: 0,
            my_locked: false,
            partner_locked: false,
            my_items: [
                None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            partner_items: [
                None, None, None, None, None, None, None, None, None, None, None, None,
            ],
        }
    }

    /// Rozpoczyna handel
    pub fn start_trade(&mut self) {
        self.is_active = true;
        self.my_gold = 0;
        self.partner_gold = 0;
        self.my_locked = false;
        self.partner_locked = false;
        self.my_items = [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ];
        self.partner_items = [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ];
    }

    /// Akceptuje/blokuje swoja czesc handlu
    pub fn lock_trade(&mut self) -> Result<(), &'static str> {
        if !self.is_active {
            return Err("Handel nie jest aktywny.");
        }
        self.my_locked = true;
        Ok(())
    }

    /// Akceptuje/blokuje czesc handlu partnera
    pub fn lock_partner_trade(&mut self) -> Result<(), &'static str> {
        if !self.is_active {
            return Err("Handel nie jest aktywny.");
        }
        self.partner_locked = true;
        Ok(())
    }

    /// Ustawia wlasne zloto (yang)
    pub fn set_my_gold(&mut self, gold: i64) -> Result<(), &'static str> {
        if !self.is_active {
            return Err("Handel nie jest aktywny.");
        }
        if self.my_locked {
            return Err("Handel jest zablokowany, nie mozna zmienic zlota.");
        }
        self.my_gold = gold;
        Ok(())
    }

    /// Ustawia zloto (yang) partnera
    pub fn set_partner_gold(&mut self, gold: i64) -> Result<(), &'static str> {
        if !self.is_active {
            return Err("Handel nie jest aktywny.");
        }
        if self.partner_locked {
            return Err("Handel partnera jest zablokowany, nie mozna zmienic zlota.");
        }
        self.partner_gold = gold;
        Ok(())
    }

    /// Anuluje / zamyka handel
    pub fn cancel_trade(&mut self) {
        self.is_active = false;
        self.my_gold = 0;
        self.partner_gold = 0;
        self.my_locked = false;
        self.partner_locked = false;
        self.my_items = [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ];
        self.partner_items = [
            None, None, None, None, None, None, None, None, None, None, None, None,
        ];
    }

    /// Dodaje swoj przedmiot do handlu
    pub fn add_my_item(&mut self, slot: usize, item: InventoryItem) -> Result<(), &'static str> {
        if !self.is_active {
            return Err("Handel nie jest aktywny.");
        }
        if self.my_locked {
            return Err("Handel jest zablokowany, nie mozna dodac przedmiotu.");
        }
        if slot >= 12 {
            return Err("Nieprawidlowy slot.");
        }
        self.my_items[slot] = Some(item);
        Ok(())
    }

    /// Dodaje przedmiot partnera do handlu
    pub fn add_partner_item(&mut self, slot: usize, item: InventoryItem) -> Result<(), &'static str> {
        if !self.is_active {
            return Err("Handel nie jest aktywny.");
        }
        if self.partner_locked {
            return Err("Handel partnera jest zablokowany, nie mozna dodac przedmiotu.");
        }
        if slot >= 12 {
            return Err("Nieprawidlowy slot.");
        }
        self.partner_items[slot] = Some(item);
        Ok(())
    }
}

// ==========================================
// C-FFI
// ==========================================

/// Inicjalizuje globalnego menedzera handlu (zwraca rowniez wskaznik, by kod C mogl zwalniac/uzywac w innych API).
#[no_mangle]
pub unsafe extern "C" fn m2_trade_manager_new() -> *mut TradeManager {
    let result = catch_unwind(|| unsafe {
        let manager = Box::new(TradeManager::new());
        let ptr = Box::into_raw(manager);
        GLOBAL_TRADE_MANAGER = ptr;
        ptr
    });
    result.unwrap_or(std::ptr::null_mut())
}

/// Zwalnia globalnego/przekazanego menedzera handlu.
#[no_mangle]
pub unsafe extern "C" fn m2_trade_manager_free(ptr: *mut TradeManager) {
    let _ = catch_unwind(|| unsafe {
        if !ptr.is_null() {
            let _ = Box::from_raw(ptr);
            if ptr == GLOBAL_TRADE_MANAGER {
                GLOBAL_TRADE_MANAGER = std::ptr::null_mut();
            }
        }
    });
}

/// Sprawdza czy wlasna czesc handlu (w domyslnym globalnym kontekscie) jest zablokowana.
/// Wymagana w zadaniu sygnatura: `pub unsafe extern "C" fn m2_trade_is_locked() -> bool`
#[no_mangle]
pub unsafe extern "C" fn m2_trade_is_locked() -> bool {
    let result = catch_unwind(|| unsafe {
        if GLOBAL_TRADE_MANAGER.is_null() {
            return false;
        }
        let manager = &*GLOBAL_TRADE_MANAGER;
        manager.my_locked
    });
    result.unwrap_or(false)
}
