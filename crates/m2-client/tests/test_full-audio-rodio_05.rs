#[path = "../src/rodio_sfx_player.rs"]
mod rodio_sfx_player;

use rodio_sfx_player::RodioSfxPlayer;

// A small valid WAV file representing silence
const DUMMY_WAV: &[u8] = &[
    0x52, 0x49, 0x46, 0x46, 0x24, 0x00, 0x00, 0x00, 0x57, 0x41, 0x56, 0x45, 0x66, 0x6d, 0x74, 0x20,
    0x10, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x44, 0xac, 0x00, 0x00, 0x88, 0x58, 0x01, 0x00,
    0x02, 0x00, 0x10, 0x00, 0x64, 0x61, 0x74, 0x61, 0x00, 0x00, 0x00, 0x00
];

#[test]
fn test_player_init() {
    let player = RodioSfxPlayer::new();
    assert!(player.is_ok());
}

#[test]
fn test_player_play_wav() {
    let mut player = RodioSfxPlayer::new().unwrap();
    let _id = player.play_wav_buffer(DUMMY_WAV);
    // Might return None in environments with no audio devices, so we just check it doesn't crash
}

#[test]
fn test_player_play_invalid_wav() {
    let mut player = RodioSfxPlayer::new().unwrap();
    let invalid_buffer = &[0, 1, 2, 3];
    let id = player.play_wav_buffer(invalid_buffer);
    assert!(id.is_none());
}

#[test]
fn test_player_set_volume() {
    let mut player = RodioSfxPlayer::new().unwrap();
    player.set_volume(0.5);
    player.set_volume(-1.0);
    player.set_volume(2.0);
}

#[test]
fn test_player_stop_all() {
    let mut player = RodioSfxPlayer::new().unwrap();
    let _ = player.play_wav_buffer(DUMMY_WAV);
    player.stop_all();
}

#[test]
fn test_player_stop_single() {
    let mut player = RodioSfxPlayer::new().unwrap();
    if let Some(id) = player.play_wav_buffer(DUMMY_WAV) {
        player.stop(id);
        assert!(!player.is_playing(id));
    }
}

#[test]
fn test_is_playing() {
    let mut player = RodioSfxPlayer::new().unwrap();
    let id = player.play_wav_buffer(DUMMY_WAV);
    if let Some(id) = id {
        let _ = player.is_playing(id);
    }
}
