use crate::wgpu_skybox_lighting::EnvironmentUniforms;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SkyboxVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub tex_coord: [f32; 2],
}

impl SkyboxVertex {
    pub const fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SkyboxVertex>() as wgpu::BufferAddress,
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
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 3]>() + std::mem::size_of::<[f32; 4]>()) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub struct SkyboxRenderer {
    pub pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
}

pub fn generate_skybox_geometry() -> (Vec<SkyboxVertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    
    let base_color = [1.0, 1.0, 1.0, 1.0];
    
    // Front face (Z = 1)
    vertices.push(SkyboxVertex { position: [-1.0, -1.0, 1.0], color: base_color, tex_coord: [0.0, 1.0] });
    vertices.push(SkyboxVertex { position: [ 1.0, -1.0, 1.0], color: base_color, tex_coord: [1.0, 1.0] });
    vertices.push(SkyboxVertex { position: [ 1.0,  1.0, 1.0], color: base_color, tex_coord: [1.0, 0.0] });
    vertices.push(SkyboxVertex { position: [-1.0,  1.0, 1.0], color: base_color, tex_coord: [0.0, 0.0] });
    
    // Back face (Z = -1)
    vertices.push(SkyboxVertex { position: [ 1.0, -1.0, -1.0], color: base_color, tex_coord: [0.0, 1.0] });
    vertices.push(SkyboxVertex { position: [-1.0, -1.0, -1.0], color: base_color, tex_coord: [1.0, 1.0] });
    vertices.push(SkyboxVertex { position: [-1.0,  1.0, -1.0], color: base_color, tex_coord: [1.0, 0.0] });
    vertices.push(SkyboxVertex { position: [ 1.0,  1.0, -1.0], color: base_color, tex_coord: [0.0, 0.0] });
    
    // Left face (X = -1)
    vertices.push(SkyboxVertex { position: [-1.0, -1.0, -1.0], color: base_color, tex_coord: [0.0, 1.0] });
    vertices.push(SkyboxVertex { position: [-1.0, -1.0,  1.0], color: base_color, tex_coord: [1.0, 1.0] });
    vertices.push(SkyboxVertex { position: [-1.0,  1.0,  1.0], color: base_color, tex_coord: [1.0, 0.0] });
    vertices.push(SkyboxVertex { position: [-1.0,  1.0, -1.0], color: base_color, tex_coord: [0.0, 0.0] });
    
    // Right face (X = 1)
    vertices.push(SkyboxVertex { position: [ 1.0, -1.0,  1.0], color: base_color, tex_coord: [0.0, 1.0] });
    vertices.push(SkyboxVertex { position: [ 1.0, -1.0, -1.0], color: base_color, tex_coord: [1.0, 1.0] });
    vertices.push(SkyboxVertex { position: [ 1.0,  1.0, -1.0], color: base_color, tex_coord: [1.0, 0.0] });
    vertices.push(SkyboxVertex { position: [ 1.0,  1.0,  1.0], color: base_color, tex_coord: [0.0, 0.0] });
    
    // Top face (Y = 1)
    vertices.push(SkyboxVertex { position: [-1.0,  1.0,  1.0], color: base_color, tex_coord: [0.0, 1.0] });
    vertices.push(SkyboxVertex { position: [ 1.0,  1.0,  1.0], color: base_color, tex_coord: [1.0, 1.0] });
    vertices.push(SkyboxVertex { position: [ 1.0,  1.0, -1.0], color: base_color, tex_coord: [1.0, 0.0] });
    vertices.push(SkyboxVertex { position: [-1.0,  1.0, -1.0], color: base_color, tex_coord: [0.0, 0.0] });
    
    // Bottom face (Y = -1)
    vertices.push(SkyboxVertex { position: [-1.0, -1.0, -1.0], color: base_color, tex_coord: [0.0, 1.0] });
    vertices.push(SkyboxVertex { position: [ 1.0, -1.0, -1.0], color: base_color, tex_coord: [1.0, 1.0] });
    vertices.push(SkyboxVertex { position: [ 1.0, -1.0,  1.0], color: base_color, tex_coord: [1.0, 0.0] });
    vertices.push(SkyboxVertex { position: [-1.0, -1.0,  1.0], color: base_color, tex_coord: [0.0, 0.0] });

    for face in 0..6 {
        let offset = face * 4;
        indices.push(offset);
        indices.push(offset + 1);
        indices.push(offset + 2);
        
        indices.push(offset);
        indices.push(offset + 2);
        indices.push(offset + 3);
    }
    
    (vertices, indices)
}

const SKYBOX_WGSL: &str = r#"
struct EnvironmentUniforms {
    skybox_view_proj: mat4x4<f32>,
    sun_direction: vec4<f32>,
    ambient_color: vec4<f32>,
    diffuse_color: vec4<f32>,
    fog_color: vec4<f32>,
    fog_params: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> env: EnvironmentUniforms;

@group(0) @binding(1)
var t_skybox: texture_2d<f32>;

@group(0) @binding(2)
var s_skybox: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) tex_coord: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    var pos = vec4<f32>(model.position, 1.0);
    var clip_pos = env.skybox_view_proj * pos;
    
    // W zaleznosci od kamery i perspektywy, skybox jest zawsze na far plane (Z = W)
    out.clip_position = vec4<f32>(clip_pos.x, clip_pos.y, clip_pos.w, clip_pos.w);
    
    out.tex_coord = model.tex_coord;
    out.color = model.color;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(t_skybox, s_skybox, in.tex_coord);
    let sky_color = in.color;
    
    // Blend diffuse texture with vertex color as done in C++
    if (tex_color.a < 0.05) {
        let horizon_color = vec4<f32>(0.72, 0.82, 0.94, 1.0);
        let zenith_color = vec4<f32>(0.28, 0.48, 0.82, 1.0);
        let t = clamp(in.tex_coord.y, 0.0, 1.0);
        let grad = mix(zenith_color, horizon_color, t);
        return grad * sky_color;
    }
    return tex_color * sky_color;
}
"#;

