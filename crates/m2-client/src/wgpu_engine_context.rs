// wgpu_engine_context.rs - Modern DirectX 12 / Vulkan Rendering Pipeline for Metin2 Pure Rust
use std::sync::Arc;
use std::num::NonZeroIsize;
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle,
    RawWindowHandle, Win32WindowHandle, WindowHandle, WindowsDisplayHandle,
};
use wgpu::util::DeviceExt;

#[derive(Debug, Clone)]
pub struct WgpuWindowTarget {
    pub hwnd: isize,
    pub hinstance: Option<isize>,
}

impl HasWindowHandle for WgpuWindowTarget {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let non_zero_hwnd = NonZeroIsize::new(self.hwnd)
            .ok_or(HandleError::Unavailable)?;
        let mut handle = Win32WindowHandle::new(non_zero_hwnd);
        if let Some(hinst) = self.hinstance {
            if let Some(non_zero_hinst) = NonZeroIsize::new(hinst) {
                handle.hinstance = Some(non_zero_hinst);
            }
        }
        unsafe { Ok(WindowHandle::borrow_raw(RawWindowHandle::Win32(handle))) }
    }
}

impl HasDisplayHandle for WgpuWindowTarget {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        let handle = WindowsDisplayHandle::new();
        unsafe { Ok(DisplayHandle::borrow_raw(RawDisplayHandle::Windows(handle))) }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex2D {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ScreenUniforms {
    pub screen_size: [f32; 2],
    pub _padding: [f32; 2],
}

#[derive(Clone, Debug)]
pub struct Quad2D {
    pub rect: [f32; 4],  // [x, y, width, height] in pixels
    pub uv: [f32; 4],    // [u0, v0, u1, v1] in texture coordinates (0.0..1.0)
    pub color: [f32; 4], // RGBA float multiplier (default [1.0, 1.0, 1.0, 1.0])
}

impl Quad2D {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            rect: [x, y, w, h],
            uv: [0.0, 0.0, 1.0, 1.0],
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    pub fn with_uv(x: f32, y: f32, w: f32, h: f32, u0: f32, v0: f32, u1: f32, v1: f32) -> Self {
        Self {
            rect: [x, y, w, h],
            uv: [u0, v0, u1, v1],
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

pub struct GpuTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub bind_group: wgpu::BindGroup,
    pub width: u32,
    pub height: u32,
}

pub struct WgpuEngineContext {
    pub instance: wgpu::Instance,
    pub surface: Option<wgpu::Surface<'static>>,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: Option<wgpu::SurfaceConfiguration>,
    pub width: u32,
    pub height: u32,
    pub is_headless: bool,

    // 2D Pipeline
    pub pipeline_2d: Option<wgpu::RenderPipeline>,
    pub uniform_buffer: Option<wgpu::Buffer>,
    pub uniform_bind_group: Option<wgpu::BindGroup>,
    pub texture_bind_group_layout: Option<wgpu::BindGroupLayout>,
    pub sampler: Option<wgpu::Sampler>,
}

const SHADER_2D_WGSL: &str = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct ScreenUniforms {
    screen_size: vec2<f32>,
    padding: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: ScreenUniforms;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let ndc_x = (model.position.x / uniforms.screen_size.x) * 2.0 - 1.0;
    let ndc_y = 1.0 - (model.position.y / uniforms.screen_size.y) * 2.0;
    out.clip_position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    out.tex_coords = model.tex_coords;
    out.color = model.color;
    return out;
}

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    return tex_color * in.color;
}
"#;

impl WgpuEngineContext {
    /// Inicjalizuje nowoczesny kontekst wgpu (DirectX 12 / Vulkan) z obsluga okna Win32 HWND.
    /// W srodowiskach testowych lub bez fizycznego okna inicjalizuje tryb headless (fallback).
    pub fn new(hwnd: usize, width: u32, height: u32) -> Result<Self, String> {
        let width = if width == 0 { 800 } else { width };
        let height = if height == 0 { 600 } else { height };

        let instance = wgpu::Instance::default();

        if hwnd != 0 && hwnd != 0x12345678 {
            let target = Arc::new(WgpuWindowTarget {
                hwnd: hwnd as isize,
                hinstance: None,
            });

            let surface = instance.create_surface(target.clone())
                .map_err(|e| format!("Failed to create wgpu surface: {:?}", e))?;

            let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                ..Default::default()
            })).map_err(|e| format!("Failed to find suitable GPU adapter: {:?}", e))?;

            let (device, queue) = pollster::block_on(adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Metin2 Pure Rust GPU Device"),
                    ..Default::default()
                },
            )).map_err(|e| format!("Failed to create GPU device: {:?}", e))?;

