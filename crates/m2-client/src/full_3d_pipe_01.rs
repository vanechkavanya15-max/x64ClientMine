use crate::wgpu_engine_context::{WgpuEngineContext, Quad2D, GpuTexture};
use crate::wgpu_render_pipeline_mgr::WgpuRenderPipelineManager;
use wgpu::util::DeviceExt;

pub struct SkyboxPass;
impl SkyboxPass {
    pub fn execute(
        render_pass: &mut wgpu::RenderPass,
        pipeline_mgr: &WgpuRenderPipelineManager,
        pass_execution_flags: &mut u32,
        pass_flag: u32,
    ) {
        *pass_execution_flags |= pass_flag;
        if let Some(pipeline) = &pipeline_mgr.skybox_pipeline {
            render_pass.set_pipeline(pipeline);
            render_pass.draw(0..6, 0..1);
        }
    }
}

pub struct TerrainSplattingPass;
impl TerrainSplattingPass {
    pub fn execute(
        render_pass: &mut wgpu::RenderPass,
        pipeline_mgr: &WgpuRenderPipelineManager,
        pass_execution_flags: &mut u32,
        pass_flag: u32,
    ) {
        *pass_execution_flags |= pass_flag;
        if let (Some(pipeline), Some(vb), Some(bg)) = (
            &pipeline_mgr.terrain_pipeline,
            &pipeline_mgr.terrain_vertex_buffer,
            &pipeline_mgr.terrain_bind_group,
        ) {
            render_pass.set_pipeline(pipeline);
            render_pass.set_bind_group(0, bg, &[]);
            render_pass.set_vertex_buffer(0, vb.slice(..));
            render_pass.draw(0..pipeline_mgr.terrain_vertex_count, 0..1);
        }
    }
}

// SpeedTree was combined or next. Wait, requirement says: SkyboxPass -> TerrainSplattingPass -> WaterPlanePass -> SkeletalActorPass -> ParticleBillboardPass -> TextTailPass -> UI2DPass
// So I will make them match exactly.

pub struct WaterPlanePass;
impl WaterPlanePass {
    pub fn execute(
        render_pass: &mut wgpu::RenderPass,
        pipeline_mgr: &WgpuRenderPipelineManager,
        pass_execution_flags: &mut u32,
        pass_flag: u32,
    ) {
        *pass_execution_flags |= pass_flag;
        if let (Some(pipeline), Some(vb)) = (&pipeline_mgr.water_pipeline, &pipeline_mgr.water_vertex_buffer) {
            render_pass.set_pipeline(pipeline);
            render_pass.set_vertex_buffer(0, vb.slice(..));
            render_pass.draw(0..pipeline_mgr.water_vertex_count, 0..1);
        }
    }
}

pub struct SkeletalActorPass;
impl SkeletalActorPass {
    pub fn execute(
        render_pass: &mut wgpu::RenderPass,
        pipeline_mgr: &WgpuRenderPipelineManager,
        pass_execution_flags: &mut u32,
        pass_flag: u32,
    ) {
        *pass_execution_flags |= pass_flag;
        if let (Some(pipeline), Some(vb)) = (&pipeline_mgr.character_pipeline, &pipeline_mgr.character_vertex_buffer) {
            render_pass.set_pipeline(pipeline);
            render_pass.set_vertex_buffer(0, vb.slice(..));
            render_pass.draw(0..pipeline_mgr.character_vertex_count, 0..1);
        }
    }
}

pub struct ParticleBillboardPass;
impl ParticleBillboardPass {
    pub fn execute(
        render_pass: &mut wgpu::RenderPass,
        pipeline_mgr: &WgpuRenderPipelineManager,
        pass_execution_flags: &mut u32,
        pass_flag: u32,
    ) {
        *pass_execution_flags |= pass_flag;
        if let (Some(pipeline), Some(vb)) = (&pipeline_mgr.magic_effect_pipeline, &pipeline_mgr.magic_effect_vertex_buffer) {
            render_pass.set_pipeline(pipeline);
            render_pass.set_vertex_buffer(0, vb.slice(..));
            render_pass.draw(0..pipeline_mgr.magic_effect_vertex_count, 0..1);
        }
        if let (Some(pipeline), Some(vb)) = (&pipeline_mgr.projectile_pipeline, &pipeline_mgr.projectile_vertex_buffer) {
            render_pass.set_pipeline(pipeline);
            render_pass.set_vertex_buffer(0, vb.slice(..));
            render_pass.draw(0..pipeline_mgr.projectile_vertex_count, 0..1);
        }
    }
}

pub struct TextTailPass;
impl TextTailPass {
    pub fn execute(
        render_pass: &mut wgpu::RenderPass,
        pipeline_mgr: &WgpuRenderPipelineManager,
        pass_execution_flags: &mut u32,
        pass_flag: u32,
    ) {
        *pass_execution_flags |= pass_flag;
        if let (Some(pipeline), Some(vb)) = (&pipeline_mgr.text_tail_pipeline, &pipeline_mgr.text_tail_vertex_buffer) {
            render_pass.set_pipeline(pipeline);
            render_pass.set_vertex_buffer(0, vb.slice(..));
            render_pass.draw(0..pipeline_mgr.text_tail_vertex_count, 0..1);
        }
    }
}

pub struct UI2DPass;
impl UI2DPass {
    pub fn execute<'a>(
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        ctx: &WgpuEngineContext,
        ui_batches: &[(&GpuTexture, &[Quad2D])],
        pass_execution_flags: &mut u32,
        pass_flag: u32,
    ) {
        *pass_execution_flags |= pass_flag;
        
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
                    view,
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
    }
}
