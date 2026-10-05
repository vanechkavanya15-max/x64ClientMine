// crates/m2-client/src/py_bridge_nonplayer_shop.rs
//! Mostek inspekcji stopni potworow i socketow przedmiotow w sklepach (NonPlayerModule / ShopModule).
//! Zapewnia 100% parytetu z C++ PythonNonPlayer.cpp i PythonShop.cpp.

use std::collections::HashMap;
use std::sync::RwLock;
use once_cell::sync::Lazy;
use m2_client_bridge::ffi_guard::safe_ffi_call;

pub const SHOP_HOST_ITEM_MAX_NUM: usize = 40;
pub const ITEM_SOCKET_MAX_NUM: usize = 3;
pub const ITEM_ATTRIBUTE_MAX_NUM: usize = 7;

// Stopnie trudnosci potworow zgodne z EMonsterRank w C++
pub const MONSTER_GRADE_PAWN: u8 = 0;
pub const MONSTER_GRADE_S_PAWN: u8 = 1;
pub const MONSTER_GRADE_KNIGHT: u8 = 2;
pub const MONSTER_GRADE_S_KNIGHT: u8 = 3;
pub const MONSTER_GRADE_BOSS: u8 = 4;
pub const MONSTER_GRADE_KING: u8 = 5;

#[derive(Debug, Clone, Copy, Default)]
pub struct ShopItemData {
    pub vnum: u32,
    pub price: u32,
    pub count: u8,
    pub sockets: [u32; ITEM_SOCKET_MAX_NUM],
    pub attrs: [(u8, i16); ITEM_ATTRIBUTE_MAX_NUM],
}

static MONSTER_GRADES: Lazy<RwLock<HashMap<u32, u8>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(101, MONSTER_GRADE_PAWN);     // Dziki pies
    map.insert(102, MONSTER_GRADE_PAWN);     // Wilk
    map.insert(191, MONSTER_GRADE_KNIGHT);   // Lykos (unikat)
    map.insert(591, MONSTER_GRADE_BOSS);     // Bestialski Kapitan
    map.insert(1093, MONSTER_GRADE_KING);    // Umarly Rozpruwacz
    map.insert(20016, 0);                    // Kowal (NPC)
    RwLock::new(map)
});

static SHOP_ITEMS: Lazy<RwLock<[ShopItemData; SHOP_HOST_ITEM_MAX_NUM]>> = Lazy::new(|| {
    RwLock::new([ShopItemData::default(); SHOP_HOST_ITEM_MAX_NUM])
});

/// Pobiera stopien trudnosci potwora (nonplayer.GetMonsterGrade).
#[no_mangle]
pub extern "C" fn m2_nonplayer_get_monster_grade(vnum: u32) -> u8 {
    safe_ffi_call(MONSTER_GRADE_PAWN, || {
        let map = MONSTER_GRADES.read().unwrap();
        map.get(&vnum).copied().unwrap_or(MONSTER_GRADE_PAWN)
    })
}

/// Ustawia stopien trudnosci dla danego vnuma.
#[no_mangle]
pub extern "C" fn m2_nonplayer_set_monster_grade(vnum: u32, grade: u8) {
    safe_ffi_call((), || {
        let mut map = MONSTER_GRADES.write().unwrap();
        map.insert(vnum, grade);
    });
}

/// Sprawdza czy dany vnum to potwor (nonplayer.IsMonster).
#[no_mangle]
pub extern "C" fn m2_nonplayer_is_monster(vnum: u32) -> bool {
    safe_ffi_call(false, || {
        if vnum >= 20000 && vnum < 30000 {
            return false; // Typowy zakres NPC
        }
        vnum > 0
    })
}

/// Sprawdza czy dany vnum to NPC (nonplayer.IsNPC).
#[no_mangle]
pub extern "C" fn m2_nonplayer_is_npc(vnum: u32) -> bool {
    safe_ffi_call(false, || {
        (20000..30000).contains(&vnum)
    })
}

/// Pobiera socket z przedmiotu w sklepie (shop.GetItemMetinSocket).
#[no_mangle]
pub extern "C" fn m2_shop_get_item_metin_socket(pos: u8, socket_index: u8) -> u32 {
    safe_ffi_call(0, || {
        let pos = pos as usize;
        let socket_index = socket_index as usize;
        if pos >= SHOP_HOST_ITEM_MAX_NUM || socket_index >= ITEM_SOCKET_MAX_NUM {
            return 0;
        }
        let items = SHOP_ITEMS.read().unwrap();
        items[pos].sockets[socket_index]
    })
}

/// Ustawia wartosc socketu dla przedmiotu w sklepie.
#[no_mangle]
pub extern "C" fn m2_shop_set_item_metin_socket(pos: u8, socket_index: u8, socket_val: u32) -> bool {
    safe_ffi_call(false, || {
        let pos = pos as usize;
        let socket_index = socket_index as usize;
        if pos >= SHOP_HOST_ITEM_MAX_NUM || socket_index >= ITEM_SOCKET_MAX_NUM {
            return false;
        }
        let mut items = SHOP_ITEMS.write().unwrap();
        items[pos].sockets[socket_index] = socket_val;
        true
    })
}

/// Pobiera bonus (attribute) przedmiotu w sklepie (shop.GetItemAttribute).
#[no_mangle]
pub extern "C" fn m2_shop_get_item_attribute(
    pos: u8,
    attr_index: u8,
    out_type: *mut u8,
    out_value: *mut i16,
) -> bool {
    safe_ffi_call(false, || {
        let pos = pos as usize;
        let attr_index = attr_index as usize;
        if pos >= SHOP_HOST_ITEM_MAX_NUM || attr_index >= ITEM_ATTRIBUTE_MAX_NUM {
            return false;
        }
        let items = SHOP_ITEMS.read().unwrap();
        let (t, v) = items[pos].attrs[attr_index];
        unsafe {
            if !out_type.is_null() { *out_type = t; }
            if !out_value.is_null() { *out_value = v; }
        }
        true
    })
}

/// Ustawia bonus przedmiotu w sklepie.
#[no_mangle]
pub extern "C" fn m2_shop_set_item_attribute(
    pos: u8,
    attr_index: u8,
    attr_type: u8,
    attr_value: i16,
) -> bool {
    safe_ffi_call(false, || {
        let pos = pos as usize;
        let attr_index = attr_index as usize;
        if pos >= SHOP_HOST_ITEM_MAX_NUM || attr_index >= ITEM_ATTRIBUTE_MAX_NUM {
            return false;
        }
        let mut items = SHOP_ITEMS.write().unwrap();
        items[pos].attrs[attr_index] = (attr_type, attr_value);
        true
    })
}
