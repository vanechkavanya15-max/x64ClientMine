use std::collections::HashMap;

// Struktura przechowujaca informacje o umiejetnosci.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Skill {
    pub vnum: u32,
    pub level: u8,
    pub cooldown_ms: u32,
    pub mana_cost: u16,
}

// Struktura reprezentujaca zbior umiejetnosci danej postaci.
#[derive(Debug, Clone, Default)]
pub struct CharacterSkills {
    // Mapowanie vnum skilla na jego dane. Zapobiega to bledom indeksowania.
    pub skills: HashMap<u32, Skill>,
}

impl CharacterSkills {
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
        }
    }

    // Dodaje lub aktualizuje umiejetnosc dla postaci.
    pub fn set_skill(&mut self, skill: Skill) {
        self.skills.insert(skill.vnum, skill);
    }

    // Zwraca kopie danych umiejetnosci, o ile istnieje w mapie.
    pub fn get_skill(&self, vnum: u32) -> Option<Skill> {
        self.skills.get(&vnum).copied()
    }

    // Usuwa umiejetnosc o podanym vnum.
    pub fn remove_skill(&mut self, vnum: u32) {
        self.skills.remove(&vnum);
    }
}

// Menedzer zarzadzajacy umiejetnosciami dla wszystkich postaci.
#[derive(Debug, Clone, Default)]
pub struct SkillsManager {
    // Mapowanie VID na zestaw umiejetnosci postaci.
    pub characters: HashMap<u32, CharacterSkills>,
}

impl SkillsManager {
    pub fn new() -> Self {
        Self {
            characters: HashMap::new(),
        }
    }

    // Inicjalizuje nowa pusta pule umiejetnosci dla danego VID.
    pub fn add_character(&mut self, vid: u32) {
        self.characters.entry(vid).or_insert_with(CharacterSkills::new);
    }

    // Usuwa wszystkie umiejetnosci zwiazane z podanym VID.
    pub fn remove_character(&mut self, vid: u32) {
        self.characters.remove(&vid);
    }

    // Aktualizuje lub dodaje nowa umiejetnosc dla postaci o danym VID.
    // Jesli postac nie istnieje, tworzy dla niej nowy rekord zapobiegajac bledom.
    pub fn set_skill(&mut self, vid: u32, skill: Skill) {
        let char_skills = self.characters.entry(vid).or_insert_with(CharacterSkills::new);
        char_skills.set_skill(skill);
    }

    // Zwraca umiejetnosc postaci o podanym VID i vnum.
    // Odporne na nieistniejace VID oraz vnum - w obu przypadkach po prostu zwraca None.
    pub fn get_skill(&self, vid: u32, vnum: u32) -> Option<Skill> {
        self.characters.get(&vid).and_then(|char_skills| char_skills.get_skill(vnum))
    }

    // Usuwa umiejetnosc dla danego VID i vnum.
    pub fn remove_skill(&mut self, vid: u32, vnum: u32) {
        if let Some(char_skills) = self.characters.get_mut(&vid) {
            char_skills.remove_skill(vnum);
        }
    }
}
