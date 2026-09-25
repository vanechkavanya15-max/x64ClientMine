//! Modul frustum_culler dla skrzynki m2-client-render
//! Wycinanie ostroslupa widzenia (Frustum Culling) dla bryl AABB i kul
//! Wyznaczanie 6 plaszczyzn frustum z macierzy VP, testy widocznosci bryl otaczajacych

/// Struktura reprezentujaca plaszczyzne w 3D
#[derive(Debug, Clone, Copy, Default)]
pub struct Plane {
    pub normal: [f32; 3],
    pub distance: f32,
}

impl Plane {
    /// Normalizuje wektor normalny i dystans
    pub fn normalize(&mut self) {
        let len = (self.normal[0] * self.normal[0] + self.normal[1] * self.normal[1] + self.normal[2] * self.normal[2]).sqrt();
        if len > 0.0 {
            self.normal[0] /= len;
            self.normal[1] /= len;
            self.normal[2] /= len;
            self.distance /= len;
        }
    }

    /// Oblicza odleglosc punktu od plaszczyzny
    pub fn distance_to_point(&self, p: &[f32; 3]) -> f32 {
        self.normal[0] * p[0] + self.normal[1] * p[1] + self.normal[2] * p[2] + self.distance
    }
}

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct FrustumCuller {
    pub initialized: bool,
    pub planes: [Plane; 6],
}

impl FrustumCuller {
    pub fn new() -> Self {
        Self {
            initialized: true,
            planes: [Plane::default(); 6],
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Aktualizuje plaszczyzny na podstawie macierzy View-Projection w ukladzie wierszowym.
    pub fn update(&mut self, m: &[f32; 16]) {
        // Lewa
        self.planes[0] = Plane {
            normal: [m[12] + m[0], m[13] + m[1], m[14] + m[2]],
            distance: m[15] + m[3],
        };
        // Prawa
        self.planes[1] = Plane {
            normal: [m[12] - m[0], m[13] - m[1], m[14] - m[2]],
            distance: m[15] - m[3],
        };
        // Dolna
        self.planes[2] = Plane {
            normal: [m[12] + m[4], m[13] + m[5], m[14] + m[6]],
            distance: m[15] + m[7],
        };
        // Gorna
        self.planes[3] = Plane {
            normal: [m[12] - m[4], m[13] - m[5], m[14] - m[6]],
            distance: m[15] - m[7],
        };
        // Bliska
        self.planes[4] = Plane {
            normal: [m[12] + m[8], m[13] + m[9], m[14] + m[10]],
            distance: m[15] + m[11],
        };
        // Daleka
        self.planes[5] = Plane {
            normal: [m[12] - m[8], m[13] - m[9], m[14] - m[10]],
            distance: m[15] - m[11],
        };

        for plane in self.planes.iter_mut() {
            plane.normalize();
        }
    }

    /// Testuje, czy punkt znajduje sie wewnatrz frustum.
    pub fn test_point(&self, point: &[f32; 3]) -> bool {
        for plane in &self.planes {
            if plane.distance_to_point(point) < 0.0 {
                return false;
            }
        }
        true
    }

    /// Testuje, czy kula (srodek, promien) znajduje sie wewnatrz lub przecina frustum.
    pub fn test_sphere(&self, center: &[f32; 3], radius: f32) -> bool {
        for plane in &self.planes {
            if plane.distance_to_point(center) < -radius {
                return false;
            }
        }
        true
    }

    /// Testuje, czy bryla AABB (min, max) znajduje sie wewnatrz lub przecina frustum.
    pub fn test_aabb(&self, min: &[f32; 3], max: &[f32; 3]) -> bool {
        for plane in &self.planes {
            let px = if plane.normal[0] > 0.0 { max[0] } else { min[0] };
            let py = if plane.normal[1] > 0.0 { max[1] } else { min[1] };
            let pz = if plane.normal[2] > 0.0 { max[2] } else { min[2] };

            if plane.distance_to_point(&[px, py, pz]) < 0.0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_frustum_culler() {
        let obj = FrustumCuller::new();
        assert!(obj.is_ready());
    }
}
