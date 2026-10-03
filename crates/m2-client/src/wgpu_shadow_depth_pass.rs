use glam::{Mat4, Vec3};

pub struct WgpuShadowDepthPass {
    pub shadow_map_size: u32,
    pub light_view_matrix: Mat4,
    pub light_proj_matrix: Mat4,
}

impl Default for WgpuShadowDepthPass {
    fn default() -> Self {
        Self {
            shadow_map_size: 2048,
            light_view_matrix: Mat4::IDENTITY,
            light_proj_matrix: Mat4::IDENTITY,
        }
    }
}

pub const SUN_SHADOW_SHADER: &str = r#"
struct ShadowUniforms {
    light_view_proj: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: ShadowUniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = uniforms.light_view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) {
    // Depth-only render pass writes automatically.
}
"#;

impl WgpuShadowDepthPass {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_depth_texture(
        &self,
        device: &wgpu::Device,
    ) -> (wgpu::Texture, wgpu::TextureView, wgpu::Sampler) {
        let size_ext = wgpu::Extent3d {
            width: self.shadow_map_size,
            height: self.shadow_map_size,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Sun Shadow Map Depth Texture"),
            size: size_ext,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Sun Shadow PCF Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });

        (texture, view, sampler)
    }

    /// Calculates Light View & Proj matrices.
    /// Following the orthographic projection for shadows.
    pub fn calculate_light_matrices(&mut self, target: Vec3, sun_direction: Vec3) {
        // Assume sun_direction is normalized and points *towards* the target
        let mut dir = sun_direction;
        if dir.length_squared() < 0.0001 {
            dir = Vec3::new(0.0, -1.0, 0.0);
        } else {
            dir = dir.normalize();
        }

        // Place the camera some distance away from the target along the opposite of the sun direction
        let distance = 5000.0;
        let eye = target - dir * distance;
        
        let mut up = Vec3::new(0.0, 0.0, 1.0);
        if dir.cross(up).length_squared() < 0.001 {
            up = Vec3::new(0.0, 1.0, 0.0);
        }

        self.light_view_matrix = Mat4::look_at_rh(eye, target, up);

        let w = 5000.0;
        let h = 5000.0;
        let zn = 1.0;
        let zf = 10000.0;

        self.light_proj_matrix = Mat4::orthographic_rh(
            -w / 2.0, w / 2.0,
            -h / 2.0, h / 2.0,
            zn, zf,
        );
    }

    pub fn create_pipeline(
        &self,
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Sun Shadow Shader"),
            source: wgpu::ShaderSource::Wgsl(SUN_SHADOW_SHADER.into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Sun Shadow Pipeline Layout"),
            bind_group_layouts: &[Some(bind_group_layout)],
            immediate_size: 0,
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Sun Shadow Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3],
                })],
                compilation_options: Default::default(),
            },
            fragment: None,
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Front), // Front-face culling for shadow maps
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::LessEqual),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState {
                    constant: 2,
                    slope_scale: 2.0,
                    clamp: 0.0,
                },
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        })
    }
}
