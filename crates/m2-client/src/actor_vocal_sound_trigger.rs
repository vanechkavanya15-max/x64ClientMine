use crate::item_anti_flags::{PlayerClass, PlayerSex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VocalSoundCategory {
    Emote(u32),
    BattleShout(u32),
}

pub fn get_vocal_sound_file(
    class: PlayerClass,
    sex: PlayerSex,
    category: VocalSoundCategory,
) -> String {
    let class_str = match class {
        PlayerClass::Warrior => "warrior",
        PlayerClass::Assassin => "assassin",
        PlayerClass::Sura => "sura",
        PlayerClass::Shaman => "shaman",
    };

    let sex_str = match sex {
        PlayerSex::Male => "male",
        PlayerSex::Female => "female",
    };

    match category {
        VocalSoundCategory::Emote(id) => {
            format!("sound/vocal/{}_{}/emote_{}.wav", class_str, sex_str, id)
        }
        VocalSoundCategory::BattleShout(id) => {
            format!("sound/vocal/{}_{}/battle_{}.wav", class_str, sex_str, id)
        }
    }
}
