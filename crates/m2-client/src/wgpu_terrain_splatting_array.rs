use crate::wgpu_terrain_renderer::TerrainVertex;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct TerrainUniformsArray {
    pub view_proj: [[f32; 4]; 4],
    pub sun_dir: [f32; 4],
    pub sun_color: [f32; 4],
    pub ambient_color: [f32; 4],
    pub fog_color: [f32; 4],
    pub fog_params: [f32; 4], // x: fog_start, y: fog_end, z: unused, w: unused
    pub camera_pos: [f32; 4],
}

pub struct TerrainSplattingArrayRenderer {
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_buffer: wgpu::Buffer,
    pub bind_group: Option<wgpu::BindGroup>,
}

impl TerrainSplattingArrayRenderer {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Terrain Splatting Array Bind Group Layout"),
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
                    binding: 1, // splat_alpha
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2, // splat_sampler
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3, // tex_array
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4, // tex_sampler
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Terrain Splatting Array Pipeline Layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        let shader_source = "
            struct Uniforms {
                view_proj: mat4x4<f32>,
                sun_dir: vec4<f32>,
                sun_color: vec4<f32>,
                ambient_color: vec4<f32>,
                fog_color: vec4<f32>,
                fog_params: vec4<f32>,
                camera_pos: vec4<f32>,
            };

            @group(0) @binding(0) var<uniform> uniforms: Uniforms;
            @group(0) @binding(1) var splat_alpha: texture_2d<f32>;
            @group(0) @binding(2) var splat_sampler: sampler;
            @group(0) @binding(3) var tex_array: texture_2d_array<f32>;
            @group(0) @binding(4) var tex_sampler: sampler;

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
                @location(3) dist_to_camera: f32,
            };

            @vertex
            fn vs_main(model: VertexInput) -> VertexOutput {
                var out: VertexOutput;
                let world_position = vec4<f32>(model.position, 1.0);
                out.clip_position = uniforms.view_proj * world_position;
                out.normal = model.normal;
                out.tex_coord = model.tex_coord;
                out.world_pos = model.position;
                out.dist_to_camera = distance(model.position, uniforms.camera_pos.xyz);
                return out;
            }

            @fragment
            fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
                // Bilinear sampling of splat weights (smooth interpolation on edges)
                let splat_weights = textureSample(splat_alpha, splat_sampler, in.tex_coord);
                
                // Textures (UV scaling for tiling on 16x16 tile)
                let uv = in.tex_coord * 4.0;
                
                // Sample texture array
                let c0 = textureSample(tex_array, tex_sampler, uv, 0u); // Grass
                let c1 = textureSample(tex_array, tex_sampler, uv, 1u); // Soil
                let c2 = textureSample(tex_array, tex_sampler, uv, 2u); // Rock
                let c3 = textureSample(tex_array, tex_sampler, uv, 3u); // Sand
                
                // Blend
                let albedo = c0 * splat_weights.r + c1 * splat_weights.g + c2 * splat_weights.b + c3 * splat_weights.a;
                
                // Normal & Light Dir
                let n = normalize(in.normal);
                let l = normalize(-uniforms.sun_dir.xyz);
                
                // Lambertian diffuse (Sun lighting from normal vector)
                let diffuse_factor = max(dot(n, l), 0.0);
                let diffuse = uniforms.sun_color.rgb * diffuse_factor;
                
                // Base lighting
                let lit_color = albedo.rgb * (uniforms.ambient_color.rgb + diffuse);
                
                // Distance Fog
                let fog_start = uniforms.fog_params.x;
                let fog_end = uniforms.fog_params.y;
                let fog_factor = clamp((in.dist_to_camera - fog_start) / (fog_end - fog_start), 0.0, 1.0);
                
                let final_color = mix(lit_color, uniforms.fog_color.rgb, fog_factor);
                
                return vec4<f32>(final_color, 1.0);
            }
        ";

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Terrain Splatting Array WGSL Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Terrain Splatting Array Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                buffers: &[Some(TerrainVertex::desc())],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
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

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Terrain Splatting Array Uniform Buffer"),
            size: std::mem::size_of::<TerrainUniformsArray>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            bind_group_layout,
            uniform_buffer,
            bind_group: None,
        }
    }

    pub fn load_bind_group(
        &mut self,
        device: &wgpu::Device,
        splat_alpha_view: &wgpu::TextureView,
        splat_sampler: &wgpu::Sampler,
        tex_array_view: &wgpu::TextureView,
        tex_sampler: &wgpu::Sampler,
    ) {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Terrain Splatting Array Bind Group"),
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
                    resource: wgpu::BindingResource::TextureView(tex_array_view),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Sampler(tex_sampler),
                },
            ],
        });

        self.bind_group = Some(bind_group);
    }

    pub fn update_uniforms(
        &self,
        queue: &wgpu::Queue,
        uniforms: &TerrainUniformsArray,
    ) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[*uniforms]));
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
