use bytemuck::{Pod, Zeroable};
use glam::{Vec3, Vec4};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct WaterUniforms {
    pub u_time: f32,
    pub u_wave_speed: f32,
    pub u_wave_strength: f32,
    pub _pad1: f32,
    pub u_water_color: [f32; 4],
    pub u_sun_direction: [f32; 4],
}

impl Default for WaterUniforms {
    fn default() -> Self {
        Self {
            u_time: 0.0,
            u_wave_speed: 1.0,
            u_wave_strength: 0.1,
            _pad1: 0.0,
            u_water_color: [0.2, 0.4, 0.6, 0.8],
            u_sun_direction: [0.0, -1.0, 0.0, 0.0],
        }
    }
}

pub const WATER_DISTORTION_SHADER: &str = r#"
struct WaterUniforms {
    time: f32,
    wave_speed: f32,
    wave_strength: f32,
    _pad: f32,
    water_color: vec4<f32>,
    sun_direction: vec4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: WaterUniforms;
@group(0) @binding(1) var dudv_map: texture_2d<f32>;
@group(0) @binding(2) var dudv_sampler: sampler;
@group(0) @binding(3) var normal_map: texture_2d<f32>;
@group(0) @binding(4) var normal_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) world_position: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0); // Assuming already in clip space or orthographic for now. To be proper, add view_proj matrix.
    out.uv = model.uv;
    out.world_position = model.position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let time = uniforms.time * uniforms.wave_speed;
    let dudv_uv = in.uv + vec2<f32>(time, time);
    
    let distortion = textureSample(dudv_map, dudv_sampler, dudv_uv).rg * 2.0 - 1.0;
    let distorted_uv = in.uv + distortion * uniforms.wave_strength;
    
    let normal_color = textureSample(normal_map, normal_sampler, distorted_uv).rgb;
    let normal = normalize(normal_color * 2.0 - 1.0);
    
    let sun_dir = normalize(uniforms.sun_direction.xyz);
    let view_dir = vec3<f32>(0.0, 0.0, 1.0); // simplified view dir
    let half_vec = normalize(sun_dir + view_dir);
    
    let specular = pow(max(dot(normal, half_vec), 0.0), 32.0);
    
    var final_color = uniforms.water_color.rgb + vec3<f32>(specular);
    return vec4<f32>(final_color, uniforms.water_color.a);
}
"#;

pub struct WgpuWaterDistortionPipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_buffer: wgpu::Buffer,
}

impl WgpuWaterDistortionPipeline {
    pub fn new(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Water Distortion Shader"),
            source: wgpu::ShaderSource::Wgsl(WATER_DISTORTION_SHADER.into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Water Distortion Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<WaterUniforms>() as _),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Water Distortion Pipeline Layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Water Distortion Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    Some(wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                        ],
                    })
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        let default_uniforms = WaterUniforms::default();
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Water Uniforms Buffer"),
            contents: bytemuck::cast_slice(&[default_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            pipeline,
            bind_group_layout,
            uniform_buffer,
        }
    }
    
    pub fn update_uniforms(&self, queue: &wgpu::Queue, uniforms: &WaterUniforms) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(std::slice::from_ref(uniforms)));
    }
}
