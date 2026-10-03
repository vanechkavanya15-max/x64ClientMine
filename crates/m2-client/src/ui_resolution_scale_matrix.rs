use glam::{Mat4, Quat, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum UiScaleMode {
    Stretch = 0,
    Fit = 1,
    Fill = 2,
}

/// Oblicza macierz transformacji 2D dopasowującą elementy UI do rozdzielczości ekranu.
/// Transformacja obejmuje skalowanie oraz centrowanie.
/// 
/// `screen_w`, `screen_h`: Szerokość i wysokość okna.
/// `base_w`, `base_h`: Bazowa szerokość i wysokość obszaru UI (np. referencyjne 800x600).
/// `mode`: Tryb skalowania (Stretch - rozciągnięcie do brzegów, Fit - dopasowanie z zachowaniem proporcji i czarnymi pasami, Fill - wypełnienie całości przy zachowaniu proporcji z ucięciem marginesów).
pub fn calculate_ui_scale_matrix(
    screen_w: u32,
    screen_h: u32,
    base_w: u32,
    base_h: u32,
    mode: UiScaleMode,
) -> Mat4 {
    if screen_w == 0 || screen_h == 0 || base_w == 0 || base_h == 0 {
        return Mat4::IDENTITY;
    }

    let screen_ratio = screen_w as f32 / screen_h as f32;
    let base_ratio = base_w as f32 / base_h as f32;

    let (scale_x, scale_y) = match mode {
        UiScaleMode::Stretch => {
            (screen_w as f32 / base_w as f32, screen_h as f32 / base_h as f32)
        }
        UiScaleMode::Fit => {
            if screen_ratio > base_ratio {
                // Ekran jest szerszy niż proporcja bazowa, dopasowujemy do wysokości.
                let s = screen_h as f32 / base_h as f32;
                (s, s)
            } else {
                // Ekran jest węższy (bądź taki sam), dopasowujemy do szerokości.
                let s = screen_w as f32 / base_w as f32;
                (s, s)
            }
        }
        UiScaleMode::Fill => {
            if screen_ratio > base_ratio {
                // Ekran jest szerszy niż proporcja bazowa, wypełniamy szerokość (ucinamy górę i dół).
                let s = screen_w as f32 / base_w as f32;
                (s, s)
            } else {
                // Ekran jest węższy (bądź taki sam), wypełniamy wysokość (ucinamy boki).
                let s = screen_h as f32 / base_h as f32;
                (s, s)
            }
        }
    };

    let scaled_w = base_w as f32 * scale_x;
    let scaled_h = base_h as f32 * scale_y;

    let offset_x = (screen_w as f32 - scaled_w) / 2.0;
    let offset_y = (screen_h as f32 - scaled_h) / 2.0;

    Mat4::from_scale_rotation_translation(
        Vec3::new(scale_x, scale_y, 1.0),
        Quat::IDENTITY,
        Vec3::new(offset_x, offset_y, 0.0),
    )
}
