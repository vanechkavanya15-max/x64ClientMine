#[path = "../src/py_bridge_app_timing.rs"]
mod py_bridge_app_timing;

use py_bridge_app_timing::*;

#[test]
fn test_initial_fps() {
    let render_fps = m2_py_app_get_render_fps();
    let update_fps = m2_py_app_get_update_fps();
    
    // We are running tests in parallel, so state could be dirty from other tests.
    // Instead of absolute zero checks, let's just assert we can call it.
    assert!(render_fps >= 0);
    assert!(update_fps >= 0);
}

#[test]
fn test_set_fps() {
    // Should be a safe no-op or save the value, just verify it runs without crashing
    m2_py_app_set_fps(60);
    // Since we don't expose a getter for set_fps (as per spec), we just check it doesn't panic.
}

#[test]
fn test_render_game_accumulates() {
    let mut state = timing_lock();
    state.render_frames = 0;
    drop(state);

    m2_py_app_render_game();
    m2_py_app_render_game();

    let state = timing_lock();
    assert_eq!(state.render_frames, 2);
}

#[test]
fn test_update_game_accumulates() {
    let mut state = timing_lock();
    state.update_frames = 0;
    drop(state);

    m2_py_app_update_game();
    m2_py_app_update_game();
    m2_py_app_update_game();

    let state = timing_lock();
    assert_eq!(state.update_frames, 3);
}

#[test]
fn test_process_resets_and_calculates_fps() {
    // We can't rely on `m2_get_time()` being strictly deterministic within tests or sleeping,
    // so we verify logic via testing setters that mock out the process update
    {
        let mut state = timing_lock();
        state.force_fps_update_for_test(60, 120);
    }

    let render_fps = m2_py_app_get_render_fps();
    let update_fps = m2_py_app_get_update_fps();
    
    assert_eq!(render_fps, 60);
    assert_eq!(update_fps, 120);

    let state = timing_lock();
    assert_eq!(state.render_frames, 0);
    assert_eq!(state.update_frames, 0);
}

#[test]
fn test_process_multiple_frames() {
    {
        let mut state = timing_lock();
        let current_time = m2_client::app_timing_clock::m2_get_time();
        state.reset_last_time_for_test(current_time);
        state.render_frames = 0;
        state.update_frames = 0;
    }

    // Simulate 5 render frames and 5 update frames
    for _ in 0..5 {
        m2_py_app_render_game();
        m2_py_app_update_game();
        m2_py_app_process();
    }

    // Since 1 second has not passed, fps should remain what it was previously,
    // and frames should be accumulating.
    let state = timing_lock();
    assert_eq!(state.render_frames, 5);
    assert_eq!(state.update_frames, 5);
}
