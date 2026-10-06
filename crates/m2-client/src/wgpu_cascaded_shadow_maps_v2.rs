use std::mem;


pub const CSM_SHADOW_MAP_SIZE: u32 = 2048;
pub const CSM_CASCADE_COUNT: usize = 3;
pub const CSM_PCF_FILTER_SIZE: i32 = 4;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CsmUniforms {
    pub light_view_proj: [[f32; 16]; CSM_CASCADE_COUNT],
    pub cascade_distances: [f32; 4],
    pub light_dir: [f32; 4],
    pub pcf_enabled: u32,
    pub _padding: [u32; 3],
}

pub struct CascadedShadowMapV2 {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub uniform_buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub cascade_views: Vec<wgpu::TextureView>,
}

impl CascadedShadowMapV2 {
    pub fn new(device: &wgpu::Device) -> Self {
        let size = wgpu::Extent3d {
            width: CSM_SHADOW_MAP_SIZE,
            height: CSM_SHADOW_MAP_SIZE,
            depth_or_array_layers: CSM_CASCADE_COUNT as u32,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("CSM Depth Texture Array"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("CSM Array View"),
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            array_layer_count: Some(CSM_CASCADE_COUNT as u32),
            ..Default::default()
        });

        let mut cascade_views = Vec::new();
        for i in 0..CSM_CASCADE_COUNT as u32 {
            cascade_views.push(texture.create_view(&wgpu::TextureViewDescriptor {
                label: Some(&format!("CSM Cascade View {}", i)),
                dimension: Some(wgpu::TextureViewDimension::D2),
                base_array_layer: i,
                array_layer_count: Some(1),
                ..Default::default()
            }));
        }

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("CSM PCF Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("CSM Uniforms Buffer"),
            size: mem::size_of::<CsmUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("CSM Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(mem::size_of::<CsmUniforms>() as u64),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("CSM Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        Self {
            texture,
            view,
            sampler,
            uniform_buffer,
            bind_group_layout,
            bind_group,
            cascade_views,
        }
    }
}

pub const CSM_SHADER_SOURCE: &str = r#"
struct CsmUniforms {
    light_view_proj: array<mat4x4<f32>, 3>,
    cascade_distances: vec4<f32>,
    light_dir: vec4<f32>,
    pcf_enabled: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
};

@group(0) @binding(0) var<uniform> uniforms: CsmUniforms;
@group(0) @binding(1) var shadow_map: texture_depth_2d_array;
@group(0) @binding(2) var shadow_sampler: sampler_comparison;

fn get_cascade_index(view_z: f32) -> i32 {
    if (view_z < uniforms.cascade_distances.x) {
        return 0;
    } else if (view_z < uniforms.cascade_distances.y) {
        return 1;
    } else if (view_z < uniforms.cascade_distances.z) {
        return 2;
    }
    return -1;
}

fn calculate_csm_shadow(world_pos: vec3<f32>, view_z: f32, normal: vec3<f32>) -> f32 {
    let cascade_idx = get_cascade_index(view_z);
    if (cascade_idx < 0) {
        return 1.0;
    }

    let light_space_pos = uniforms.light_view_proj[cascade_idx] * vec4<f32>(world_pos, 1.0);
    let proj_coords = light_space_pos.xyz / light_space_pos.w;
    
    let tex_coords = vec2<f32>(
        proj_coords.x * 0.5 + 0.5,
        -proj_coords.y * 0.5 + 0.5
    );
    
    if (tex_coords.x < 0.0 || tex_coords.x > 1.0 || 
        tex_coords.y < 0.0 || tex_coords.y > 1.0 || 
        proj_coords.z < 0.0 || proj_coords.z > 1.0) {
        return 1.0;
    }
    
    let n_dot_l = max(dot(normal, uniforms.light_dir.xyz), 0.0);
    let bias = max(0.005 * (1.0 - n_dot_l), 0.001);
    let current_depth = proj_coords.z - bias;
    
    var shadow: f32 = 0.0;
    
    if (uniforms.pcf_enabled == 1u) {
        let texel_size = 1.0 / 2048.0;
        // 4x4 PCF
        for (var y: i32 = -1; y <= 2; y++) {
            for (var x: i32 = -1; x <= 2; x++) {
                let offset = vec2<f32>(f32(x), f32(y)) * texel_size;
                shadow += textureSampleCompare(
                    shadow_map, 
                    shadow_sampler, 
                    tex_coords + offset, 
                    cascade_idx,
                    current_depth
                );
            }
        }
        shadow /= 16.0;
    } else {
        shadow = textureSampleCompare(
            shadow_map, 
            shadow_sampler, 
            tex_coords, 
            cascade_idx,
            current_depth
        );
    }
    
    return shadow;
}
"#;

pub fn get_metin2_light_dir() -> glam::Vec3 {
    let target = glam::Vec3::new(0.0, 0.0, 0.0);
    // Mimicking the C++ logic:
    // v3Eye(v3Target.x - 1.732f * 1250.0f,
    //       v3Target.y - 1250.0f,
    //       v3Target.z + 2.0f * 1.732f * 1250.0f);
    let eye = glam::Vec3::new(
        target.x - 1.732 * 1250.0,
        target.y - 1250.0,
        target.z + 2.0 * 1.732 * 1250.0,
    );
    (target - eye).normalize()
}

pub fn update_cascades(
    camera_view: glam::Mat4,
    camera_proj: glam::Mat4,
    light_dir: glam::Vec3,
    cascade_splits: &[f32; CSM_CASCADE_COUNT + 1],
) -> [glam::Mat4; CSM_CASCADE_COUNT] {
    let mut light_view_projs = [glam::Mat4::IDENTITY; CSM_CASCADE_COUNT];
    
    let inv_camera = (camera_proj * camera_view).inverse();
    
    for i in 0..CSM_CASCADE_COUNT {
        let near_clip = cascade_splits[i];
        let far_clip = cascade_splits[i + 1];
        
        let mut frustum_corners = [
            glam::Vec3::new(-1.0, -1.0, -1.0),
            glam::Vec3::new( 1.0, -1.0, -1.0),
            glam::Vec3::new(-1.0,  1.0, -1.0),
            glam::Vec3::new( 1.0,  1.0, -1.0),
            glam::Vec3::new(-1.0, -1.0,  1.0),
            glam::Vec3::new( 1.0, -1.0,  1.0),
            glam::Vec3::new(-1.0,  1.0,  1.0),
            glam::Vec3::new( 1.0,  1.0,  1.0),
        ];

        // Hacky way to get the frustum slice:
        let proj_near = (camera_proj * glam::Vec4::new(0.0, 0.0, -near_clip, 1.0)).z / (camera_proj * glam::Vec4::new(0.0, 0.0, -near_clip, 1.0)).w;
        let proj_far = (camera_proj * glam::Vec4::new(0.0, 0.0, -far_clip, 1.0)).z / (camera_proj * glam::Vec4::new(0.0, 0.0, -far_clip, 1.0)).w;

        for j in 0..4 {
            frustum_corners[j].z = proj_near;
            frustum_corners[j + 4].z = proj_far;
        }

        let mut center = glam::Vec3::ZERO;
        for j in 0..8 {
            let pt = inv_camera * glam::Vec4::new(frustum_corners[j].x, frustum_corners[j].y, frustum_corners[j].z, 1.0);
            frustum_corners[j] = pt.truncate() / pt.w;
            center += frustum_corners[j];
        }
        center /= 8.0;

        let mut radius = 0.0f32;
        for j in 0..8 {
            let dist = (frustum_corners[j] - center).length();
            if dist > radius {
                radius = dist;
            }
        }
        
        // Ensure radius is snapped to avoid shimmering
        radius = (radius * 16.0).ceil() / 16.0;

        let max_extents = glam::Vec3::new(radius, radius, radius);
        let min_extents = -max_extents;
        
        let light_view = glam::Mat4::look_at_rh(
            center - light_dir * max_extents.z,
            center,
            glam::Vec3::Y,
        );
        
        // Calculate ortho matrix
        let mut light_proj = glam::Mat4::orthographic_rh(
            min_extents.x, max_extents.x,
            min_extents.y, max_extents.y,
            0.0, max_extents.z * 2.0,
        );
        
        // Stabilize cascaded shadow map (texel snapping)
        let shadow_matrix = light_proj * light_view;
        let shadow_origin = shadow_matrix * glam::Vec4::new(0.0, 0.0, 0.0, 1.0);
        let shadow_origin = shadow_origin.truncate() / shadow_origin.w;
        let shadow_origin_pixels = shadow_origin * (CSM_SHADOW_MAP_SIZE as f32 / 2.0);
        
        let rounded_origin = shadow_origin_pixels.round();
        let rounded_offset = (rounded_origin - shadow_origin_pixels) * (2.0 / CSM_SHADOW_MAP_SIZE as f32);
        
        light_proj.w_axis.x += rounded_offset.x;
        light_proj.w_axis.y += rounded_offset.y;

        light_view_projs[i] = light_proj * light_view;
    }
    
    light_view_projs
}
