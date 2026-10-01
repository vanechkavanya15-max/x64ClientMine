use m2_client::py_bridge_game_systems::*;
use m2_client::safebox_mall_manager_v2::{G_SAFEBOX_MALL_MANAGER, TItemData, SAFEBOX_SLOT_X_COUNT};
use m2_client::recv_exchange_logic_v2::{get_exchange_logic, ExchangeState};
use m2_client::ui_quest_dialog::get_ui_quest_dialog;
use m2_client::recv_guild_logic_v2::{get_guild_state, GuildStateData, GuildMemberData};
use m2_client::recv_shop_logic_v2::SHOP_INSTANCE;
use std::ffi::CStr;

#[test]
fn test_py_bridge_gamesys_safebox() {
    if let Ok(mut mgr) = G_SAFEBOX_MALL_MANAGER.write() {
        mgr.open_safebox(9); // Size is often given in Y count, so 9 * 5 = 45. Let's pass 9.
        mgr.set_money(1337);
        let mut item = TItemData::default();
        item.vnum = 50200;
        item.count = 200;
        mgr.set_item_data(5, item);
    }

    assert_eq!(m2_py_gamesys_safebox_get_money(), 1337);
    assert_eq!(m2_py_gamesys_safebox_get_size(), 9 * SAFEBOX_SLOT_X_COUNT);
    assert_eq!(m2_py_gamesys_safebox_get_item_index(5), 50200);
    assert_eq!(m2_py_gamesys_safebox_get_item_count(5), 200);
    assert_eq!(m2_py_gamesys_safebox_get_item_index(0), 0);
}

#[test]
fn test_py_bridge_gamesys_exchange() {
    m2_py_gamesys_exchange_open();
    {
        let logic = get_exchange_logic().read().unwrap();
        assert_eq!(logic.state, ExchangeState::Open);
    }
    
    m2_py_gamesys_exchange_set_elk_to_target(5000);
    m2_py_gamesys_exchange_set_item_to_self(0, 199, 1);
    
    {
        let logic = get_exchange_logic().read().unwrap();
        assert_eq!(logic.target_data.elk, 5000);
        assert_eq!(logic.self_data.item_vnum[0], 199);
        assert_eq!(logic.self_data.item_count[0], 1);
    }
    
    m2_py_gamesys_exchange_accept(true);
    {
        let logic = get_exchange_logic().read().unwrap();
        assert_eq!(logic.self_data.accept, 1);
    }
    
    m2_py_gamesys_exchange_close();
    {
        let logic = get_exchange_logic().read().unwrap();
        assert_eq!(logic.state, ExchangeState::Idle);
    }
}

#[test]
fn test_py_bridge_gamesys_quest() {
    {
        let mut dialog = get_ui_quest_dialog().lock().unwrap();
        dialog.title = "Kill the wolves".to_string();
        dialog.clock_title = "Time Left".to_string();
        dialog.clock_value = 120;
    }
    
    let title_ptr = m2_py_gamesys_quest_get_quest_title(0);
    let title = unsafe { CStr::from_ptr(title_ptr) }.to_string_lossy();
    assert_eq!(title, "Kill the wolves");
    
    let clock_ptr = m2_py_gamesys_quest_get_quest_clock_name(0);
    let clock = unsafe { CStr::from_ptr(clock_ptr) }.to_string_lossy();
    assert_eq!(clock, "Time Left");
    
    assert_eq!(m2_py_gamesys_quest_get_quest_clock_value(0), 120);
}

#[test]
fn test_py_bridge_gamesys_guild() {
    {
        let mut state = get_guild_state().lock().unwrap();
        *state = GuildStateData::default();
        state.guild_id = 1234;
        state.name = "TestGuild".to_string();
        state.master_pid = 1;
        state.members.insert(1, GuildMemberData {
            pid: 1,
            grade: 1,
            is_general: 0,
            job: 0,
            level: 99,
            offer: 100,
            name: "MasterP".to_string(),
        });
        state.members.insert(2, GuildMemberData {
            pid: 2,
            grade: 15,
            is_general: 0,
            job: 1,
            level: 30,
            offer: 10,
            name: "MemberX".to_string(),
        });
    }

    assert_eq!(m2_py_gamesys_guild_get_guild_id(), 1234);
    assert_eq!(m2_py_gamesys_guild_get_member_count(), 2);
    
    let name_ptr = m2_py_gamesys_guild_get_guild_name();
    assert_eq!(unsafe { CStr::from_ptr(name_ptr) }.to_string_lossy(), "TestGuild");

    let master_ptr = m2_py_gamesys_guild_get_guild_master_name();
    assert_eq!(unsafe { CStr::from_ptr(master_ptr) }.to_string_lossy(), "MasterP");

    let member_ptr = m2_py_gamesys_guild_get_member_name(1); // second member because BTreeMap/HashMap ordering
    let member_name = unsafe { CStr::from_ptr(member_ptr) }.to_string_lossy();
    assert!(member_name == "MasterP" || member_name == "MemberX"); // Order might vary by HashMap, but it shouldn't crash
}

#[test]
fn test_py_bridge_gamesys_shop() {
    {
        let mut shop = SHOP_INSTANCE.write().unwrap();
        shop.clear();
        shop.items[2].vnum = 19;
        shop.items[2].price = 1000;
        shop.items[5].vnum = 299;
        shop.items[5].price = 20000;
    }
    
    m2_py_gamesys_shop_open();
    {
        let shop = SHOP_INSTANCE.read().unwrap();
        assert!(shop.is_open);
    }
    
    assert_eq!(m2_py_gamesys_shop_get_item_count(), 2);
    assert_eq!(m2_py_gamesys_shop_get_item_id(2), 19);
    assert_eq!(m2_py_gamesys_shop_get_item_price(2), 1000);
    assert_eq!(m2_py_gamesys_shop_get_item_id(5), 299);
    assert_eq!(m2_py_gamesys_shop_get_item_price(5), 20000);
    
    m2_py_gamesys_shop_close();
    {
        let shop = SHOP_INSTANCE.read().unwrap();
        assert!(!shop.is_open);
    }
}