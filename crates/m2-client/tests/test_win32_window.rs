use m2_client::win32_window::*;
use std::ptr;

#[test]
fn test_window_config_init() {
    let config = WindowConfig::default();
    assert_eq!(config.title, ptr::null());
    assert_eq!(config.width, 800);
    assert_eq!(config.height, 600);
    assert_eq!(config.is_fullscreen, false);
    assert_eq!(config.is_borderless, false);
}

#[cfg(not(windows))]
#[test]
fn test_create_windowed_mode() {
    let mut config = WindowConfig::default();
    let title = std::ffi::CString::new("Metin2").unwrap();
    config.title = title.as_ptr();
    config.width = 1024;
    config.height = 768;

    let hwnd = create_game_window(&config).expect("Failed to create window");
    assert!(!hwnd.is_null());

    let actual_style = _get_mock_window_style(hwnd).unwrap();
    assert_eq!((actual_style & WS_OVERLAPPEDWINDOW), WS_OVERLAPPEDWINDOW);

    let actual_title = _get_mock_window_title(hwnd).unwrap();
    assert_eq!(actual_title, "Metin2");
}

#[cfg(not(windows))]
#[test]
fn test_create_fullscreen_mode() {
    let mut config = WindowConfig::default();
    let title = std::ffi::CString::new("Metin2 Fullscreen").unwrap();
    config.title = title.as_ptr();
    config.width = 1920;
    config.height = 1080;
    config.is_fullscreen = true;

    let hwnd = create_game_window(&config).expect("Failed to create window");
    assert!(!hwnd.is_null());

    let actual_style = _get_mock_window_style(hwnd).unwrap();
    assert_eq!((actual_style & WS_POPUP), WS_POPUP);
    assert_eq!((actual_style & WS_VISIBLE), WS_VISIBLE);
    assert_eq!((actual_style & WS_OVERLAPPEDWINDOW), 0);

    let actual_title = _get_mock_window_title(hwnd).unwrap();
    assert_eq!(actual_title, "Metin2 Fullscreen");
}

#[cfg(not(windows))]
#[test]
fn test_set_window_fullscreen() {
    let mut config = WindowConfig::default();
    let title = std::ffi::CString::new("Dynamic Window").unwrap();
    config.title = title.as_ptr();
    
    let hwnd = create_game_window(&config).expect("Failed to create window");
    
    // Default should be windowed
    let style_windowed = _get_mock_window_style(hwnd).unwrap();
    assert_eq!((style_windowed & WS_OVERLAPPEDWINDOW), WS_OVERLAPPEDWINDOW);

    // Switch to fullscreen
    set_window_fullscreen(hwnd, true);
    let style_fullscreen = _get_mock_window_style(hwnd).unwrap();
    assert_eq!((style_fullscreen & WS_POPUP), WS_POPUP);
    assert_eq!((style_fullscreen & WS_VISIBLE), WS_VISIBLE);
    assert_eq!((style_fullscreen & WS_OVERLAPPEDWINDOW), 0);

    // Switch back to windowed
    set_window_fullscreen(hwnd, false);
    let style_windowed_again = _get_mock_window_style(hwnd).unwrap();
    assert_eq!((style_windowed_again & WS_OVERLAPPEDWINDOW), WS_OVERLAPPEDWINDOW);
    assert_eq!((style_windowed_again & WS_POPUP), 0);
}

#[cfg(not(windows))]
#[test]
fn test_set_window_title() {
    let mut config = WindowConfig::default();
    let initial_title = std::ffi::CString::new("Initial").unwrap();
    config.title = initial_title.as_ptr();
    
    let hwnd = create_game_window(&config).expect("Failed to create window");
    assert_eq!(_get_mock_window_title(hwnd).unwrap(), "Initial");

    set_window_title(hwnd, "New Metin2 Title");
    assert_eq!(_get_mock_window_title(hwnd).unwrap(), "New Metin2 Title");
}

#[test]
fn test_invalid_window_title() {
    let mut config = WindowConfig::default();
    config.title = ptr::null(); // Null title should be handled gracefully

    let hwnd = create_game_window(&config).expect("Failed to create window");
    assert!(!hwnd.is_null());

    #[cfg(not(windows))]
    {
        assert_eq!(_get_mock_window_title(hwnd).unwrap(), "");
    }
}
