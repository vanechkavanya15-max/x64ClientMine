use glam::Mat4;

#[derive(Debug)]
pub struct ShadowConfig {
    pub texture_size: u32,
    pub format: wgpu::TextureFormat,
}

impl Default for ShadowConfig {
    fn default() -> Self {
        Self {
            texture_size: 2048,
            format: wgpu::TextureFormat::Depth32Float,
        }
    }
}

pub struct ShadowDepthTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub size: u32,
    pub light_vp_matrix: Mat4,
}

impl ShadowDepthTexture {
    pub fn new(device: &wgpu::Device, config: &ShadowConfig) -> Self {
        let size = wgpu::Extent3d {
            width: config.texture_size,
            height: config.texture_size,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some("Shadow Depth Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // Setup PCF comparison sampler for shadow mapping
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Shadow Sampler PCF"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
            size: config.texture_size,
            light_vp_matrix: Mat4::IDENTITY,
        }
    }

    pub fn set_light_vp_matrix(&mut self, matrix: Mat4) {
        self.light_vp_matrix = matrix;
    }

    pub fn get_light_vp_matrix(&self) -> Mat4 {
        self.light_vp_matrix
    }

    pub fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Shadow Bind Group Layout"),
            entries: &[
                // Shadow Uniforms
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<ShadowUniforms>() as u64),
                    },
                    count: None,
                },
                // Shadow Map Texture
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // Shadow Map Sampler (Comparison)
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                    count: None,
                },
            ],
        })
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ShadowUniforms {
    pub light_view_proj: [f32; 16],
    pub light_dir: [f32; 4], // w is padding
    pub bias: f32,
    pub pcf_enabled: u32,
    pub _padding: [u32; 2],
}

pub fn calculate_light_view_proj(light_dir: glam::Vec3, center: glam::Vec3, radius: f32) -> Mat4 {
    let light_dir = light_dir.normalize();
    let light_pos = center - light_dir * radius;
    let view = Mat4::look_at_rh(light_pos, center, glam::Vec3::Y);
    let proj = Mat4::orthographic_rh(
        -radius, radius,
        -radius, radius,
        0.1, radius * 2.0,
    );
    proj * view
}

pub const SHADOW_SHADER_SOURCE: &str = r#"
struct ShadowUniforms {
    light_view_proj: mat4x4<f32>,
    light_dir: vec4<f32>,
    bias: f32,
    pcf_enabled: u32,
    _padding: vec2<u32>,
};

@group(0) @binding(0) var<uniform> shadow_uniforms: ShadowUniforms;
@group(0) @binding(1) var shadow_map: texture_depth_2d;
@group(0) @binding(2) var shadow_sampler: sampler_comparison;

fn calculate_shadow(world_pos: vec3<f32>, normal: vec3<f32>) -> f32 {
    let light_space_pos = shadow_uniforms.light_view_proj * vec4<f32>(world_pos, 1.0);
    
    // Perspective divide (not strictly necessary for ortho, but good practice)
    let proj_coords = light_space_pos.xyz / light_space_pos.w;
    
    // Transform to texture space [0, 1]
    let tex_coords = vec2<f32>(
        proj_coords.x * 0.5 + 0.5,
        -proj_coords.y * 0.5 + 0.5
    );
    
    // Frustum check
    if (tex_coords.x < 0.0 || tex_coords.x > 1.0 || 
        tex_coords.y < 0.0 || tex_coords.y > 1.0 || 
        proj_coords.z < 0.0 || proj_coords.z > 1.0) {
        return 1.0; // Outside shadow map is lit
    }
    
    // Dynamic slope bias to fix shadow acne
    let n_dot_l = max(dot(normal, shadow_uniforms.light_dir.xyz), 0.0);
    let current_depth = proj_coords.z - max(shadow_uniforms.bias * (1.0 - n_dot_l), 0.001);
    
    var shadow: f32 = 0.0;
    
    if (shadow_uniforms.pcf_enabled == 1u) {
        // Simple 3x3 PCF filter
        let texel_size = 1.0 / vec2<f32>(textureDimensions(shadow_map));
        for (var y: i32 = -1; y <= 1; y++) {
            for (var x: i32 = -1; x <= 1; x++) {
                let offset = vec2<f32>(f32(x), f32(y)) * texel_size;
                shadow += textureSampleCompareLevel(
                    shadow_map, 
                    shadow_sampler, 
                    tex_coords + offset, 
                    current_depth
                );
            }
        }
        shadow /= 9.0;
    } else {
        shadow = textureSampleCompareLevel(shadow_map, shadow_sampler, tex_coords, current_depth);
    }
    
    return shadow;
}
"#;

pub struct ShadowPass<'a> {
    pub pass: wgpu::RenderPass<'a>,
}

impl<'a> ShadowPass<'a> {
    pub fn begin(encoder: &'a mut wgpu::CommandEncoder, target_view: &'a wgpu::TextureView) -> Self {
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Shadow Render Pass"),
            color_attachments: &[],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: target_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            ..Default::default()
        });

        Self { pass }
    }
}
