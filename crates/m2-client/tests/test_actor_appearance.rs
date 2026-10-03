use m2_client::actor_appearance::*;

struct MockItemManager;

impl ItemDataManager for MockItemManager {
    fn get_item_type(&self, vnum: u32) -> Option<u8> {
        match vnum {
            10..=19 | 20..=29 | 30..=39 | 40..=49 | 50..=59 | 60..=69 => Some(ITEM_TYPE_WEAPON),
            100 => Some(ITEM_TYPE_ROD),
            200 => Some(ITEM_TYPE_ARMOR),
            _ => None,
        }
    }

    fn get_weapon_type(&self, vnum: u32) -> Option<u8> {
        match vnum {
            10..=19 => Some(WEAPON_SWORD),
            20..=29 => Some(WEAPON_DAGGER),
            30..=39 => Some(WEAPON_BOW),
            40..=49 => Some(WEAPON_TWO_HANDED),
            50..=59 => Some(WEAPON_BELL),
            60..=69 => Some(WEAPON_FAN),
            70..=79 => Some(WEAPON_ARROW),
            _ => None,
        }
    }
}

#[test]
fn test_part_assignment() {
    let mut app = ActorAppearance::new();

    app.set_part_vnum(PART_MAIN, 11299);
    assert_eq!(app.get_part_vnum(PART_MAIN), 11299);
    assert_eq!(app.armor_vnum, 11299);

    app.set_part_vnum(PART_WEAPON, 299);
    assert_eq!(app.get_part_vnum(PART_WEAPON), 299);
    assert_eq!(app.weapon_vnum, 299);

    app.set_part_vnum(PART_MAX_NUM + 1, 999);
    assert_eq!(app.get_part_vnum(PART_MAX_NUM + 1), 0);
}

#[test]
fn test_methods_attach_weapon_change_armor_set_hair() {
    let mut app = ActorAppearance::new();
    
    app.attach_weapon(300);
    assert_eq!(app.weapon_vnum, 300);
    assert_eq!(app.get_part_vnum(PART_WEAPON), 300);

    app.change_armor(12000);
    assert_eq!(app.armor_vnum, 12000);
    assert_eq!(app.get_part_vnum(PART_MAIN), 12000);

    app.set_hair(250);
    assert_eq!(app.hair_vnum, 250);
    assert_eq!(app.get_part_vnum(PART_HAIR), 250);
}

static TEST_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn test_ffi_functions() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let vid = 999;
    {
        let mut mgr = m2_client::character_manager_core::get_character_manager().write().unwrap();
        mgr.clear();
        mgr.create_instance(vid, 0, 0.0, 0.0, 0.0);
    }
    
    assert_eq!(m2_actor_get_weapon(vid), 0);
    assert_eq!(m2_actor_get_armor(vid), 0);
    
    assert!(m2_actor_attach_weapon(vid, 500));
    assert_eq!(m2_actor_get_weapon(vid), 500);
    
    assert!(m2_actor_change_armor(vid, 11299));
    assert_eq!(m2_actor_get_armor(vid), 11299);
}

#[test]
fn test_motion_mode_general() {
    let mut app = ActorAppearance::new();
    let item_mgr = MockItemManager;

    app.refresh_state(&item_mgr);
    assert_eq!(app.get_motion_mode(), MODE_GENERAL);
}

#[test]
fn test_motion_mode_sword() {
    let mut app = ActorAppearance::new();
    let item_mgr = MockItemManager;

    app.set_part_vnum(PART_WEAPON, 15);
    app.refresh_state(&item_mgr);
    assert_eq!(app.get_motion_mode(), MODE_ONEHAND_SWORD);
}

#[test]
fn test_motion_mode_twohand_sword() {
    let mut app = ActorAppearance::new();
    let item_mgr = MockItemManager;

    app.set_part_vnum(PART_WEAPON, 45);
    app.refresh_state(&item_mgr);
    assert_eq!(app.get_motion_mode(), MODE_TWOHAND_SWORD);
}

#[test]
fn test_motion_mode_bow() {
    let mut app = ActorAppearance::new();
    let item_mgr = MockItemManager;

    app.set_part_vnum(PART_WEAPON, 35);
    app.refresh_state(&item_mgr);
    assert_eq!(app.get_motion_mode(), MODE_BOW);
}

#[test]
fn test_motion_mode_horse_with_weapon() {
    let mut app = ActorAppearance::new();
    let item_mgr = MockItemManager;

    app.set_mounting(true);
    app.set_part_vnum(PART_WEAPON, 15); // Sword
    app.refresh_state(&item_mgr);
    assert_eq!(app.get_motion_mode(), MODE_HORSE_ONEHAND_SWORD);

    app.set_part_vnum(PART_WEAPON, 45); // Two-handed
    app.refresh_state(&item_mgr);
    assert_eq!(app.get_motion_mode(), MODE_HORSE_TWOHAND_SWORD);
}

#[test]
fn test_motion_mode_horse_without_weapon() {
    let mut app = ActorAppearance::new();
    let item_mgr = MockItemManager;

    app.set_mounting(true);
    app.refresh_state(&item_mgr);
    assert_eq!(app.get_motion_mode(), MODE_HORSE);
}

#[test]
fn test_motion_mode_poly_priority() {
    let mut app = ActorAppearance::new();
    let item_mgr = MockItemManager;

    app.set_poly(true);
    app.set_wearing_dress(true);
    app.set_mounting(true);
    app.set_part_vnum(PART_WEAPON, 15); // Sword

    app.refresh_state(&item_mgr);
    // Poly should override everything else
    assert_eq!(app.get_motion_mode(), MODE_GENERAL);
}

#[test]
fn test_motion_mode_wedding_dress() {
    let mut app = ActorAppearance::new();
    let item_mgr = MockItemManager;

    app.set_wearing_dress(true);
    app.set_part_vnum(PART_WEAPON, 15); // Sword

    app.refresh_state(&item_mgr);
    // Dress should override mounting/weapon but not poly
    assert_eq!(app.get_motion_mode(), MODE_WEDDING_DRESS);
}

#[test]
fn test_motion_mode_fishing() {
    let mut app = ActorAppearance::new();
    let item_mgr = MockItemManager;

    app.set_part_vnum(PART_WEAPON, 100); // Rod
    app.refresh_state(&item_mgr);
    assert_eq!(app.get_motion_mode(), MODE_FISHING);
}

#[test]
fn test_motion_mode_fishing_on_horse() {
    let mut app = ActorAppearance::new();
    let item_mgr = MockItemManager;

    app.set_mounting(true);
    app.set_part_vnum(PART_WEAPON, 100); // Rod
    app.refresh_state(&item_mgr);
    assert_eq!(app.get_motion_mode(), MODE_HORSE);
}

#[test]
fn test_reset_parts() {
    let mut app = ActorAppearance::new();
    app.set_part_vnum(PART_MAIN, 11299);
    app.set_part_vnum(PART_WEAPON, 299);
    app.set_part_vnum(PART_HAIR, 100);

    assert_eq!(app.get_part_vnum(PART_MAIN), 11299);
    assert_eq!(app.get_part_vnum(PART_WEAPON), 299);
    assert_eq!(app.get_part_vnum(PART_HAIR), 100);

    app.reset_parts();

    assert_eq!(app.get_part_vnum(PART_MAIN), 0);
    assert_eq!(app.get_part_vnum(PART_WEAPON), 0);
    assert_eq!(app.get_part_vnum(PART_HAIR), 0);
}
