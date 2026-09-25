//! Modul pipeline_terrain dla skrzynki m2-client-render
//! Potok renderowania terenu ze splattingiem 4 tekstur
//! Format wierzcholka terenu, bufor indeksow, mieszanie tekstur wg wag alfa

/// Struktura reprezentujaca pojedynczy wierzcholek terenu
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct TerrainVertex {
    /// Pozycja wierzcholka w przestrzeni 3D (x, y, z)
    pub position: [f32; 3],
    /// Wektor normalny wierzcholka dla oswietlenia (x, y, z)
    pub normal: [f32; 3],
    /// Koordynaty tekstury bazowej (u, v)
    pub tex_coord: [f32; 2],
    /// Wagi alfa dla maksymalnie 4 splatowanych tekstur
    pub alpha_weights: [f32; 4],
}

impl TerrainVertex {
    /// Tworzy nowy wierzcholek terenu
    pub fn new(
        position: [f32; 3],
        normal: [f32; 3],
        tex_coord: [f32; 2],
        alpha_weights: [f32; 4],
    ) -> Self {
        Self {
            position,
            normal,
            tex_coord,
            alpha_weights,
        }
    }
}

/// Struktura przechowujaca dane siatki terenu
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TerrainMesh {
    /// Lista wierzcholkow
    pub vertices: Vec<TerrainVertex>,
    /// Lista indeksow (trojkaty)
    pub indices: Vec<u32>,
}

impl TerrainMesh {
    /// Tworzy nowa, pusta siatke terenu
    pub fn new() -> Self {
        Self::default()
    }

    /// Dodaje wierzcholek do siatki
    pub fn add_vertex(&mut self, vertex: TerrainVertex) {
        self.vertices.push(vertex);
    }

    /// Dodaje indeks do siatki
    pub fn add_index(&mut self, index: u32) {
        self.indices.push(index);
    }

    /// Zwraca referencje do wierzcholka pod zadanym indeksem
    pub fn get_vertex(&self, index: usize) -> Option<&TerrainVertex> {
        self.vertices.get(index)
    }

    /// Cysci dane siatki
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }
}

/// Glowna struktura implementacyjna potoku terenu
#[derive(Debug, Clone, Default)]
pub struct PipelineTerrain {
    pub initialized: bool,
    pub mesh: TerrainMesh,
    pub textures: [Option<String>; 4],
}

impl PipelineTerrain {
    /// Tworzy nowy potok terenu
    pub fn new() -> Self {
        Self {
            initialized: true,
            mesh: TerrainMesh::new(),
            textures: [None, None, None, None],
        }
    }

    /// Sprawdza czy potok jest gotowy
    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Ustawia teksture w danym slocie (0-3)
    pub fn set_texture(&mut self, slot: usize, texture_path: String) -> Result<(), &'static str> {
        if slot < 4 {
            self.textures[slot] = Some(texture_path);
            Ok(())
        } else {
            Err("Nieprawidlowy slot tekstury (wymagany 0-3)")
        }
    }

    /// Pobiera sciezke do tekstury w danym slocie
    pub fn get_texture(&self, slot: usize) -> Option<&String> {
        if slot < 4 {
            self.textures[slot].as_ref()
        } else {
            None
        }
    }

    /// Dodaje wierzcholek terenu do siatki
    pub fn add_vertex(&mut self, vertex: TerrainVertex) {
        self.mesh.add_vertex(vertex);
    }

    /// Zwraca liczbe wierzcholkow w siatce
    pub fn vertex_count(&self) -> usize {
        self.mesh.vertices.len()
    }

    /// Zwraca liczbe indeksow w siatce
    pub fn index_count(&self) -> usize {
        self.mesh.indices.len()
    }

    /// Czysci caly potok (siatka i tekstury)
    pub fn clear(&mut self) {
        self.mesh.clear();
        for i in 0..4 {
            self.textures[i] = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_pipeline_terrain() {
        let obj = PipelineTerrain::new();
        assert!(obj.is_ready());
    }
}
