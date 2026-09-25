//! Modul gr2_mesh dla skrzynki m2-client-granny
//! Siatka geometryczna 3D: bufory wierzcholkow PNT i indeksow trojkatow
//! Vertex (pozycja, normalna, UV), IndexBuffer trojkatow, grupy submesh

/// Reprezentuje pojedynczy wierzcholek w formacie PNT (Position, Normal, Texture)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: [0.0; 3],
            normal: [0.0; 3],
            uv: [0.0; 2],
        }
    }
}

/// Reprezentuje podsiatke (submesh) - czesc siatki powiazana z jednym materialem
#[derive(Debug, Clone, PartialEq)]
pub struct Submesh {
    pub start_index: u32,
    pub index_count: u32,
    pub material_index: u32,
}

/// Glowna struktura implementacyjna siatki 3D
#[derive(Debug, Clone, Default)]
pub struct Gr2Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub submeshes: Vec<Submesh>,
}

impl Gr2Mesh {
    /// Tworzy nowa, pusta siatke
    pub fn new() -> Self {
        Self::default()
    }

    /// Sprawdza czy siatka zawiera jakiekolwiek dane
    pub fn is_ready(&self) -> bool {
        !self.vertices.is_empty()
    }

    /// Dodaje wierzcholek do bufora wierzcholkow
    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    /// Dodaje indeks do bufora indeksow
    pub fn add_index(&mut self, index: u32) {
        self.indices.push(index);
    }

    /// Dodaje nowa podsiatke (submesh).
    /// Zwraca Result, aby zapewnic bezpieczenstwo - nie mozna dodac submesha,
    /// ktory odwoluje sie do indeksow poza buforem indeksow.
    pub fn add_submesh(&mut self, submesh: Submesh) -> Result<(), &'static str> {
        let max_index_needed = submesh.start_index.saturating_add(submesh.index_count);
        if max_index_needed as usize > self.indices.len() {
            return Err("Submesh odwoluje sie do indeksow poza rozmiarem bufora indeksow");
        }
        self.submeshes.push(submesh);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_gr2_mesh() {
        let mut obj = Gr2Mesh::new();
        assert!(!obj.is_ready());
        obj.add_vertex(Vertex::default());
        assert!(obj.is_ready());
    }
}
