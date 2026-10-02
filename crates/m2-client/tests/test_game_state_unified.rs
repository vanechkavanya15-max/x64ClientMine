use m2_client::game_state_unified::*;
use std::sync::Arc;
use std::thread;

#[test]
fn test_player_stats() {
    let state = UnifiedGameState::new();
    assert_eq!(state.get_player_stat(5), Some(0)); // POINT_HP or similar default 0
    state.set_player_stat(5, 100);
    assert_eq!(state.get_player_stat(5), Some(100));
    assert_eq!(state.get_player_stat(POINT_MAX_NUM + 1), None);
}

#[test]
fn test_inventory_items() {
    let state = UnifiedGameState::new();
    
    let item = TItemData {
        vnum: 11299,
        count: 1,
        flags: 0,
        anti_flags: 0,
        alSockets: [0; 3],
        aAttr: [TPlayerItemAttribute::default(); 7],
    };

    state.set_item(10, item.clone());
    
    let inventory = state.inventory.read().unwrap();
    assert_eq!(inventory.items.get(&10).unwrap().vnum, 11299);
    drop(inventory);
    
    let removed = state.remove_item(10);
    assert_eq!(removed.unwrap().vnum, 11299);
    
    let inventory = state.inventory.read().unwrap();
    assert!(inventory.items.get(&10).is_none());
}

#[test]
fn test_actors() {
    let state = UnifiedGameState::new();
    
    let actor = ActorInstanceData {
        vid: 12345,
        x: 100.0,
        y: 200.0,
        z: 300.0,
        rotation: 45.0,
        name: "Wild Dog".to_string(),
    };

    state.add_actor(12345, actor.clone());
    
    let actors = state.actors.read().unwrap();
    assert_eq!(actors.get(&12345).unwrap().name, "Wild Dog");
    drop(actors);
    
    let removed = state.remove_actor(12345);
    assert_eq!(removed.unwrap().name, "Wild Dog");
    
    let actors = state.actors.read().unwrap();
    assert!(actors.get(&12345).is_none());
}

#[test]
fn test_chat_history() {
    let state = UnifiedGameState::new();
    
    for i in 0..600 {
        let mut chat = state.chat_history.write().unwrap();
        chat.push(format!("Message {}", i));
    }
    
    let chat = state.chat_history.read().unwrap();
    assert_eq!(chat.messages.len(), 500);
    assert_eq!(chat.messages.front().unwrap(), "Message 100");
    assert_eq!(chat.messages.back().unwrap(), "Message 599");
}

#[test]
fn test_concurrent_access() {
    let state = Arc::new(UnifiedGameState::new());
    
    let mut handles = vec![];
    
    // Writers
    for i in 0..10 {
        let state_clone = Arc::clone(&state);
        let handle = thread::spawn(move || {
            let item = TItemData {
                vnum: i as u32,
                count: 1,
                flags: 0,
                anti_flags: 0,
                alSockets: [0; 3],
                aAttr: [TPlayerItemAttribute::default(); 7],
            };
            state_clone.set_item(i, item);
            state_clone.set_player_stat(i, (i * 10) as i32);
        });
        handles.push(handle);
    }
    
    // Readers
    for _ in 0..10 {
        let state_clone = Arc::clone(&state);
        let handle = thread::spawn(move || {
            let _inventory = state_clone.inventory.read().unwrap();
            let _player = state_clone.player.read().unwrap();
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let inventory = state.inventory.read().unwrap();
    for i in 0..10 {
        assert_eq!(inventory.items.get(&i).unwrap().vnum, i as u32);
        assert_eq!(state.get_player_stat(i).unwrap(), (i * 10) as i32);
    }
}
