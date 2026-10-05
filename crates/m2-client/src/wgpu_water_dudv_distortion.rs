use bytemuck::{Pod, Zeroable};

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, Pod, Zeroable, PartialEq)]
pub struct WaterDudvUniforms {
    pub time: f32,
    pub wave_speed: f32,
    pub wave_scale: f32,
    pub refraction_strength: f32,
    
    pub wind_dir_1: [f32; 2],
    pub wind_dir_2: [f32; 2],
    
    pub sun_direction: [f32; 4],
    pub water_color_shallow: [f32; 4],
    pub water_color_deep: [f32; 4],
    
    pub water_depth_factor: f32,
    pub _pad: [f32; 3],
}

impl Default for WaterDudvUniforms {
    fn default() -> Self {
        Self {
            time: 0.0,
            wave_speed: 1.0,
            wave_scale: 0.1,
            refraction_strength: 0.05,
            wind_dir_1: [1.0, 0.5],
            wind_dir_2: [-0.5, 1.0],
            sun_direction: [0.0, 0.707, 0.707, 0.0],
            water_color_shallow: [0.3, 0.6, 0.8, 0.7],
            water_color_deep: [0.1, 0.2, 0.5, 0.9],
            water_depth_factor: 10.0,
            _pad: [0.0; 3],
        }
    }
}

pub fn calculate_wave_offset(time: f32, speed: f32, wind_dir: [f32; 2]) -> [f32; 2] {
    let offset_x = wind_dir[0] * time * speed;
    let offset_y = wind_dir[1] * time * speed;
    [offset_x, offset_y]
}

pub const WATER_DUDV_SHADER: &str = r#"
struct WaterUniforms {
    time: f32,
    wave_speed: f32,
    wave_scale: f32,
    refraction_strength: f32,
    wind_dir_1: vec2<f32>,
    wind_dir_2: vec2<f32>,
    sun_direction: vec4<f32>,
    water_color_shallow: vec4<f32>,
    water_color_deep: vec4<f32>,
    water_depth_factor: f32,
    _pad: vec3<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: WaterUniforms;
@group(0) @binding(1) var dudv_map: texture_2d<f32>;
@group(0) @binding(2) var dudv_sampler: sampler;
@group(0) @binding(3) var depth_map: texture_2d<f32>;
@group(0) @binding(4) var depth_sampler: sampler;
@group(0) @binding(5) var normal_map: texture_2d<f32>;
@group(0) @binding(6) var normal_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) screen_pos: vec4<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.uv = model.uv;
    out.world_position = model.position;
    out.screen_pos = out.clip_position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let ndc = (in.screen_pos.xy / in.screen_pos.w) * 0.5 + 0.5;
    let screen_uv = vec2<f32>(ndc.x, 1.0 - ndc.y);
    
    // 1. Przemieszczanie UV (DUDV wave movement)
    let time_speed = uniforms.time * uniforms.wave_speed;
    let offset_1 = uniforms.wind_dir_1 * time_speed;
    let offset_2 = uniforms.wind_dir_2 * time_speed;
    
    let uv_1 = in.uv * uniforms.wave_scale + offset_1;
    let uv_2 = in.uv * uniforms.wave_scale + offset_2;
    
    let dudv_1 = textureSample(dudv_map, dudv_sampler, uv_1).rg * 2.0 - 1.0;
    let dudv_2 = textureSample(dudv_map, dudv_sampler, uv_2).rg * 2.0 - 1.0;
    
    // 2. Refrakcja i zalamanie (Refraction distortion)
    let total_distortion = (dudv_1 + dudv_2) * uniforms.refraction_strength;
    let distorted_screen_uv = screen_uv + total_distortion;
    let distorted_uv = in.uv + total_distortion;
    
    // 3. Mieszanie koloru glebokosci wody (Depth color mixing)
    let bg_depth = textureSample(depth_map, depth_sampler, distorted_screen_uv).r;
    let water_depth = bg_depth - in.clip_position.z;
    let depth_blend = clamp(water_depth * uniforms.water_depth_factor, 0.0, 1.0);
    
    let water_color = mix(uniforms.water_color_shallow, uniforms.water_color_deep, depth_blend);
    
    // 2b. Polysk sloneczny (Specular highlight)
    let normal_sample = textureSample(normal_map, normal_sampler, distorted_uv).rgb * 2.0 - 1.0;
    let normal = normalize(vec3<f32>(normal_sample.x, 1.0, normal_sample.y));
    
    let sun_dir = normalize(uniforms.sun_direction.xyz);
    let view_dir = vec3<f32>(0.0, 1.0, 0.0); // Uproszczony view_dir dla wody
    let half_vec = normalize(sun_dir + view_dir);
    
    let specular = pow(max(dot(normal, half_vec), 0.0), 64.0);
    
    let final_color = water_color.rgb + vec3<f32>(specular);
    return vec4<f32>(final_color, water_color.a);
}
"#;

pub struct WgpuWaterDudvPipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_buffer: wgpu::Buffer,
}

impl WgpuWaterDudvPipeline {
    pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Water DUDV Shader"),
            source: wgpu::ShaderSource::Wgsl(WATER_DUDV_SHADER.into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Water DUDV Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<WaterDudvUniforms>() as _),
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
                wgpu::BindGroupLayoutEntry {
                    binding: 5,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 6,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Water DUDV Pipeline Layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Water DUDV Render Pipeline"),
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
                    }),
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

        use wgpu::util::DeviceExt;
        let default_uniforms = WaterDudvUniforms::default();
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Water DUDV Uniform Buffer"),
            contents: bytemuck::cast_slice(&[default_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            pipeline,
            bind_group_layout,
            uniform_buffer,
        }
    }

    pub fn update_uniforms(&self, queue: &wgpu::Queue, uniforms: &WaterDudvUniforms) {
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(std::slice::from_ref(uniforms)),
        );
    }
}
