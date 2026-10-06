use wgpu::util::DeviceExt;
use std::sync::Arc;
use m2_client::wgpu_terrain_renderer::TerrainVertex;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TerrainUniforms {
    pub view_proj: [[f32; 4]; 4],
    pub sun_dir: [f32; 4],
    pub ambient_color: [f32; 4],
    pub sun_color: [f32; 4],
}

pub struct TerrainSplattingRendererV2 {
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: Option<wgpu::BindGroup>,
    pub uniform_buffer: wgpu::Buffer,
}

impl TerrainSplattingRendererV2 {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Result<Self, String> {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Terrain Splatting Bind Group Layout V2"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0, // Uniform (Camera, Sun, etc.)
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1, // Splat Alpha RGBA Map
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2, // Splat Sampler (Linear)
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3, // Shadow Map
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4, // Tex 0 (Base / Grass)
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 5, // Tex 1 (Rock)
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 6, // Tex 2 (Sand)
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 7, // Tex 3 (Dirt/Other)
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 8, // Terrain Texture Sampler (Anisotropic)
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Terrain Splatting Pipeline Layout V2"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        let shader_source = "
            struct Uniforms {
                view_proj: mat4x4<f32>,
                sun_dir: vec4<f32>,
                ambient_color: vec4<f32>,
                sun_color: vec4<f32>,
            };

            @group(0) @binding(0) var<uniform> uniforms: Uniforms;

            @group(0) @binding(1) var splat_alpha: texture_2d<f32>;
            @group(0) @binding(2) var splat_sampler: sampler;
            @group(0) @binding(3) var shadow_map: texture_2d<f32>;
            @group(0) @binding(4) var tex0: texture_2d<f32>;
            @group(0) @binding(5) var tex1: texture_2d<f32>;
            @group(0) @binding(6) var tex2: texture_2d<f32>;
            @group(0) @binding(7) var tex3: texture_2d<f32>;
            @group(0) @binding(8) var tex_sampler: sampler;

            struct VertexInput {
                @location(0) position: vec3<f32>,
                @location(1) normal: vec3<f32>,
                @location(2) tex_coord: vec2<f32>,
            };

            struct VertexOutput {
                @builtin(position) clip_position: vec4<f32>,
                @location(0) normal: vec3<f32>,
                @location(1) tex_coord: vec2<f32>,
                @location(2) world_pos: vec3<f32>,
                @location(3) shadow_coord: vec2<f32>,
            };

            @vertex
            fn vs_main(model: VertexInput) -> VertexOutput {
                var out: VertexOutput;
                out.clip_position = uniforms.view_proj * vec4<f32>(model.position, 1.0);
                out.normal = model.normal;
                out.tex_coord = model.tex_coord;
                out.world_pos = model.position;
                out.shadow_coord = model.tex_coord; // In Metin2, static shadow maps share the alpha map's UV space
                return out;
            }

            @fragment
            fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
                // Fetch weights from the Splat Alpha RGBA map
                let weights = textureSample(splat_alpha, splat_sampler, in.tex_coord);
                
                // Fetch textures (with a base scaling factor, typically 4.0 or from TTerrainTexture)
                let uv_scaled = in.tex_coord * 4.0;
                let c0 = textureSample(tex0, tex_sampler, uv_scaled);
                let c1 = textureSample(tex1, tex_sampler, uv_scaled);
                let c2 = textureSample(tex2, tex_sampler, uv_scaled);
                let c3 = textureSample(tex3, tex_sampler, uv_scaled);
                
                // Texture Splatting blending formula
                let splat_color = c0 * weights.r + c1 * weights.g + c2 * weights.b + c3 * weights.a;
                
                // Lighting: Dot product with sun direction
                let n = normalize(in.normal);
                let l = normalize(-uniforms.sun_dir.xyz);
                let diffuse_factor = max(dot(n, l), 0.0);
                
                // Shadow map fetching
                let shadow_val = textureSample(shadow_map, splat_sampler, in.shadow_coord).r;
                
                // Final light combination
                let light_color = uniforms.ambient_color.rgb + (uniforms.sun_color.rgb * diffuse_factor * shadow_val);
                
                let final_rgb = splat_color.rgb * light_color;
                
                return vec4<f32>(final_rgb, 1.0);
            }
        ";

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Terrain Splatting Shader V2"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Terrain Splatting Render Pipeline V2"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(TerrainVertex::desc())],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
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
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Terrain Splat Uniform Buffer V2"),
            contents: bytemuck::cast_slice(&[TerrainUniforms {
                view_proj: [[0.0; 4]; 4],
                sun_dir: [0.0; 4],
                ambient_color: [0.0; 4],
                sun_color: [0.0; 4],
            }]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Ok(Self {
            pipeline,
            bind_group_layout,
            bind_group: None,
            uniform_buffer,
        })
    }

    pub fn load_bind_group(
        &mut self,
        device: &wgpu::Device,
        splat_alpha_view: &wgpu::TextureView,
        splat_sampler: &wgpu::Sampler,
        shadow_map_view: &wgpu::TextureView,
        tex_views: [&wgpu::TextureView; 4],
        tex_sampler: &wgpu::Sampler,
    ) {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Terrain Splatting Bind Group V2"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(splat_alpha_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(splat_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(shadow_map_view),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(tex_views[0]),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::TextureView(tex_views[1]),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: wgpu::BindingResource::TextureView(tex_views[2]),
                },
                wgpu::BindGroupEntry {
                    binding: 7,
                    resource: wgpu::BindingResource::TextureView(tex_views[3]),
                },
                wgpu::BindGroupEntry {
                    binding: 8,
                    resource: wgpu::BindingResource::Sampler(tex_sampler),
                },
            ],
        });

        self.bind_group = Some(bind_group);
    }
    
    pub fn update_uniforms(
        &self,
        queue: &wgpu::Queue,
        uniforms: TerrainUniforms,
    ) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }

    pub fn draw<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        vertex_buffer: &'a wgpu::Buffer,
        index_buffer: &'a wgpu::Buffer,
        index_count: u32,
    ) -> Result<(), String> {
        let bg = self.bind_group.as_ref().ok_or("Bind group not initialized. Call load_bind_group first.")?;
        
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, bg, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..index_count, 0, 0..1);
        
        Ok(())
    }
}
