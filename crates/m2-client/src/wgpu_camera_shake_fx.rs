use glam::Vec3;

#[derive(Debug, Clone)]
pub struct CameraShakeFx {
    pub duration_sec: f32,
    pub elapsed_time_sec: f32,
    pub amplitude: f32,
    pub frequency: f32,
    pub decay: f32, // Tłumienie - 0.0: brak tłumienia, >0.0: szybsze tłumienie
}

impl CameraShakeFx {
    pub fn new(duration_sec: f32, amplitude: f32, frequency: f32, decay: f32) -> Self {
        Self {
            duration_sec,
            elapsed_time_sec: 0.0,
            amplitude,
            frequency,
            decay,
        }
    }

    pub fn is_active(&self) -> bool {
        self.elapsed_time_sec < self.duration_sec
    }

    /// Oblicza nowe przesunięcie kamery w bieżącej klatce i aktualizuje czas.
    pub fn update(&mut self, delta_time_sec: f32) -> Vec3 {
        if !self.is_active() {
            return Vec3::ZERO;
        }

        self.elapsed_time_sec += delta_time_sec;

        if !self.is_active() {
            return Vec3::ZERO;
        }

        // Tłumienie liniowe lub wykładnicze, w C++ często jest uproszczone wygasanie.
        // Używamy prostego opadania (decay) od 1 do 0 przez cały czas trwania wstrząsu.
        let progress = (self.elapsed_time_sec / self.duration_sec).clamp(0.0, 1.0);
        let damping_factor = 1.0 - progress.powf(self.decay.max(0.1)); 

        if damping_factor <= f32::EPSILON {
            return Vec3::ZERO;
        }

        let time_val = self.elapsed_time_sec * self.frequency;

        // Użycie sumy sinusów do symulacji pseudolosowych drgań.
        // W C++ było: rand()%nShakeSize, my robimy coś przewidywalnego bez stateful random, 
        // ale o gęstej dystrybucji drgań dla lepszego wyglądu (harmonic sum).
        let offset_x = (time_val * 43.0).sin() * (time_val * 13.0).cos();
        let offset_y = (time_val * 47.0).sin() * (time_val * 17.0).cos();
        let offset_z = (time_val * 53.0).sin() * (time_val * 19.0).cos();

        let offset = Vec3::new(offset_x, offset_y, offset_z) * self.amplitude * damping_factor;

        offset
    }

    pub fn apply_to(&mut self, eye_position: &mut Vec3, delta_time_sec: f32) {
        let offset = self.update(delta_time_sec);
        *eye_position += offset;
    }
}
