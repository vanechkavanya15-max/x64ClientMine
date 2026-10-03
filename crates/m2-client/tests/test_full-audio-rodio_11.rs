use m2_client::item_anti_flags::{PlayerClass, PlayerSex};



use m2_client::actor_vocal_sound_trigger::{get_vocal_sound_file, VocalSoundCategory};

#[test]
fn test_sura_female_emote_edge_case() {
    assert_eq!(
        get_vocal_sound_file(PlayerClass::Sura, PlayerSex::Female, VocalSoundCategory::Emote(0)),
        "sound/vocal/sura_female/emote_0.wav"
    );
}

#[test]
fn test_assassin_male_battle_edge_case() {
    assert_eq!(
        get_vocal_sound_file(PlayerClass::Assassin, PlayerSex::Male, VocalSoundCategory::BattleShout(9999)),
        "sound/vocal/assassin_male/battle_9999.wav"
    );
}

#[test]
fn test_warrior_female_battle() {
    assert_eq!(
        get_vocal_sound_file(PlayerClass::Warrior, PlayerSex::Female, VocalSoundCategory::BattleShout(42)),
        "sound/vocal/warrior_female/battle_42.wav"
    );
}

#[test]
fn test_shaman_male_emote() {
    assert_eq!(
        get_vocal_sound_file(PlayerClass::Shaman, PlayerSex::Male, VocalSoundCategory::Emote(13)),
        "sound/vocal/shaman_male/emote_13.wav"
    );
}

#[test]
fn test_sura_male_battle() {
    assert_eq!(
        get_vocal_sound_file(PlayerClass::Sura, PlayerSex::Male, VocalSoundCategory::BattleShout(7)),
        "sound/vocal/sura_male/battle_7.wav"
    );
}
