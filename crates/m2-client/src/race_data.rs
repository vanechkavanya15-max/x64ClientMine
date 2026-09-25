//! Modul `race_data` — Struktura definicji rasy i wygladu CRaceData (.msm)
//! Wzorzec referencyjny C++: `source/GameLib/RaceData.cpp oraz source/GameLib/RaceDataFile.cpp`
//! Opis: Odwzorowanie CRaceData: parsowanie skryptow modeli .msm (BaseModelFileName, ShapeData, HairData, AttachingBoneName dla broni i efektow).

#[derive(Debug, Clone, Default)]
pub struct RaceData {
    pub initialized: bool,
}

impl RaceData {
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
    fn test_skeleton_race_data() {
        let instance = RaceData::new();
        assert!(instance.is_ready());
    }
}
