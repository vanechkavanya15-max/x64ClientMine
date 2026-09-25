use m2_client_audio::audio_types::{AudioParams, SoundCategory};

#[test]
fn test_sound_category_variants() {
    let ui = SoundCategory::UI;
    let effect = SoundCategory::EFFECT;
    let bgm = SoundCategory::BGM;
    let voice = SoundCategory::VOICE;

    assert_eq!(ui, SoundCategory::UI);
    assert_eq!(effect, SoundCategory::EFFECT);
    assert_eq!(bgm, SoundCategory::BGM);
    assert_eq!(voice, SoundCategory::VOICE);

    assert_ne!(ui, effect);
    assert_ne!(effect, bgm);
    assert_ne!(bgm, voice);
    assert_ne!(voice, ui);
}

#[test]
fn test_audio_params_fields() {
    let mut params = AudioParams::new();
    assert_eq!(params.volume, 1.0);
    assert_eq!(params.pan, 0.0);
    assert_eq!(params.pitch, 1.0);

    params.volume = 0.5;
    params.pan = -0.5;
    params.pitch = 1.2;

    assert_eq!(params.volume, 0.5);
    assert_eq!(params.pan, -0.5);
    assert_eq!(params.pitch, 1.2);
}

#[test]
fn test_audio_params_equality() {
    let params1 = AudioParams {
        volume: 0.5,
        pan: -0.5,
        pitch: 1.2,
    };
    let params2 = AudioParams {
        volume: 0.5,
        pan: -0.5,
        pitch: 1.2,
    };
    let params3 = AudioParams {
        volume: 1.0,
        pan: 0.0,
        pitch: 1.0,
    };

    assert_eq!(params1, params2);
    assert_ne!(params1, params3);
}
