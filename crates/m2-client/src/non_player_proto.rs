//! Modul `non_player_proto` — Parser i baza danych potworow oraz NPC CPythonNonPlayer (mob_proto)
//! Wzorzec referencyjny C++: `source/UserInterface/PythonNonPlayer.cpp`
//! Opis: Odwzorowanie CPythonNonPlayer i TMobTable: vnum potwora, nazwa, poziom, ranga (PAWN, KNIGHT, BOSS, KING), typ (MONSTER, NPC, STONE, WARP), maksymalne HP.

#[derive(Debug, Clone, Default)]
pub struct NonPlayerProto {
    pub initialized: bool,
}

impl NonPlayerProto {
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
    fn test_skeleton_non_player_proto() {
        let instance = NonPlayerProto::new();
        assert!(instance.is_ready());
    }
}
