//! Modul render_scene dla skrzynki m2-client-render
//! Glowny orkiestrator kolejki rysowania klatki sceny
//! Sortowanie obiektow (nieprzezroczyste przod-tyl, przezroczyste tyl-przod), statystyki klatki

/// Statystyki rysowania w danej klatce.
#[derive(Debug, Clone, Default)]
pub struct FrameStats {
    pub draw_calls: u32,
    pub rendered_vertices: u32,
    pub opaque_objects_count: u32,
    pub transparent_objects_count: u32,
}

/// Struktura reprezentujaca pojedynczy obiekt do wyrenderowania.
#[derive(Debug, Clone)]
pub struct RenderObject {
    pub id: u64,
    pub distance_sq: f32,
    pub is_transparent: bool,
    pub vertices: u32,
}

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct RenderScene {
    pub initialized: bool,
    pub opaque_queue: Vec<RenderObject>,
    pub transparent_queue: Vec<RenderObject>,
    pub stats: FrameStats,
}

impl RenderScene {
    pub fn new() -> Self {
        Self {
            initialized: true,
            opaque_queue: Vec::new(),
            transparent_queue: Vec::new(),
            stats: FrameStats::default(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Dodaje obiekt do odpowiedniej kolejki na podstawie przezroczystosci.
    pub fn add_object(&mut self, id: u64, distance_sq: f32, is_transparent: bool, vertices: u32) {
        let obj = RenderObject {
            id,
            distance_sq,
            is_transparent,
            vertices,
        };

        if is_transparent {
            self.transparent_queue.push(obj);
            self.stats.transparent_objects_count += 1;
        } else {
            self.opaque_queue.push(obj);
            self.stats.opaque_objects_count += 1;
        }
        
        self.stats.draw_calls += 1;
        self.stats.rendered_vertices += vertices;
    }

    /// Sortuje obie kolejki przygotowujac je do rysowania.
    /// Obiekty nieprzezroczyste (opaque) sa rysowane od najblizszego do najdalszego (przod-tyl).
    /// Obiekty przezroczyste (transparent) sa rysowane od najdalszego do najblizszego (tyl-przod).
    pub fn sort_queues(&mut self) {
        // Opaque: przod-tyl, rosnaco wedlug distance_sq
        self.opaque_queue.sort_by(|a, b| a.distance_sq.partial_cmp(&b.distance_sq).unwrap_or(std::cmp::Ordering::Equal));
        // Transparent: tyl-przod, malejaco wedlug distance_sq
        self.transparent_queue.sort_by(|a, b| b.distance_sq.partial_cmp(&a.distance_sq).unwrap_or(std::cmp::Ordering::Equal));
    }

    /// Czyci stan sceny przed rysowaniem kolejnej klatki.
    pub fn clear(&mut self) {
        self.opaque_queue.clear();
        self.transparent_queue.clear();
        self.stats = FrameStats::default();
    }

    /// Zwraca aktualne statystyki
    pub fn get_stats(&self) -> &FrameStats {
        &self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_render_scene() {
        let obj = RenderScene::new();
        assert!(obj.is_ready());
    }
}
