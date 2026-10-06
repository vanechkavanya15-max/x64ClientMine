use std::mem;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WaterSurfaceVertexV2 {
    pub position: [f32; 3],
    pub diffuse: u32,
}

impl WaterSurfaceVertexV2 {
    pub const fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WaterSurfaceUniformsV2 {
    pub view_proj: [[f32; 4]; 4],
    pub time: f32,
    pub water_depth_scale: f32,
    pub _pad0: [f32; 2],
    pub dir1: [f32; 2],
    pub dir2: [f32; 2],
    pub sun_dir: [f32; 3],
    pub _pad1: f32,
}

pub struct WgpuWaterSurfaceWavesV2Renderer {
    pub pipeline: wgpu::RenderPipeline,
    pub uniform_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl WgpuWaterSurfaceWavesV2Renderer {
    pub fn new(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let shader_src = r#"
struct WaterUniforms {
    view_proj: mat4x4<f32>,
    time: f32,
    water_depth_scale: f32,
    _pad0: vec2<f32>,
    dir1: vec2<f32>,
    dir2: vec2<f32>,
    sun_dir: vec3<f32>,
    _pad1: f32,
};

@group(0) @binding(0) var<uniform> uniforms: WaterUniforms;
@group(0) @binding(1) var t_normal: texture_2d<f32>;
@group(0) @binding(2) var s_normal: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) diffuse: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.world_pos = model.position;
    out.clip_position = uniforms.view_proj * vec4<f32>(model.position, 1.0);
    
    // Scale position to UV coordinates
    out.uv = vec2<f32>(model.position.x, model.position.y) * 0.005;
    
    // Unpack ABGR to RGBA f32 color
    let r = f32(model.diffuse & 0xFFu) / 255.0;
    let g = f32((model.diffuse >> 8u) & 0xFFu) / 255.0;
    let b = f32((model.diffuse >> 16u) & 0xFFu) / 255.0;
    let a = f32((model.diffuse >> 24u) & 0xFFu) / 255.0;
    out.color = vec4<f32>(r, g, b, a);
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv1 = in.uv + uniforms.dir1 * uniforms.time;
    let uv2 = in.uv + uniforms.dir2 * uniforms.time;
    
    // Sample normal maps
    let n1 = textureSample(t_normal, s_normal, uv1).xyz * 2.0 - 1.0;
    let n2 = textureSample(t_normal, s_normal, uv2).xyz * 2.0 - 1.0;
    let normal = normalize(n1 + n2 + vec3<f32>(0.0, 0.0, 2.0));
    
    // Simple specular calculation
    let view_dir = normalize(vec3<f32>(0.0, -1.0, 1.0)); // Fixed camera direction for simplicity
    let sun_dir = normalize(uniforms.sun_dir);
    let half_vec = normalize(view_dir + sun_dir);
    let spec = pow(max(dot(normal, half_vec), 0.0), 32.0);
    
    // Base water color blended with vertex diffuse and specular
    var base_color = vec3<f32>(0.1, 0.4, 0.6) * in.color.rgb;
    base_color = base_color + vec3<f32>(spec);
    
    // Depth-based transparency and foam
    // Alpha channel of in.color usually contains depth information for shoreline foam
    let depth_val = in.color.a;
    
    // Add some foam near the shoreline
    let foam_threshold = 0.2;
    var foam_val = 0.0;
    if (depth_val < foam_threshold) {
        foam_val = 1.0 - (depth_val / foam_threshold);
    }
    
    // Add specular to the final color, and foam
    let final_rgb = base_color + vec3<f32>(foam_val);
    
    let alpha = clamp(0.5 + uniforms.water_depth_scale * 0.5, 0.2, 0.9);
    
    return vec4<f32>(final_rgb, alpha);
}
"#;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Water Surface Waves V2 Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_src.into()),
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Water Surface Waves V2 Uniform Buffer"),
            size: mem::size_of::<WaterSurfaceUniformsV2>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Dummy texture setup if none is provided
        let texture_size = wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 };
        let normal_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Dummy Normal Texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        
        let normal_view = normal_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let normal_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Normal Sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Water Surface Waves V2 Bind Group"),
            layout: bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&normal_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&normal_sampler),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Water Surface Waves V2 Pipeline Layout"),
            bind_group_layouts: &[Some(bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Water Surface Waves V2 Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(WaterSurfaceVertexV2::desc())],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(false),
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
        });

        Self {
            pipeline,
            uniform_buffer,
            bind_group,
        }
    }

    pub fn update_uniforms(&self, queue: &wgpu::Queue, uniforms: &WaterSurfaceUniformsV2) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(std::slice::from_ref(uniforms)));
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, vertex_buffer: &'a wgpu::Buffer, num_vertices: u32) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..num_vertices, 0..1);
    }
}
