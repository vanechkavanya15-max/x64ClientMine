use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3, Vec4};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, PartialEq)]
pub struct EnvironmentUniforms {
    pub skybox_view_proj: [[f32; 4]; 4],
    pub sun_direction: [f32; 4],
    pub ambient_color: [f32; 4],
    pub diffuse_color: [f32; 4],
    pub fog_color: [f32; 4],
    pub fog_params: [f32; 4], // [near_dist, far_dist, 0.0, 0.0]
}

impl Default for EnvironmentUniforms {
    fn default() -> Self {
        Self {
            skybox_view_proj: Mat4::IDENTITY.to_cols_array_2d(),
            sun_direction: [0.0, -1.0, 0.0, 0.0],
            ambient_color: [0.3, 0.3, 0.3, 1.0],
            diffuse_color: [0.7, 0.7, 0.7, 1.0],
            fog_color: [0.5, 0.5, 0.5, 1.0],
            fog_params: [1000.0, 5000.0, 0.0, 0.0],
        }
    }
}

pub struct SkyboxLightingSystem {
    pub uniforms: EnvironmentUniforms,
}

impl SkyboxLightingSystem {
    pub fn new() -> Self {
        Self {
            uniforms: EnvironmentUniforms::default(),
        }
    }

    /// Oblicza i aktualizuje macierz widoku dla Skyboxa (bez translacji) i mnozy przez projekcje.
    pub fn update_camera(&mut self, mut view: Mat4, proj: Mat4) {
        // Usuwamy translacje z macierzy widoku, aby skybox byl zawsze centrowany na kamerze
        view.w_axis = Vec4::new(0.0, 0.0, 0.0, 1.0);
        
        let view_proj = proj * view;
        self.uniforms.skybox_view_proj = view_proj.to_cols_array_2d();
    }

    /// Ustawia parametry mgly srodowiskowej
    pub fn update_fog(&mut self, near_dist: f32, far_dist: f32, color: [f32; 4]) {
        self.uniforms.fog_params[0] = near_dist;
        self.uniforms.fog_params[1] = far_dist;
        self.uniforms.fog_color = color;
    }

    /// Aktualizuje kierunek i kolory swiatel srodowiskowych
    pub fn update_lighting(&mut self, sun_dir: Vec3, ambient: [f32; 4], diffuse: [f32; 4]) {
        self.uniforms.sun_direction = [sun_dir.x, sun_dir.y, sun_dir.z, 0.0];
        self.uniforms.ambient_color = ambient;
        self.uniforms.diffuse_color = diffuse;
    }
}

/// Helper do liniowej interpolacji kolorow (np. miedzy dniem a noca, przy zmianie mgly)
pub fn interpolate_color(start: [f32; 4], end: [f32; 4], factor: f32) -> [f32; 4] {
    let t = factor.clamp(0.0, 1.0);
    [
        start[0] + (end[0] - start[0]) * t,
        start[1] + (end[1] - start[1]) * t,
        start[2] + (end[2] - start[2]) * t,
        start[3] + (end[3] - start[3]) * t,
    ]
}

/// Symuluje podstawowy cykl dnia i nocy zwracajac interpolowane swiatlo otoczenia, swiatlo glowne i wektor slonca.
/// `time_of_day` powinno byc z zakresu [0.0, 1.0].
pub fn calculate_day_night_cycle(
    time_of_day: f32,
    day_ambient: [f32; 4],
    night_ambient: [f32; 4],
    day_diffuse: [f32; 4],
    night_diffuse: [f32; 4],
) -> ([f32; 4], [f32; 4], Vec3) {
    let pi_2 = std::f32::consts::PI * 2.0;
    
    // Obliczamy wspolczynnik blendowania (1.0 = dzien, 0.0 = noc)
    // Czas 0.5 to srodek dnia
    let factor = (-f32::cos(time_of_day * pi_2) * 0.5 + 0.5).clamp(0.0, 1.0);

    let current_ambient = interpolate_color(night_ambient, day_ambient, factor);
    let current_diffuse = interpolate_color(night_diffuse, day_diffuse, factor);
    
    // Prosta symulacja toru slonca
    let angle = time_of_day * pi_2;
    let sun_x = f32::cos(angle);
    let sun_y = f32::sin(angle);
    let sun_z = 0.5; // Stale wzniesienie
    
    let sun_dir = Vec3::new(sun_x, sun_y, sun_z).normalize();
    
    (current_ambient, current_diffuse, sun_dir)
}
