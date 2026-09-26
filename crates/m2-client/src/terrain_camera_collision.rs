pub const MIN_CAMERA_GROUND_CLEARANCE: f32 = 50.0;

/// Zwraca nowa pozycje oczka kamery (eye_pos) po uwzglednieniu kolizji z terenem.
///
/// Logika:
/// 1. Pobiera `ground_z` przy uzyciu `get_terrain_height(eye_pos.0, eye_pos.1)`.
/// 2. Jesli `eye_pos.2 < ground_z + min_distance_from_ground`:
///    - Podnosi `eye_pos.2 = ground_z + min_distance_from_ground`.
///    - Nastepnie koryguje odleglosc (zoom in), zachowujac (lub poprawiajac w razie stromego
///      nachylenia) wektor patrzenia, w szczegolnosci pilnujac by kamera nie patrzyla
///      "do gory nogami" lub w zlym kierunku w stosunku do celu, a jedynie ewentualnie
///      przyblizyla sie do celu.
pub fn resolve_terrain_camera_collision<F>(
    target_pos: (f32, f32, f32),
    eye_pos: (f32, f32, f32),
    min_distance_from_ground: f32,
    get_terrain_height: F,
) -> (f32, f32, f32)
where
    F: Fn(f32, f32) -> f32,
{
    let ground_z = get_terrain_height(eye_pos.0, eye_pos.1);
    let target_z_min = ground_z + min_distance_from_ground;

    if eye_pos.2 >= target_z_min {
        return eye_pos;
    }

    // Nastapila kolizja (kamera zeszla ponizej ground_z + margin).
    let mut new_eye_x = eye_pos.0;
    let mut new_eye_y = eye_pos.1;
    let new_eye_z = target_z_min;

    // Poniewaz zmienilismy Z, obliczmy nowy wektor od celu do kamery na plaszczyznie XY i Z.
    let dx = new_eye_x - target_pos.0;
    let dy = new_eye_y - target_pos.1;
    let dz = new_eye_z - target_pos.2;

    // Obliczmy nowa dlugosc 2D i stary rzut 3D
    let dist2d_sq = dx * dx + dy * dy;
    let dist2d = dist2d_sq.sqrt();

    // Jesli kamera jest podniesiona, kat nachylenia (pitch) ulega zmianie.
    // Zeby to zachowalo sie jak "zoom in" na stromym zboczu, powinnismy sprawdzic czy po podniesieniu 
    // pitch nie wykracza poza ustalony margines.
    // Oryginalny Metin2 zapobiega przekroczeniu ~80 stopni.
    // Zatem pitch_limit = tan(80 stopni) to ok. 5.67. 
    // dz / dist2d to tan(pitch).
    
    // Sprawdzmy, czy jest to "zbyt strome" nachylenie.
    // Limit nachylenia = 80 stopni. tan(80) = ~5.671
    let max_tan_pitch = 5.671_f32;

    if dist2d > 0.001 {
        let pitch_tan = dz.abs() / dist2d;
        if pitch_tan > max_tan_pitch {
            // Jesli nachylenie jest zbyt strome (pitch_tan > max_tan_pitch),
            // skracamy dystans orbitalny (przybliza widok - zoom in).
            // Uzywamy odwrotnosci proporcji, aby scale bylo < 1.0.
            let scale = max_tan_pitch / pitch_tan; 
            
            new_eye_x = target_pos.0 + dx * scale;
            new_eye_y = target_pos.1 + dy * scale;
        }
    } else {
        // Skrajny przypadek - kamera idealnie nad targetem w osi Z.
        // X, Y zostaja takie same, Z podniesione do target_z_min.
        new_eye_x = target_pos.0;
        new_eye_y = target_pos.1;
    }

    (new_eye_x, new_eye_y, new_eye_z)
}
