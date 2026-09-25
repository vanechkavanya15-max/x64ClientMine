//! Modul `skill_table` — Rejestr opisow i wzorow umiejetnosci CPythonSkill
//! Wzorzec referencyjny C++: `source/UserInterface/PythonSkill.cpp`
//! Opis: Odwzorowanie CPythonSkill: parsowanie tabeli skilli (id, nazwa, klasa postaci, typ aktywny/pasywny, koszt SP, zasieg, promien splash, nazwy ikon).

#[derive(Debug, Clone, Default)]
pub struct SkillTable {
    pub initialized: bool,
}

impl SkillTable {
    pub fn new() -> Self {
        Self { initialized: true }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_skill_table() {
        let instance = SkillTable::new();
        assert!(instance.is_ready());
    }
}