impl SkyboxRenderer {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        depth_format: wgpu::TextureFormat,
    ) -> Self {
        let (vertices, indices) = generate_skybox_geometry();
        
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Skybox Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Skybox Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let initial_env = EnvironmentUniforms {
            skybox_view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
            sun_direction: [0.577, -0.577, 0.577, 0.0],
            ambient_color: [0.45, 0.50, 0.60, 1.0],
            diffuse_color: [0.95, 0.90, 0.85, 1.0],
            fog_color: [0.70, 0.75, 0.85, 1.0],
            fog_params: [2400.0, 25600.0, 0.001, 1.0],
        };
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Skybox Uniform Buffer"),
            contents: bytemuck::bytes_of(&initial_env),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Skybox Uniform Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
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
            ],
        });
        
        // Dummy texture and sampler to satisfy bindings
        let texture_extent = wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        };
        
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Dummy Skybox Texture"),
            size: texture_extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            ..Default::default()
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Skybox Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Skybox WGSL"),
            source: wgpu::ShaderSource::Wgsl(SKYBOX_WGSL.into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Skybox Pipeline Layout"),
            bind_group_layouts: &[Some(&uniform_bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Skybox Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(SkyboxVertex::desc())],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, // No culling as we render inside out
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth_format,
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
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    pub fn update_uniforms(&self, queue: &wgpu::Queue, uniforms: &EnvironmentUniforms) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(uniforms));
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}

pub fn interpolate_color_local(start: [f32; 4], end: [f32; 4], factor: f32) -> [f32; 4] {
    let t = factor.clamp(0.0, 1.0);
    [
        start[0] + (end[0] - start[0]) * t,
        start[1] + (end[1] - start[1]) * t,
        start[2] + (end[2] - start[2]) * t,
        start[3] + (end[3] - start[3]) * t,
    ]
}

pub fn update_time_of_day(
    time_factor: f32, // 0.0 to 1.0
) -> ([f32; 4], [f32; 4], [f32; 4], [f32; 3]) { // ambient, diffuse, fog, sun_dir
    let morning_ambient = [0.3, 0.3, 0.4, 1.0];
    let noon_ambient = [0.6, 0.6, 0.6, 1.0];
    let sunset_ambient = [0.5, 0.3, 0.2, 1.0];
    let night_ambient = [0.1, 0.1, 0.2, 1.0];

    let morning_diffuse = [0.8, 0.7, 0.6, 1.0];
    let noon_diffuse = [1.0, 0.9, 0.8, 1.0];
    let sunset_diffuse = [0.9, 0.4, 0.2, 1.0];
    let night_diffuse = [0.2, 0.2, 0.4, 1.0];

    let morning_fog = [0.7, 0.7, 0.8, 1.0];
    let noon_fog = [0.6, 0.7, 0.9, 1.0];
    let sunset_fog = [0.8, 0.5, 0.3, 1.0];
    let night_fog = [0.1, 0.1, 0.15, 1.0];

    let mut ambient = [0.0; 4];
    let mut diffuse = [0.0; 4];
    let mut fog = [0.0; 4];
    
    // Normalize time to 0.0..1.0 range just in case
    let t = time_factor - time_factor.floor();

    if t < 0.25 {
        // Night to Morning
        let f = t / 0.25;
        ambient = interpolate_color_local(night_ambient, morning_ambient, f);
        diffuse = interpolate_color_local(night_diffuse, morning_diffuse, f);
        fog = interpolate_color_local(night_fog, morning_fog, f);
    } else if t < 0.5 {
        // Morning to Noon
        let f = (t - 0.25) / 0.25;
        ambient = interpolate_color_local(morning_ambient, noon_ambient, f);
        diffuse = interpolate_color_local(morning_diffuse, noon_diffuse, f);
        fog = interpolate_color_local(morning_fog, noon_fog, f);
    } else if t < 0.75 {
        // Noon to Sunset
        let f = (t - 0.5) / 0.25;
        ambient = interpolate_color_local(noon_ambient, sunset_ambient, f);
        diffuse = interpolate_color_local(noon_diffuse, sunset_diffuse, f);
        fog = interpolate_color_local(noon_fog, sunset_fog, f);
    } else {
        // Sunset to Night
        let f = (t - 0.75) / 0.25;
        ambient = interpolate_color_local(sunset_ambient, night_ambient, f);
        diffuse = interpolate_color_local(sunset_diffuse, night_diffuse, f);
        fog = interpolate_color_local(sunset_fog, night_fog, f);
    }

    let pi_2 = std::f32::consts::PI * 2.0;
    let angle = t * pi_2;
    // Simple path over X and Y
    let sun_x = f32::cos(angle);
    let sun_y = f32::sin(angle);
    let sun_z = 0.5; // Constant elevation

    let len = (sun_x * sun_x + sun_y * sun_y + sun_z * sun_z).sqrt();
    let sun_dir = [sun_x / len, sun_y / len, sun_z / len];

    (ambient, diffuse, fog, sun_dir)
}
