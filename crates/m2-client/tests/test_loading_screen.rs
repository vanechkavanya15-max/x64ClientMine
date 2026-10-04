use m2_client::ui_window_loading_progress_screen::LoadingScreen;
use m2_client::ui_loading_progress_bar::LoadingPhase;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_loading_screen_initialization() {
    let screen = LoadingScreen::new();
    assert_eq!(screen.get_progress(), 0.0);
    assert!(!screen.is_completed());
    assert_eq!(screen.background_image, "default_loading.jpg");
}

#[test]
fn test_loading_screen_set_map() {
    let mut screen = LoadingScreen::new();
    screen.set_map("m1");
    assert_eq!(screen.map_name, "m1");
    assert_eq!(screen.background_image, "m1_loading.jpg");
}

#[test]
fn test_loading_screen_hints() {
    let mut screen = LoadingScreen::new();
    let hints = vec!["Hint 1".to_string(), "Hint 2".to_string(), "Hint 3".to_string()];
    screen.load_hints(hints.clone());
    
    assert!(hints.contains(&screen.current_hint));
    
    screen.update_hint();
    assert!(hints.contains(&screen.current_hint));
}

#[test]
fn test_loading_screen_progress_update() {
    let mut screen = LoadingScreen::new();
    screen.set_phase(LoadingPhase::ResourceLoading);
    
    // Upewniamy sie, ze docelowy postep zostal ustawiony (w ui_loading_progress_bar jest to 50%)
    assert_eq!(screen.progress_bar.get_target_progress(), 50.0);
    
    // Sprawdzamy stan po krotkiej aktualizacji (przykladowy czas trwania ramki)
    screen.update(0.1); 
    assert!(screen.get_progress() > 0.0);
    
    screen.force_complete();
    assert_eq!(screen.get_progress(), 100.0);
    assert!(screen.is_completed());
}

#[test]
fn test_loading_screen_load_hints_from_file() {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(file, "Hint 1").unwrap();
    writeln!(file, "# Comment").unwrap();
    writeln!(file, "Hint 2").unwrap();
    writeln!(file, "   ").unwrap();
    writeln!(file, "// Another comment").unwrap();
    writeln!(file, "Hint 3").unwrap();
    
    let path = file.path().to_str().unwrap().to_string();
    
    let mut screen = LoadingScreen::new();
    screen.load_hints_from_file(&path).unwrap();
    
    assert_eq!(screen.hints.len(), 3);
    assert_eq!(screen.hints[0], "Hint 1");
    assert_eq!(screen.hints[1], "Hint 2");
    assert_eq!(screen.hints[2], "Hint 3");
    
    assert!(!screen.current_hint.is_empty());
}
