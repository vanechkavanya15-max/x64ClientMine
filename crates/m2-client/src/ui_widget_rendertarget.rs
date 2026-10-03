use std::sync::Arc;
use wgpu;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct ActorRenderPreview {
    pub armor_vnum: u32,
    pub rotation_y: f32,
}

impl ActorRenderPreview {
    pub fn new(armor_vnum: u32, rotation_y: f32) -> Self {
        Self {
            armor_vnum,
            rotation_y,
        }
    }

    pub fn set_armor(&mut self, vnum: u32) {
        self.armor_vnum = vnum;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation_y = rotation;
    }
}

pub struct RenderTarget {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub width: u32,
    pub height: u32,
}

impl RenderTarget {
    pub fn register_render_target(
        device: &wgpu::Device,
        width: u32,
        height: u32,
    ) -> Result<Self, String> {
        if width == 0 || height == 0 {
            return Err("Width and height must be greater than zero".to_string());
        }

        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("UI RenderTarget Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Ok(Self {
            texture,
            view,
            width,
            height,
        })
    }

    pub fn render_actor_preview(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        actor: &ActorRenderPreview,
    ) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("RenderTarget Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Actor Preview Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                ..Default::default()
            });
            
            // In a full implementation, we would bind the 3D pipeline and draw the actor mesh here,
            // applying the actor.rotation_y to the transform matrices and using the actor.armor_vnum for materials.
            // As this is a pure logic wrapper for the UI offscreen target, we just perform the clear/store pass.
        }

        queue.submit(std::iter::once(encoder.finish()));
    }

    pub fn get_ui_quad_bind_group(
        &self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        sampler: &wgpu::Sampler,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("UI Quad RenderTarget Bind Group"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        })
    }
}
