pub fn calculate_terrain_normal(
    h_left: f32,
    h_right: f32,
    h_down: f32,
    h_up: f32,
    cell_size: f32,
) -> [i8; 3] {
    let dx = (h_right - h_left) / (2.0 * cell_size);
    let dy = (h_up - h_down) / (2.0 * cell_size);

    let nx = -dx;
    let ny = -dy;
    let nz = 1.0_f32;

    let len = (nx * nx + ny * ny + nz * nz).sqrt();

    let normal_x = nx / len;
    let normal_y = ny / len;
    let normal_z = nz / len;

    let packed_x = (normal_x * 127.0).clamp(-128.0, 127.0) as i8;
    let packed_y = (normal_y * 127.0).clamp(-128.0, 127.0) as i8;
    let packed_z = (normal_z * 127.0).clamp(-128.0, 127.0) as i8;

    [packed_x, packed_y, packed_z]
}

pub fn calculate_normal_map(
    width: usize,
    height: usize,
    cell_size: f32,
    get_height: impl Fn(usize, usize) -> f32,
) -> Vec<[i8; 3]> {
    let mut normal_map = Vec::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            let h_left = if x > 0 { get_height(x - 1, y) } else { get_height(x, y) };
            let h_right = if x < width - 1 { get_height(x + 1, y) } else { get_height(x, y) };
            let h_down = if y > 0 { get_height(x, y - 1) } else { get_height(x, y) };
            let h_up = if y < height - 1 { get_height(x, y + 1) } else { get_height(x, y) };

            normal_map.push(calculate_terrain_normal(
                h_left, h_right, h_down, h_up, cell_size,
            ));
        }
    }

    normal_map
}
