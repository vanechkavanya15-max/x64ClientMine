use crate::wgpu_engine_context::{GpuTexture, Quad2D, Vertex2D, WgpuEngineContext};
use wgpu::util::DeviceExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderPhase {
    SkyBoxAndAmbient = 1,
    Terrain3D = 2,
    Water3D = 3,
    SpeedTree = 4,
    CharactersAndMonsters = 5,
    GroundItems = 6,
    MagicEffects = 7,
    Projectiles = 8,
    TextTail = 9,
    UserInterface2D = 10,
}

pub struct WgpuRenderPipelineManager {
    pub depth_texture: wgpu::Texture,
    pub depth_view: wgpu::TextureView,

    pub skybox_pipeline: Option<wgpu::RenderPipeline>,
    pub skybox_bind_group: Option<wgpu::BindGroup>,
    pub terrain_pipeline: Option<wgpu::RenderPipeline>,
    pub water_pipeline: Option<wgpu::RenderPipeline>,
    pub water_bind_group: Option<wgpu::BindGroup>,
    pub speedtree_pipeline: Option<wgpu::RenderPipeline>,
    pub character_pipeline: Option<wgpu::RenderPipeline>,
    pub ground_item_pipeline: Option<wgpu::RenderPipeline>,
    pub magic_effect_pipeline: Option<wgpu::RenderPipeline>,
    pub projectile_pipeline: Option<wgpu::RenderPipeline>,
    pub text_tail_pipeline: Option<wgpu::RenderPipeline>,

    pub terrain_vertex_buffer: Option<wgpu::Buffer>,
    pub terrain_bind_group: Option<wgpu::BindGroup>,
    pub terrain_vertex_count: u32,

    pub water_vertex_buffer: Option<wgpu::Buffer>,
    pub water_vertex_count: u32,

    pub speedtree_vertex_buffer: Option<wgpu::Buffer>,
    pub speedtree_vertex_count: u32,

    pub character_vertex_buffer: Option<wgpu::Buffer>,
    pub character_vertex_count: u32,
    pub character_index_buffer: Option<wgpu::Buffer>,
    pub character_index_count: u32,
    pub character_uniform_bind_group: Option<wgpu::BindGroup>,
    pub character_texture_bind_group: Option<wgpu::BindGroup>,

    pub ground_item_vertex_buffer: Option<wgpu::Buffer>,
    pub ground_item_vertex_count: u32,

    pub magic_effect_vertex_buffer: Option<wgpu::Buffer>,
    pub magic_effect_vertex_count: u32,

    pub projectile_vertex_buffer: Option<wgpu::Buffer>,
    pub projectile_vertex_count: u32,

    pub text_tail_vertex_buffer: Option<wgpu::Buffer>,
    pub text_tail_vertex_count: u32,
}

impl WgpuRenderPipelineManager {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let size = wgpu::Extent3d {
            width: width.max(1),
            height: height.max(1),
            depth_or_array_layers: 1,
        };

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let (char_pipe, _char_ub, char_ubg, u_layout, _tex_layout) =
            WgpuEngineContext::setup_character_pipeline(device, wgpu::TextureFormat::Rgba8UnormSrgb);
        let warrior = crate::character_model_vfs_loader::load_warrior_model(device).ok();
        let (char_vb, char_vc, char_ib, char_ic) = if let Some(m) = warrior {
            (
                Some(m.gpu_buffers.vertex_buffer),
                m.gpu_buffers.vertex_count,
                Some(m.gpu_buffers.index_buffer),
                m.gpu_buffers.index_count,
            )
        } else {
            (None, 0, None, 0)
        };

        // 1. Inicjalizacja skybox_pipeline
        let skybox_renderer = crate::wgpu_skybox_renderer::SkyboxRenderer::new(
            device,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            Self::DEPTH_FORMAT,
        );
        let skybox_pipeline = Some(skybox_renderer.pipeline);
        let skybox_bind_group = Some(skybox_renderer.uniform_bind_group);

