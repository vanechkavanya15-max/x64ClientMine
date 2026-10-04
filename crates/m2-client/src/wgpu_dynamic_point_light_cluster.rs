use bytemuck::{Pod, Zeroable};
use glam::Vec3;

pub const MAX_POINT_LIGHTS: usize = 16;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, PartialEq)]
pub struct PointLightData {
    pub position: [f32; 4], // xyz, w = active (1.0) or not (0.0)
    pub color: [f32; 4],    // rgb, a = intensity
    pub attenuation: [f32; 4], // constant, linear, quadratic, radius
}

impl Default for PointLightData {
    fn default() -> Self {
        Self {
            position: [0.0; 4],
            color: [0.0; 4],
            attenuation: [1.0, 0.0, 0.0, 0.0],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, PartialEq)]
pub struct PointLightUniforms {
    pub lights: [PointLightData; MAX_POINT_LIGHTS],
    pub light_count: u32,
    pub _padding: [u32; 3], // 16-byte alignment
}

impl Default for PointLightUniforms {
    fn default() -> Self {
        Self {
            lights: [PointLightData::default(); MAX_POINT_LIGHTS],
            light_count: 0,
            _padding: [0; 3],
        }
    }
}

pub struct DynamicPointLightCluster {
    pub uniforms: PointLightUniforms,
}

impl DynamicPointLightCluster {
    pub fn new() -> Self {
        Self {
            uniforms: PointLightUniforms::default(),
        }
    }

    pub fn clear(&mut self) {
        self.uniforms.light_count = 0;
        for light in self.uniforms.lights.iter_mut() {
            light.position[3] = 0.0;
        }
    }

    pub fn add_point_light(
        &mut self,
        position: Vec3,
        color: Vec3,
        intensity: f32,
        constant: f32,
        linear: f32,
        quadratic: f32,
        radius: f32,
    ) -> bool {
        if self.uniforms.light_count as usize >= MAX_POINT_LIGHTS {
            return false;
        }

        let idx = self.uniforms.light_count as usize;
        let light = &mut self.uniforms.lights[idx];

        light.position = [position.x, position.y, position.z, 1.0];
        light.color = [color.x, color.y, color.z, intensity];
        light.attenuation = [constant, linear, quadratic, radius];

        self.uniforms.light_count += 1;
        true
    }

    pub fn add_torch_light(&mut self, position: Vec3, intensity_fluctuation: f32) -> bool {
        // Warm orange color for torch
        let color = Vec3::new(1.0, 0.6, 0.2);
        // Flickering intensity
        let base_intensity = 1.2;
        let intensity = base_intensity + intensity_fluctuation;
        
        self.add_point_light(
            position,
            color,
            intensity,
            1.0,   // constant
            0.09,  // linear
            0.032, // quadratic
            600.0, // radius
        )
    }

    pub fn add_magic_flash_light(&mut self, position: Vec3, color: Vec3) -> bool {
        self.add_point_light(
            position,
            color,
            5.0,    // high intensity
            1.0,    // constant
            0.045,  // linear
            0.0075, // quadratic (less falloff over short distances, very bright)
            800.0,  // radius
        )
    }
}

// Replicates Blinn-Phong lighting logic on the CPU for testing
pub fn calculate_blinn_phong_lighting(
    light: &PointLightData,
    frag_pos: Vec3,
    normal: Vec3,
    view_dir: Vec3,
    shininess: f32,
) -> (Vec3, Vec3) {
    if light.position[3] == 0.0 {
        return (Vec3::ZERO, Vec3::ZERO);
    }

    let light_pos = Vec3::new(light.position[0], light.position[1], light.position[2]);
    let distance = frag_pos.distance(light_pos);
    let radius = light.attenuation[3];

    if distance > radius {
        return (Vec3::ZERO, Vec3::ZERO);
    }

    let light_dir = (light_pos - frag_pos).normalize_or_zero();
    let norm = normal.normalize_or_zero();
    let view = view_dir.normalize_or_zero();

    // Attenuation
    let constant = light.attenuation[0];
    let linear = light.attenuation[1];
    let quadratic = light.attenuation[2];
    
    // Attenuation factor
    let attenuation = 1.0 / (constant + linear * distance + quadratic * (distance * distance));
    
    // Smooth windowing to avoid hard cut at radius
    let windowing = (1.0 - (distance / radius).powi(4)).clamp(0.0, 1.0).powi(2);
    let final_attenuation = attenuation * windowing;

    let light_color = Vec3::new(light.color[0], light.color[1], light.color[2]);
    let intensity = light.color[3];

    // Diffuse (Lambertian)
    let diff = norm.dot(light_dir).max(0.0);
    let diffuse = light_color * diff * intensity * final_attenuation;

    // Specular (Blinn-Phong)
    let halfway_dir = (light_dir + view).normalize_or_zero();
    let spec = norm.dot(halfway_dir).max(0.0).powf(shininess);
    let specular = light_color * spec * intensity * final_attenuation;

    (diffuse, specular)
}
