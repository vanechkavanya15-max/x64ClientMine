use std::ffi::CString;
use std::sync::{Mutex, OnceLock};

use m2_client::sound_ffi::*;

// Globalna blokada dla wszystkich testow mutujacych stan modulu FFI.
// Zapewnia, ze testy wykonuja sie sekwencyjnie i nie wchodza sobie w droge.
static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn lock_tests() -> std::sync::MutexGuard<'static, ()> {
    TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

fn reset_audio_state() {
    let mut state = get_audio_state().lock().unwrap();
    *state = AudioState::new();
    state.is_initialized = true; // Zawsze traktujmy jako zainicjowany po resecie na potrzeby testow
}

#[test]
fn test_m2_audio_init() {
    let _guard = lock_tests();
    let mut state = get_audio_state().lock().unwrap();
    *state = AudioState::new(); // Stan poczatkowy niezainicjowany
    drop(state);

    let result = m2_audio_init();
    assert!(result, "Inicjalizacja powinna zwrocic true");

    let state = get_audio_state().lock().unwrap();
    assert!(state.is_initialized, "Stan powinien byc is_initialized = true");
}

#[test]
fn test_m2_audio_play_sound_null_ptr() {
    let _guard = lock_tests();
    reset_audio_state();

    let result_2d = m2_audio_play_sound_2d(std::ptr::null(), 1.0);
    assert!(!result_2d, "Odtwarzanie 2D powinno odrzucic null ptr");

    let result_3d = m2_audio_play_sound_3d(std::ptr::null(), 0.0, 0.0, 0.0, 1.0);
    assert!(!result_3d, "Odtwarzanie 3D powinno odrzucic null ptr");
    
    let result_bgm = m2_audio_play_bgm(std::ptr::null(), false);
    assert!(!result_bgm, "Odtwarzanie BGM powinno odrzucic null ptr");
}

#[test]
fn test_m2_audio_master_volume_calculation_2d() {
    let _guard = lock_tests();
    reset_audio_state();

    // Ustawienie master volume na 0.5
    m2_audio_set_master_volume(0.5);

    let sound_name = CString::new("sound/test.wav").unwrap();
    let result = m2_audio_play_sound_2d(sound_name.as_ptr(), 0.8);
    assert!(result, "Odtwarzanie dzwieku 2D powinno sie powiesc");

    let state = get_audio_state().lock().unwrap();
    assert_eq!(state.master_volume, 0.5);
    
    let (name, vol) = state.last_played_2d.as_ref().unwrap();
    assert_eq!(name, "sound/test.wav");
    assert_eq!(*vol, 0.4); // 0.8 * 0.5
}

#[test]
fn test_m2_audio_mute_all() {
    let _guard = lock_tests();
    reset_audio_state();

    m2_audio_mute_all(true);

    let sound_name = CString::new("sound/test.wav").unwrap();
    let result_2d = m2_audio_play_sound_2d(sound_name.as_ptr(), 1.0);
    assert!(!result_2d, "Dzwiek 2D nie powinien byc odtwarzany podczas mute");

    let result_3d = m2_audio_play_sound_3d(sound_name.as_ptr(), 10.0, 20.0, 30.0, 1.0);
    assert!(!result_3d, "Dzwiek 3D nie powinien byc odtwarzany podczas mute");
}

#[test]
fn test_m2_audio_bgm_lifecycle() {
    let _guard = lock_tests();
    reset_audio_state();

    // Odtworz BGM
    let bgm_name = CString::new("bgm/main.mp3").unwrap();
    let result = m2_audio_play_bgm(bgm_name.as_ptr(), true);
    assert!(result, "BGM powinno sie wlaczyc");

    {
        let state = get_audio_state().lock().unwrap();
        assert_eq!(state.current_bgm, Some("bgm/main.mp3".to_string()));
    }

    // Zastopuj BGM
    let result_stop = m2_audio_stop_bgm(true);
    assert!(result_stop, "BGM powinno sie poprawnie zatrzymac");

    {
        let state = get_audio_state().lock().unwrap();
        assert_eq!(state.current_bgm, None, "Stan powinien nie miec zdefiniowanego BGM");
    }
}

#[test]
fn test_m2_audio_volume_clamping() {
    let _guard = lock_tests();
    reset_audio_state();

    // Set master volume above 1.0 (should clamp to 1.0)
    m2_audio_set_master_volume(1.5);
    // Set category volume above 1.0 (should clamp to 1.0)
    m2_audio_set_category_volume(1, 2.0);

    let state = get_audio_state().lock().unwrap();
    assert_eq!(state.master_volume, 1.0, "Master volume powinen zostac obciety do 1.0");
    assert_eq!(state.category_volumes[1], 1.0, "Category volume powinen zostac obciety do 1.0");
}

#[test]
fn test_m2_audio_play_sound_3d_calculation() {
    let _guard = lock_tests();
    reset_audio_state();
    
    m2_audio_set_master_volume(0.8);
    
    let sound_name = CString::new("sound/test_3d.wav").unwrap();
    let result = m2_audio_play_sound_3d(sound_name.as_ptr(), 100.0, 200.0, 300.0, 0.5);
    assert!(result, "Odtwarzanie dzwieku 3D powinno sie powiesc");
    
    let state = get_audio_state().lock().unwrap();
    let (name, x, y, z, vol) = state.last_played_3d.as_ref().unwrap();
    
    assert_eq!(name, "sound/test_3d.wav");
    assert_eq!(*x, 100.0);
    assert_eq!(*y, 200.0);
    assert_eq!(*z, 300.0);
    assert_eq!(*vol, 0.4); // 0.8 * 0.5
}
