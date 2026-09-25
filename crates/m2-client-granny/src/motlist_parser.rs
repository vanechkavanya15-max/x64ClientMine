//! Modul motlist_parser dla skrzynki m2-client-granny
//! Parser rejestru ruchow postaci motlist.txt
//! Mapowanie trybow walki (WAIT, WALK, RUN, ATTACK_01, COMBO) na sciezki plikow .msa

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct MotlistParser {
    pub initialized: bool,
}

impl MotlistParser {
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
    fn test_skeleton_motlist_parser() {
        let obj = MotlistParser::new();
        assert!(obj.is_ready());
    }
}
