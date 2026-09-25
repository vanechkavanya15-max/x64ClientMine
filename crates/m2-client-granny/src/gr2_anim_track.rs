//! Modul gr2_anim_track dla skrzynki m2-client-granny
//! Sciezki animacji: probki klatek kluczowych i krzywe
//! Klatki kluczowe rotacji (kwaterniony), translacji i skalowania w czasie

/// Wektor 3D
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Liniowa interpolacja (Lerp)
    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        Self {
            x: a.x + (b.x - a.x) * t,
            y: a.y + (b.y - a.y) * t,
            z: a.z + (b.z - a.z) * t,
        }
    }
}

/// Kwaternion reprezentujacy rotacje
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Liniowa interpolacja sferyczna ze znormalizowana sciezka (NLerp), z upewnieniem o krotszej drodze
    pub fn nlerp(mut a: Self, b: Self, t: f32) -> Self {
        // Obliczenie iloczynu skalarnego, aby sprawdzic czy podazamy "krotsza" droga
        let dot = a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w;

        if dot < 0.0 {
            // Odwracamy kwaternion a, aby isc po krotszej drodze na sferze
            a.x = -a.x;
            a.y = -a.y;
            a.z = -a.z;
            a.w = -a.w;
        }

        let mut out = Self {
            x: a.x + (b.x - a.x) * t,
            y: a.y + (b.y - a.y) * t,
            z: a.z + (b.z - a.z) * t,
            w: a.w + (b.w - a.w) * t,
        };

        let len = (out.x * out.x + out.y * out.y + out.z * out.z + out.w * out.w).sqrt();
        if len > 0.00001 {
            out.x /= len;
            out.y /= len;
            out.z /= len;
            out.w /= len;
        }

        out
    }
}

/// Klatka kluczowa dla translacji (wektor 3D) w zadanym czasie
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TranslationKeyframe {
    pub time: f32,
    pub value: Vector3,
}

/// Klatka kluczowa dla rotacji (kwaternion) w zadanym czasie
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RotationKeyframe {
    pub time: f32,
    pub value: Quaternion,
}

/// Klatka kluczowa dla skali (wektor 3D) w zadanym czasie
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScaleKeyframe {
    pub time: f32,
    pub value: Vector3,
}

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct Gr2AnimTrack {
    pub bone_name: String,
    pub translations: Vec<TranslationKeyframe>,
    pub rotations: Vec<RotationKeyframe>,
    pub scales: Vec<ScaleKeyframe>,
}

impl Gr2AnimTrack {
    pub fn new(bone_name: &str) -> Self {
        Self {
            bone_name: bone_name.to_string(),
            translations: Vec::new(),
            rotations: Vec::new(),
            scales: Vec::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        !self.translations.is_empty() || !self.rotations.is_empty() || !self.scales.is_empty()
    }

    pub fn add_translation(&mut self, time: f32, value: Vector3) {
        self.translations.push(TranslationKeyframe { time, value });
        // Zachowajmy wektor klatek posortowany po czasie
        self.translations.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }

    pub fn add_rotation(&mut self, time: f32, value: Quaternion) {
        self.rotations.push(RotationKeyframe { time, value });
        self.rotations.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }

    pub fn add_scale(&mut self, time: f32, value: Vector3) {
        self.scales.push(ScaleKeyframe { time, value });
        self.scales.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }

    /// Pobieranie interpolowanej translacji
    pub fn sample_translation(&self, time: f32) -> Option<Vector3> {
        if self.translations.is_empty() {
            return None;
        }

        if time <= self.translations.first().unwrap().time {
            return Some(self.translations.first().unwrap().value);
        }

        if time >= self.translations.last().unwrap().time {
            return Some(self.translations.last().unwrap().value);
        }

        for i in 0..self.translations.len() - 1 {
            let k1 = &self.translations[i];
            let k2 = &self.translations[i + 1];

            if time >= k1.time && time <= k2.time {
                let t = (time - k1.time) / (k2.time - k1.time);
                return Some(Vector3::lerp(k1.value, k2.value, t));
            }
        }

        None
    }

    /// Pobieranie interpolowanej rotacji
    pub fn sample_rotation(&self, time: f32) -> Option<Quaternion> {
        if self.rotations.is_empty() {
            return None;
        }

        if time <= self.rotations.first().unwrap().time {
            return Some(self.rotations.first().unwrap().value);
        }

        if time >= self.rotations.last().unwrap().time {
            return Some(self.rotations.last().unwrap().value);
        }

        for i in 0..self.rotations.len() - 1 {
            let k1 = &self.rotations[i];
            let k2 = &self.rotations[i + 1];

            if time >= k1.time && time <= k2.time {
                let t = (time - k1.time) / (k2.time - k1.time);
                return Some(Quaternion::nlerp(k1.value, k2.value, t));
            }
        }

        None
    }

    /// Pobieranie interpolowanej skali
    pub fn sample_scale(&self, time: f32) -> Option<Vector3> {
        if self.scales.is_empty() {
            return None;
        }

        if time <= self.scales.first().unwrap().time {
            return Some(self.scales.first().unwrap().value);
        }

        if time >= self.scales.last().unwrap().time {
            return Some(self.scales.last().unwrap().value);
        }

        for i in 0..self.scales.len() - 1 {
            let k1 = &self.scales[i];
            let k2 = &self.scales[i + 1];

            if time >= k1.time && time <= k2.time {
                let t = (time - k1.time) / (k2.time - k1.time);
                return Some(Vector3::lerp(k1.value, k2.value, t));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_gr2_anim_track() {
        let mut obj = Gr2AnimTrack::new("Bip01");
        assert!(!obj.is_ready());
        obj.add_translation(0.0, Vector3::new(0.0, 0.0, 0.0));
        assert!(obj.is_ready());
    }
}
