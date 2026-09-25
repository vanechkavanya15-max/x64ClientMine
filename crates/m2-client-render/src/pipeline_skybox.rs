//! Modul pipeline_skybox dla skrzynki m2-client-render
//! Potok renderowania kopuly nieba (Sky Dome)
//! Siatka sferyczna nieba, gradient barwny pory dnia, rotacja tekstur chmur

use std::f32::consts::PI;

/// Reprezentacja koloru RGBA w przestrzeni liniowej
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Liniowa interpolacja miedzy dwoma kolorami
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0)
    }
}

/// Punkt kontrolny gradientu nieba
#[derive(Debug, Clone)]
pub struct ColorKeyframe {
    /// Czas w formacie 24-godzinnym (0.0 - 24.0)
    pub time: f32,
    pub color: Color,
}

/// Zarzadza kolorem nieba na podstawie pory dnia
#[derive(Debug, Clone, Default)]
pub struct SkyColorGradient {
    pub keyframes: Vec<ColorKeyframe>,
}

impl SkyColorGradient {
    pub fn new() -> Self {
        Self { keyframes: Vec::new() }
    }

    pub fn add_keyframe(&mut self, time: f32, color: Color) {
        let mut time = time % 24.0;
        if time < 0.0 {
            time += 24.0;
        }
        self.keyframes.push(ColorKeyframe { time, color });
        self.keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }

    /// Pobiera zinterpolowany kolor na podstawie pory dnia (0.0 - 24.0)
    pub fn get_color_at_time(&self, time: f32) -> Color {
        if self.keyframes.is_empty() {
            return Color::default();
        }
        if self.keyframes.len() == 1 {
            return self.keyframes[0].color;
        }

        let mut time = time % 24.0;
        if time < 0.0 {
            time += 24.0;
        }

        let mut next_idx = 0;
        for (i, keyframe) in self.keyframes.iter().enumerate() {
            if keyframe.time > time {
                next_idx = i;
                break;
            }
        }

        let (prev_idx, next_idx) = if next_idx == 0 {
            (self.keyframes.len() - 1, 0)
        } else {
            (next_idx - 1, next_idx)
        };

        let prev = &self.keyframes[prev_idx];
        let next = &self.keyframes[next_idx];

        let mut time_diff = next.time - prev.time;
        if time_diff <= 0.0 {
            time_diff += 24.0;
        }

        let mut t_diff = time - prev.time;
        if t_diff < 0.0 {
            t_diff += 24.0;
        }

        let t = t_diff / time_diff;
        prev.color.lerp(&next.color, t)
    }
}

/// Reprezentuje warstwe chmur na niebie
#[derive(Debug, Clone)]
pub struct CloudLayer {
    pub texture_id: u32,
    /// Kat obrotu w stopniach (0.0 - 360.0)
    pub rotation: f32,
    /// Predkosc obrotu w stopniach na sekunde
    pub rotation_speed: f32,
    pub scale: f32,
    pub height_offset: f32,
    pub color_tint: Color,
}

impl CloudLayer {
    pub fn new(texture_id: u32, rotation_speed: f32) -> Self {
        Self {
            texture_id,
            rotation: 0.0,
            rotation_speed,
            scale: 1.0,
            height_offset: 0.0,
            color_tint: Color::default(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.rotation += self.rotation_speed * delta_time;
        self.rotation %= 360.0;
        if self.rotation < 0.0 {
            self.rotation += 360.0;
        }
    }
}

/// Wierzcholek siatki kopuly nieba
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

/// Siatka sferyczna reprezentujaca kopule nieba
#[derive(Debug, Clone, Default)]
pub struct SkyDomeMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl SkyDomeMesh {
    /// Generuje siatke polsfery uzywana do renderowania nieba
    pub fn generate(radius: f32, rings: u32, segments: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Generowanie wierzcholkow
        for r in 0..=rings {
            // Theta przechodzi od 0 (czubek kopuly) do PI/2 (horyzont)
            let theta = (r as f32 / rings as f32) * (PI / 2.0);
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();
            
            // Wspolrzedna v (0 na czubku, 1 na horyzoncie)
            let v = r as f32 / rings as f32;

            for s in 0..=segments {
                // Phi przechodzi dookola osi Y (0 do 2*PI)
                let phi = (s as f32 / segments as f32) * 2.0 * PI;
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();

                let x = radius * sin_theta * cos_phi;
                let y = radius * cos_theta;
                let z = radius * sin_theta * sin_phi;
                
                // Wspolrzedna u (0 do 1 wkolo sfery)
                let u = s as f32 / segments as f32;

                vertices.push(Vertex {
                    position: [x, y, z],
                    uv: [u, v],
                });
            }
        }

        // Generowanie indeksow
        let verts_per_ring = segments + 1;
        for r in 0..rings {
            for s in 0..segments {
                let current = (r * verts_per_ring + s) as u16;
                let next = current + 1;
                let bottom = current + verts_per_ring as u16;
                let bottom_next = bottom + 1;

                // Pierwszy trojkat (current, bottom, next)
                indices.push(current);
                indices.push(bottom);
                indices.push(next);

                // Drugi trojkat (next, bottom, bottom_next)
                indices.push(next);
                indices.push(bottom);
                indices.push(bottom_next);
            }
        }

        Self { vertices, indices }
    }
}

/// Glowna struktura potoku kopuly nieba
#[derive(Debug, Clone, Default)]
pub struct PipelineSkybox {
    pub initialized: bool,
    pub time_of_day: f32, // 0.0 do 24.0
    pub current_color: Color,
    pub color_gradient: SkyColorGradient,
    pub cloud_layers: Vec<CloudLayer>,
    pub dome_mesh: SkyDomeMesh,
}

impl PipelineSkybox {
    pub fn new() -> Self {
        Self {
            initialized: true,
            time_of_day: 12.0,
            current_color: Color::default(),
            color_gradient: SkyColorGradient::new(),
            cloud_layers: Vec::new(),
            dome_mesh: SkyDomeMesh::generate(100.0, 16, 32),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
    
    pub fn set_time_of_day(&mut self, time: f32) {
        let mut t = time % 24.0;
        if t < 0.0 {
            t += 24.0;
        }
        self.time_of_day = t;
        self.current_color = self.color_gradient.get_color_at_time(self.time_of_day);
    }
    
    pub fn add_cloud_layer(&mut self, layer: CloudLayer) {
        self.cloud_layers.push(layer);
    }
    
    pub fn update(&mut self, delta_time: f32) {
        for layer in &mut self.cloud_layers {
            layer.update(delta_time);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_pipeline_skybox() {
        let obj = PipelineSkybox::new();
        assert!(obj.is_ready());
    }
}
