//! Modul gr2_morph_targets dla skrzynki m2-client-granny
//! Obsluga celow ksztaltu (Morph Targets) i animacji mimiki
//! Przesuniecia wierzcholkow delta positions, wagi blendshape

use std::collections::HashMap;

/// Reprezentuje przesuniecie pojedynczego wierzcholka dla danego celu ksztaltu.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MorphVertexDelta {
    pub vertex_index: usize,
    pub delta_position: [f32; 3],
    pub delta_normal: [f32; 3],
}

/// Reprezentuje pojedynczy cel ksztaltu (blendshape) z nazwa, waga i lista przesuniec.
#[derive(Debug, Clone)]
pub struct MorphTarget {
    pub name: String,
    pub weight: f32,
    pub deltas: Vec<MorphVertexDelta>,
}

/// Wynik ewaluacji celow ksztaltu dla wierzcholka.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EvaluatedVertexDelta {
    pub delta_position: [f32; 3],
    pub delta_normal: [f32; 3],
}

/// Glowna struktura implementacyjna zarzadzajaca celami ksztaltu.
#[derive(Debug, Clone, Default)]
pub struct Gr2MorphTargets {
    pub initialized: bool,
    pub targets: Vec<MorphTarget>,
}

impl Gr2MorphTargets {
    /// Tworzy nowa instancje menedzera celow ksztaltu.
    pub fn new() -> Self {
        Self {
            initialized: true,
            targets: Vec::new(),
        }
    }

    /// Sprawdza, czy menedzer jest zainicjowany.
    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Dodaje nowy cel ksztaltu o zadanej nazwie i przesunieciach. Poczatkowa waga to 0.0.
    pub fn add_target(&mut self, name: String, deltas: Vec<MorphVertexDelta>) {
        self.targets.push(MorphTarget {
            name,
            weight: 0.0,
            deltas,
        });
    }

    /// Ustawia wage dla podanego celu ksztaltu. Waga jest ograniczona do przedzialu [0.0, 1.0].
    pub fn set_weight(&mut self, name: &str, weight: f32) -> Result<(), &'static str> {
        let clamped_weight = weight.clamp(0.0, 1.0);
        for target in &mut self.targets {
            if target.name == name {
                target.weight = clamped_weight;
                return Ok(());
            }
        }
        Err("Nie znaleziono celu ksztaltu o podanej nazwie")
    }

    /// Zwraca obecna wage podanego celu ksztaltu.
    pub fn get_weight(&self, name: &str) -> Option<f32> {
        for target in &self.targets {
            if target.name == name {
                return Some(target.weight);
            }
        }
        None
    }

    /// Oblicza skumulowane przesuniecia wierzcholkow na podstawie aktualnych wag celow ksztaltu.
    /// Zwraca mape, gdzie kluczem jest indeks wierzcholka, a wartoscia wynikowe przesuniecie.
    pub fn evaluate(&self) -> HashMap<usize, EvaluatedVertexDelta> {
        let mut result = HashMap::new();

        for target in &self.targets {
            if target.weight > 0.0 {
                let w = target.weight;
                for delta in &target.deltas {
                    let res = result.entry(delta.vertex_index).or_insert_with(EvaluatedVertexDelta::default);
                    res.delta_position[0] += delta.delta_position[0] * w;
                    res.delta_position[1] += delta.delta_position[1] * w;
                    res.delta_position[2] += delta.delta_position[2] * w;
                    
                    res.delta_normal[0] += delta.delta_normal[0] * w;
                    res.delta_normal[1] += delta.delta_normal[1] * w;
                    res.delta_normal[2] += delta.delta_normal[2] * w;
                }
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_gr2_morph_targets() {
        let obj = Gr2MorphTargets::new();
        assert!(obj.is_ready());
    }
}
