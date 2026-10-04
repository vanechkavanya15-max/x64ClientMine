use glam::Vec3;
use rand::Rng;

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

        let mut rng = rand::thread_rng();
        let mut offset = Vec3::ZERO;
        
        // Zgodnie z C++ uzywamy losowych wartosci.
        // W C++ bylo to: int nShakeSize=10; switch (rand()%2) { case 0: v3Pos.x+=rand()%nShakeSize; break; ... }
        // Tutaj zmienna bedaca nShakeSize to amplitude, zachowujemy precyzje uzywajac f32.
        
        // Poniewaz w C++ bylo rand() (a to generowalo wstrzas FPS-zalezny), uzywamy delta_time do unormowania tego w frequency 
        // Tak wiec aby oddac wstrzas zachowujac fps independence mnozymy wychylenie zaleznie od uplywu czasu bazujac na frequency,
        // Albo (co jest lepsze) rzutujemy nowy drganie tylko kiedy minela jakas stala ilosc czasu. 
        // W tym module aby zachowac gladkosc i poprawic C++, mnozymy wychylenie (amplitude) przez delta i frequency.
        // Poniewaz C++ wywolywal to w update z uzyciem stalego nShakeSize (np. =10) to w klatkach bylo np `x += (rand() % 10)`.
        
        if self.amplitude > 0.0 {
            let shake_val = self.amplitude * (self.frequency.max(1.0) * delta_time_sec * 60.0).clamp(0.0, 1.0); // Normalizacja na 60 FPS aby zrownowazyc i dac poprawny efekt.
            
            if rng.gen_bool(0.5) {
                offset.x += rng.gen_range(0.0..=shake_val);
            } else {
                offset.x -= rng.gen_range(0.0..=shake_val);
            }

            if rng.gen_bool(0.5) {
                offset.y += rng.gen_range(0.0..=shake_val);
            } else {
                offset.y -= rng.gen_range(0.0..=shake_val);
            }

            if rng.gen_bool(0.5) {
                offset.z += rng.gen_range(0.0..=shake_val);
            } else {
                offset.z -= rng.gen_range(0.0..=shake_val);
            }
        }
        
        offset * damping_factor
    }

    pub fn apply_to(&mut self, eye_position: &mut Vec3, delta_time_sec: f32) {
        let offset = self.update(delta_time_sec);
        *eye_position += offset;
    }
}
