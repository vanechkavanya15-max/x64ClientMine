use wgpu;
use glam::{Mat4, Quat, Vec3};
use crate::wgpu_skeletal_mesh_renderer::{SkeletalUniforms, WgpuSkeletalMeshRenderer};
use crate::wgpu_rendertarget_integration::RenderTarget;
use crate::ui_texture_mgr::UiImageResource;

/// Struktura odpowiedzialna za renderowanie modelu aktora 3D do tekstury wgpu
pub struct ActorPreviewRenderer {
    pub depth_texture: wgpu::Texture,
    pub depth_view: wgpu::TextureView,
    pub width: u32,
    pub height: u32,
}

impl ActorPreviewRenderer {
    /// Tworzy nowy kontekst renderowania podgladu
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("ActorPreview_DepthTexture"),
            size: wgpu::Extent3d {
                width: width.max(1),
                height: height.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            depth_texture,
            depth_view,
            width,
            height,
        }
    }

    /// Renderuje aktora do przekazanego RenderTarget i zwraca jego reprezentacje dla UI
    pub fn render_actor(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target: &RenderTarget,
        renderer: &WgpuSkeletalMeshRenderer,
        uniform_buffer: &wgpu::Buffer,
        uniform_bind_group: &wgpu::BindGroup,
        vertex_buffer: &wgpu::Buffer,
        index_buffer: &wgpu::Buffer,
        texture_bind_group: &wgpu::BindGroup,
        num_indices: u32,
        rotation_y_deg: f32,
    ) -> UiImageResource {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("ActorPreview_Encoder"),
        });

        // Konfiguracja kamery dla podgladu
        let eye = Vec3::new(0.0, -300.0, 100.0);
        let target_pos = Vec3::new(0.0, 0.0, 100.0);
        let up = Vec3::new(0.0, 0.0, 1.0);
        let fov_y = 45.0f32;

        let rot_quat = Quat::from_rotation_z(rotation_y_deg.to_radians());
        let model = Mat4::from_scale_rotation_translation(
            Vec3::ONE,
            rot_quat,
            Vec3::ZERO,
        );

        let aspect = if self.height > 0 {
            self.width as f32 / self.height as f32
        } else {
            1.0
        };

        let proj = Mat4::perspective_rh(fov_y.to_radians(), aspect, 0.1, 10000.0);
        let view = Mat4::look_at_rh(eye, target_pos, up);
        let view_proj = proj * view;

        let mut bone_matrices = [[[0.0; 4]; 4]; 128];
        let model_array = model.to_cols_array_2d();
        for bone_matrix in bone_matrices.iter_mut() {
            *bone_matrix = model_array;
        }

        let uniforms = SkeletalUniforms {
            view_proj: view_proj.to_cols_array_2d(),
            light_dir: [0.0, 0.0, -1.0, 0.5],
            light_color: [1.0, 1.0, 1.0, 1.0],
            bone_matrices,
        };

        queue.write_buffer(uniform_buffer, 0, bytemuck::bytes_of(&uniforms));

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ActorPreview_Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &target.texture.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
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

            renderer.draw(
                &mut render_pass,
                vertex_buffer,
                index_buffer,
                uniform_bind_group,
                texture_bind_group,
                num_indices,
            );
        }

        queue.submit(std::iter::once(encoder.finish()));

        target.as_ui_resource()
    }
}
