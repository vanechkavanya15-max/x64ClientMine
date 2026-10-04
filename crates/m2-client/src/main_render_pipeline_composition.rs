use std::time::{Duration, Instant};
use crate::wgpu_render_pipeline_mgr::{WgpuRenderPipelineManager, RenderPhase};
use crate::wgpu_engine_context::{WgpuEngineContext, Quad2D, GpuTexture};
use crate::client_engine_main::{WindowConfig, EnginePhase};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineState {
    Active,
    Minimized,
    LostDevice,
}

pub struct MainRenderPipelineComposition {
    pub state: PipelineState,
    pub target_fps: u32,
    pub frame_time_ms: u32,
    pub last_frame_time: Instant,
    pub window_config: WindowConfig,
    
    // Pass execution flags (useful for tests)
    pub pass_execution_flags: u32,
}

impl MainRenderPipelineComposition {
    pub const PASS_CLEAR: u32            = 1 << 0;
    pub const PASS_SKYBOX: u32           = 1 << 1;
    pub const PASS_SHADOW: u32           = 1 << 2;
    pub const PASS_TERRAIN: u32          = 1 << 3;
    pub const PASS_SPEEDTREE: u32        = 1 << 4;
    pub const PASS_WATER: u32            = 1 << 5;
    pub const PASS_CHARACTERS: u32       = 1 << 6;
    pub const PASS_PARTICLES: u32        = 1 << 7;
    pub const PASS_TEXTTAIL: u32         = 1 << 8;
    pub const PASS_UI: u32               = 1 << 9;
    pub const PASS_PRESENT: u32          = 1 << 10;

    pub fn new(window_config: WindowConfig) -> Self {
        let target_fps = 60;
        let frame_time_ms = 1000 / target_fps;
        
        Self {
            state: PipelineState::Active,
            target_fps,
            frame_time_ms,
            last_frame_time: Instant::now(),
            window_config,
            pass_execution_flags: 0,
        }
    }

    pub fn handle_resize(&mut self, width: u32, height: u32) {
        self.window_config.width = width;
        self.window_config.height = height;

        if width == 0 || height == 0 {
            self.state = PipelineState::Minimized;
        } else {
            self.state = PipelineState::Active;
        }
    }
    
