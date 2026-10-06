use std::borrow::Cow;
use wgpu::util::DeviceExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum FogModeV2 {
    Linear,
    Exp,
}

#[derive(Debug, Clone)]
pub struct DepthFogV2 {
    pub enabled: bool,
    pub mode: FogModeV2,
    pub color: [f32; 4],
    pub near_distance: f32,
    pub far_distance: f32,
    pub density: f32,
}

impl Default for DepthFogV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DepthFogV2 {
    pub fn new() -> Self {
        Self {
            enabled: false,
            mode: FogModeV2::Linear,
            color: [0.0, 0.0, 0.0, 1.0],
            near_distance: 0.0,
            far_distance: 0.0,
            density: 0.0,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_mode(&mut self, mode: FogModeV2) {
        self.mode = mode;
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.color = [r, g, b, a];
    }

    pub fn set_distances(&mut self, near: f32, far: f32) {
        self.near_distance = near;
        self.far_distance = far;
    }

    pub fn set_density(&mut self, density: f32) {
        self.density = density;
    }

    pub fn calculate_fog_density(&self, depth: f32, fog_level: u8) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        let i_fog_level = fog_level as usize;
        let clamped_level = if i_fog_level > 2 { 2 } else { i_fog_level };
        
        match self.mode {
            FogModeV2::Exp => {
                let fog_density_level = [0.000006, 0.000004, 0.000002];
                let f_density = self.density * fog_density_level[clamped_level];
                
                let f = 1.0 / std::f32::consts::E.powf(depth * f_density);
                1.0 - f.clamp(0.0, 1.0)
            }
            FogModeV2::Linear => {
                let fog_scale_level = [0.75, 1.0, 1.25];
                let f_fog_near = self.near_distance * fog_scale_level[clamped_level];
                let f_fog_far = self.far_distance * fog_scale_level[clamped_level];
                
                if f_fog_far <= f_fog_near {
                    return 1.0;
                }
                
                let f = (f_fog_far - depth) / (f_fog_far - f_fog_near);
                1.0 - f.clamp(0.0, 1.0)
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DepthFogUniform {
    pub fog_color: [f32; 4],
    pub fog_params: [f32; 4], // [near/density, far, mode(0=linear, 1=exp), enabled]
}

pub struct DepthFogPipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    pub sampler: wgpu::Sampler,
}

impl DepthFogPipeline {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let uniform = DepthFogUniform {
            fog_color: [0.0, 0.0, 0.0, 0.0],
            fog_params: [0.0, 0.0, 0.0, 0.0],
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Depth Fog Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default() // Let it pick the correct mipmap_filter depending on the version
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Depth Fog Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
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
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: uniform_buffer.as_entire_binding(),
                },
            ],
            label: Some("Depth Fog Dummy Uniform Bind Group"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Depth Fog Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(
                "
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 2.0;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 2.0;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.tex_coords = vec2<f32>((x + 1.0) * 0.5, (1.0 - y) * 0.5);
    return out;
}

struct DepthFogUniform {
    fog_color: vec4<f32>,
    fog_params: vec4<f32>, // [near/density, far, mode(0=linear, 1=exp), enabled]
};

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var t_depth: texture_depth_2d;
@group(0) @binding(2)
var s_sampler: sampler;
@group(0) @binding(3)
var<uniform> u_fog: DepthFogUniform;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(t_diffuse, s_sampler, in.tex_coords);
    if u_fog.fog_params[3] < 0.5 {
        return color;
    }
    
    let depth_val = textureSample(t_depth, s_sampler, in.tex_coords);
    
    // Linearize depth calculation assuming a standard projection matrix
    // In a full implementation, you would need near and far plane values of the camera
    let z_n = 2.0 * depth_val - 1.0; 
    let z_eye = (2.0 * 10.0 * 25600.0) / (25600.0 + 10.0 - z_n * (25600.0 - 10.0));
    
    var fog_factor = 0.0;
    if u_fog.fog_params[2] < 0.5 { // Linear
        let near = u_fog.fog_params[0];
        let far = u_fog.fog_params[1];
        if far > near {
            fog_factor = (far - z_eye) / (far - near);
        } else {
            fog_factor = 0.0;
        }
        fog_factor = 1.0 - clamp(fog_factor, 0.0, 1.0);
    } else { // Exp
        let density = u_fog.fog_params[0];
        let f = 1.0 / exp(z_eye * density);
        fog_factor = 1.0 - clamp(f, 0.0, 1.0);
    }
    
    return mix(color, u_fog.fog_color, fog_factor);
}
                ",
            )),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Depth Fog Pipeline Layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        // Wgpu version robust struct initialization trick
        let mut render_pipeline_desc = wgpu::RenderPipelineDescriptor {
            label: Some("Depth Fog Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
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
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        };
        
        let pipeline = device.create_render_pipeline(&render_pipeline_desc);

        Self {
            pipeline,
            bind_group_layout,
            uniform_buffer,
            uniform_bind_group,
            sampler,
        }
    }

    pub fn create_bind_group(
        &self,
        device: &wgpu::Device,
        diffuse_view: &wgpu::TextureView,
        depth_view: &wgpu::TextureView,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(diffuse_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(depth_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: self.uniform_buffer.as_entire_binding(),
                },
            ],
            label: Some("Depth Fog Bind Group"),
        })
    }

    pub fn update_uniform(
        &self,
        queue: &wgpu::Queue,
        fog: &DepthFogV2,
        fog_level: u8,
    ) {
        let mut uniform = DepthFogUniform {
            fog_color: fog.color,
            fog_params: [0.0, 0.0, 0.0, if fog.enabled { 1.0 } else { 0.0 }],
        };

        let i_fog_level = fog_level as usize;
        let clamped_level = if i_fog_level > 2 { 2 } else { i_fog_level };

        match fog.mode {
            FogModeV2::Linear => {
                let fog_scale_level = [0.75, 1.0, 1.25];
                let f_fog_near = fog.near_distance * fog_scale_level[clamped_level];
                let f_fog_far = fog.far_distance * fog_scale_level[clamped_level];

                uniform.fog_params[0] = f_fog_near;
                uniform.fog_params[1] = f_fog_far;
                uniform.fog_params[2] = 0.0;
            }
            FogModeV2::Exp => {
                let fog_density_level = [0.000006, 0.000004, 0.000002];
                let f_density = fog.density * fog_density_level[clamped_level];

                uniform.fog_params[0] = f_density;
                uniform.fog_params[2] = 1.0;
            }
        }

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniform]));
    }
}
