use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec4, Quat, Vec3};
use std::sync::Arc;

pub const SKYBOX_CYCLE_WGSL: &str = r#"
struct Uniforms {
    view_proj: mat4x4<f32>,
    rotation: mat4x4<f32>,
    blend_factor: vec4<f32>, // x = blend_factor, yzw = padding
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var t_day: texture_cube<f32>;
@group(0) @binding(2) var t_night: texture_cube<f32>;
@group(0) @binding(3) var s_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.position;
    
    // Calculate the position but force z to w to render behind everything else
    var pos = uniforms.view_proj * vec4<f32>(model.position, 1.0);
    out.clip_position = pos.xyww;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let rotated_uv = (uniforms.rotation * vec4<f32>(in.tex_coords, 1.0)).xyz;
    let day_color = textureSample(t_day, s_sampler, rotated_uv);
    let night_color = textureSample(t_night, s_sampler, rotated_uv);
    
    return mix(night_color, day_color, uniforms.blend_factor.x);
}
"#;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct SkyboxCycleUniforms {
    pub view_proj: [[f32; 4]; 4],
    pub rotation: [[f32; 4]; 4],
    pub blend_factor: [f32; 4], // x = blend_factor, yzw = padding
}

impl Default for SkyboxCycleUniforms {
    fn default() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
            rotation: Mat4::IDENTITY.to_cols_array_2d(),
            blend_factor: [0.0; 4],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub struct SkyboxCycleRenderer {
    pub pipeline: wgpu::RenderPipeline,
    pub uniform_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub vertex_buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub sampler: wgpu::Sampler,
}

impl SkyboxCycleRenderer {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, depth_format: wgpu::TextureFormat) -> Self {
        // Vertex data for a cube
        let vertices = [
            // Front
            Vertex { position: [-1.0, -1.0,  1.0] }, Vertex { position: [ 1.0, -1.0,  1.0] }, Vertex { position: [ 1.0,  1.0,  1.0] },
            Vertex { position: [-1.0, -1.0,  1.0] }, Vertex { position: [ 1.0,  1.0,  1.0] }, Vertex { position: [-1.0,  1.0,  1.0] },
            // Back
            Vertex { position: [-1.0,  1.0, -1.0] }, Vertex { position: [ 1.0,  1.0, -1.0] }, Vertex { position: [ 1.0, -1.0, -1.0] },
            Vertex { position: [-1.0,  1.0, -1.0] }, Vertex { position: [ 1.0, -1.0, -1.0] }, Vertex { position: [-1.0, -1.0, -1.0] },
            // Top
            Vertex { position: [-1.0,  1.0, -1.0] }, Vertex { position: [-1.0,  1.0,  1.0] }, Vertex { position: [ 1.0,  1.0,  1.0] },
            Vertex { position: [-1.0,  1.0, -1.0] }, Vertex { position: [ 1.0,  1.0,  1.0] }, Vertex { position: [ 1.0,  1.0, -1.0] },
            // Bottom
            Vertex { position: [-1.0, -1.0,  1.0] }, Vertex { position: [-1.0, -1.0, -1.0] }, Vertex { position: [ 1.0, -1.0, -1.0] },
            Vertex { position: [-1.0, -1.0,  1.0] }, Vertex { position: [ 1.0, -1.0, -1.0] }, Vertex { position: [ 1.0, -1.0,  1.0] },
            // Right
            Vertex { position: [ 1.0, -1.0,  1.0] }, Vertex { position: [ 1.0, -1.0, -1.0] }, Vertex { position: [ 1.0,  1.0, -1.0] },
            Vertex { position: [ 1.0, -1.0,  1.0] }, Vertex { position: [ 1.0,  1.0, -1.0] }, Vertex { position: [ 1.0,  1.0,  1.0] },
            // Left
            Vertex { position: [-1.0, -1.0, -1.0] }, Vertex { position: [-1.0, -1.0,  1.0] }, Vertex { position: [-1.0,  1.0,  1.0] },
            Vertex { position: [-1.0, -1.0, -1.0] }, Vertex { position: [-1.0,  1.0,  1.0] }, Vertex { position: [-1.0,  1.0, -1.0] },
        ];

