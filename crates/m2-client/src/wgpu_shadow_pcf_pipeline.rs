use glam::{Mat4, Vec3};

pub const SHADOW_MAP_SIZE: u32 = 2048;

pub const SHADOW_PCF_WGSL: &str = r#"
struct ShadowPcfUniforms {
    light_view_proj_near: mat4x4<f32>,
    light_view_proj_far: mat4x4<f32>,
    cascade_splits: vec4<f32>, // x: near split, y: far split, z: blend zone size, w: padding
    bias: f32,
    _padding0: u32,
    _padding1: u32,
    _padding2: u32,
};

@group(0) @binding(0) var<uniform> uniforms: ShadowPcfUniforms;
@group(0) @binding(1) var shadow_map_near: texture_depth_2d;
@group(0) @binding(2) var shadow_map_far: texture_depth_2d;
@group(0) @binding(3) var shadow_sampler: sampler_comparison;

fn calculate_shadow_pcf_16tap(
    shadow_map: texture_depth_2d,
    shadow_sampler: sampler_comparison,
    shadow_coord: vec3<f32>,
    texel_size: vec2<f32>
) -> f32 {
    var shadow: f32 = 0.0;
    
    // 16-tap PCF (4x4)
    for (var y: i32 = -1; y <= 2; y++) {
        for (var x: i32 = -1; x <= 2; x++) {
            let offset = vec2<f32>(f32(x) - 0.5, f32(y) - 0.5) * texel_size;
            shadow += textureSampleCompareLevel(
                shadow_map,
                shadow_sampler,
                shadow_coord.xy + offset,
                shadow_coord.z
            );
        }
    }
    
    return shadow / 16.0;
}

fn get_shadow_factor(world_pos: vec3<f32>, view_z: f32, tex_dim: vec2<f32>) -> f32 {
    let near_split = uniforms.cascade_splits.x;
    let far_split = uniforms.cascade_splits.y;
    let blend_zone = uniforms.cascade_splits.z;
    
    // Near cascade
    if (view_z < near_split - blend_zone) {
        let light_space = uniforms.light_view_proj_near * vec4<f32>(world_pos, 1.0);
        let proj = light_space.xyz / light_space.w;
        let tex = vec2<f32>(proj.x * 0.5 + 0.5, -proj.y * 0.5 + 0.5);
        if (tex.x >= 0.0 && tex.x <= 1.0 && tex.y >= 0.0 && tex.y <= 1.0 && proj.z >= 0.0 && proj.z <= 1.0) {
            return calculate_shadow_pcf_16tap(shadow_map_near, shadow_sampler, vec3<f32>(tex, proj.z - uniforms.bias), 1.0 / tex_dim);
        }
    }
    
    // Blend zone
    if (view_z >= near_split - blend_zone && view_z < near_split) {
        let light_space_near = uniforms.light_view_proj_near * vec4<f32>(world_pos, 1.0);
        let proj_near = light_space_near.xyz / light_space_near.w;
        let tex_near = vec2<f32>(proj_near.x * 0.5 + 0.5, -proj_near.y * 0.5 + 0.5);
        
        let light_space_far = uniforms.light_view_proj_far * vec4<f32>(world_pos, 1.0);
        let proj_far = light_space_far.xyz / light_space_far.w;
        let tex_far = vec2<f32>(proj_far.x * 0.5 + 0.5, -proj_far.y * 0.5 + 0.5);
        
        var shadow_near = 1.0;
        if (tex_near.x >= 0.0 && tex_near.x <= 1.0 && tex_near.y >= 0.0 && tex_near.y <= 1.0 && proj_near.z >= 0.0 && proj_near.z <= 1.0) {
            shadow_near = calculate_shadow_pcf_16tap(shadow_map_near, shadow_sampler, vec3<f32>(tex_near, proj_near.z - uniforms.bias), 1.0 / tex_dim);
        }
        
        var shadow_far = 1.0;
        if (tex_far.x >= 0.0 && tex_far.x <= 1.0 && tex_far.y >= 0.0 && tex_far.y <= 1.0 && proj_far.z >= 0.0 && proj_far.z <= 1.0) {
            shadow_far = calculate_shadow_pcf_16tap(shadow_map_far, shadow_sampler, vec3<f32>(tex_far, proj_far.z - uniforms.bias), 1.0 / tex_dim);
        }
        
        let blend_factor = (view_z - (near_split - blend_zone)) / blend_zone;
        return mix(shadow_near, shadow_far, blend_factor);
    }
    
    // Far cascade
    if (view_z >= near_split && view_z < far_split) {
        let light_space = uniforms.light_view_proj_far * vec4<f32>(world_pos, 1.0);
        let proj = light_space.xyz / light_space.w;
        let tex = vec2<f32>(proj.x * 0.5 + 0.5, -proj.y * 0.5 + 0.5);
        if (tex.x >= 0.0 && tex.x <= 1.0 && tex.y >= 0.0 && tex.y <= 1.0 && proj.z >= 0.0 && proj.z <= 1.0) {
            return calculate_shadow_pcf_16tap(shadow_map_far, shadow_sampler, vec3<f32>(tex, proj.z - uniforms.bias), 1.0 / tex_dim);
        }
    }
    
    return 1.0; // Fully lit
}
"#;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ShadowPcfUniforms {
    pub light_view_proj_near: [f32; 16],
    pub light_view_proj_far: [f32; 16],
    pub cascade_splits: [f32; 4], // x: near, y: far, z: blend zone, w: padding
    pub bias: f32,
    pub _padding: [u32; 3],
}

