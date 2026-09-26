/// Implementacja algorytmu maskowania przezroczystosci ColorKey
/// np. dla starych grafik Metin2 (ikony .bmp, .tga bez kanalu alpha)
/// uzywajacych koloru fioletowego (Magenta RGB(255, 0, 255) / 0xFF00FF)
/// jako oznaczenia przezroczystosci.

/// Aplikuje klucz koloru z ustalonym progiem tolerancji (0 to brak tolerancji).
/// Jesli piksel trafia w zdefiniowany przedzial koloru z uwzglednieniem progu,
/// wtedy jego wartosc Alpha bedzie ustawiona na 0 (przezroczystosc).
/// W przeciwnym razie otrzyma wartosc Alpha na 255 (pelen widok).
pub fn apply_color_key_tolerance(rgba_buf: &mut [u8], key_r: u8, key_g: u8, key_b: u8, tolerance: u8) {
    for pixel in rgba_buf.chunks_exact_mut(4) {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];

        let r_diff = r.abs_diff(key_r);
        let g_diff = g.abs_diff(key_g);
        let b_diff = b.abs_diff(key_b);

        if r_diff <= tolerance && g_diff <= tolerance && b_diff <= tolerance {
            pixel[3] = 0;
        } else {
            pixel[3] = 255;
        }
    }
}

/// Aplikuje klucz koloru na przezroczystosc dokladnie bez tolerancji.
/// Ustawia 0 (przezroczystosc) w Alpha dla odpowiedniego koloru 
/// oraz 255 dla reszty.
pub fn apply_color_key(rgba_buf: &mut [u8], key_r: u8, key_g: u8, key_b: u8) {
    apply_color_key_tolerance(rgba_buf, key_r, key_g, key_b, 0);
}