            let mut config = surface.get_default_config(&adapter, width, height)
                .ok_or_else(|| "Failed to get default surface config".to_string())?;
            config.present_mode = wgpu::PresentMode::Fifo; // VSync

            surface.configure(&device, &config);

            let (pipeline_2d, uniform_buf, uniform_bg, tex_bg_layout, sampler) =
                Self::setup_2d_pipeline(&device, config.format, width, height);

            Ok(Self {
                instance,
                surface: Some(surface),
                adapter,
                device,
                queue,
                config: Some(config),
                width,
                height,
                is_headless: false,
                pipeline_2d: Some(pipeline_2d),
                uniform_buffer: Some(uniform_buf),
                uniform_bind_group: Some(uniform_bg),
                texture_bind_group_layout: Some(tex_bg_layout),
                sampler: Some(sampler),
            })
        } else {
            // Headless fallback dla testow jednostkowych
            let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: None,
                ..Default::default()
            })).map_err(|e| format!("Failed to find headless adapter: {:?}", e))?;

            let (device, queue) = pollster::block_on(adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Metin2 Headless Test Device"),
                    ..Default::default()
                },
            )).map_err(|e| format!("Failed to create headless device: {:?}", e))?;

            let (pipeline_2d, uniform_buf, uniform_bg, tex_bg_layout, sampler) =
                Self::setup_2d_pipeline(&device, wgpu::TextureFormat::Rgba8UnormSrgb, width, height);

            Ok(Self {
                instance,
                surface: None,
                adapter,
                device,
                queue,
                config: None,
                width,
                height,
                is_headless: true,
                pipeline_2d: Some(pipeline_2d),
                uniform_buffer: Some(uniform_buf),
                uniform_bind_group: Some(uniform_bg),
                texture_bind_group_layout: Some(tex_bg_layout),
                sampler: Some(sampler),
            })
        }
    }

    fn setup_2d_pipeline(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        width: u32,
        height: u32,
    ) -> (
        wgpu::RenderPipeline,
        wgpu::Buffer,
        wgpu::BindGroup,
        wgpu::BindGroupLayout,
        wgpu::Sampler,
    ) {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Metin2 2D Shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER_2D_WGSL.into()),
        });

        let uniforms = ScreenUniforms {
            screen_size: [width as f32, height as f32],
            _padding: [0.0, 0.0],
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Screen Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Screen Uniform Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Screen Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Texture Bind Group Layout"),
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

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("2D Pipeline Layout"),
            bind_group_layouts: &[Some(&uniform_bind_group_layout), Some(&texture_bind_group_layout)],
            immediate_size: 0,
        });

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex2D>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 8,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 16,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        };

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("2D Textured Quad Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(vertex_buffer_layout)],
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
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        (
            pipeline,
            uniform_buffer,
            uniform_bind_group,
            texture_bind_group_layout,
            sampler,
        )
    }

    /// Tworzy GPU Texture z surowych bajtow RGBA8
    pub fn create_texture_from_rgba(
        &self,
        rgba: &[u8],
        width: u32,
        height: u32,
        label: Option<&str>,
    ) -> Result<GpuTexture, String> {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let layout = self.texture_bind_group_layout.as_ref()
            .ok_or_else(|| "Texture bind group layout not initialized".to_string())?;
        let sampler = self.sampler.as_ref()
            .ok_or_else(|| "Sampler not initialized".to_string())?;

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        });

        Ok(GpuTexture {
            texture,
            view,
            bind_group,
            width,
            height,
        })
    }

    /// Tworzy GPU Texture dekodujac bezposrednio z pamieci (JPG, TGA, PNG)
    pub fn create_texture_from_asset_bytes(&self, bytes: &[u8], is_tga: bool) -> Result<GpuTexture, String> {
        if is_tga {
            let tga = crate::tga_decoder::TgaImage::from_bytes(bytes)?;
            self.create_texture_from_rgba(&tga.rgba_data, tga.width as u32, tga.height as u32, Some("TGA Texture"))
        } else {
            let img = image::load_from_memory(bytes)
                .map_err(|e| format!("Failed to decode image bytes: {:?}", e))?;
            let rgba = img.to_rgba8();
            self.create_texture_from_rgba(&rgba, img.width(), img.height(), Some("Image Texture"))
        }
    }

    /// Aktualizuje wymiary renderera po zmianie rozmiaru okna
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        if new_width > 0 && new_height > 0 {
            self.width = new_width;
            self.height = new_height;
            if let (Some(surface), Some(config)) = (&self.surface, &mut self.config) {
                config.width = new_width;
                config.height = new_height;
                surface.configure(&self.device, config);
            }
            if let Some(buf) = &self.uniform_buffer {
                let uniforms = ScreenUniforms {
                    screen_size: [new_width as f32, new_height as f32],
                    _padding: [0.0, 0.0],
                };
                self.queue.write_buffer(buf, 0, bytemuck::cast_slice(&[uniforms]));
            }
        }
    }

    /// Renderuje kompletna klatke z partiami teksturowanych kwadratow (2D textured quads)
    pub fn render_frame_with_quads(
        &mut self,
        clear_color: wgpu::Color,
        batches: &[(&GpuTexture, &[Quad2D])],
    ) -> Result<(), String> {
        if self.is_headless || self.surface.is_none() {
            return Ok(());
        }

        let surface = self.surface.as_ref().unwrap();
        let frame_status = surface.get_current_texture();

        let frame = match frame_status {
            wgpu::CurrentSurfaceTexture::Success(f) | wgpu::CurrentSurfaceTexture::Suboptimal(f) => f,
            wgpu::CurrentSurfaceTexture::Lost => {
                if let Some(config) = &self.config {
                    surface.configure(&self.device, config);
                }
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => return Ok(()),
            wgpu::CurrentSurfaceTexture::Timeout => return Ok(()),
            wgpu::CurrentSurfaceTexture::Occluded => return Ok(()),
            wgpu::CurrentSurfaceTexture::Validation => {
                return Err("Validation error getting current texture".to_string())
            }
        };

        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Metin2 2D Render Frame Encoder"),
        });

        // Przygotuj bufory wierzcholkow przed otwarciem render passa
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

            let v_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Quad Batch Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            vertex_buffers.push(Some((v_buf, vertices.len() as u32)));
        }

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Metin2 2D RenderPass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            if let (Some(pipeline), Some(uniform_bg)) = (&self.pipeline_2d, &self.uniform_bind_group) {
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

        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(frame);

        Ok(())
    }

    /// Renderuje partia klatke do bufora pamieci i zwraca obraz RgbaImage (do walidacji offscreen i testow)
    pub fn render_to_rgba_image(
        &self,
        clear_color: wgpu::Color,
        batches: &[(&GpuTexture, &[Quad2D])],
    ) -> Result<image::RgbaImage, String> {
        let width = self.width;
        let height = self.height;

        let render_target = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Offscreen Render Target"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let view = render_target.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Offscreen 2D Render Frame Encoder"),
        });

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

            let v_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Offscreen Quad Batch Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            vertex_buffers.push(Some((v_buf, vertices.len() as u32)));
        }

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Offscreen Metin2 2D RenderPass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            if let (Some(pipeline), Some(uniform_bg)) = (&self.pipeline_2d, &self.uniform_bind_group) {
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

        let unpadded_bytes_per_row = width * 4;
        let align = 256;
        let padded_bytes_per_row = (unpadded_bytes_per_row + align - 1) & !(align - 1);
        let buffer_size = (padded_bytes_per_row * height) as u64;

        let output_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Offscreen Output Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &render_target,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &output_buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_bytes_per_row),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        self.queue.submit(std::iter::once(encoder.finish()));

        let buffer_slice = output_buffer.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = tx.send(result);
        });
        let _ = self.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
        rx.recv().map_err(|e| format!("Map buffer error: {:?}", e))?
            .map_err(|e| format!("Map async failed: {:?}", e))?;

        let padded_view = buffer_slice.get_mapped_range()
            .map_err(|e| format!("Get mapped range error: {:?}", e))?;
        let padded_slice: &[u8] = &padded_view;
        let mut unpadded_data = Vec::with_capacity((width * height * 4) as usize);
        for row in 0..height {
            let start = (row * padded_bytes_per_row) as usize;
            let end = start + unpadded_bytes_per_row as usize;
            unpadded_data.extend_from_slice(&padded_slice[start..end]);
        }
        drop(padded_view);
        output_buffer.unmap();

        image::RgbaImage::from_raw(width, height, unpadded_data)
            .ok_or_else(|| "Failed to construct RgbaImage from buffer".to_string())
    }

    /// Renderuje i prezentuje czysta klatke z kolorem tla (np. ciemny motyw Metin2)
    pub fn render_clear_frame(&mut self, color: wgpu::Color) -> Result<(), String> {
        if self.is_headless || self.surface.is_none() {
            return Ok(());
        }

        let surface = self.surface.as_ref().unwrap();
        let frame_status = surface.get_current_texture();

        let frame = match frame_status {
            wgpu::CurrentSurfaceTexture::Success(f) | wgpu::CurrentSurfaceTexture::Suboptimal(f) => f,
            wgpu::CurrentSurfaceTexture::Lost => {
                if let Some(config) = &self.config {
                    surface.configure(&self.device, config);
                }
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => return Ok(()),
            wgpu::CurrentSurfaceTexture::Timeout => return Ok(()),
            wgpu::CurrentSurfaceTexture::Occluded => return Ok(()),
            wgpu::CurrentSurfaceTexture::Validation => {
                return Err("Validation error getting current texture".to_string())
            }
        };

        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Metin2 Clear Frame Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Metin2 Clear RenderPass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(color),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(frame);

        Ok(())
    }

    /// Informacja diagnostyczna o aktywnym API graficznym (DirectX 12 / Vulkan)
    pub fn get_adapter_info(&self) -> String {
        let info = self.adapter.get_info();
        format!(
            "Adapter: {} [{:?}], Driver: {} (Vendor: 0x{:04x}, Device: 0x{:04x})",
            info.name, info.backend, info.driver_info, info.vendor, info.device
        )
    }
}

