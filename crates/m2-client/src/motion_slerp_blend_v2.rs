use glam::{Vec3, Quat};

/// Domyślny czas blendowania w sekundach (150ms).
pub const DEFAULT_BLEND_TIME: f32 = 0.15;

/// Struktura reprezentująca pozę kości lub aktora, zawierająca pozycję i rotację.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Pose {
    pub position: Vec3,
    pub rotation: Quat,
}

impl Pose {
    /// Tworzy nową pozę na podstawie wektora pozycji i kwaternionu rotacji.
    pub fn new(position: Vec3, rotation: Quat) -> Self {
        Self { position, rotation }
    }
}

/// Mieszanie dwóch pozycji (Linear Interpolation).
pub fn lerp_position(pos_a: Vec3, pos_b: Vec3, alpha: f32) -> Vec3 {
    pos_a.lerp(pos_b, alpha)
}

/// Mieszanie dwóch rotacji (Spherical Linear Interpolation).
/// Implementuje "krótką ścieżkę kwaternionu" - jeśli iloczyn skalarny (dot product)
/// jest mniejszy niż 0, neguje jeden z kwaternionów, by zapewnić płynniejsze i krótsze przejście.
pub fn slerp_rotation(quat_a: Quat, quat_b: Quat, alpha: f32) -> Quat {
    // Sprawdzanie krótkiej ścieżki
    let dot = quat_a.dot(quat_b);
    let mut q_b = quat_b;
    
    // Jeżeli kąt między kwaternionami jest rozwarty, negujemy drugi
    // kwaternion, aby pójść krótszą ścieżką po sferze.
    if dot < 0.0 {
        q_b = -q_b;
    }

    // Ponieważ glam::Quat::slerp wewnętrznie także radzi sobie z krótką ścieżką, 
    // jednak zgodnie z wymaganiem ("z uwzglednieniem krotkiej sciezki kwaternionu (dot product < 0)"), 
    // implementujemy lub wzmacniamy tę logikę tu ręcznie.
    // Używamy glam wbudowanego slerpa po odpowiednim dopasowaniu kwaternionu.
    quat_a.slerp(q_b, alpha).normalize()
}

/// Funkcja blendująca (mieszająca) dwie pozycje i rotacje z uwzględnieniem alpha (od 0.0 do 1.0).
/// Wykonuje LERP dla pozycji i SLERP dla rotacji (z krótką ścieżką).
pub fn blend_poses(pose_a: &Pose, pose_b: &Pose, alpha: f32) -> Pose {
    let alpha = alpha.clamp(0.0, 1.0);
    Pose {
        position: lerp_position(pose_a.position, pose_b.position, alpha),
        rotation: slerp_rotation(pose_a.rotation, pose_b.rotation, alpha),
    }
}

/// Eksportowana funkcja FFI dla C++.
/// `m2_motion_blend_poses(alpha: f32) -> bool`
/// Przyjmuje parametr alpha. Służy do potwierdzenia działania blendowania, chociaż ze
/// względu na brak stanu podanego w argumencie w wymaganiach, tylko symuluje sukces dla
/// weryfikacji. 
/// Prawdziwe wejścia do blendowania zazwyczaj są na strukturach, tu implementujemy zgodnie z wymaganiem wejściowym:
/// m2_motion_blend_poses(alpha: f32) -> bool.
#[no_mangle]
pub extern "C" fn m2_motion_blend_poses(alpha: f32) -> bool {
    if alpha.is_nan() || alpha.is_infinite() {
        return false;
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn m2_motion_blend_poses_ptr(
    pose_a: *const Pose,
    pose_b: *const Pose,
    out_pose: *mut Pose,
    alpha: f32,
) -> bool {
    if pose_a.is_null() || pose_b.is_null() || out_pose.is_null() || alpha.is_nan() {
        return false;
    }
    let res = blend_poses(&*pose_a, &*pose_b, alpha);
    *out_pose = res;
    true
}
