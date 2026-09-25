//! Modul render_target dla skrzynki m2-client-render
//! Deskryptor celow renderowania (Render Targets) i antyaliasingu
//! Bufor koloru, bufor glebi/szablonu (Depth-Stencil), konfiguracja MSAA

/// Dostepne formaty tekstur uzywane w celach renderowania.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    /// Standardowy format koloru (32-bit RGBA).
    Rgba8Unorm,
    /// Standardowy format koloru z przestrzenia sRGB.
    Rgba8UnormSrgb,
    /// Format bufora glebi (32-bit zmiennoprzecinkowy).
    Depth32Float,
    /// Format bufora glebi i szablonu (24-bit glebia, 8-bit szablon).
    Depth24PlusStencil8,
    /// Brak formatu.
    None,
}

/// Deskryptor konfiguracji celu renderowania.
#[derive(Debug, Clone, Copy)]
pub struct RenderTargetDescriptor {
    /// Szerokosc w pikselach.
    pub width: u32,
    /// Wysokosc w pikselach.
    pub height: u32,
    /// Format tekstury koloru.
    pub color_format: TextureFormat,
    /// Format tekstury glebi/szablonu.
    pub depth_stencil_format: TextureFormat,
    /// Liczba probek dla antyaliasingu MSAA (1 oznacza wylaczone MSAA).
    pub sample_count: u32,
}

impl Default for RenderTargetDescriptor {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            color_format: TextureFormat::Rgba8Unorm,
            depth_stencil_format: TextureFormat::Depth24PlusStencil8,
            sample_count: 1,
        }
    }
}

/// Struktura reprezentujaca cel renderowania.
#[derive(Debug, Clone)]
pub struct RenderTarget {
    /// Deskryptor z konfiguracja.
    pub descriptor: RenderTargetDescriptor,
    /// Flaga inicjalizacji.
    pub initialized: bool,
}

impl Default for RenderTarget {
    fn default() -> Self {
        Self::new(RenderTargetDescriptor::default())
    }
}

impl RenderTarget {
    /// Tworzy nowy cel renderowania na podstawie deskryptora.
    pub fn new(descriptor: RenderTargetDescriptor) -> Self {
        Self {
            descriptor,
            initialized: true,
        }
    }

    /// Sprawdza, czy cel renderowania jest gotowy.
    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Zmienia rozmiar celu renderowania.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.descriptor.width = width;
            self.descriptor.height = height;
        }
    }

    /// Ustawia liczbe probek dla MSAA.
    pub fn set_sample_count(&mut self, sample_count: u32) {
        if sample_count > 0 {
            self.descriptor.sample_count = sample_count;
        }
    }

    /// Zwraca informacje, czy cel renderowania posiada bufor glebi/szablonu.
    pub fn has_depth_stencil(&self) -> bool {
        self.descriptor.depth_stencil_format != TextureFormat::None
    }

    /// Zwraca informacje, czy MSAA jest wlaczone (liczba probek > 1).
    pub fn is_msaa_enabled(&self) -> bool {
        self.descriptor.sample_count > 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_render_target() {
        let obj = RenderTarget::default();
        assert!(obj.is_ready());
    }
}