use std::sync::{Mutex, OnceLock};

pub static GLOBAL_WGPU_CONTEXT: OnceLock<Mutex<Option<WgpuEngineContext>>> = OnceLock::new();
pub static GLOBAL_LOGIN_BG: OnceLock<Mutex<Option<GpuTexture>>> = OnceLock::new();
pub static GLOBAL_THINBOARD: OnceLock<Mutex<Option<GpuTexture>>> = OnceLock::new();

pub fn set_global_wgpu_context(ctx: WgpuEngineContext) {
    let mutex = GLOBAL_WGPU_CONTEXT.get_or_init(|| Mutex::new(None));
    *mutex.lock().unwrap() = Some(ctx);
}

pub fn set_global_login_bg(tex: GpuTexture) {
    let mutex = GLOBAL_LOGIN_BG.get_or_init(|| Mutex::new(None));
    *mutex.lock().unwrap() = Some(tex);
}

pub fn set_global_thinboard(tex: GpuTexture) {
    let mutex = GLOBAL_THINBOARD.get_or_init(|| Mutex::new(None));
    *mutex.lock().unwrap() = Some(tex);
}

/// Natywna petla gry dla silnika Pure Rust - wywolywana bezposrednio lub z poziomu app.Loop()
#[no_mangle]
pub extern "C" fn m2_run_game_loop() -> i32 {
    println!("[+] m2_run_game_loop: rozpoczynanie natywnej petli renderowania GPU Pure Rust...");
    let mut frame_count: u64 = 0;
    let mut last_fps_time = std::time::Instant::now();
    let clear_color = wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };

    loop {
        let status = crate::win32_message_dispatcher::poll_window_messages();
        if let crate::win32_message_dispatcher::MsgQueueStatus::Quit(code) = status {
            println!("[+] Okno zamkniete przez uzytkownika (exit_code={})", code);
            break;
        }

        // 1. Zbieranie partii drzewa okien UI poza blokada kontekstu GPU (eliminuje deadlock)
        let ui_batches = crate::ui_window_mgr::collect_ui_render_batches(1024, 768);

        if let Some(guard) = GLOBAL_WGPU_CONTEXT.get() {
            if let Ok(mut ctx_opt) = guard.lock() {
                if let Some(ctx) = ctx_opt.as_mut() {
                    let w = ctx.width as f32;
                    let h = ctx.height as f32;

                    let mut batches: Vec<(&GpuTexture, &[Quad2D])> = Vec::new();
                    
                    // Tlo logowania (login.jpg) - renderowane TYLKO w fazach LOGIN i SELECT
                    let current_phase = crate::ui_bridge_ffi::CURRENT_PHASE.lock()
                        .map(|p| p.clone()).unwrap_or_default();
                    let show_login_bg = current_phase != "GAME";
                    
                    let bg_quad = Quad2D::with_uv(
                        0.0, 0.0, w, h,
                        0.0, 0.0, 1.0, 0.75,
                    );
                    let bg_slice_arr = [bg_quad];
                    let bg_lock = GLOBAL_LOGIN_BG.get().and_then(|m| m.lock().ok());
                    
                    if show_login_bg {
                        if let Some(ref bg_opt) = bg_lock {
                            if let Some(bg_tex) = bg_opt.as_ref() {
                                batches.push((bg_tex, &bg_slice_arr));
                            }
                        }
                    }

                    for (tex, quads) in &ui_batches {
                        batches.push((tex.as_ref(), quads.as_slice()));
                    }

                    if !batches.is_empty() {
                        let _ = ctx.render_frame_with_quads(clear_color, &batches);
                    } else {
                        let _ = ctx.render_clear_frame(clear_color);
                    }
                }
            }
        }

        frame_count += 1;
        if last_fps_time.elapsed().as_secs() >= 5 {
            let elapsed = last_fps_time.elapsed().as_secs_f64();
            let fps = frame_count as f64 / elapsed;
            println!("[+] GPU Renderer: {:.1} FPS (wyrenderowano {} klatek)", fps, frame_count);
            last_fps_time = std::time::Instant::now();
            frame_count = 0;
        }

        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    println!("[+] m2_run_game_loop zakonczona sukcesem.");
    0
}

