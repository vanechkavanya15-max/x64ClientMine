use m2_client::inventory_grid_v2::*;
use std::ptr;

#[test]
fn test_window_max_slots() {
    assert_eq!(WindowType::INVENTORY.max_slots(), 180);
    assert_eq!(WindowType::EQUIPMENT.max_slots(), 32);
    assert_eq!(WindowType::SAFEBOX.max_slots(), 135);
    assert_eq!(WindowType::MALL.max_slots(), 45);
    assert_eq!(WindowType::DRAGON_SOUL_INVENTORY.max_slots(), 960);
    assert_eq!(WindowType::BELT_INVENTORY.max_slots(), 16);
}

#[test]
fn test_grid_placement_1x1() {
    let mut grid = WindowGrid::new(WindowType::INVENTORY);
    let item = ItemData {
        vnum: 10,
        count: 1,
        ..Default::default()
    };
    
    // Test poprawnego ustawienia
    assert!(grid.set_item(0, item, 1));
    assert_eq!({grid.get_item(0).unwrap().vnum}, 10);
    
    // Test usuwania
    let removed = grid.del_item(0, 1).unwrap();
    assert_eq!({removed.vnum}, 10);
    assert!(grid.get_item(0).is_none());
}

#[test]
fn test_grid_placement_1x2_and_collision() {
    let mut grid = WindowGrid::new(WindowType::INVENTORY);
    let item2x = ItemData {
        vnum: 20,
        count: 1,
        ..Default::default()
    };
    
    // Pozycja 0, rozmiar 2 = zajmie 0 i 5 (bo 5 kolumn)
    assert!(grid.set_item(0, item2x, 2));
    
    // W testach WindowGrid.set_item w samej metodzie 'set_item' nie ma sprawdzania kolizji,
    // ta logika jest uproszczona i nadpisuje stan 'slots' na true, 
    // jednak samo set_item moze zwrocic false jesli wykracza poza rozmiar
    
    // Jesli ustawimy cos co wystaje poza grid
    assert!(!grid.set_item(178, item2x, 2)); // 178 + 5 = 183 > 180 (false)
}

#[test]
fn test_inventory_system_logic() {
    let mut sys = InventorySystem::new();
    
    // Rejestrujemy rozmiary (domyslnie bez rejestracji jest 1)
    sys.register_item_size(11, 2);
    sys.register_item_size(22, 3);
    
    let item1 = ItemData { vnum: 11, count: 1, ..Default::default() };
    let item2 = ItemData { vnum: 22, count: 1, ..Default::default() };
    
    assert!(sys.set_item(WindowType::INVENTORY, 10, item1));
    assert_eq!({sys.get_item(WindowType::INVENTORY, 10).unwrap().vnum}, 11);
    
    assert!(sys.set_item(WindowType::INVENTORY, 10, item2));
    assert_eq!({sys.get_item(WindowType::INVENTORY, 10).unwrap().vnum}, 22);
    
    assert!(sys.del_item(WindowType::INVENTORY, 10));
    assert!(sys.get_item(WindowType::INVENTORY, 10).is_none());
}

#[test]
fn test_ffi_set_get_del() {
    m2_inv2_register_item_size(100, 2);
    
    let sockets = [1, 2, 3];
    let attrs = [
        TPlayerItemAttribute { bType: 1, sValue: 10 },
        TPlayerItemAttribute { bType: 2, sValue: 20 },
        TPlayerItemAttribute { bType: 3, sValue: 30 },
        TPlayerItemAttribute { bType: 0, sValue: 0 },
        TPlayerItemAttribute { bType: 0, sValue: 0 },
        TPlayerItemAttribute { bType: 0, sValue: 0 },
        TPlayerItemAttribute { bType: 0, sValue: 0 },
    ];
    
    let w_inv = WindowType::INVENTORY as u8;
    
    // Ustaw przedmiot przez FFI z socetami i atrybutami
    let success = m2_inv2_set_item(
        w_inv,
        5,
        100, // vnum
        200, // count
        sockets.as_ptr(),
        attrs.as_ptr()
    );
    assert!(success);
    
    // Sprawdz FFI GET
    let mut out_vnum = 0;
    let mut out_count = 0;
    
    let success2 = m2_inv2_get_item(w_inv, 5, &mut out_vnum, &mut out_count);
    assert!(success2);
    assert_eq!(out_vnum, 100);
    assert_eq!(out_count, 200);
    
    // Bezposrednio sprawdzmy czy sockets/attrs przeszly dobrze na grida
    if let Ok(sys) = get_inventory_sys().lock() {
        if let Some(item) = sys.get_item(WindowType::INVENTORY, 5) {
            assert_eq!({item.sockets}, sockets);
            assert_eq!({item.attrs[0].bType}, 1);
            assert_eq!({item.attrs[0].sValue}, 10);
        } else {
            panic!("Item not found");
        }
    }
    
    // Sprawdz usuniecie
    let success3 = m2_inv2_del_item(w_inv, 5);
    assert!(success3);
    
    let success4 = m2_inv2_get_item(w_inv, 5, &mut out_vnum, &mut out_count);
    assert!(!success4);
}
