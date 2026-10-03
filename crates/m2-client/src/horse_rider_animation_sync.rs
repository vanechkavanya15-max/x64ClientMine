#[repr(C)]
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

    pub fn add(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[repr(C)]
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

    pub fn identity() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }

    pub fn multiply(&self, other: &Quaternion) -> Quaternion {
        Quaternion {
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        }
    }

    pub fn rotate_vector(&self, v: &Vector3) -> Vector3 {
        let u = Vector3::new(self.x, self.y, self.z);
        let s = self.w;

        let dot_u_v = u.x * v.x + u.y * v.y + u.z * v.z;
        let dot_u_u = u.x * u.x + u.y * u.y + u.z * u.z;

        let cross_u_v = Vector3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        );

        Vector3::new(
            2.0 * dot_u_v * u.x + (s * s - dot_u_u) * v.x + 2.0 * s * cross_u_v.x,
            2.0 * dot_u_v * u.y + (s * s - dot_u_u) * v.y + 2.0 * s * cross_u_v.y,
            2.0 * dot_u_v * u.z + (s * s - dot_u_u) * v.z + 2.0 * s * cross_u_v.z,
        )
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RideState {
    Dismounted = 0,
    Riding = 1,
    Combat = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HorseBoneInfo {
    pub position: Vector3,
    pub rotation: Quaternion,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RiderTransform {
    pub position: Vector3,
    pub rotation: Quaternion,
}

/// Oblicza nową transformację (pozycję i rotację) gracza na podstawie kości grzbietu konia
/// i jego obecnego stanu jazdy. Funkcja ta synchronizuje animację jeźdźca z koniem.
pub fn calculate_rider_transform(
    horse_spine_bone: &HorseBoneInfo,
    ride_state: RideState,
    base_player_pos: &Vector3,
    base_player_rot: &Quaternion,
) -> RiderTransform {
    match ride_state {
        RideState::Dismounted => RiderTransform {
            position: *base_player_pos,
            rotation: *base_player_rot,
        },
        RideState::Riding | RideState::Combat => {
            // Różne offsety w zależności od tego, czy postać jedzie spokojnie, czy walczy.
            // Zazwyczaj w walce postać może lekko unosić się lub pochylać.
            let position_offset = if ride_state == RideState::Combat {
                Vector3::new(0.0, 10.0, 25.0) // Lekko z przodu i wyżej
            } else {
                Vector3::new(0.0, 0.0, 20.0) // Zwykły offset w osi Z (w górę względem siodła)
            };

            // Rotacja offsetu zależy od orientacji kości konia
            let rotated_offset = horse_spine_bone.rotation.rotate_vector(&position_offset);
            
            let final_position = horse_spine_bone.position.add(&rotated_offset);

            // W przypadku walki, rider może być lekko pochylony w kierunku ciosu, 
            // natomiast my tutaj po prostu bierzemy rotację kości grzbietu.
            // Można dodać też dodatkową rotację.
            let combat_rotation_offset = if ride_state == RideState::Combat {
                // Quaternion lekko pochylający postać do przodu (pitch)
                // Zakładając kąt np. 15 stopni wokół osi X.
                let half_angle: f32 = (15.0 * std::f32::consts::PI / 180.0) / 2.0;
                Quaternion::new(half_angle.sin(), 0.0, 0.0, half_angle.cos())
            } else {
                Quaternion::identity()
            };

            // Mnożymy rotację kości konia i rotację bazową gracza lub offset
            let final_rotation = horse_spine_bone.rotation.multiply(&combat_rotation_offset);

            RiderTransform {
                position: final_position,
                rotation: final_rotation,
            }
        }
    }
}
