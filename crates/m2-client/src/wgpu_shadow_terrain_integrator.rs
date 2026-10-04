use glam::{Mat4, Vec3};

pub const TERRAIN_SHADOW_PCF_WGSL: &str = r#"
// PCF 3x3 filter snippet for terrain shadow evaluation
fn calculate_shadow_pcf(
    shadow_map: texture_depth_2d,
    shadow_sampler: sampler_comparison,
    shadow_coord: vec3<f32>,
    texel_size: vec2<f32>
) -> f32 {
    var shadow: f32 = 0.0;
    
    // Sample a 3x3 grid
    for (var y: i32 = -1; y <= 1; y++) {
        for (var x: i32 = -1; x <= 1; x++) {
            let offset = vec2<f32>(f32(x), f32(y)) * texel_size;
            let sample_pos = shadow_coord.xy + offset;
            shadow += textureSampleCompareLevel(
                shadow_map,
                shadow_sampler,
                sample_pos,
                shadow_coord.z
            );
        }
    }
    
    return shadow / 9.0;
}

fn get_terrain_shadow_factor(
    shadow_map: texture_depth_2d,
    shadow_sampler: sampler_comparison,
    light_view_proj: mat4x4<f32>,
    world_pos: vec3<f32>,
    tex_dim: vec2<f32>,
    bias: f32,
) -> f32 {
    let light_space_pos = light_view_proj * vec4<f32>(world_pos, 1.0);
    let proj_coords = light_space_pos.xyz / light_space_pos.w;
    
    // Transform from NDC to texture coordinates [0, 1]
    let tex_coords = vec2<f32>(
        proj_coords.x * 0.5 + 0.5,
        -proj_coords.y * 0.5 + 0.5
    );

    // Frustum check
    if (tex_coords.x < 0.0 || tex_coords.x > 1.0 || 
        tex_coords.y < 0.0 || tex_coords.y > 1.0 || 
        proj_coords.z < 0.0 || proj_coords.z > 1.0) {
        return 1.0; // Outside shadow map is fully lit
    }

    let shadow_coord = vec3<f32>(tex_coords.x, tex_coords.y, proj_coords.z - bias);
    let texel_size = 1.0 / tex_dim;

    return calculate_shadow_pcf(shadow_map, shadow_sampler, shadow_coord, texel_size);
}
"#;

pub struct ShadowTerrainIntegrator {
    pub light_view_matrix: Mat4,
    pub light_proj_matrix: Mat4,
}

impl Default for ShadowTerrainIntegrator {
    fn default() -> Self {
        Self {
            light_view_matrix: Mat4::IDENTITY,
            light_proj_matrix: Mat4::IDENTITY,
        }
    }
}

impl ShadowTerrainIntegrator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculates Light View & Proj matrices matching `CMapOutdoor::BeginRenderCharacterShadowToTexture` exactly.
    pub fn calculate_light_matrices(&mut self, target: Vec3) {
        // C++ reference:
        // D3DXVECTOR3 v3Eye(v3Target.x - 1.732f * 1250.0f,
        //                   v3Target.y - 1250.0f,
        //                   v3Target.z + 2.0f * 1.732f * 1250.0f);
        let eye = Vec3::new(
            target.x - 1.732 * 1250.0,
            target.y - 1250.0,
            target.z + 2.0 * 1.732 * 1250.0,
        );

        // C++ reference:
        // const auto vv = D3DXVECTOR3(0.0f, 0.0f, 1.0f);
        // D3DXMatrixLookAtRH(&matLightView, &v3Eye, &v3Target, &vv);
        let up = Vec3::new(0.0, 0.0, 1.0);
        self.light_view_matrix = Mat4::look_at_rh(eye, target, up);

        // C++ reference:
        // D3DXMatrixOrthoRH(&matLightProj, 2550.0f, 2550.0f, 1.0f, 15000.0f);
        let w = 2550.0;
        let h = 2550.0;
        let zn = 1.0;
        let zf = 15000.0;

        self.light_proj_matrix = Mat4::orthographic_rh(
            -w / 2.0, w / 2.0,
            -h / 2.0, h / 2.0,
            zn, zf,
        );
    }

    #[allow(dead_code)]
    pub fn get_light_view_proj_matrix(&self) -> Mat4 {
        self.light_proj_matrix * self.light_view_matrix
    }

    #[allow(dead_code)]
    pub fn create_bind_group_layout(
        device: &wgpu::Device,
        binding_texture: u32,
        binding_sampler: u32,
    ) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Shadow Terrain Integrator Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: binding_texture,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: binding_sampler,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                    count: None,
                },
            ],
        })
    }

    #[allow(dead_code)]
    pub fn create_bind_group(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        texture_view: &wgpu::TextureView,
        sampler: &wgpu::Sampler,
        binding_texture: u32,
        binding_sampler: u32,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Shadow Terrain Integrator Bind Group"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: binding_texture,
                    resource: wgpu::BindingResource::TextureView(texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: binding_sampler,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        })
    }
}
