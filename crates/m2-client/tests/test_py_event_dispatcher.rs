use m2_client::py_event_dispatcher::*;
use serial_test::serial;

#[test]
fn dummy_test_to_make_sure_it_compiles() {
    assert!(true);
}

#[test]
#[serial]
fn test_dispatch_gc_player_points() {
    clear_call_log();
    dispatch_gc_player_points();
    let log = get_call_log();
    assert_eq!(log.len(), 1);
    assert_eq!(log[0], "game.RefreshStatus");
}

#[test]
#[serial]
fn test_dispatch_gc_item_set_or_del() {
    clear_call_log();
    dispatch_gc_item_set_or_del();
    let log = get_call_log();
    assert_eq!(log.len(), 1);
    assert_eq!(log[0], "game.RefreshInventory");
}

#[test]
#[serial]
fn test_dispatch_gc_command() {
    clear_call_log();
    dispatch_gc_command("dance1");
    let log = get_call_log();
    assert_eq!(log.len(), 2);
    assert_eq!(log[0], "ARGS: format='(s)' values='dance1'");
    assert_eq!(log[1], "game.BINARY_OnServerCommand");
}

#[test]
#[serial]
fn test_dispatch_gc_quest_info() {
    clear_call_log();
    dispatch_gc_quest_info(5, 42);
    let log = get_call_log();
    assert_eq!(log.len(), 2);
    assert_eq!(log[0], "ARGS: format='(ii)' values='5, 42'");
    assert_eq!(log[1], "game.OpenQuestWindow");
}

#[test]
#[serial]
fn test_dispatch_gc_shop_open() {
    clear_call_log();
    dispatch_gc_shop_open(1337);
    let log = get_call_log();
    assert_eq!(log.len(), 2);
    assert_eq!(log[0], "ARGS: format='(i)' values='1337'");
    assert_eq!(log[1], "game.OpenShop");
}

#[test]
#[serial]
fn test_dispatch_gc_chat() {
    clear_call_log();
    dispatch_gc_chat(1, "Hello world");
    let log = get_call_log();
    assert_eq!(log.len(), 2);
    assert_eq!(log[0], "ARGS: format='(is)' values='1, Hello world'");
    assert_eq!(log[1], "chat.AppendChat");
}

#[test]
#[serial]
fn test_dispatch_gc_command_empty() {
    clear_call_log();
    dispatch_gc_command("");
    let log = get_call_log();
    assert_eq!(log.len(), 2);
    assert_eq!(log[0], "ARGS: format='(s)' values=''");
    assert_eq!(log[1], "game.BINARY_OnServerCommand");
}

#[test]
#[serial]
fn test_dispatch_gc_chat_empty() {
    clear_call_log();
    dispatch_gc_chat(0, "");
    let log = get_call_log();
    assert_eq!(log.len(), 2);
    assert_eq!(log[0], "ARGS: format='(is)' values='0, '");
    assert_eq!(log[1], "chat.AppendChat");
}