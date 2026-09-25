//! Modul gr2_skeleton dla skrzynki m2-client-granny
//! Hierarchia szkieletu: drzewo kosci i macierze inverse bind pose
//! Struktura Bone, indeksy rodzica, macierze lokalne i globalne szkieletu

/// Reprezentacja macierzy 4x4 (column-major lub row-major, standardowo przechowujemy 16 elementow)
pub type Matrix4x4 = [f32; 16];

/// Funkcja pomocnicza do mnozenia macierzy 4x4
pub fn multiply_matrices(a: &Matrix4x4, b: &Matrix4x4) -> Matrix4x4 {
    let mut result = [0.0; 16];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i * 4 + j] += a[i * 4 + k] * b[k * 4 + j];
            }
        }
    }
    result
}

/// Zwraca macierz identycznosciowa
pub fn identity_matrix() -> Matrix4x4 {
    [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]
}

/// Struktura reprezentujaca pojedyncza kosc w szkielecie
#[derive(Debug, Clone, PartialEq)]
pub struct Bone {
    pub name: String,
    /// Indeks rodzica, -1 oznacza brak rodzica (kosc glowna)
    pub parent_index: i32,
    /// Lokalna transformacja kosci wzgledem rodzica
    pub local_transform: Matrix4x4,
    /// Macierz inverse bind pose uzywana przy skinningu
    pub inverse_bind_transform: Matrix4x4,
}

impl Bone {
    pub fn new(name: &str, parent_index: i32) -> Self {
        Self {
            name: name.to_string(),
            parent_index,
            local_transform: identity_matrix(),
            inverse_bind_transform: identity_matrix(),
        }
    }
}

/// Glowna struktura implementacyjna szkieletu
#[derive(Debug, Clone, Default)]
pub struct Gr2Skeleton {
    pub initialized: bool,
    pub bones: Vec<Bone>,
}

impl Gr2Skeleton {
    pub fn new() -> Self {
        Self {
            initialized: true,
            bones: Vec::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Dodaje kosc do szkieletu i zwraca jej indeks
    pub fn add_bone(&mut self, bone: Bone) -> usize {
        let index = self.bones.len();
        self.bones.push(bone);
        index
    }

    /// Zwraca ilosc kosci w szkielecie
    pub fn bone_count(&self) -> usize {
        self.bones.len()
    }

    /// Pobiera referencje do kosci po indeksie
    pub fn get_bone(&self, index: usize) -> Option<&Bone> {
        self.bones.get(index)
    }

    /// Pobiera mutowalna referencje do kosci po indeksie
    pub fn get_bone_mut(&mut self, index: usize) -> Option<&mut Bone> {
        self.bones.get_mut(index)
    }

    /// Oblicza globalna macierz transformacji kosci (uwzgledniajac hierarchie rodzicow)
    pub fn get_global_transform(&self, index: usize) -> Option<Matrix4x4> {
        if index >= self.bones.len() {
            return None;
        }

        let mut current_index = index as i32;
        let mut global_transform = identity_matrix();
        let mut visited = std::collections::HashSet::new();

        while current_index >= 0 {
            if !visited.insert(current_index) {
                // Wykryto cykl w hierarchii kosci, przerwij by uniknac nieskonczonej petli
                break;
            }

            if let Some(bone) = self.bones.get(current_index as usize) {
                global_transform = multiply_matrices(&bone.local_transform, &global_transform);
                current_index = bone.parent_index;
            } else {
                // Nieprawidlowy indeks rodzica, przerwij
                break;
            }
        }

        Some(global_transform)
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

    #[test]
    fn test_multiply_matrices() {
        let a = [
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 10.0, 11.0, 12.0,
            13.0, 14.0, 15.0, 16.0,
        ];
        let id = identity_matrix();
        let res = multiply_matrices(&a, &id);
        assert_eq!(res, a);
    }
}
