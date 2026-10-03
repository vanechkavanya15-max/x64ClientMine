use wgpu::util::DeviceExt;
use glam::{Mat4, Vec4, Vec3};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ViewProjectionMatrix {
    pub view_proj: [f32; 16],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightDir {
    pub direction: [f32; 3],
    pub _pad: u32,
    pub ambient: [f32; 4],
    pub diffuse: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FogParams {
    pub color: [f32; 4],
    pub density: f32,
    pub start: f32,
    pub end: f32,
    pub enable: u32,
}

pub struct PipelineCompleteContext {
    pub view_proj_buffer: wgpu::Buffer,
    pub light_dir_buffer: wgpu::Buffer,
    pub fog_params_buffer: wgpu::Buffer,
    pub shared_bind_group_layout: wgpu::BindGroupLayout,
    pub shared_bind_group: wgpu::BindGroup,
}

impl PipelineCompleteContext {
    pub fn new(device: &wgpu::Device) -> Self {
        let view_proj_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ViewProjectionMatrix Buffer"),
            size: std::mem::size_of::<ViewProjectionMatrix>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let light_dir_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("LightDir Buffer"),
            size: std::mem::size_of::<LightDir>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let fog_params_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("FogParams Buffer"),
            size: std::mem::size_of::<FogParams>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let shared_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Shared Pipeline Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<ViewProjectionMatrix>() as u64),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<LightDir>() as u64),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<FogParams>() as u64),
                    },
                    count: None,
                },
            ],
        });

        let shared_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Shared Pipeline Bind Group"),
            layout: &shared_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: view_proj_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: light_dir_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: fog_params_buffer.as_entire_binding(),
                },
            ],
        });

        Self {
            view_proj_buffer,
            light_dir_buffer,
            fog_params_buffer,
            shared_bind_group_layout,
            shared_bind_group,
        }
    }
}

pub fn create_skybox_pipeline(device: &wgpu::Device, format: wgpu::TextureFormat, shared_layout: &wgpu::BindGroupLayout) -> wgpu::RenderPipeline {
    create_generic_pipeline(device, format, shared_layout, "SkyBox Pipeline", wgpu::BlendState::REPLACE, true)
}

pub fn create_terrain_pipeline(device: &wgpu::Device, format: wgpu::TextureFormat, shared_layout: &wgpu::BindGroupLayout) -> wgpu::RenderPipeline {
    create_generic_pipeline(device, format, shared_layout, "Terrain Pipeline", wgpu::BlendState::REPLACE, true)
}

pub fn create_water_pipeline(device: &wgpu::Device, format: wgpu::TextureFormat, shared_layout: &wgpu::BindGroupLayout) -> wgpu::RenderPipeline {
    create_generic_pipeline(device, format, shared_layout, "Water Pipeline", wgpu::BlendState::ALPHA_BLENDING, false)
}

pub fn create_character_pipeline(device: &wgpu::Device, format: wgpu::TextureFormat, shared_layout: &wgpu::BindGroupLayout) -> wgpu::RenderPipeline {
    create_generic_pipeline(device, format, shared_layout, "Character Pipeline", wgpu::BlendState::REPLACE, true)
}

pub fn create_magic_effect_pipeline(device: &wgpu::Device, format: wgpu::TextureFormat, shared_layout: &wgpu::BindGroupLayout) -> wgpu::RenderPipeline {
    create_generic_pipeline(device, format, shared_layout, "Magic Effect Pipeline", wgpu::BlendState::ALPHA_BLENDING, false)
}

fn create_generic_pipeline(
    device: &wgpu::Device, 
    format: wgpu::TextureFormat, 
    shared_layout: &wgpu::BindGroupLayout, 
    label: &str,
    blend_state: wgpu::BlendState,
    depth_write: bool
) -> wgpu::RenderPipeline {
    
    // Fallback/dummy shader for compiling in test, real logic would load compiled spir-v or wgsl
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(r#"
            struct ViewProjection {
                view_proj: mat4x4<f32>,
            };
            @group(0) @binding(0) var<uniform> vp: ViewProjection;

            @vertex
            fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
                return vp.view_proj * vec4<f32>(0.0, 0.0, 0.0, 1.0);
            }

            @fragment
            fn fs_main() -> @location(0) vec4<f32> {
                return vec4<f32>(1.0, 1.0, 1.0, 1.0);
            }
        "#)),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(&format!("{} Layout", label)),
        bind_group_layouts: &[Some(shared_layout)],
        immediate_size: 0,
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[], // Would need actual vertex layouts in real usage
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(blend_state),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24Plus,
            depth_write_enabled: Some(depth_write),
            depth_compare: Some(wgpu::CompareFunction::LessEqual),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview_mask: None,
        cache: None,
    })
}
