
use m2_client::ui_sound_event_router::{UiSoundEvent, UiSoundEventRouter, RouterError};

use rodio::OutputStream;

use std::fs::{self, File};
use std::path::Path;

fn ensure_test_file(path: &str) {
    if !Path::new(path).exists() {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent).unwrap();
        }
        // Minimalny plik WAV aby dekoder nie rzucil bledem
        let mut file = File::create(path).unwrap();
        use std::io::Write;
        let wav_header: [u8; 44] = [
            0x52, 0x49, 0x46, 0x46, // "RIFF"
            0x24, 0x00, 0x00, 0x00, // ChunkSize
            0x57, 0x41, 0x56, 0x45, // "WAVE"
            0x66, 0x6d, 0x74, 0x20, // "fmt "
            0x10, 0x00, 0x00, 0x00, // Subchunk1Size
            0x01, 0x00,             // AudioFormat (PCM)
            0x01, 0x00,             // NumChannels
            0x44, 0xac, 0x00, 0x00, // SampleRate
            0x88, 0x58, 0x01, 0x00, // ByteRate
            0x02, 0x00,             // BlockAlign
            0x10, 0x00,             // BitsPerSample
            0x64, 0x61, 0x74, 0x61, // "data"
            0x00, 0x00, 0x00, 0x00, // Subchunk2Size
        ];
        file.write_all(&wav_header).unwrap();
    }
}

// In CI we might not have audio device. Let's create dummy handle or handle it gracefully.
fn get_stream_handle() -> Option<rodio::OutputStreamHandle> {
    if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
        Some(stream_handle)
    } else {
        None
    }
}

#[test]
fn test_router_mapping() {
    // Uzywamy czegokolwiek zeby stworzyc router byle mozna przetestowac sciezki.
    // Aby to zrobic zmienimy strumien na opcjonalny
    let router = UiSoundEventRouter::new_for_test();
    
    assert_eq!(router.get_sound_path(UiSoundEvent::ButtonClick), "sound/ui/click.wav");
    assert_eq!(router.get_sound_path(UiSoundEvent::OpenInventory), "sound/ui/open_inventory.wav");
    assert_eq!(router.get_sound_path(UiSoundEvent::PickYang), "sound/ui/money.wav");
    assert_eq!(router.get_sound_path(UiSoundEvent::DropItem), "sound/ui/drop.wav");
}

#[test]
fn test_router_play_button_click() {
    let mut router = UiSoundEventRouter::new_for_test();
    if let Some(stream_handle) = get_stream_handle() {
        router.set_stream_handle(stream_handle);
        ensure_test_file("sound/ui/click.wav");
        let result = router.play_sound(UiSoundEvent::ButtonClick);
        assert!(result.is_ok());
    } else {
        println!("Skipping test due to no audio device");
    }
}

#[test]
fn test_router_play_open_inventory() {
    let mut router = UiSoundEventRouter::new_for_test();
    if let Some(stream_handle) = get_stream_handle() {
        router.set_stream_handle(stream_handle);
        ensure_test_file("sound/ui/open_inventory.wav");
        let result = router.play_sound(UiSoundEvent::OpenInventory);
        assert!(result.is_ok());
    } else {
        println!("Skipping test due to no audio device");
    }
}

#[test]
fn test_router_play_pick_yang() {
    let mut router = UiSoundEventRouter::new_for_test();
    if let Some(stream_handle) = get_stream_handle() {
        router.set_stream_handle(stream_handle);
        ensure_test_file("sound/ui/money.wav");
        let result = router.play_sound(UiSoundEvent::PickYang);
        assert!(result.is_ok());
    } else {
        println!("Skipping test due to no audio device");
    }
}

#[test]
fn test_router_play_drop_item() {
    let mut router = UiSoundEventRouter::new_for_test();
    if let Some(stream_handle) = get_stream_handle() {
        router.set_stream_handle(stream_handle);
        ensure_test_file("sound/ui/drop.wav");
        let result = router.play_sound(UiSoundEvent::DropItem);
        assert!(result.is_ok());
    } else {
        println!("Skipping test due to no audio device");
    }
}

#[test]
fn test_router_file_not_found() {
    let mut router = UiSoundEventRouter::new_for_test();
    if let Some(stream_handle) = get_stream_handle() {
        router.set_stream_handle(stream_handle);
        // Upewniamy sie ze usuniemy plik jesli istnieje (na wypadek, zeby rzucilo blad)
        let path = "sound/ui/click.wav";
        let _ = fs::remove_file(path);
        
        let result = router.play_sound(UiSoundEvent::ButtonClick);
        assert_eq!(result.err(), Some(RouterError::FileNotFound));
    } else {
        println!("Skipping test due to no audio device");
    }
}
