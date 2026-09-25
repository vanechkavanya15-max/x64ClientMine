//! Modul gr2_bone_bindings dla skrzynki m2-client-granny
//! Wagi kosci wierzcholkow dla animacji szkieletowej (Skinning)
//! Mapowanie wierzcholkow do max 4 kosci, wagi skinningu, normalizacja do sumy 1.0

/// Maksymalna liczba kosci przypisanych do wierzcholka.
pub const MAX_BONES_PER_VERTEX: usize = 4;

/// Struktura reprezentujaca wiazanie pojedynczego wierzcholka z koscmi szkieletu.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VertexBoneData {
    pub bone_indices: [u32; MAX_BONES_PER_VERTEX],
    pub bone_weights: [f32; MAX_BONES_PER_VERTEX],
    pub bone_count: usize,
}

impl Default for VertexBoneData {
    fn default() -> Self {
        Self::new()
    }
}

impl VertexBoneData {
    /// Tworzy pusta strukture powiazania wierzcholka.
    pub fn new() -> Self {
        Self {
            bone_indices: [0; MAX_BONES_PER_VERTEX],
            bone_weights: [0.0; MAX_BONES_PER_VERTEX],
            bone_count: 0,
        }
    }

    /// Dodaje kosc z przypisana do niej waga.
    /// Zwraca blad jesli osiagnieto juz limit MAX_BONES_PER_VERTEX.
    pub fn add_bone(&mut self, bone_index: u32, weight: f32) -> Result<(), &'static str> {
        if self.bone_count >= MAX_BONES_PER_VERTEX {
            return Err("Przekroczono maksymalna liczbe kosci dla wierzcholka");
        }
        self.bone_indices[self.bone_count] = bone_index;
        self.bone_weights[self.bone_count] = weight;
        self.bone_count += 1;
        Ok(())
    }

    /// Normalizuje wagi, tak aby ich suma wynosila dokladnie 1.0.
    /// Jesli suma wag wynosi zero, ustawia wage pierwszej przypisanej kosci na 1.0.
    pub fn normalize_weights(&mut self) {
        if self.bone_count == 0 {
            return;
        }

        let mut sum = 0.0;
        for i in 0..self.bone_count {
            sum += self.bone_weights[i];
        }

        if sum == 0.0 {
            self.bone_weights[0] = 1.0;
            for i in 1..self.bone_count {
                self.bone_weights[i] = 0.0;
            }
        } else {
            for i in 0..self.bone_count {
                self.bone_weights[i] /= sum;
            }
        }
    }
}

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct Gr2BoneBindings {
    pub vertices: Vec<VertexBoneData>,
    pub initialized: bool,
}

impl Gr2BoneBindings {
    /// Tworzy nowa pusta strukture wiazan kosci.
    pub fn new() -> Self {
        Self { 
            vertices: Vec::new(),
            initialized: true 
        }
    }

    /// Tworzy nowa strukture z wstepnie zarezerwowana pojemnoscia (zwieksza wydajnosc).
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vertices: Vec::with_capacity(capacity),
            initialized: true
        }
    }

    /// Sprawdza czy struktura zostala zainicjalizowana.
    pub fn is_ready(&self) -> bool {
        self.initialized
    }
    
    /// Dodaje dane wiazan dla pojedynczego wierzcholka.
    pub fn add_vertex_binding(&mut self, binding: VertexBoneData) {
        self.vertices.push(binding);
    }
    
    /// Normalizuje wagi dla wszystkich wierzcholkow modelu.
    pub fn normalize_all(&mut self) {
        for vertex in &mut self.vertices {
            vertex.normalize_weights();
        }
    }
    
    /// Zwraca ilosc powiazan wierzcholkow.
    pub fn len(&self) -> usize {
        self.vertices.len()
    }
    
    /// Zwraca informacje czy kontener jest pusty.
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_bone_data_normalization() {
        let mut vertex = VertexBoneData::new();
        assert!(vertex.add_bone(1, 0.5).is_ok());
        assert!(vertex.add_bone(2, 0.5).is_ok());
        assert!(vertex.add_bone(3, 1.0).is_ok());
        
        vertex.normalize_weights();
        
        assert_eq!(vertex.bone_weights[0], 0.25);
        assert_eq!(vertex.bone_weights[1], 0.25);
        assert_eq!(vertex.bone_weights[2], 0.5);
        
        let sum: f32 = vertex.bone_weights.iter().sum();
        assert_eq!(sum, 1.0);
    }

    #[test]
    fn test_zero_weights_normalization() {
        let mut vertex = VertexBoneData::new();
        assert!(vertex.add_bone(5, 0.0).is_ok());
        assert!(vertex.add_bone(7, 0.0).is_ok());
        
        vertex.normalize_weights();
        
        assert_eq!(vertex.bone_weights[0], 1.0);
        assert_eq!(vertex.bone_weights[1], 0.0);
        
        let sum: f32 = vertex.bone_weights.iter().sum();
        assert_eq!(sum, 1.0);
    }

    #[test]
    fn test_max_bones_limit() {
        let mut vertex = VertexBoneData::new();
        assert!(vertex.add_bone(1, 0.1).is_ok());
        assert!(vertex.add_bone(2, 0.2).is_ok());
        assert!(vertex.add_bone(3, 0.3).is_ok());
        assert!(vertex.add_bone(4, 0.4).is_ok());
        
        // Piate dodanie powinno zwrocic blad
        assert!(vertex.add_bone(5, 0.5).is_err());
    }

    #[test]
    fn test_skeleton_gr2_bone_bindings() {
        let obj = Gr2BoneBindings::new();
        assert!(obj.is_ready());
        assert!(obj.is_empty());
    }
}
