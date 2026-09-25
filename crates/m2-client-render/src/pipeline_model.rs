//! Modul pipeline_model dla skrzynki m2-client-render
//! Potok renderowania postaci z GPU Skinningiem (do 4 kosci)
//! Format wierzcholka PNT+wagi, paleta macierzy transformacji kosci szkieletu na GPU

/// Wierzcholek ze wspolrzednymi PNT (Pozycja, Normalna, Tekstura) oraz wagami kosci.
/// Obsluguje do 4 kosci dla GPU skinningu.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VertexPNTW {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texcoord: [f32; 2],
    pub bone_indices: [u32; 4],
    pub bone_weights: [f32; 4],
}

impl Default for VertexPNTW {
    fn default() -> Self {
        Self {
            position: [0.0; 3],
            normal: [0.0; 3],
            texcoord: [0.0; 2],
            bone_indices: [0; 4],
            bone_weights: [0.0; 4],
        }
    }
}

/// Macierz transformacji 4x4
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4x4 {
    pub data: [f32; 16],
}

impl Default for Matrix4x4 {
    fn default() -> Self {
        let mut mat = Self { data: [0.0; 16] };
        mat.data[0] = 1.0;
        mat.data[5] = 1.0;
        mat.data[10] = 1.0;
        mat.data[15] = 1.0;
        mat
    }
}

/// Paleta macierzy przechowujaca macierze kosci (do 256 kosci) do wyslania na GPU.
#[derive(Debug, Clone)]
pub struct MatrixPalette {
    pub matrices: Vec<Matrix4x4>,
}

impl Default for MatrixPalette {
    fn default() -> Self {
        Self {
            matrices: vec![Matrix4x4::default(); 256],
        }
    }
}

/// Glowna struktura implementacyjna potoku modelu
#[derive(Debug, Clone, Default)]
pub struct PipelineModel {
    pub initialized: bool,
    pub vertices: Vec<VertexPNTW>,
    pub matrix_palette: MatrixPalette,
}

impl PipelineModel {
    pub fn new() -> Self {
        Self {
            initialized: true,
            vertices: Vec::new(),
            matrix_palette: MatrixPalette::default(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
    
    /// Dodaje nowy wierzcholek do bufora wierzcholkow
    pub fn add_vertex(&mut self, vertex: VertexPNTW) {
        self.vertices.push(vertex);
    }
    
    /// Zmienia macierz kosci o zadanym indeksie.
    /// Zwraca true jesli pomyslnie przypisano, false jesli index poza zasiegiem.
    pub fn set_bone_matrix(&mut self, index: usize, matrix: Matrix4x4) -> bool {
        if index < self.matrix_palette.matrices.len() {
            self.matrix_palette.matrices[index] = matrix;
            true
        } else {
            false
        }
    }
    
    /// Cysci wszystkie zbuforowane wierzcholki i resetuje palete macierzy
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.matrix_palette = MatrixPalette::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_pipeline_model() {
        let obj = PipelineModel::new();
        assert!(obj.is_ready());
    }
}