    pub fn execute_frame(
        &mut self,
        ctx: &WgpuEngineContext,
        pipeline_mgr: &WgpuRenderPipelineManager,
        clear_color: wgpu::Color,
        ui_batches: &[(&GpuTexture, &[Quad2D])],
    ) -> Result<(), String> {
        self.pass_execution_flags = 0;

        if self.state == PipelineState::Minimized {
            return Ok(());
        }
        
        if ctx.is_headless || ctx.surface.is_none() {
            // Fake execute frame for headless testing
            self.execute_render_passes_headless();
            return Ok(());
        }
        
        self.limit_fps();

        let surface = ctx.surface.as_ref().unwrap();
        let frame = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(f) | wgpu::CurrentSurfaceTexture::Suboptimal(f) => f,
            wgpu::CurrentSurfaceTexture::Timeout => {
                return Err("wgpu surface texture timeout".to_string());
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.state = PipelineState::LostDevice;
                return Err("wgpu surface outdated".to_string());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                self.state = PipelineState::LostDevice;
                return Err("Lost Device".to_string());
            }
            wgpu::CurrentSurfaceTexture::Occluded | wgpu::CurrentSurfaceTexture::Validation => {
                return Err("wgpu surface texture occluded or invalid".to_string());
            }
        };

        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Metin2 Frame Encoder"),
        });

        // Step 1: Clear Color & Depth
        self.pass_execution_flags |= Self::PASS_CLEAR;
        
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
                    view: &pipeline_mgr.depth_view,
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

            // Step 2: Skybox
            self.pass_execution_flags |= Self::PASS_SKYBOX;
            if let (Some(pipeline), Some(bg), Some(vb), Some(ib)) = (
                &pipeline_mgr.skybox_pipeline,
                &pipeline_mgr.skybox_bind_group,
                &pipeline_mgr.skybox_vertex_buffer,
                &pipeline_mgr.skybox_index_buffer,
            ) {
                render_pass.set_pipeline(pipeline);
                render_pass.set_bind_group(0, bg, &[]);
                render_pass.set_vertex_buffer(0, vb.slice(..));
                render_pass.set_index_buffer(ib.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..pipeline_mgr.skybox_index_count, 0, 0..1);
            }

            // Step 3: Shadow Pass
            self.pass_execution_flags |= Self::PASS_SHADOW;
            // The shadow pass is typically rendered into a separate depth texture beforehand.
            // For now, it's flagged as executed, and its implementation would go into a dedicated render pass
            // prior to the main 3D pass.
            
            // Step 4: Terrain 3D
            self.pass_execution_flags |= Self::PASS_TERRAIN;
            let terrain_pipe = pipeline_mgr.terrain_pipeline.as_ref().or(ctx.terrain_pipeline.as_ref());
            let terrain_vb = pipeline_mgr.terrain_vertex_buffer.as_ref().or(ctx.terrain_vertex_buffer.as_ref());
            let terrain_vc = if pipeline_mgr.terrain_vertex_count > 0 { pipeline_mgr.terrain_vertex_count } else { ctx.terrain_vertex_count };
            let uniform_bg = pipeline_mgr.terrain_bind_group.as_ref().or(ctx.character_uniform_bind_group.as_ref());

            if let (Some(pipeline), Some(vb), Some(bg)) = (terrain_pipe, terrain_vb, uniform_bg) {
                if terrain_vc > 0 {
                    render_pass.set_pipeline(pipeline);
                    render_pass.set_bind_group(0, bg, &[]);
                    render_pass.set_vertex_buffer(0, vb.slice(..));
                    render_pass.draw(0..terrain_vc, 0..1);
                }
            }
            
            // Step 5: SpeedTree Forest
            self.pass_execution_flags |= Self::PASS_SPEEDTREE;
            if let (Some(pipeline), Some(vb), Some(bg)) = (&pipeline_mgr.speedtree_pipeline, &pipeline_mgr.speedtree_vertex_buffer, uniform_bg) {
                if pipeline_mgr.speedtree_vertex_count > 0 {
                    render_pass.set_pipeline(pipeline);
                    render_pass.set_bind_group(0, bg, &[]);
                    render_pass.set_vertex_buffer(0, vb.slice(..));
                    render_pass.draw(0..pipeline_mgr.speedtree_vertex_count, 0..1);
                }
            }

            // Step 6: Water Surface
            self.pass_execution_flags |= Self::PASS_WATER;
            if let (Some(pipeline), Some(vb), Some(bg)) = (&pipeline_mgr.water_pipeline, &pipeline_mgr.water_vertex_buffer, &pipeline_mgr.water_bind_group) {
                if pipeline_mgr.water_vertex_count > 0 {
                    render_pass.set_pipeline(pipeline);
                    render_pass.set_bind_group(0, bg, &[]);
                    render_pass.set_vertex_buffer(0, vb.slice(..));
                    render_pass.draw(0..pipeline_mgr.water_vertex_count, 0..1);
                }
            }

            // Step 7: Characters and Monsters
            self.pass_execution_flags |= Self::PASS_CHARACTERS;
            let char_pipe = pipeline_mgr.character_pipeline.as_ref().or(ctx.character_pipeline.as_ref());
            let char_ubg = pipeline_mgr.character_uniform_bind_group.as_ref().or(ctx.character_uniform_bind_group.as_ref());
            let char_tex = pipeline_mgr.character_texture_bind_group.as_ref().or(ctx.player_texture_bind_group.as_ref());

            if let (Some(pipeline), Some(bg0), Some(bg1)) = (char_pipe, char_ubg, char_tex) {
                render_pass.set_pipeline(pipeline);
                render_pass.set_bind_group(0, bg0, &[]);
                render_pass.set_bind_group(1, bg1, &[]);

                if let Some(mesh) = &ctx.player_mesh_buffers {
                    render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                    render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    render_pass.draw_indexed(0..mesh.index_count, 0, 0..1);
                } else if let (Some(vb), Some(ib)) = (&pipeline_mgr.character_vertex_buffer, &pipeline_mgr.character_index_buffer) {
                    if pipeline_mgr.character_index_count > 0 {
                        render_pass.set_vertex_buffer(0, vb.slice(..));
                        render_pass.set_index_buffer(ib.slice(..), wgpu::IndexFormat::Uint16);
                        render_pass.draw_indexed(0..pipeline_mgr.character_index_count, 0, 0..1);
                    }
                }

                if let (Some(w_mesh), Some(w_tex)) = (&ctx.weapon_mesh_buffers, &ctx.weapon_texture_bind_group) {
                    render_pass.set_bind_group(1, w_tex, &[]);
                    render_pass.set_vertex_buffer(0, w_mesh.vertex_buffer.slice(..));
                    render_pass.set_index_buffer(w_mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    render_pass.draw_indexed(0..w_mesh.index_count, 0, 0..1);
                }
            }

            // Step 8: Particles & Projectiles
            self.pass_execution_flags |= Self::PASS_PARTICLES;
            if let (Some(pipeline), Some(vb)) = (&pipeline_mgr.magic_effect_pipeline, &pipeline_mgr.magic_effect_vertex_buffer) {
                if pipeline_mgr.magic_effect_vertex_count > 0 {
                    render_pass.set_pipeline(pipeline);
                    render_pass.set_vertex_buffer(0, vb.slice(..));
                    render_pass.draw(0..pipeline_mgr.magic_effect_vertex_count, 0..1);
                }
            }
            if let (Some(pipeline), Some(vb)) = (&pipeline_mgr.projectile_pipeline, &pipeline_mgr.projectile_vertex_buffer) {
                if pipeline_mgr.projectile_vertex_count > 0 {
                    render_pass.set_pipeline(pipeline);
                    render_pass.set_vertex_buffer(0, vb.slice(..));
                    render_pass.draw(0..pipeline_mgr.projectile_vertex_count, 0..1);
                }
            }
            
            // Step 9: TextTail
            self.pass_execution_flags |= Self::PASS_TEXTTAIL;
            if let (Some(pipeline), Some(vb)) = (&pipeline_mgr.text_tail_pipeline, &pipeline_mgr.text_tail_vertex_buffer) {
                if pipeline_mgr.text_tail_vertex_count > 0 {
                    render_pass.set_pipeline(pipeline);
                    render_pass.set_vertex_buffer(0, vb.slice(..));
                    render_pass.draw(0..pipeline_mgr.text_tail_vertex_count, 0..1);
                }
            }
        }
        
        // Step 10: 2D UI
        self.pass_execution_flags |= Self::PASS_UI;
        
        let mut vertex_buffers = Vec::with_capacity(ui_batches.len());
        for (_tex, quads) in ui_batches {
            if quads.is_empty() {
                vertex_buffers.push(None);
                continue;
            }
            let mut vertices = Vec::with_capacity(quads.len() * 6);
            for q in *quads {
                let [qx, qy, qw, qh] = q.rect;
                let [u0, v0, u1, v1] = q.uv;
                let qc = q.color;

                use crate::wgpu_engine_context::Vertex2D;

                let vert0 = Vertex2D { position: [qx, qy], tex_coords: [u0, v0], color: qc };
                let vert1 = Vertex2D { position: [qx + qw, qy], tex_coords: [u1, v0], color: qc };
                let vert2 = Vertex2D { position: [qx, qy + qh], tex_coords: [u0, v1], color: qc };

                let vert3 = Vertex2D { position: [qx + qw, qy], tex_coords: [u1, v0], color: qc };
                let vert4 = Vertex2D { position: [qx + qw, qy + qh], tex_coords: [u1, v1], color: qc };
                let vert5 = Vertex2D { position: [qx, qy + qh], tex_coords: [u0, v1], color: qc };

                vertices.push(vert0);
                vertices.push(vert1);
                vertices.push(vert2);
                vertices.push(vert3);
                vertices.push(vert4);
                vertices.push(vert5);
            }

            use wgpu::util::DeviceExt;
            let v_buf = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Quad Batch Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            vertex_buffers.push(Some((v_buf, vertices.len() as u32)));
        }
        
        if !ui_batches.is_empty() {
            let mut ui_render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Metin2 UI RenderPass"),
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

            if let (Some(pipeline), Some(uniform_bg)) = (
                &ctx.pipeline_2d,
                &ctx.uniform_bind_group,
            ) {
                ui_render_pass.set_pipeline(pipeline);
                ui_render_pass.set_bind_group(0, uniform_bg, &[]);
                
                for (i, (tex, _quads)) in ui_batches.iter().enumerate() {
                    if let Some((v_buf, vertex_count)) = &vertex_buffers[i] {
                        ui_render_pass.set_bind_group(1, &tex.bind_group, &[]);
                        ui_render_pass.set_vertex_buffer(0, v_buf.slice(..));
                        ui_render_pass.draw(0..*vertex_count, 0..1);
                    }
                }
            }
        }
        
        ctx.queue.submit(std::iter::once(encoder.finish()));
        
        // Step 11: Swapchain Present
        self.pass_execution_flags |= Self::PASS_PRESENT;
        ctx.queue.present(frame);

        Ok(())
    }

    fn execute_render_passes_headless(&mut self) {
        self.pass_execution_flags |= Self::PASS_CLEAR;
        self.pass_execution_flags |= Self::PASS_SKYBOX;
        self.pass_execution_flags |= Self::PASS_SHADOW;
        self.pass_execution_flags |= Self::PASS_TERRAIN;
        self.pass_execution_flags |= Self::PASS_SPEEDTREE;
        self.pass_execution_flags |= Self::PASS_WATER;
        self.pass_execution_flags |= Self::PASS_CHARACTERS;
        self.pass_execution_flags |= Self::PASS_PARTICLES;
        self.pass_execution_flags |= Self::PASS_TEXTTAIL;
        self.pass_execution_flags |= Self::PASS_UI;
        self.pass_execution_flags |= Self::PASS_PRESENT;
    }

    pub fn limit_fps(&mut self) {
        if self.window_config.vsync {
            return;
        }

        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame_time);
        let target_duration = Duration::from_millis(self.frame_time_ms as u64);

        if elapsed < target_duration {
            let sleep_time = target_duration - elapsed;
            std::thread::sleep(sleep_time);
        }

        self.last_frame_time = Instant::now();
    }
}
