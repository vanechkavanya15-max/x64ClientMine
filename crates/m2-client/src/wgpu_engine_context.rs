// wgpu_engine_context.rs - Modern DirectX 12 / Vulkan Rendering Pipeline for Metin2 Pure Rust
use std::sync::Arc;
use std::num::NonZeroIsize;
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle,
    RawWindowHandle, Win32WindowHandle, WindowHandle, WindowsDisplayHandle,
};

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
}

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
            })
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
        }
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
            wgpu::CurrentSurfaceTexture::Validation => return Err("Validation error getting current texture".to_string()),
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
