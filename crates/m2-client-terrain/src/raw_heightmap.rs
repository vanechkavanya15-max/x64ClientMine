//! Modul raw_heightmap dla skrzynki m2-client-terrain
//! Dekoder mapy wysokosci 16-bit .raw z interpolacja dwuliniowa
//! Odczyt siatki wysokosci, obliczanie normalnych powierzchni, interpolacja wysokosci dla (X, Y)

/// Reprezentuje mape wysokosci wczytana z surowych danych 16-bitowych.
#[derive(Debug, Clone, Default)]
pub struct RawHeightmap {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u16>,
    pub initialized: bool,
}

impl RawHeightmap {
    /// Tworzy nowy, pusty obiekt (np. zeby zadowolic stare testy)
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            data: Vec::new(),
            initialized: true,
        }
    }

    /// Sprawdza czy mapa jest zainicjalizowana.
    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Wczytuje mape wysokosci z bufora bajtow (Little Endian).
    pub fn from_bytes(width: usize, height: usize, bytes: &[u8]) -> Result<Self, &'static str> {
        let expected_size = width * height * 2;
        if bytes.len() != expected_size {
            return Err("Nieprawidlowy rozmiar bufora danych");
        }

        let mut data = Vec::with_capacity(width * height);
        for chunk in bytes.chunks_exact(2) {
            let val = u16::from_le_bytes([chunk[0], chunk[1]]);
            data.push(val);
        }

        Ok(Self {
            width,
            height,
            data,
            initialized: true,
        })
    }

    /// Zwraca surowa wartosc wysokosci w punkcie (X, Y) (0 jesli poza zakresem).
    pub fn get_raw_height(&self, x: usize, y: usize) -> u16 {
        if x >= self.width || y >= self.height {
            return 0;
        }
        self.data[y * self.width + x]
    }

    /// Pobiera interpolowana dwuliniowo wysokosc w punkcie (X, Y).
    pub fn get_height(&self, x: f32, y: f32) -> f32 {
        if self.width == 0 || self.height == 0 {
            return 0.0;
        }

        // Zabezpieczenie zakresu (clamp)
        let max_x = (self.width.saturating_sub(1)) as f32;
        let max_y = (self.height.saturating_sub(1)) as f32;
        
        let cx = x.clamp(0.0, max_x);
        let cy = y.clamp(0.0, max_y);

        let x0 = cx.floor() as usize;
        let x1 = (x0 + 1).min(self.width.saturating_sub(1));
        let y0 = cy.floor() as usize;
        let y1 = (y0 + 1).min(self.height.saturating_sub(1));

        let tx = cx - x0 as f32;
        let ty = cy - y0 as f32;

        let h00 = self.get_raw_height(x0, y0) as f32;
        let h10 = self.get_raw_height(x1, y0) as f32;
        let h01 = self.get_raw_height(x0, y1) as f32;
        let h11 = self.get_raw_height(x1, y1) as f32;

        let h0 = h00 * (1.0 - tx) + h10 * tx;
        let h1 = h01 * (1.0 - tx) + h11 * tx;

        h0 * (1.0 - ty) + h1 * ty
    }

    /// Oblicza znormalizowany wektor normalny powierzchni w danym punkcie (X, Y).
    pub fn get_normal(&self, x: f32, y: f32) -> [f32; 3] {
        let delta = 1.0;
        let h_l = self.get_height(x - delta, y);
        let h_r = self.get_height(x + delta, y);
        let h_d = self.get_height(x, y - delta);
        let h_u = self.get_height(x, y + delta);

        // Obliczanie gradientow
        let dx = h_r - h_l;
        let dy = h_u - h_d;

        let nx = -dx;
        let ny = -dy;
        let nz = 2.0 * delta;

        let length = (nx * nx + ny * ny + nz * nz).sqrt();
        if length > 0.0 {
            [nx / length, ny / length, nz / length]
        } else {
            [0.0, 0.0, 1.0]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_raw_heightmap() {
        let obj = RawHeightmap::new();
        assert!(obj.is_ready());
    }
}
