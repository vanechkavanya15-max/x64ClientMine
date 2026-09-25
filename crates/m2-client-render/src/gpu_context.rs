//! Modul gpu_context dla skrzynki m2-client-render
//! Kontekst urzadzenia GPU: viewport, bufory koloru i glebi
//! GpuContext, zarzadzanie rozdzielczoscia okna, konfiguracja bufora glebi Z-Buffer

/// Format koloru
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorFormat {
    Rgba8Unorm,
    Bgra8Unorm,
    Unknown,
}

impl Default for ColorFormat {
    fn default() -> Self {
        Self::Rgba8Unorm
    }
}

/// Format bufora glebi
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthFormat {
    Depth24Plus,
    Depth32Float,
    Unknown,
}

impl Default for DepthFormat {
    fn default() -> Self {
        Self::Depth32Float
    }
}

/// Konfiguracja viewportu
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

impl Viewport {
    pub fn new(x: f32, y: f32, width: f32, height: f32, min_depth: f32, max_depth: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            min_depth,
            max_depth,
        }
    }
}

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct GpuContext {
    pub initialized: bool,
    pub resolution_width: u32,
    pub resolution_height: u32,
    pub viewport: Viewport,
    pub color_format: ColorFormat,
    pub depth_format: DepthFormat,
    pub vsync_enabled: bool,
    pub z_buffer_enabled: bool,
}

impl GpuContext {
    pub fn new() -> Self {
        Self {
            initialized: true,
            resolution_width: 800,
            resolution_height: 600,
            viewport: Viewport::new(0.0, 0.0, 800.0, 600.0, 0.0, 1.0),
            color_format: ColorFormat::default(),
            depth_format: DepthFormat::default(),
            vsync_enabled: false,
            z_buffer_enabled: true,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Zmiana rozdzielczosci okna
    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.resolution_width = width;
        self.resolution_height = height;
        // Automatyczna aktualizacja wymiarow viewportu do nowej rozdzielczosci
        self.viewport.width = width as f32;
        self.viewport.height = height as f32;
    }

    /// Pobranie aktualnej rozdzielczosci okna
    pub fn resolution(&self) -> (u32, u32) {
        (self.resolution_width, self.resolution_height)
    }

    /// Aktualizacja viewportu
    pub fn set_viewport(&mut self, viewport: Viewport) {
        self.viewport = viewport;
    }

    /// Pobranie viewportu
    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    /// Zmiana wlaczenia bufora glebi
    pub fn set_z_buffer_enabled(&mut self, enabled: bool) {
        self.z_buffer_enabled = enabled;
    }

    /// Sprawdzenie czy z-buffer jest wlaczony
    pub fn z_buffer_enabled(&self) -> bool {
        self.z_buffer_enabled
    }

    /// Ustawienie formatu bufora glebi
    pub fn set_depth_format(&mut self, format: DepthFormat) {
        self.depth_format = format;
    }

    /// Pobranie formatu bufora glebi
    pub fn depth_format(&self) -> DepthFormat {
        self.depth_format
    }

    /// Ustawienie formatu koloru
    pub fn set_color_format(&mut self, format: ColorFormat) {
        self.color_format = format;
    }

    /// Pobranie formatu koloru
    pub fn color_format(&self) -> ColorFormat {
        self.color_format
    }

    /// Zmiana wlaczenia synchronizacji pionowej (VSync)
    pub fn set_vsync(&mut self, enabled: bool) {
        self.vsync_enabled = enabled;
    }

    /// Sprawdzenie czy VSync jest wlaczony
    pub fn vsync_enabled(&self) -> bool {
        self.vsync_enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_gpu_context() {
        let obj = GpuContext::new();
        assert!(obj.is_ready());
    }

    #[test]
    fn test_initialization() {
        let ctx = GpuContext::new();
        assert_eq!(ctx.resolution(), (800, 600));
        assert_eq!(ctx.z_buffer_enabled(), true);
        assert_eq!(ctx.vsync_enabled(), false);
    }

    #[test]
    fn test_set_resolution_updates_viewport() {
        let mut ctx = GpuContext::new();
        ctx.set_resolution(1024, 768);
        assert_eq!(ctx.resolution(), (1024, 768));
        assert_eq!(ctx.viewport().width, 1024.0);
        assert_eq!(ctx.viewport().height, 768.0);
    }
}
