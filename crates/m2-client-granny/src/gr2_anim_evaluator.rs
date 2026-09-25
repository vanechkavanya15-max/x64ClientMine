//! Modul gr2_anim_evaluator dla skrzynki m2-client-granny
//! Ewaluator animacji szkieletowej w czasie T z interpolacja SLERP
//! Interpolacja kwaternionow SLERP, translacji LERP, obliczanie macierzy kosci w klatce

/// Wektor 3D do translacji
#[derive(Debug, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Interpolacja liniowa (LERP) dla translacji
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            x: self.x + t * (other.x - self.x),
            y: self.y + t * (other.y - self.y),
            z: self.z + t * (other.z - self.z),
        }
    }
}

/// Kwaternion do rotacji
#[derive(Debug, Clone, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Default for Quat {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }
}

impl Quat {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn normalize(&mut self) {
        let len_sq = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
        if len_sq > 0.0 {
            let len = len_sq.sqrt();
            self.x /= len;
            self.y /= len;
            self.z /= len;
            self.w /= len;
        } else {
            self.x = 0.0;
            self.y = 0.0;
            self.z = 0.0;
            self.w = 1.0;
        }
    }

    /// Sferyczna interpolacja liniowa (SLERP) dla rotacji/kwaternionow
    pub fn slerp(&self, other: &Self, t: f32) -> Self {
        let mut dot = self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;
        let mut other_q = other.clone();

        // Krotka sciezka (short path)
        if dot < 0.0 {
            other_q.x = -other_q.x;
            other_q.y = -other_q.y;
            other_q.z = -other_q.z;
            other_q.w = -other_q.w;
            dot = -dot;
        }

        const DOT_THRESHOLD: f32 = 0.9995;
        if dot > DOT_THRESHOLD {
            // Liniowa interpolacja dla bliskich wartosci
            let mut result = Self {
                x: self.x + t * (other_q.x - self.x),
                y: self.y + t * (other_q.y - self.y),
                z: self.z + t * (other_q.z - self.z),
                w: self.w + t * (other_q.w - self.w),
            };
            result.normalize();
            return result;
        }

        let theta_0 = dot.acos();
        let theta = theta_0 * t;
        let sin_theta = theta.sin();
        let sin_theta_0 = theta_0.sin();

        let s0 = (theta_0 - theta).sin() / sin_theta_0;
        let s1 = sin_theta / sin_theta_0;

        Self {
            x: (self.x * s0) + (other_q.x * s1),
            y: (self.y * s0) + (other_q.y * s1),
            z: (self.z * s0) + (other_q.z * s1),
            w: (self.w * s0) + (other_q.w * s1),
        }
    }
}

/// Macierz 4x4
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix4 {
    pub m: [[f32; 4]; 4],
}

impl Default for Matrix4 {
    fn default() -> Self {
        Self::identity()
    }
}

impl Matrix4 {
    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Obliczanie macierzy z translacji i rotacji
    pub fn from_translation_rotation(t: &Vec3, q: &Quat) -> Self {
        let mut mat = Self::identity();

        let xx = q.x * q.x;
        let xy = q.x * q.y;
        let xz = q.x * q.z;
        let xw = q.x * q.w;

        let yy = q.y * q.y;
        let yz = q.y * q.z;
        let yw = q.y * q.w;

        let zz = q.z * q.z;
        let zw = q.z * q.w;

        mat.m[0][0] = 1.0 - 2.0 * (yy + zz);
        mat.m[0][1] = 2.0 * (xy - zw);
        mat.m[0][2] = 2.0 * (xz + yw);
        mat.m[0][3] = 0.0;

        mat.m[1][0] = 2.0 * (xy + zw);
        mat.m[1][1] = 1.0 - 2.0 * (xx + zz);
        mat.m[1][2] = 2.0 * (yz - xw);
        mat.m[1][3] = 0.0;

        mat.m[2][0] = 2.0 * (xz - yw);
        mat.m[2][1] = 2.0 * (yz + xw);
        mat.m[2][2] = 1.0 - 2.0 * (xx + yy);
        mat.m[2][3] = 0.0;

        mat.m[3][0] = t.x;
        mat.m[3][1] = t.y;
        mat.m[3][2] = t.z;
        mat.m[3][3] = 1.0;

        mat
    }

    /// Mnozenie dwoch macierzy (self * other)
    pub fn mul(&self, other: &Self) -> Self {
        let mut res = Self::default();
        for i in 0..4 {
            for j in 0..4 {
                res.m[i][j] = self.m[i][0] * other.m[0][j]
                            + self.m[i][1] * other.m[1][j]
                            + self.m[i][2] * other.m[2][j]
                            + self.m[i][3] * other.m[3][j];
            }
        }
        res
    }
}

/// Transformacja kosci
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoneTransform {
    pub translation: Vec3,
    pub rotation: Quat,
}

impl BoneTransform {
    pub fn new(translation: Vec3, rotation: Quat) -> Self {
        Self { translation, rotation }
    }
}

/// Klatka kluczowa animacji
#[derive(Debug, Clone, PartialEq)]
pub struct Keyframe {
    pub time: f32,
    pub transform: BoneTransform,
}

impl Keyframe {
    pub fn new(time: f32, transform: BoneTransform) -> Self {
        Self { time, transform }
    }
}

/// Glowna struktura implementacyjna - ewaluator
#[derive(Debug, Clone, Default)]
pub struct Gr2AnimEvaluator {
    pub initialized: bool,
    pub keyframes: Vec<Keyframe>,
}

impl Gr2AnimEvaluator {
    pub fn new() -> Self {
        Self {
            initialized: true,
            keyframes: Vec::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn add_keyframe(&mut self, keyframe: Keyframe) {
        self.keyframes.push(keyframe);
    }

    /// Obliczanie transformacji (interpolacja) w czasie T
    pub fn evaluate_transform(&self, t: f32) -> BoneTransform {
        if self.keyframes.is_empty() {
            return BoneTransform::default();
        }

        if self.keyframes.len() == 1 || t <= self.keyframes[0].time {
            return self.keyframes[0].transform.clone();
        }

        let last_idx = self.keyframes.len() - 1;
        if t >= self.keyframes[last_idx].time {
            return self.keyframes[last_idx].transform.clone();
        }

        // Znajdz klatki do interpolacji
        for i in 0..last_idx {
            let k0 = &self.keyframes[i];
            let k1 = &self.keyframes[i + 1];

            if t >= k0.time && t < k1.time {
                let dt = k1.time - k0.time;
                let factor = (t - k0.time) / dt;

                return BoneTransform {
                    translation: k0.transform.translation.lerp(&k1.transform.translation, factor),
                    rotation: k0.transform.rotation.slerp(&k1.transform.rotation, factor),
                };
            }
        }

        self.keyframes[last_idx].transform.clone()
    }

    /// Obliczanie macierzy kosci w klatce na podstawie ewaluowanej transformacji
    pub fn evaluate_matrix(&self, t: f32) -> Matrix4 {
        let transform = self.evaluate_transform(t);
        Matrix4::from_translation_rotation(&transform.translation, &transform.rotation)
    }
}
