use glam::{Mat4, Vec3};

pub const CHARACTER_SHADOW_SHADER: &str = r#"
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

pub const TERRAIN_SHADOW_PCF_SHADER_SNIPPET: &str = r#"
// PCF 3x3 filter snippet for terrain shadow evaluation
fn calculate_shadow_pcf(
    shadow_map: texture_depth_2d,
    shadow_sampler: sampler_comparison,
    shadow_coord: vec3<f32>,
    texel_size: vec2<f32>
) -> f32 {
    var shadow: f32 = 0.0;
    
    // Sample a 3x3 grid
    for (var y: i32 = -1; y <= 1; y++) {
        for (var x: i32 = -1; x <= 1; x++) {
            let offset = vec2<f32>(f32(x), f32(y)) * texel_size;
            let sample_pos = shadow_coord.xy + offset;
            shadow += textureSampleCompareLevel(
                shadow_map,
                shadow_sampler,
                sample_pos,
                shadow_coord.z
            );
        }
    }
    
    return shadow / 9.0;
}
"#;

pub struct WgpuShadowDepthPassV2 {
    pub shadow_map_size: u32,
    pub light_view_matrix: Mat4,
    pub light_proj_matrix: Mat4,
}

impl Default for WgpuShadowDepthPassV2 {
    fn default() -> Self {
        Self {
            shadow_map_size: 2048,
            light_view_matrix: Mat4::IDENTITY,
            light_proj_matrix: Mat4::IDENTITY,
        }
    }
}

impl WgpuShadowDepthPassV2 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_shadow_map_size(&mut self, size: u32) {
        self.shadow_map_size = size;
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
            label: Some("Character Shadow Map Depth Texture"),
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
            label: Some("Character Shadow PCF Sampler"),
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

    pub fn calculate_light_matrices(&mut self, target: Vec3) {
        // C++ reference:
        // D3DXVECTOR3 v3Eye(v3Target.x - 1.732f * 1250.0f,
        //                   v3Target.y - 1250.0f,
        //                   v3Target.z + 2.0f * 1.732f * 1250.0f);
        let eye = Vec3::new(
            target.x - 1.732 * 1250.0,
            target.y - 1250.0,
            target.z + 2.0 * 1.732 * 1250.0,
        );

        // C++ reference:
        // const auto vv = D3DXVECTOR3(0.0f, 0.0f, 1.0f);
        // D3DXMatrixLookAtRH(&matLightView, &v3Eye, &v3Target, &vv);
        let up = Vec3::new(0.0, 0.0, 1.0);
        self.light_view_matrix = Mat4::look_at_rh(eye, target, up);

        // C++ reference:
        // D3DXMatrixOrthoRH(&matLightProj, 2550.0f, 2550.0f, 1.0f, 15000.0f);
        let w = 2550.0;
        let h = 2550.0;
        let zn = 1.0;
        let zf = 15000.0;

        self.light_proj_matrix = Mat4::orthographic_rh(
            -w / 2.0, w / 2.0,
            -h / 2.0, h / 2.0,
            zn, zf,
        );
    }

    pub fn get_light_view_proj_matrix(&self) -> Mat4 {
        self.light_proj_matrix * self.light_view_matrix
    }

    pub fn create_pipeline(
        &self,
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Character Shadow Shader"),
            source: wgpu::ShaderSource::Wgsl(CHARACTER_SHADOW_SHADER.into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Character Shadow Pipeline Layout"),
            bind_group_layouts: &[Some(bind_group_layout)],
            immediate_size: 0,
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Character Shadow Pipeline"),
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
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Front),
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