        // 2. Inicjalizacja terrain_pipeline oraz bufora wierzcholkow terenu
        let terrain_pipeline = Some(crate::wgpu_terrain_renderer::create_terrain_pipeline(
            device,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            &u_layout,
        ));
        let terrain_bind_group = Some(char_ubg.clone());
        let (terrain_vertex_buffer, terrain_vertex_count) = if let Some((buf, count)) = WgpuEngineContext::load_terrain_mesh_from_vfs(device) {
            (Some(buf), count)
        } else {
            let heights = vec![0.0f32; 33 * 33];
            let raw_map = crate::terrain_height_patch::RawHeightMap::new(33, 33, heights);
            let verts = crate::wgpu_terrain_renderer::generate_terrain_mesh(&raw_map);
            let count = verts.len() as u32;
            let buf = if !verts.is_empty() {
                Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Terrain Vertex Buffer"),
                    contents: bytemuck::cast_slice(&verts),
                    usage: wgpu::BufferUsages::VERTEX,
                }))
            } else {
                None
            };
            (buf, count)
        };

        // 3. Inicjalizacja water_pipeline
        let water_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Water Bind Group Layout"),
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
        let water_renderer = crate::wgpu_water_plane_renderer::WgpuWaterPlaneRenderer::new(
            device,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            &water_bind_group_layout,
        );
        let water_pipeline = Some(water_renderer.pipeline);
        let water_bind_group = Some(water_renderer.bind_group);

        let water_verts = [
            crate::wgpu_water_plane_renderer::WaterVertex { position: [-5000.0, -5000.0, 0.0], diffuse: 0xFFFFFFFF },
            crate::wgpu_water_plane_renderer::WaterVertex { position: [ 5000.0, -5000.0, 0.0], diffuse: 0xFFFFFFFF },
            crate::wgpu_water_plane_renderer::WaterVertex { position: [ 5000.0,  5000.0, 0.0], diffuse: 0xFFFFFFFF },
            crate::wgpu_water_plane_renderer::WaterVertex { position: [-5000.0, -5000.0, 0.0], diffuse: 0xFFFFFFFF },
            crate::wgpu_water_plane_renderer::WaterVertex { position: [ 5000.0,  5000.0, 0.0], diffuse: 0xFFFFFFFF },
            crate::wgpu_water_plane_renderer::WaterVertex { position: [-5000.0,  5000.0, 0.0], diffuse: 0xFFFFFFFF },
        ];
        let water_vertex_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Water Plane Vertex Buffer"),
            contents: bytemuck::cast_slice(&water_verts),
            usage: wgpu::BufferUsages::VERTEX,
        }));
        let water_vertex_count = water_verts.len() as u32;

        // 4. Inicjalizacja text_tail_pipeline
        let text_tail_pipeline = Some(Self::create_text_tail_pipeline(
            device,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            Self::DEPTH_FORMAT,
        ));

        Self {
            depth_texture,
            depth_view,
            skybox_pipeline,
            skybox_bind_group,
            terrain_pipeline,
            water_pipeline,
            water_bind_group,
            speedtree_pipeline: None,
            character_pipeline: Some(char_pipe),
            ground_item_pipeline: None,
            magic_effect_pipeline: None,
            projectile_pipeline: None,
            text_tail_pipeline,

            terrain_vertex_buffer,
            terrain_bind_group,
            terrain_vertex_count,
            water_vertex_buffer,
            water_vertex_count,
            speedtree_vertex_buffer: None,
            speedtree_vertex_count: 0,
            character_vertex_buffer: char_vb,
            character_vertex_count: char_vc,
            character_index_buffer: char_ib,
            character_index_count: char_ic,
            character_uniform_bind_group: Some(char_ubg),
            character_texture_bind_group: None,
            ground_item_vertex_buffer: None,
            ground_item_vertex_count: 0,
            magic_effect_vertex_buffer: None,
            magic_effect_vertex_count: 0,
            projectile_vertex_buffer: None,
            projectile_vertex_count: 0,
            text_tail_vertex_buffer: None,
            text_tail_vertex_count: 0,
        }
    }

    pub fn create_skybox_pipeline(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        depth_format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        let skybox_renderer = crate::wgpu_skybox_renderer::SkyboxRenderer::new(device, format, depth_format);
        skybox_renderer.pipeline
    }

    pub fn create_text_tail_pipeline(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        depth_format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("TextTail WGSL Shader"),
            source: wgpu::ShaderSource::Wgsl(r#"
                struct VertexInput {
                    @location(0) position: vec2<f32>,
                    @location(1) tex_coords: vec2<f32>,
                    @location(2) color: vec4<f32>,
                };
                struct VertexOutput {
                    @builtin(position) clip_position: vec4<f32>,
                    @location(0) tex_coords: vec2<f32>,
                    @location(1) color: vec4<f32>,
                };
                @vertex
                fn vs_main(model: VertexInput) -> VertexOutput {
                    var out: VertexOutput;
                    out.clip_position = vec4<f32>(model.position, 0.0, 1.0);
                    out.tex_coords = model.tex_coords;
                    out.color = model.color;
                    return out;
                }
                @fragment
                fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
                    return in.color;
                }
            "#.into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("TextTail Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("TextTail Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(crate::wgpu_engine_context::Vertex2D::desc())],
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
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth_format,
                depth_write_enabled: Some(false),
                depth_compare: Some(wgpu::CompareFunction::Always),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        })
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        if width > 0 && height > 0 {
            let size = wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            };

            self.depth_texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Depth Texture Resized"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: Self::DEPTH_FORMAT,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });

            self.depth_view = self.depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        }
    }

    pub fn execute_frame(
        &self,
        ctx: &WgpuEngineContext,
        clear_color: wgpu::Color,
        batches: &[(&GpuTexture, &[Quad2D])],
    ) -> Result<(), String> {
        if ctx.is_headless || ctx.surface.is_none() {
            return Ok(());
        }

        let surface = ctx.surface.as_ref().unwrap();
        let frame = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(f) | wgpu::CurrentSurfaceTexture::Suboptimal(f) => f,
            _ => return Ok(()),
        };

        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Metin2 Frame Encoder"),
        });

        // 3D Rendering Pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Metin2 3D RenderPass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            let phases = [
                RenderPhase::SkyBoxAndAmbient,
                RenderPhase::Terrain3D,
                RenderPhase::Water3D,
                RenderPhase::SpeedTree,
                RenderPhase::CharactersAndMonsters,
                RenderPhase::GroundItems,
                RenderPhase::MagicEffects,
                RenderPhase::Projectiles,
                RenderPhase::TextTail,
            ];

            for phase in phases {
                match phase {
                    RenderPhase::SkyBoxAndAmbient => {
                        if let Some(pipeline) = &self.skybox_pipeline {
                            render_pass.set_pipeline(pipeline);
                            if let Some(bg) = &self.skybox_bind_group {
                                render_pass.set_bind_group(0, bg, &[]);
                            }
                            render_pass.draw(0..6, 0..1);
                        }
                    }
                    RenderPhase::Terrain3D => {
                        if let (Some(pipeline), Some(vb), Some(bg)) = (
                            &self.terrain_pipeline,
                            &self.terrain_vertex_buffer,
                            &self.terrain_bind_group,
                        ) {
                            render_pass.set_pipeline(pipeline);
                            render_pass.set_bind_group(0, bg, &[]);
                            render_pass.set_vertex_buffer(0, vb.slice(..));
                            render_pass.draw(0..self.terrain_vertex_count, 0..1);
                        }
                    }
                    RenderPhase::Water3D => {
                        if let (Some(pipeline), Some(vb)) = (&self.water_pipeline, &self.water_vertex_buffer) {
                            render_pass.set_pipeline(pipeline);
                            if let Some(bg) = &self.water_bind_group {
                                render_pass.set_bind_group(0, bg, &[]);
                            }
                            render_pass.set_vertex_buffer(0, vb.slice(..));
                            render_pass.draw(0..self.water_vertex_count, 0..1);
                        }
                    }
                    RenderPhase::SpeedTree => {
                        if let (Some(pipeline), Some(vb)) = (&self.speedtree_pipeline, &self.speedtree_vertex_buffer) {
                            render_pass.set_pipeline(pipeline);
                            render_pass.set_vertex_buffer(0, vb.slice(..));
                            render_pass.draw(0..self.speedtree_vertex_count, 0..1);
                        }
                    }
                    RenderPhase::CharactersAndMonsters => {
                        if let (Some(pipeline), Some(vb)) = (&self.character_pipeline, &self.character_vertex_buffer) {
                            render_pass.set_pipeline(pipeline);
                            if let Some(bg0) = &self.character_uniform_bind_group {
                                render_pass.set_bind_group(0, bg0, &[]);
                            }
                            if let Some(bg1) = &self.character_texture_bind_group {
                                render_pass.set_bind_group(1, bg1, &[]);
                            }
                            render_pass.set_vertex_buffer(0, vb.slice(..));
                            if let Some(ib) = &self.character_index_buffer {
                                render_pass.set_index_buffer(ib.slice(..), wgpu::IndexFormat::Uint16);
                                render_pass.draw_indexed(0..self.character_index_count, 0, 0..1);
                            } else {
                                render_pass.draw(0..self.character_vertex_count, 0..1);
                            }
                        }
                    }
                    RenderPhase::GroundItems => {
                        if let (Some(pipeline), Some(vb)) = (&self.ground_item_pipeline, &self.ground_item_vertex_buffer) {
                            render_pass.set_pipeline(pipeline);
                            render_pass.set_vertex_buffer(0, vb.slice(..));
                            render_pass.draw(0..self.ground_item_vertex_count, 0..1);
                        }
                    }
                    RenderPhase::MagicEffects => {
                        if let (Some(pipeline), Some(vb)) = (&self.magic_effect_pipeline, &self.magic_effect_vertex_buffer) {
                            render_pass.set_pipeline(pipeline);
                            render_pass.set_vertex_buffer(0, vb.slice(..));
                            render_pass.draw(0..self.magic_effect_vertex_count, 0..1);
                        }
                    }
                    RenderPhase::Projectiles => {
                        if let (Some(pipeline), Some(vb)) = (&self.projectile_pipeline, &self.projectile_vertex_buffer) {
                            render_pass.set_pipeline(pipeline);
                            render_pass.set_vertex_buffer(0, vb.slice(..));
                            render_pass.draw(0..self.projectile_vertex_count, 0..1);
                        }
                    }
                    RenderPhase::TextTail => {
                        if let (Some(pipeline), Some(vb)) = (&self.text_tail_pipeline, &self.text_tail_vertex_buffer) {
                            render_pass.set_pipeline(pipeline);
                            render_pass.set_vertex_buffer(0, vb.slice(..));
                            render_pass.draw(0..self.text_tail_vertex_count, 0..1);
                        }
                    }
                    RenderPhase::UserInterface2D => unreachable!(),
                }
            }
        } // Drop 3D render pass

        // Phase 10: 2D UI Pass
        if !batches.is_empty() {
            let mut vertex_buffers = Vec::with_capacity(batches.len());
            for (_tex, quads) in batches {
                if quads.is_empty() {
                    vertex_buffers.push(None);
                    continue;
                }
                let mut vertices = Vec::with_capacity(quads.len() * 6);
                for q in *quads {
                    let [qx, qy, qw, qh] = q.rect;
                    let [u0, v0, u1, v1] = q.uv;
                    let qc = q.color;

                    vertices.push(Vertex2D { position: [qx, qy], tex_coords: [u0, v0], color: qc });
                    vertices.push(Vertex2D { position: [qx + qw, qy], tex_coords: [u1, v0], color: qc });
                    vertices.push(Vertex2D { position: [qx, qy + qh], tex_coords: [u0, v1], color: qc });
                    vertices.push(Vertex2D { position: [qx + qw, qy], tex_coords: [u1, v0], color: qc });
                    vertices.push(Vertex2D { position: [qx + qw, qy + qh], tex_coords: [u1, v1], color: qc });
                    vertices.push(Vertex2D { position: [qx, qy + qh], tex_coords: [u0, v1], color: qc });
                }

                let v_buf = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Quad Batch Vertex Buffer"),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });
                vertex_buffers.push(Some((v_buf, vertices.len() as u32)));
            }

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Metin2 2D UI RenderPass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });

                if let (Some(pipeline), Some(uniform_bg)) = (&ctx.pipeline_2d, &ctx.uniform_bind_group) {
                    render_pass.set_pipeline(pipeline);
                    render_pass.set_bind_group(0, uniform_bg, &[]);

                    for (idx, (tex, _quads)) in batches.iter().enumerate() {
                        if let Some((v_buf, vertex_count)) = &vertex_buffers[idx] {
                            render_pass.set_vertex_buffer(0, v_buf.slice(..));
                            render_pass.set_bind_group(1, &tex.bind_group, &[]);
                            render_pass.draw(0..*vertex_count, 0..1);
                        }
                    }
                }
            }
        }

        ctx.queue.submit(std::iter::once(encoder.finish()));
        
        ctx.queue.present(frame);

        Ok(())
    }
}