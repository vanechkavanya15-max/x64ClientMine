//! Modul gr2_transform dla skrzynki m2-client-granny
//! Matematyka transformacji 3D: translacja, kwaternion, skala
//! Vector3, Quaternion, Macierz 4x4, operacje mnozenia i inwersji transformacji

use std::ops::{Add, Sub, Mul};

/// Reprezentuje wektor 3D
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 1e-6 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            *self
        }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

/// Reprezentuje kwaternion
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Default for Quaternion {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn multiply(&self, other: &Self) -> Self {
        Self {
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 1e-6 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
                w: self.w / len,
            }
        } else {
            *self
        }
    }

    pub fn inverse(&self) -> Self {
        let len_sq = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
        if len_sq > 1e-6 {
            Self {
                x: -self.x / len_sq,
                y: -self.y / len_sq,
                z: -self.z / len_sq,
                w: self.w / len_sq,
            }
        } else {
            *self
        }
    }

    pub fn to_matrix(&self) -> Matrix4x4 {
        let n = self.normalize();
        let xx = n.x * n.x;
        let yy = n.y * n.y;
        let zz = n.z * n.z;
        let xy = n.x * n.y;
        let xz = n.x * n.z;
        let yz = n.y * n.z;
        let wx = n.w * n.x;
        let wy = n.w * n.y;
        let wz = n.w * n.z;

        let mut m = Matrix4x4::identity();
        m.m[0][0] = 1.0 - 2.0 * (yy + zz);
        m.m[0][1] = 2.0 * (xy + wz);
        m.m[0][2] = 2.0 * (xz - wy);

        m.m[1][0] = 2.0 * (xy - wz);
        m.m[1][1] = 1.0 - 2.0 * (xx + zz);
        m.m[1][2] = 2.0 * (yz + wx);

        m.m[2][0] = 2.0 * (xz + wy);
        m.m[2][1] = 2.0 * (yz - wx);
        m.m[2][2] = 1.0 - 2.0 * (xx + yy);

        m
    }
}

/// Reprezentuje macierz 4x4
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4x4 {
    pub m: [[f32; 4]; 4],
}

impl Default for Matrix4x4 {
    fn default() -> Self {
        Self::identity()
    }
}

impl Matrix4x4 {
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

    pub fn multiply(&self, other: &Self) -> Self {
        let mut result = Self { m: [[0.0; 4]; 4] };
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.m[i][j] += self.m[i][k] * other.m[k][j];
                }
            }
        }
        result
    }

    pub fn inverse(&self) -> Option<Self> {
        let mut inv = Self::identity();
        let mut a = self.m;

        for i in 0..4 {
            let mut pivot = i;
            for j in i + 1..4 {
                if a[j][i].abs() > a[pivot][i].abs() {
                    pivot = j;
                }
            }

            if a[pivot][i].abs() < 1e-6 {
                return None; // Macierz osobliwa
            }

            if pivot != i {
                a.swap(i, pivot);
                inv.m.swap(i, pivot);
            }

            let div = a[i][i];
            for j in 0..4 {
                a[i][j] /= div;
                inv.m[i][j] /= div;
            }

            for j in 0..4 {
                if i != j {
                    let mul = a[j][i];
                    for k in 0..4 {
                        a[j][k] -= a[i][k] * mul;
                        inv.m[j][k] -= inv.m[i][k] * mul;
                    }
                }
            }
        }

        Some(inv)
    }
}

/// Transformacja zawartosci (Translacja, Rotacja - kwaternion, Skala)
#[derive(Debug, Clone, PartialEq)]
pub struct Gr2Transform {
    pub translation: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}

impl Default for Gr2Transform {
    fn default() -> Self {
        Self {
            translation: Vector3::default(),
            rotation: Quaternion::default(),
            scale: Vector3 { x: 1.0, y: 1.0, z: 1.0 },
        }
    }
}

impl Gr2Transform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_ready(&self) -> bool {
        true
    }

    pub fn to_matrix(&self) -> Matrix4x4 {
        let mut t = Matrix4x4::identity();
        t.m[3][0] = self.translation.x;
        t.m[3][1] = self.translation.y;
        t.m[3][2] = self.translation.z;

        let mut s = Matrix4x4::identity();
        s.m[0][0] = self.scale.x;
        s.m[1][1] = self.scale.y;
        s.m[2][2] = self.scale.z;

        let r = self.rotation.to_matrix();

        // T * R * S w systemie rzedow, ale typowo stosowane w silnikach:
        // Wynik = S * R * T (w notacji kolumnowej: T * R * S dla wierzcholkow V * S * R * T)
        // Zakladajac mnozenie v' = v * M, wtedy M = S * R * T.
        s.multiply(&r).multiply(&t)
    }
}
