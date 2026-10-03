use m2_client::item_anti_flags::{PlayerClass, PlayerSex};



use m2_client::actor_vocal_sound_trigger::{get_vocal_sound_file, VocalSoundCategory};

#[test]
fn test_warrior_male_emote() {
    assert_eq!(
        get_vocal_sound_file(PlayerClass::Warrior, PlayerSex::Male, VocalSoundCategory::Emote(1)),
        "sound/vocal/warrior_male/emote_1.wav"
    );
}

#[test]
fn test_shaman_female_battle() {
    assert_eq!(
        get_vocal_sound_file(PlayerClass::Shaman, PlayerSex::Female, VocalSoundCategory::BattleShout(5)),
        "sound/vocal/shaman_female/battle_5.wav"
    );
}
