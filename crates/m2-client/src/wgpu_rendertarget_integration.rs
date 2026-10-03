use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use m2_client::wgpu_engine_context::{GpuTexture, WgpuEngineContext};
use m2_client::ui_texture_mgr::UiImageResource;

/// Reprezentacja kamery specyficzna dla pojedynczego RenderTargetu.
#[derive(Clone, Debug)]
pub struct RenderTargetCamera {
    pub eye: glam::Vec3,
    pub target: glam::Vec3,
    pub up: glam::Vec3,
    pub fov_y: f32,
}

impl Default for RenderTargetCamera {
    fn default() -> Self {
        Self {
            eye: glam::Vec3::new(0.0, -1000.0, 100.0),
            target: glam::Vec3::new(0.0, 0.0, 100.0),
            up: glam::Vec3::new(0.0, 0.0, 1.0),
            fov_y: 45.0,
        }
    }
}

/// Główna struktura RenderTarget przechowująca teksturę offscreen i lokalną kamerę.
pub struct RenderTarget {
    pub id: u8,
    pub width: u32,
    pub height: u32,
    pub camera: RenderTargetCamera,
    pub texture: Arc<GpuTexture>,
    pub visibility: bool,
}

impl RenderTarget {
    /// Tworzy nowy RenderTarget (teksturę offscreen) o podanych wymiarach.
    pub fn new(ctx: &WgpuEngineContext, id: u8, width: u32, height: u32) -> Result<Self, String> {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        // Tworzymy teksturę z flagami RENDER_ATTACHMENT (żeby w nią rysować) 
        // oraz TEXTURE_BINDING (żeby użyć w UI).
        let texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&format!("RenderTarget_{}", id)),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("RenderTargetSampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        // Potrzebujemy układu BindGroup dla tej tekstury, by UiTextureManager mógł ją odczytać.
        // Używamy układu, który przyjmuje teksturę i sampler (zgodne z `wgpu_engine_context.rs`).
        let bind_group_layout = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("RenderTarget_BindGroupLayout"),
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
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("RenderTarget_BindGroup_{}", id)),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let gpu_texture = GpuTexture {
            texture,
            view,
            bind_group,
            width,
            height,
        };

        Ok(Self {
            id,
            width,
            height,
            camera: RenderTargetCamera::default(),
            texture: Arc::new(gpu_texture),
            visibility: false,
        })
    }

    /// Oczyszcza RenderTarget przeźroczystym kolorem (przydatne jako tło do renderu modelu).
    pub fn clear(&self, ctx: &WgpuEngineContext) {
        let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some(&format!("RenderTarget_Clear_{}", self.id)),
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some(&format!("RenderTarget_ClearPass_{}", self.id)),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.texture.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None, // Ignore for clear
                })],
                depth_stencil_attachment: None, // No depth for simple clear atm
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
        }

        ctx.queue.submit(std::iter::once(encoder.finish()));
    }

    /// Konwertuje RenderTarget do UiImageResource, dzięki czemu Python UI może to wyświetlić.
    pub fn as_ui_resource(&self) -> UiImageResource {
        UiImageResource {
            texture: self.texture.clone(),
            uv: [0.0, 0.0, 1.0, 1.0],
            width: self.width,
            height: self.height,
        }
    }
}

/// Menadżer RenderTargetów utrzymujący ich cykl życia i identyfikatory
pub struct RenderTargetManager {
    targets: HashMap<u8, RenderTarget>,
}

impl RenderTargetManager {
    pub fn new() -> Self {
        Self {
            targets: HashMap::new(),
        }
    }

    /// Tworzy nowy cel o określonym ID
    pub fn create_target(&mut self, ctx: &WgpuEngineContext, id: u8, width: u32, height: u32) -> Result<(), String> {
        if self.targets.contains_key(&id) {
            return Err(format!("RenderTarget z ID {} już istnieje", id));
        }

        let target = RenderTarget::new(ctx, id, width, height)?;
        self.targets.insert(id, target);
        Ok(())
    }

    /// Usuwa z pamięci
    pub fn delete_target(&mut self, id: u8) {
        self.targets.remove(&id);
    }

    /// Zwraca referencję do RenderTargetu
    pub fn get_target(&self, id: u8) -> Option<&RenderTarget> {
        self.targets.get(&id)
    }

    /// Zwraca modyfikowalną referencję do RenderTargetu
    pub fn get_target_mut(&mut self, id: u8) -> Option<&mut RenderTarget> {
        self.targets.get_mut(&id)
    }
    
    /// Przekazuje zasób UI (obraz do wyświetlenia np w oknie uiRenderTarget.py)
    pub fn get_target_ui_resource(&self, id: u8) -> Option<UiImageResource> {
        self.targets.get(&id).map(|t| t.as_ui_resource())
    }
}
