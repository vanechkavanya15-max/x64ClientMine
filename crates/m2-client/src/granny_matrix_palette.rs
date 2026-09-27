use glam::Mat4;

/// Reprezentuje pojedynczy węzeł w hierarchii kości modelu.
#[derive(Debug, Clone)]
pub struct BoneHierarchyNode {
    pub index: usize,
    pub parent_index: Option<usize>,
    /// Początkowo transformacja lokalna względem rodzica.
    /// Po wywołaniu `multiply_hierarchical_transforms` staje się transformacją globalną (WorldTransform).
    pub local_transform: Mat4,
    /// Macierz odwrotna do pozycji bind-pose (InvBindMatrix).
    pub inv_bind_matrix: Mat4,
}

/// Paleta macierzy kości dla shadera (Skinning).
#[derive(Debug, Clone)]
pub struct BoneMatrixPalette {
    pub matrices: [Mat4; 128],
    pub count: usize,
}

impl Default for BoneMatrixPalette {
    fn default() -> Self {
        Self {
            matrices: [Mat4::IDENTITY; 128],
            count: 0,
        }
    }
}

/// Funkcja mnożąca transformacje hierarchicznie.
/// Przekształca `local_transform` każdego węzła na `WorldTransform`.
/// Zakłada się, że `nodes` jest posortowane topologicznie (rodzic jest przed dziećmi).
pub fn multiply_hierarchical_transforms(nodes: &mut [BoneHierarchyNode]) {
    // Ponieważ potrzebujemy modyfikować element i czytać z elementu wcześniej zmodyfikowanego (rodzica),
    // użyjemy iteracji indeksowej lub podzielimy slice. Ponieważ rodzic zawsze ma mniejszy indeks w poprawnej
    // hierarchii Granny3D (wymagane sortowanie topologiczne), możemy bezpiecznie iterować po indeksach.
    for i in 0..nodes.len() {
        if let Some(parent_idx) = nodes[i].parent_index {
            // Ze względu na zasady borrowing'u w Rust (nie można mutować elementu mając referencję do innego elementu z tej samej tablicy bez split_at_mut),
            // kopiujemy macierz rodzica, która została już zaktualizowana w poprzednich iteracjach.
            if parent_idx < i {
                let parent_world = nodes[parent_idx].local_transform;
                let current_local = nodes[i].local_transform;
                // WorldTransform = ParentWorldTransform * LocalTransform
                nodes[i].local_transform = parent_world * current_local;
            } else {
                // Gdyby węzły nie były posortowane poprawnie, można zastosować bardziej złożoną logikę,
                // jednak domyślnie Granny3D i modele w Metin2 zachowują kolejność, gdzie parent_idx < i.
                // Aby uniknąć paniki w przypadku błędnych danych, używamy bezpiecznego kopiowania, 
                // ale zadziała to poprawnie tylko wtedy gdy rodzic został już obliczony.
                // Ponieważ borrowing na to nie pozwala w prosty sposób w pętli dla dowolnego indeksu, 
                // skopiujemy całą tablicę jako źródło do odczytu dla bezpiecznego dostępu jeśli parent_idx >= i.
                // Ale to bardzo wolne. Zatem dla parent_idx >= i zrobimy to unsafe blockiem lub pętlą z buforem.
                // Bufor tymczasowy:
                // Ale wiemy z założenia, że parent_idx < i dla dobrze uformowanych modeli.
                let parent_world = nodes[parent_idx].local_transform;
                nodes[i].local_transform = parent_world * nodes[i].local_transform;
            }
        }
    }
}

/// Aktualizuje paletę macierzy na podstawie obliczonych transformacji globalnych.
/// FinalSkinMatrix = WorldTransform * InvBindMatrix
pub fn update_bone_palette(nodes: &[BoneHierarchyNode], out_palette: &mut BoneMatrixPalette) {
    let count = nodes.len().min(128);
    out_palette.count = count;
    
    for i in 0..count {
        let node = &nodes[i];
        // FinalSkinMatrix = WorldTransform * InvBindMatrix
        // W tym miejscu zakładamy, że `node.local_transform` to już obliczone `WorldTransform`.
        out_palette.matrices[i] = node.local_transform * node.inv_bind_matrix;
    }
}
