use glam::Vec3;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PointLightData {
    pub position: [f32; 3],
    pub radius: f32,
    pub color: [f32; 3],
    pub attenuation0: f32,
    pub attenuation1: f32,
    pub attenuation2: f32,
    pub padding: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PointLightUniforms {
    pub lights: [PointLightData; 32],
    pub light_count: u32,
    pub padding: [u32; 3],
}

#[derive(Clone, Debug)]
pub struct PointLight {
    pub position: Vec3,
    pub color: Vec3,
    pub radius: f32,
    pub attenuation0: f32,
    pub attenuation1: f32,
    pub attenuation2: f32,
    pub is_dynamic_flare: bool,
    pub base_radius: f32,
}

impl PointLight {
    pub fn new(position: Vec3, color: Vec3, radius: f32, att0: f32, att1: f32, att2: f32) -> Self {
        Self {
            position,
            color,
            radius,
            attenuation0: att0,
            attenuation1: att1,
            attenuation2: att2,
            is_dynamic_flare: false,
            base_radius: radius,
        }
    }

    pub fn with_dynamic_flare(mut self, is_flare: bool) -> Self {
        self.is_dynamic_flare = is_flare;
        self
    }
}

pub struct DynamicPointLightCluster {
    lights: Vec<PointLight>,
    max_lights: usize,
}

impl DynamicPointLightCluster {
    pub fn new(max_lights: usize) -> Self {
        Self {
            lights: Vec::with_capacity(max_lights),
            max_lights: max_lights.min(32), // Hard clamp to match PointLightUniforms capacity
        }
    }

    pub fn add_light(&mut self, light: PointLight) -> Result<(), &'static str> {
        if self.lights.len() >= self.max_lights {
            return Err("Max lights reached");
        }
        self.lights.push(light);
        Ok(())
    }

    pub fn add_torch_light(&mut self, position: Vec3) -> Result<(), &'static str> {
        let light = PointLight::new(position, Vec3::new(1.0, 0.6, 0.2), 300.0, 1.0, 0.005, 0.0001)
            .with_dynamic_flare(true);
        self.add_light(light)
    }

    pub fn add_magic_flash_light(&mut self, position: Vec3, color: Vec3) -> Result<(), &'static str> {
        let light = PointLight::new(position, color, 500.0, 1.0, 0.01, 0.0005)
            .with_dynamic_flare(true);
        self.add_light(light)
    }

    pub fn clear_lights(&mut self) {
        self.lights.clear();
    }

    pub fn get_lights(&self) -> &[PointLight] {
        &self.lights
    }

    pub fn update_dynamic_flares(&mut self, time_seconds: f32) {
        for light in &mut self.lights {
            if light.is_dynamic_flare {
                // Simulate fire flicker: base radius +/- some percentage based on sine waves
                let flicker_1 = (time_seconds * 10.0).sin() * 0.1;
                let flicker_2 = (time_seconds * 25.0).sin() * 0.05;
                let flicker = flicker_1 + flicker_2;
                
                // Clamp radius modification to prevent going below zero or getting too crazy
                light.radius = (light.base_radius * (1.0 + flicker)).max(0.1);
            }
        }
    }

    pub fn get_uniform_data(&self) -> PointLightUniforms {
        let mut uniforms = PointLightUniforms::default();
        uniforms.light_count = self.lights.len() as u32;

        for (i, l) in self.lights.iter().enumerate() {
            uniforms.lights[i] = PointLightData {
                position: l.position.into(),
                radius: l.radius,
                color: l.color.into(),
                attenuation0: l.attenuation0,
                attenuation1: l.attenuation1,
                attenuation2: l.attenuation2,
                padding: [0.0; 2],
            };
        }
        
        uniforms
    }
}

pub fn calculate_blinn_phong_lighting(
    view_pos: Vec3,
    frag_pos: Vec3,
    normal: Vec3,
    lights: &[PointLight],
    ambient_color: Vec3,
    diffuse_color: Vec3,
    specular_color: Vec3,
    shininess: f32,
) -> Vec3 {
    let mut total_light = ambient_color;
    let view_dir = (view_pos - frag_pos).normalize_or_zero();
    let n = normal.normalize_or_zero();

    for light in lights {
        let light_dir = light.position - frag_pos;
        let distance = light_dir.length();
        
        if distance > light.radius {
            continue; // Outside radius
        }

        let l = light_dir / distance;
        
        // Attenuation based on Att0, Att1, Att2
        let att = 1.0 / (light.attenuation0 + light.attenuation1 * distance + light.attenuation2 * distance * distance);
        
        // Soft edge dropoff based on radius
        let falloff = 1.0 - (distance / light.radius).clamp(0.0, 1.0);
        let final_att = att * falloff;

        // Diffuse
        let diff = n.dot(l).max(0.0);
        let diffuse = diffuse_color * light.color * diff * final_att;

        // Specular (Blinn-Phong)
        let halfway_dir = (l + view_dir).normalize_or_zero();
        let spec = n.dot(halfway_dir).max(0.0).powf(shininess);
        let specular = specular_color * light.color * spec * final_att;

        total_light += diffuse + specular;
    }

    total_light.clamp(Vec3::ZERO, Vec3::ONE)
}