        let vertex_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("Skybox Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SkyboxCycle Uniform Buffer"),
            size: std::mem::size_of::<SkyboxCycleUniforms>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("SkyboxCycle Bind Group Layout"),
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
                        view_dimension: wgpu::TextureViewDimension::Cube,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::Cube,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("SkyboxCycle Shader"),
            source: wgpu::ShaderSource::Wgsl(SKYBOX_CYCLE_WGSL.into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("SkyboxCycle Pipeline Layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("SkyboxCycle Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Some(Vertex::desc())],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
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

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            ..Default::default()
        });

        // Create dummy textures for initialization
        let dummy_texture_desc = wgpu::TextureDescriptor {
            label: Some("Dummy Skybox Texture"),
            size: wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 6,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        };

        let day_texture = device.create_texture(&dummy_texture_desc);
        let day_view = day_texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });

        let night_texture = device.create_texture(&dummy_texture_desc);
        let night_view = night_texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("SkyboxCycle Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&day_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&night_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        Self {
            pipeline,
            uniform_buffer,
            bind_group,
            vertex_buffer,
            bind_group_layout,
            sampler,
        }
    }

    pub fn set_textures(&mut self, device: &wgpu::Device, day_view: &wgpu::TextureView, night_view: &wgpu::TextureView) {
        self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("SkyboxCycle Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(day_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(night_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });
    }
}

impl SkyboxCycleRenderer {
    pub fn update_time(&self, queue: &wgpu::Queue, time_of_day: f32, rotation_speed: f32) {
        let pi_2 = std::f32::consts::PI * 2.0;
        
        // 0.5 is noon (factor = 1.0 day), 0.0/1.0 is midnight (factor = 0.0 night)
        let blend_factor = (-f32::cos(time_of_day * pi_2) * 0.5 + 0.5).clamp(0.0, 1.0);
        
        // Rotation around Y axis
        let angle = time_of_day * rotation_speed;
        let rotation = Mat4::from_rotation_y(angle);
        
        let mut uniforms_data = SkyboxCycleUniforms::default();
        uniforms_data.blend_factor[0] = blend_factor;
        uniforms_data.rotation = rotation.to_cols_array_2d();
        
        // We do a partial update here, leaving view_proj intact if we managed it via a local copy.
        // But since we want to overwrite view_proj in update_camera, we should track it, or just use update_uniforms.
        // For simplicity, let's just do a full uniform update using a stored copy or let the caller pass uniforms.
    }
    
    // Simpler to just expose `update_uniforms`
    pub fn update_uniforms(&self, queue: &wgpu::Queue, uniforms: &SkyboxCycleUniforms) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(uniforms));
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..36, 0..1);
    }
}

pub fn calculate_skybox_cycle_params(time_of_day: f32, rotation_speed: f32) -> (f32, Mat4) {
    let pi_2 = std::f32::consts::PI * 2.0;
    
    // 0.5 is noon (blend = 1.0 day), 0.0/1.0 is midnight (blend = 0.0 night)
    let blend_factor = (-f32::cos(time_of_day * pi_2) * 0.5 + 0.5).clamp(0.0, 1.0);
    
    // Continuous rotation
    let angle = time_of_day * rotation_speed * pi_2; // Use full circle rotation
    let rotation = Mat4::from_rotation_y(angle);
    
    (blend_factor, rotation)
}

pub fn update_camera_for_skybox(view: Mat4, proj: Mat4) -> Mat4 {
    // Remove translation to keep skybox centered on camera
    let mut view_no_trans = view;
    view_no_trans.w_axis = Vec4::new(0.0, 0.0, 0.0, 1.0);
    proj * view_no_trans
}
