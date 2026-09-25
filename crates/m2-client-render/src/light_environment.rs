//! Modul light_environment dla skrzynki m2-client-render
//! Model oswietlenia slonecznego, barwy otoczenia i mgly
//! Wektor slonca, kolor swiatla kierunkowego, kolor ambient, parametry liniowej mgly

/// Wektor w przestrzeni 3D
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    /// Tworzy nowy wektor
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Normalizuje wektor (oblicza wektor o dlugosci 1)
    pub fn normalize(&mut self) {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if length > 0.0 {
            self.x /= length;
            self.y /= length;
            self.z /= length;
        }
    }
}

/// Reprezentacja koloru RGBA
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Tworzy nowy kolor
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

/// Parametry mgly liniowej
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct FogParameters {
    pub enabled: bool,
    pub color: Color,
    pub near: f32,
    pub far: f32,
}

impl FogParameters {
    /// Tworzy nowe parametry mgly
    pub fn new(enabled: bool, color: Color, near: f32, far: f32) -> Self {
        Self {
            enabled,
            color,
            near,
            far,
        }
    }
}

/// Glowna struktura implementacyjna srodowiska oswietlenia
#[derive(Debug, Clone, Default)]
pub struct LightEnvironment {
    pub sun_vector: Vector3,
    pub directional_light_color: Color,
    pub ambient_color: Color,
    pub fog_parameters: FogParameters,
    pub initialized: bool,
}

impl LightEnvironment {
    /// Inicjuje srodowisko oswietlenia domyslnymi wartosciami
    pub fn new() -> Self {
        Self {
            sun_vector: Vector3::default(),
            directional_light_color: Color::default(),
            ambient_color: Color::default(),
            fog_parameters: FogParameters::default(),
            initialized: true,
        }
    }

    /// Sprawdza, czy srodowisko oswietlenia jest gotowe do uzycia
    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Ustawia wektor slonca i automatycznie go normalizuje
    pub fn set_sun_vector(&mut self, mut vector: Vector3) {
        vector.normalize();
        self.sun_vector = vector;
    }

    /// Ustawia kolor swiatla kierunkowego
    pub fn set_directional_light_color(&mut self, color: Color) {
        self.directional_light_color = color;
    }

    /// Ustawia kolor otoczenia (ambient)
    pub fn set_ambient_color(&mut self, color: Color) {
        self.ambient_color = color;
    }

    /// Ustawia parametry mgly
    pub fn set_fog_parameters(&mut self, fog_parameters: FogParameters) {
        self.fog_parameters = fog_parameters;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_light_environment() {
        let obj = LightEnvironment::new();
        assert!(obj.is_ready());
    }

    #[test]
    fn test_vector3_normalize() {
        let mut v = Vector3::new(3.0, 0.0, 4.0);
        v.normalize();
        assert!((v.x - 0.6).abs() < 1e-5);
        assert!((v.y - 0.0).abs() < 1e-5);
        assert!((v.z - 0.8).abs() < 1e-5);
    }
}
