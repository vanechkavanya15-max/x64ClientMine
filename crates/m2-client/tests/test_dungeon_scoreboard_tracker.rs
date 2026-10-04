use std::ffi::CString;

#[path = "../src/dungeon_scoreboard_tracker.rs"]
mod dungeon_scoreboard_tracker;

use dungeon_scoreboard_tracker::{
    m2_dungeon_tracker_set_floor, m2_dungeon_tracker_set_objective,
    m2_dungeon_tracker_set_state, m2_dungeon_tracker_set_time_left,
    m2_dungeon_tracker_tick, DungeonState, get_tracker,
};

#[test]
fn test_dungeon_tracker_sequential() {
    // 1. Initial State
    {
        let tracker = get_tracker().lock().unwrap();
        assert_eq!(tracker.current_floor, 0);
        assert_eq!(tracker.objective_text, "");
        assert_eq!(tracker.objective_count, 0);
        assert_eq!(tracker.objective_total, 0);
        assert_eq!(tracker.time_left, 0);
        assert_eq!(tracker.state, DungeonState::Waiting);
    }

    // 2. Update Floor and Objective
    m2_dungeon_tracker_set_floor(5);
    
    let obj_text = CString::new("Destroy all metins").unwrap();
    m2_dungeon_tracker_set_objective(obj_text.as_ptr(), 3, 4);
    
    {
        let tracker = get_tracker().lock().unwrap();
        assert_eq!(tracker.current_floor, 5);
        assert_eq!(tracker.objective_text, "Destroy all metins");
        assert_eq!(tracker.objective_count, 3);
        assert_eq!(tracker.objective_total, 4);
    }

    // 3. Countdown Timer
    m2_dungeon_tracker_set_state(1); // InProgress
    m2_dungeon_tracker_set_time_left(10);
    
    {
        let tracker = get_tracker().lock().unwrap();
        assert_eq!(tracker.state, DungeonState::InProgress);
        assert_eq!(tracker.time_left, 10);
    }
    
    m2_dungeon_tracker_tick(3);
    
    {
        let tracker = get_tracker().lock().unwrap();
        assert_eq!(tracker.time_left, 7);
    }
    
    m2_dungeon_tracker_tick(8); // Tick beyond remaining time
    
    {
        let tracker = get_tracker().lock().unwrap();
        assert_eq!(tracker.time_left, 0);
        assert_eq!(tracker.state, DungeonState::Failure);
    }

    // 4. FFI Bindings State
    m2_dungeon_tracker_set_state(2); // FloorCleared
    {
        let tracker = get_tracker().lock().unwrap();
        assert_eq!(tracker.state, DungeonState::FloorCleared);
    }

    // Tick shouldn't affect time if not InProgress
    m2_dungeon_tracker_set_time_left(5);
    m2_dungeon_tracker_tick(1);
    
    {
        let tracker = get_tracker().lock().unwrap();
        assert_eq!(tracker.time_left, 5);
    }
}
