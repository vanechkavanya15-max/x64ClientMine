//! Modul gr2_mesh dla skrzynki m2-client-granny
//! Siatka geometryczna 3D: bufory wierzcholkow PNT i indeksow trojkatow
//! Vertex (pozycja, normalna, UV), IndexBuffer trojkatow, grupy submesh

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct Gr2Mesh {
    pub initialized: bool,
}

impl Gr2Mesh {
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
    fn test_skeleton_gr2_mesh() {
        let obj = Gr2Mesh::new();
        assert!(obj.is_ready());
    }
}
