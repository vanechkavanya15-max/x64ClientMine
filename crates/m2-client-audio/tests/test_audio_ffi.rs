use m2_client_audio::audio_ffi::*;
use std::ffi::CString;

#[test]
fn test_integration_audio_ffi() {
    // 1. Stan niezainicjowany
    reset_audio_state();
    let path = CString::new("test.wav").unwrap();
    
    assert_eq!(m2_audio_play_sound(path.as_ptr(), 0.0, 0.0, 0.0), -1);
    assert_eq!(m2_audio_play_bgm(path.as_ptr(), 1.0), -1);
    assert_eq!(m2_audio_set_volume(0.5), -1);
    
    // 2. Inicjalizacja i odtwarzanie dzwieku
    reset_audio_state();
    assert_eq!(m2_audio_init(), 0);
    assert_eq!(m2_audio_play_sound(path.as_ptr(), 1.0, 2.0, 3.0), 0);
    assert_eq!(m2_audio_play_sound(std::ptr::null(), 1.0, 2.0, 3.0), -1);
    
    let bad_path = b"bad\xFFpath\0";
    assert_eq!(m2_audio_play_sound(bad_path.as_ptr() as *const _, 0.0, 0.0, 0.0), -1);
    
    // 3. Odtwarzanie muzyki tla
    assert_eq!(m2_audio_play_bgm(path.as_ptr(), 0.5), 0);
    assert_eq!(m2_audio_play_bgm(std::ptr::null(), 0.5), -1);
    assert_eq!(m2_audio_play_bgm(path.as_ptr(), 1.5), -1); // Zbyt duza glosnosc
    assert_eq!(m2_audio_play_bgm(path.as_ptr(), -0.1), -1); // Ujemna glosnosc
    
    // 4. Ustawianie glosnosci
    assert_eq!(m2_audio_set_volume(0.75), 0);
    assert_eq!(get_current_volume(), 75);
    
    assert_eq!(m2_audio_set_volume(1.5), -1); // Zbyt duza glosnosc
    assert_eq!(m2_audio_set_volume(-0.5), -1); // Ujemna glosnosc
}
