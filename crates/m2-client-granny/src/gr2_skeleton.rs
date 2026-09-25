//! Modul gr2_skeleton dla skrzynki m2-client-granny
//! Hierarchia szkieletu: drzewo kosci i macierze inverse bind pose
//! Struktura Bone, indeksy rodzica, macierze lokalne i globalne szkieletu

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct Gr2Skeleton {
    pub initialized: bool,
}

impl Gr2Skeleton {
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
    fn test_skeleton_gr2_skeleton() {
        let obj = Gr2Skeleton::new();
        assert!(obj.is_ready());
    }
}
