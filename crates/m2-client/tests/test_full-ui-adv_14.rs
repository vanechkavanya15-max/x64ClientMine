#[path = "../src/ui_loading_progress_bar.rs"]
mod ui_loading_progress_bar;

use ui_loading_progress_bar::{LoadingProgressBar, LoadingPhase};

#[test]
fn test_initialization() {
    let bar = LoadingProgressBar::new();
    assert_eq!(bar.get_current_progress(), 0.0);
    assert_eq!(bar.get_target_progress(), 0.0);
    assert_eq!(bar.get_phase(), LoadingPhase::Init);
    assert_eq!(bar.is_fully_loaded(), false);
}

#[test]
fn test_phase_transitions() {
    let mut bar = LoadingProgressBar::new();
    
    bar.set_phase(LoadingPhase::MapLoading);
    assert_eq!(bar.get_phase(), LoadingPhase::MapLoading);
    assert_eq!(bar.get_target_progress(), 10.0);
    
    bar.set_phase(LoadingPhase::ResourceLoading);
    assert_eq!(bar.get_phase(), LoadingPhase::ResourceLoading);
    assert_eq!(bar.get_target_progress(), 50.0);
    
    bar.set_phase(LoadingPhase::Finishing);
    assert_eq!(bar.get_phase(), LoadingPhase::Finishing);
    assert_eq!(bar.get_target_progress(), 90.0);
}

#[test]
fn test_progress_clamping_and_adding() {
    let mut bar = LoadingProgressBar::new();
    
    // Add progress beyond 100
    bar.add_progress(150.0);
    assert_eq!(bar.get_target_progress(), 100.0); // clamped
    
    let mut bar2 = LoadingProgressBar::new();
    bar2.add_progress(30.0);
    assert_eq!(bar2.get_target_progress(), 30.0);
    
    // Test that shrinking target progress also clamps current progress if needed
    bar2.force_complete();
    assert_eq!(bar2.get_current_progress(), 100.0);
    bar2.set_target_progress(50.0);
    assert_eq!(bar2.get_current_progress(), 50.0);
}

#[test]
fn test_animation_update() {
    let mut bar = LoadingProgressBar::new().with_animation_speed(10.0);
    bar.set_target_progress(20.0);
    
    // Update by 1 second -> should add 10.0 to current progress
    bar.update(1.0);
    assert_eq!(bar.get_current_progress(), 10.0);
    
    // Update by 0.5 seconds -> should add 5.0
    bar.update(0.5);
    assert_eq!(bar.get_current_progress(), 15.0);
    
    // Update by 2 seconds -> should try to add 20.0, but clamp at target 20.0
    bar.update(2.0);
    assert_eq!(bar.get_current_progress(), 20.0);
}

#[test]
fn test_complete_status() {
    let mut bar = LoadingProgressBar::new();
    
    assert_eq!(bar.is_fully_loaded(), false);
    
    bar.set_phase(LoadingPhase::Complete);
    // Even if phase is complete, current_progress needs to be 100
    bar.update(1.0); // Wait for animation
    
    // Force completion for test
    bar.force_complete();
    assert_eq!(bar.is_fully_loaded(), true);
    assert_eq!(bar.get_current_progress(), 100.0);
    assert_eq!(bar.get_target_progress(), 100.0);
    assert_eq!(bar.get_phase(), LoadingPhase::Complete);
}
