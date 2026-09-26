#[derive(Debug, Clone)]
pub struct WarpTransition {
    pub velocity: [f32; 3],
    pub interpolation_queue: Vec<[f32; 3]>,
    pub map_base_x: i32,
    pub map_base_y: i32,
    pub local_pos: [f32; 3],
    pub physics_locked: bool,
}

impl WarpTransition {
    pub fn new() -> Self {
        Self {
            velocity: [0.0, 0.0, 0.0],
            interpolation_queue: Vec::new(),
            map_base_x: 0,
            map_base_y: 0,
            local_pos: [0.0, 0.0, 0.0],
            physics_locked: false,
        }
    }

    pub fn warp(&mut self, global_x: i32, global_y: i32, new_base_x: i32, new_base_y: i32, height: f32) {
        // Zerowanie biezacych wektorow predkosci i kolejki interpolacji
        self.velocity = [0.0, 0.0, 0.0];
        self.interpolation_queue.clear();

        // Zmiana wspolrzednych mapy bazowej
        self.map_base_x = new_base_x;
        self.map_base_y = new_base_y;

        // Ustawienie nowej pozycji lokalnej gracza
        self.local_pos = [
            (global_x - new_base_x) as f32,
            (global_y - new_base_y) as f32,
            height,
        ];

        // Odczekanie na zaladowanie kafelkow terenu przed odblokowaniem fizyki
        self.physics_locked = true;
    }

    pub fn on_terrain_loaded(&mut self) {
        self.physics_locked = false;
    }

    pub fn set_velocity(&mut self, vx: f32, vy: f32, vz: f32) {
        if !self.physics_locked {
            self.velocity = [vx, vy, vz];
        }
    }

    pub fn add_interpolation_point(&mut self, px: f32, py: f32, pz: f32) {
        if !self.physics_locked {
            self.interpolation_queue.push([px, py, pz]);
        }
    }

    pub fn is_physics_locked(&self) -> bool {
        self.physics_locked
    }
}

impl Default for WarpTransition {
    fn default() -> Self {
        Self::new()
    }
}
