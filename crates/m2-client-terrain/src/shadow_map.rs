//! Modul shadow_map dla skrzynki m2-client-terrain
//! Odczyt wypieczonych map cieniowania terenu (.smf / .raw)
//! Natezenie cienia slonecznego, cieniowanie statyczne otoczenia

/// Reprezentuje bledy podczas ladowania i przetwarzania mapy cieni
#[derive(Debug, PartialEq, Eq)]
pub enum ShadowMapError {
    InvalidDataSize,
    InvalidDimensions,
}

/// Glowna struktura implementacyjna dla mapy cieni
#[derive(Debug, Clone, Default)]
pub struct ShadowMap {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u16>,
}

impl ShadowMap {
    /// Tworzy nowa instancje mapy cieni na podstawie szerokosci, wysokosci i danych bajtowych
    /// Wartosc 16-bitowa przechowuje cieniowanie sloneczne w starszym bajcie i statyczne w mlodszym
    pub fn new(width: usize, height: usize, data: Vec<u16>) -> Result<Self, ShadowMapError> {
        if width == 0 || height == 0 {
            return Err(ShadowMapError::InvalidDimensions);
        }
        if data.len() != width * height {
            return Err(ShadowMapError::InvalidDataSize);
        }
        Ok(Self {
            width,
            height,
            data,
        })
    }

    /// Pobiera natezenie cienia slonecznego na podstawie danych (starszy bajt)
    pub fn get_sun_shadow(&self, x: usize, y: usize) -> Option<u8> {
        let val = self.get_raw_value(x, y)?;
        Some((val >> 8) as u8)
    }

    /// Pobiera statyczne oswietlenie otoczenia (mlodszy bajt)
    pub fn get_static_ambient(&self, x: usize, y: usize) -> Option<u8> {
        let val = self.get_raw_value(x, y)?;
        Some((val & 0xFF) as u8)
    }

    /// Pobiera surowa wartosc z wczytanej macierzy
    pub fn get_raw_value(&self, x: usize, y: usize) -> Option<u16> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.data[y * self.width + x])
    }

    /// Pobiera interpolowane cieniowanie sloneczne uzywajac dwuliniowej interpolacji
    pub fn get_interpolated_sun_shadow(&self, x: f32, y: f32) -> Option<f32> {
        self.interpolate(x, y, |cx, cy| self.get_sun_shadow(cx, cy).map(|v| v as f32))
    }

    /// Pobiera interpolowane statyczne oswietlenie uzywajac dwuliniowej interpolacji
    pub fn get_interpolated_static_ambient(&self, x: f32, y: f32) -> Option<f32> {
        self.interpolate(x, y, |cx, cy| self.get_static_ambient(cx, cy).map(|v| v as f32))
    }

    /// Wewnetrzna funkcja do dwuliniowej interpolacji miedzy kafelkami mapy cieni
    fn interpolate<F>(&self, x: f32, y: f32, mut fetcher: F) -> Option<f32>
    where
        F: FnMut(usize, usize) -> Option<f32>,
    {
        if x < 0.0 || y < 0.0 || x >= (self.width - 1) as f32 || y >= (self.height - 1) as f32 {
            return None;
        }

        let x0 = x.floor() as usize;
        let x1 = x0 + 1;
        let y0 = y.floor() as usize;
        let y1 = y0 + 1;

        let tx = x - x0 as f32;
        let ty = y - y0 as f32;

        let v00 = fetcher(x0, y0)?;
        let v10 = fetcher(x1, y0)?;
        let v01 = fetcher(x0, y1)?;
        let v11 = fetcher(x1, y1)?;

        let top = v00 + (v10 - v00) * tx;
        let bottom = v01 + (v11 - v01) * tx;

        Some(top + (bottom - top) * ty)
    }

    /// Oblicza ostateczny wspolczynnik oswietlenia dla danego punktu z uwzglednieniem
    /// globalnego swiatla (global_ambient) i natezenia slonca (light_intensity)
    pub fn calculate_shading(&self, x: f32, y: f32, global_ambient: f32, light_intensity: f32) -> Option<f32> {
        let sun_factor = self.get_interpolated_sun_shadow(x, y)? / 255.0;
        let ambient_factor = self.get_interpolated_static_ambient(x, y)? / 255.0;

        let total = (sun_factor * light_intensity) + (ambient_factor * global_ambient);
        Some(total.clamp(0.0, 1.0))
    }
}