impl Default for ShadowPcfUniforms {
    fn default() -> Self {
        Self {
            light_view_proj_near: Mat4::IDENTITY.to_cols_array(),
            light_view_proj_far: Mat4::IDENTITY.to_cols_array(),
            cascade_splits: [2500.0, 15000.0, 500.0, 0.0],
            bias: 0.005,
            _padding: [0; 3],
        }
    }
}

pub struct WgpuShadowPcfPipeline {
    pub uniforms: ShadowPcfUniforms,
    pub shadow_map_size: u32,
}

impl Default for WgpuShadowPcfPipeline {
    fn default() -> Self {
        Self {
            uniforms: ShadowPcfUniforms::default(),
            shadow_map_size: SHADOW_MAP_SIZE,
        }
    }
}

impl WgpuShadowPcfPipeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_depth_textures(
        &self,
        device: &wgpu::Device,
    ) -> (wgpu::Texture, wgpu::TextureView, wgpu::Texture, wgpu::TextureView, wgpu::Sampler) {
        let size_ext = wgpu::Extent3d {
            width: self.shadow_map_size,
            height: self.shadow_map_size,
            depth_or_array_layers: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: Some("Shadow Map Depth Texture"),
            size: size_ext,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };

        let tex_near = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Shadow Map Near Cascade"),
            ..desc
        });
        let view_near = tex_near.create_view(&wgpu::TextureViewDescriptor::default());

        let tex_far = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Shadow Map Far Cascade"),
            ..desc
        });
        let view_far = tex_far.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Shadow PCF Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });

        (tex_near, view_near, tex_far, view_far, sampler)
    }

    pub fn calculate_sun_light_matrix(target: Vec3, sun_dir: Vec3, shadow_distance: f32) -> Mat4 {
        let light_dir = sun_dir.normalize();
        
        // Similar to C++ terrain/shadow calculations:
        // Move eye backwards along light direction.
        let eye = target - light_dir * shadow_distance;
        let up = Vec3::new(0.0, 0.0, 1.0);
        
        let view = Mat4::look_at_rh(eye, target, up);
        
        // C++: D3DXMatrixOrthoRH(&matLightProj, 2550.0f, 2550.0f, 1.0f, 15000.0f);
        let radius = shadow_distance;
        let proj = Mat4::orthographic_rh(
            -radius, radius,
            -radius, radius,
            1.0, 15000.0,
        );
        
        proj * view
    }

    pub fn update_cascades(&mut self, target_near: Vec3, target_far: Vec3, sun_dir: Vec3) {
        let near_view_proj = Self::calculate_sun_light_matrix(target_near, sun_dir, 1250.0);
        let far_view_proj = Self::calculate_sun_light_matrix(target_far, sun_dir, 5000.0);
        
        self.uniforms.light_view_proj_near = near_view_proj.to_cols_array();
        self.uniforms.light_view_proj_far = far_view_proj.to_cols_array();
    }
}
